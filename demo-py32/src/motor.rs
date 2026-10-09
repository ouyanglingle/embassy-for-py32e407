//!┌─────────────────────────────────────┐
//!│   kinematics.rs                     │  高层：机器人速度 → 左右轮 RPM
//!│   DifferentialChassis               │
//!│   set_velocity(v, omega)            │
//!├─────────────────────────────────────┤
//!│   motor.rs                          │  中层：单个电机速度闭环
//!│   MotorController<Tpwm, Tenc>       │
//!│   ├── set_linear_velocity(f32)      │
//!│   └── update()                      │  速度环 PID → PWM（双通道 H 桥）
//!├─────────────────────────────────────┤
//!│   pid.rs                            │  底层：PID 控制器
//!│   IncPid / Pid                      │
//!└─────────────────────────────────────┘

// eg: 电机减速比为1:118，编码器PPR为7, 四倍频，所以118x7x4 = 3304PPR/RPM
use defmt::{error, info};

use embassy_py32::timer::pwm::{Channel, Pwm, PwmInstance};
use embassy_py32::timer::qei::{EncoderInstance, Qei};
use embassy_py32::timer::Error;

use crate::pid::simple_pid::Pid;

/// 单个直流电机速度闭环控制器（embassy-py32 版）。
///
/// 通过**同一定时器的两个 PWM 通道**驱动 H 桥（AT8870 / AT8236 的 IN1/IN2 接法）：
/// 输出为正时占空比走 `ch_pos`，为负时走 `ch_neg`，实现调速 + 换向。
/// 使用编码器反馈做速度闭环，内部集成 PID 控制器，自动调节占空比达到目标转速。
///
/// 与 embassy-stm32 版的差异（移植要点）：
/// - embassy-py32 的 `pwm::PwmChannel` 是 `&mut Pwm` 的借用视图，两个通道无法同时
///   借用同一个 `Pwm`，因此本结构**直接持有 `Pwm`**，用通道号区分正/反转；
/// - `set_duty(u32)` 会返回 `Result`（embassy-stm32 是不返回错误的固有方法）；
/// - 编码器 bound 为 `EncoderInstance`（TIM1/2/3/4/5/8/18，其中 TIM2 为 32 位计数器）。
///
/// ### 泛型参数
/// - `Tpwm`: PWM 定时器实例（`PwmInstance`），两个换向通道必须在**同一个定时器**上
/// - `Tenc`: 编码器定时器实例（`EncoderInstance`）
/// - `P`: PID 控制器（[`Pid`] trait 的任一实现）
/// - `PPR`: 减速器输出轴每转一圈，编码器的脉冲数，编译时常量
/// - `WHEEL_DIA_MM`: 轮子直径，单位：毫米，编译时常量
///
/// ### 示例
/// ```ignore
/// // 减速比 1:118，编码器 PPR=7，四倍频 → 3304 PPR/转
/// const MOTOR_PPR: u16 = 118 * 7 * 4;
/// let mut motor = MotorController::new(qei, pwm, Channel::Ch1, Channel::Ch2, pid)?;
/// motor.init();
/// ```
pub struct MotorController<'d, Tpwm, Tenc, P, const PPR: u16, const WHEEL_DIA_MM: u16>
where
    Tpwm: PwmInstance,
    Tenc: EncoderInstance,
    P: Pid,
{
    enc: Qei<'d, Tenc>,
    pwm: Pwm<'d, Tpwm>,
    /// 正转通道（PID 输出 > 0 时的占空比输出）。
    ch_pos: Channel,
    /// 反转通道（PID 输出 < 0 时的占空比输出）。
    ch_neg: Channel,
    pub pid: P,
    delta_count: i16,
    vel_mm_s: f32,
}

impl<'d, Tpwm, Tenc, P, const PPR: u16, const WHEEL_DIA_MM: u16>
    MotorController<'d, Tpwm, Tenc, P, PPR, WHEEL_DIA_MM>
where
    Tpwm: PwmInstance,
    Tenc: EncoderInstance,
    P: Pid,
{
    /// 创建电机控制器。
    ///
    /// # Errors
    ///
    /// 两个换向通道的通道号超出该 PWM 实例的通道数时返回
    /// [`Error::InvalidChannel`]——把配置错误挡在构造期，而不是运行中的速度环里。
    pub fn new(
        enc: Qei<'d, Tenc>,
        pwm: Pwm<'d, Tpwm>,
        ch_pos: Channel,
        ch_neg: Channel,
        pid: P,
    ) -> Result<Self, Error> {
        let n = pwm.channels() as usize;
        if ch_pos.index() >= n || ch_neg.index() >= n {
            return Err(Error::InvalidChannel);
        }
        Ok(Self {
            enc,
            pwm,
            ch_pos,
            ch_neg,
            pid,
            delta_count: 0,
            vel_mm_s: 0.0,
        })
    }

    /// 对齐 PID 输出限幅与 PWM 满量程。
    ///
    /// 若 PID 输出上限大于占空比满量程（`ARR`），积分会顶着限幅饱和，恢复时
    /// 占空比却上不去——把输出限幅压到 `max_duty_cycle - 1`，保证 PID 输出
    /// 永远落在 `set_duty` 的合法区间内。
    pub fn init(&mut self) {
        let max_duty = self.pwm.max_duty_cycle() as f32;
        if self.pid.max_out() > max_duty {
            info!(
                "PID max_out {} > max_duty_cycle {}",
                self.pid.max_out(),
                max_duty
            );
            self.pid.set_output_limit(max_duty - 1.0);
        }
    }

    /// 计算当前线速度（单位：mm/秒）。
    fn linear_velocity(&mut self, dt: f32) -> f32 {
        // count() 返回 u32（TIM2 为 32 位计数器），先截到 u16 再按补码解释为
        // 有符号增量，与 16 位实例（ARR = 0xFFFF）语义一致。
        // 隐含假设：单个 dt 内计数增量 < 32768（3304 PPR 时约 9.9 转/s）。
        self.delta_count = (self.enc.count() as u16) as i16;
        self.enc.reset();
        // 转换为转速（转/秒）
        let rps = self.delta_count as f32 / (PPR as f32) / dt;
        // 轮子周长（mm）= π * 直径，线速度 = 周长 * 转速
        rps * core::f32::consts::PI * WHEEL_DIA_MM as f32
    }

    /// 设置目标线速度（单位：mm/秒）。
    pub fn set_linear_velocity(&mut self, velocity: f32) {
        self.pid.set_setpoint(velocity);
    }

    /// 当前实测线速度（单位：mm/秒，上次 `update()` 的结果）。
    pub fn linear_velocity_mm_s(&self) -> f32 {
        self.vel_mm_s
    }

    /// 设置某通道占空比。
    ///
    /// 通道号已在构造期校验，运行期若仍出错（理论不可达）只记日志不 panic
    /// ——速度环里 panic 会让机器人直接失控。
    fn set_duty(&mut self, ch: Channel, duty: u32) {
        if let Err(_e) = self.pwm.set_duty(ch, duty) {
            error!("motor: set_duty failed on channel index {}", ch.index());
        }
    }

    /// 更新速度环计算（编码器 → PID → PWM<正/反转通道>）。
    pub fn update(&mut self, dt: f32) {
        self.vel_mm_s = self.linear_velocity(dt);
        let pid_output = self.pid.update(self.vel_mm_s, dt) as i32;
        // unsigned_abs：即使 i32::MIN 也不会 panic（debug 下取反会溢出）
        if pid_output > 0 {
            self.set_duty(self.ch_pos, pid_output as u32);
        } else {
            self.set_duty(self.ch_neg, pid_output.unsigned_abs());
        }
    }
}

/// 供 `defmt` 打印整个控制器状态：实测线速度、编码器增量与 PID 内部状态。
///
/// PWM / 编码器外设句柄不参与格式化，避免要求底层类型实现 `Format`。
impl<'d, Tpwm, Tenc, P, const PPR: u16, const WHEEL_DIA_MM: u16> defmt::Format
    for MotorController<'d, Tpwm, Tenc, P, PPR, WHEEL_DIA_MM>
where
    Tpwm: PwmInstance,
    Tenc: EncoderInstance,
    P: Pid + defmt::Format,
{
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Motor(vel={}, delta={}, pid={})",
            self.vel_mm_s,
            self.delta_count,
            self.pid
        );
    }
}
