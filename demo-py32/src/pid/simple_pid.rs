//! PID 控制器（位置式 / 增量式）
//!
//! 参考 DJI RoboMaster 的 `pid.c/h`，改写为符合 Rust 习惯的实现：
//! - 将**位置式**（[`PositionalPid`]）与**增量式**（[`IncrementalPid`]）拆成两个独立结构体，
//!   不再用一个 `mode` 枚举复用同一份结构体，类型即文档、状态互不污染。
//! - 内部状态私有化，只通过方法访问，避免外部直接改坏历史项。
//! - 显式传入采样周期 `dt`：积分项 `×dt`、微分项 `÷dt`，天然支持变周期采样；
//!   当 `dt` 恒为 `1.0` 时，与原始 C 实现**完全等价**。
//! - 输出限幅 + 积分限幅（抗积分饱和 / anti-windup）。
//!
//! 两个控制器都实现了 [`Pid`] trait，可在泛型代码中互换使用。
//!
//! # 例
//! ```ignore
//! use crate::pid::{IncrementalPid, Pid};
//!
//! let mut pid = IncrementalPid::new(2.0, 0.5, 0.1, 1000.0, 500.0);
//! pid.set_setpoint(300.0);
//! let out = pid.update(measured_speed, 0.01); // 10ms 控制周期
//! ```

/// 将值对称限幅到 `[-max, max]`，等价于 C 版 `LimitMax` 宏。
#[inline]
fn clamp_symmetric(value: f32, max: f32) -> f32 {
    value.clamp(-max, max)
}

/// PID 控制器的统一接口，位置式与增量式都实现它。
pub trait Pid {
    /// 设置目标值（设定值 / setpoint）。
    fn set_setpoint(&mut self, setpoint: f32);
    /// 读取当前目标值。
    fn setpoint(&self) -> f32;

    /// 执行一次 PID 计算，返回控制输出。
    ///
    /// - `measured`：当前反馈量（测量值）。
    /// - `dt`：本次距上次的采样周期（秒），用于积分 / 微分的尺度归一化。
    fn update(&mut self, measured: f32, dt: f32) -> f32;

    /// 清零内部状态（误差、积分、输出），保留增益与限幅参数。
    fn reset(&mut self);

    /// 上一次的输出值。
    fn last_output(&self) -> f32;

    fn max_out(&self) -> f32;

    fn set_output_limit(&mut self, max_out: f32);
}

// ============================================================================
// 位置式 PID
// ============================================================================

/// 位置式（绝对式）PID。
///
/// 输出 `out = P + I + D` 是对当前误差的直接绝对响应：
/// - `P = Kp * error`
/// - `I += Ki * error * dt`（累积，带积分限幅抗饱和）
/// - `D = Kd * (error - last_error) / dt`
///
/// 适合需要直接得到绝对控制量（如位置环、角度环）的场合。
#[derive(Clone, Copy, defmt::Format)]
pub struct PositionalPid {
    // ── 增益 ──
    kp: f32,
    ki: f32,
    kd: f32,

    // ── 限幅 ──
    max_out: f32,  // 输出限幅
    max_iout: f32, // 积分限幅（抗积分饱和）

    // ── 设定 / 状态 ──
    setpoint: f32,
    integral: f32,   // 累积积分项（= Iout）
    last_error: f32, // 上一次误差
    out: f32,        // 最新输出

    // ── 末次分量（供观察 / 调参）──
    p_out: f32,
    i_out: f32,
    d_out: f32,
}

impl PositionalPid {
    /// 新建位置式 PID。
    ///
    /// - `kp / ki / kd`：比例、积分、微分增益。
    /// - `max_out`：输出限幅，输出被钳制到 `[-max_out, max_out]`。
    /// - `max_iout`：积分限幅，累积积分被钳制到 `[-max_iout, max_iout]`（抗积分饱和）。
    #[inline]
    pub const fn new(kp: f32, ki: f32, kd: f32, max_out: f32, max_iout: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            max_out,
            max_iout,
            setpoint: 0.0,
            integral: 0.0,
            last_error: 0.0,
            out: 0.0,
            p_out: 0.0,
            i_out: 0.0,
            d_out: 0.0,
        }
    }

    /// 设置 PID 三参数（运行时调参）。
    #[inline]
    pub fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }

    /// 设置积分限幅（抗饱和）。
    #[inline]
    pub fn set_integral_limit(&mut self, max_iout: f32) {
        self.max_iout = max_iout;
    }

    #[inline]
    pub fn kp(&self) -> f32 {
        self.kp
    }
    #[inline]
    pub fn ki(&self) -> f32 {
        self.ki
    }
    #[inline]
    pub fn kd(&self) -> f32 {
        self.kd
    }
    #[inline]
    pub fn integral(&self) -> f32 {
        self.integral
    }
    #[inline]
    pub fn p_out(&self) -> f32 {
        self.p_out
    }
    #[inline]
    pub fn i_out(&self) -> f32 {
        self.i_out
    }
    #[inline]
    pub fn d_out(&self) -> f32 {
        self.d_out
    }
}

impl Pid for PositionalPid {
    #[inline]
    fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    #[inline]
    fn setpoint(&self) -> f32 {
        self.setpoint
    }

    fn update(&mut self, measured: f32, dt: f32) -> f32 {
        let error = self.setpoint - measured;

        // 比例项
        self.p_out = self.kp * error;

        // 积分项：累积并抗饱和
        self.integral += self.ki * error * dt;
        self.integral = clamp_symmetric(self.integral, self.max_iout);
        self.i_out = self.integral;

        // 微分项：差分（dt 非正时跳过，避免除零 / 倒流）
        self.d_out = if dt > 0.0 {
            self.kd * (error - self.last_error) / dt
        } else {
            0.0
        };

        // 合成并输出限幅
        self.out = clamp_symmetric(self.p_out + self.i_out + self.d_out, self.max_out);
        self.last_error = error;
        self.out
    }

    fn reset(&mut self) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.out = 0.0;
        self.p_out = 0.0;
        self.i_out = 0.0;
        self.d_out = 0.0;
    }

    #[inline]
    fn last_output(&self) -> f32 {
        self.out
    }

    fn max_out(&self) -> f32 {
        self.max_out
    }
    
    fn set_output_limit(&mut self, max_out: f32) {
        self.max_out = max_out;
    }
}

impl Default for PositionalPid {
    /// 全零增益 + 不限幅的空控制器，再用 setter 细调。
    #[inline]
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, f32::INFINITY, f32::INFINITY)
    }
}

// ============================================================================
// 增量式 PID
// ============================================================================

/// 增量式（差分式）PID。
///
/// 每次只算输出的**增量** `Δout`，再累加到上一次输出：
/// - `ΔP = Kp * (e0 - e1)`
/// - `ΔI = Ki * e0 * dt`
/// - `ΔD = Kd * (e0 - 2·e1 + e2) / dt`
/// - `out += ΔP + ΔI + ΔD`
///
/// 输出是历史累加值，对单步扰动不敏感，常用于电机速度环、步进/增量执行器。
///
/// > 注：增量式本身不再单独对积分项限幅，只对**累积输出** `out` 限幅，
/// > 因此构造参数 `max_iout` 不参与计算，保留仅为与位置式 API 对齐。
#[derive(Clone, Copy, defmt::Format)]
pub struct IncrementalPid {
    // ── 增益 ──
    kp: f32,
    ki: f32,
    kd: f32,

    // ── 限幅 ──
    max_out: f32,  // 累积输出限幅
    max_iout: f32, // 占位，对齐位置式 API（增量式未使用）

    // ── 设定 / 状态 ──
    setpoint: f32,
    /// 误差历史 `[e0, e1, e2]`：e0 最新，e1 上一次，e2 上上次（同 C 版 error[3]）。
    error: [f32; 3],
    out: f32, // 累积输出

    // ── 末次增量分量（供观察 / 调参）──
    p_out: f32,
    i_out: f32,
    d_out: f32,
}

impl IncrementalPid {
    /// 新建增量式 PID。
    ///
    /// - `kp / ki / kd`：比例、积分、微分增益。
    /// - `max_out`：累积输出限幅，钳制到 `[-max_out, max_out]`。
    /// - `max_iout`：占位参数，增量式未使用（见结构体文档说明）。
    #[inline]
    pub const fn new(kp: f32, ki: f32, kd: f32, max_out: f32, max_iout: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            max_out,
            max_iout,
            setpoint: 0.0,
            error: [0.0; 3],
            out: 0.0,
            p_out: 0.0,
            i_out: 0.0,
            d_out: 0.0,
        }
    }

    /// 设置 PID 三参数（运行时调参）。
    #[inline]
    pub fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }

    #[inline]
    pub fn kp(&self) -> f32 {
        self.kp
    }
    #[inline]
    pub fn ki(&self) -> f32 {
        self.ki
    }
    #[inline]
    pub fn kd(&self) -> f32 {
        self.kd
    }
    #[inline]
    pub fn p_out(&self) -> f32 {
        self.p_out
    }
    #[inline]
    pub fn i_out(&self) -> f32 {
        self.i_out
    }
    #[inline]
    pub fn d_out(&self) -> f32 {
        self.d_out
    }
}

impl Pid for IncrementalPid {
    #[inline]
    fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    #[inline]
    fn setpoint(&self) -> f32 {
        self.setpoint
    }

    fn update(&mut self, measured: f32, dt: f32) -> f32 {
        // 误差历史滚动：e2 <- e1 <- e0 <- 新值
        self.error[2] = self.error[1];
        self.error[1] = self.error[0];
        self.error[0] = self.setpoint - measured;

        let [e0, e1, e2] = self.error;

        // 增量三分量
        self.p_out = self.kp * (e0 - e1);
        self.i_out = self.ki * e0 * dt;
        self.d_out = if dt > 0.0 {
            self.kd * (e0 - 2.0 * e1 + e2) / dt
        } else {
            0.0
        };

        // 累加并限幅
        self.out = clamp_symmetric(self.out + self.p_out + self.i_out + self.d_out, self.max_out);
        self.out
    }

    fn reset(&mut self) {
        self.error = [0.0; 3];
        self.out = 0.0;
        self.p_out = 0.0;
        self.i_out = 0.0;
        self.d_out = 0.0;
    }

    #[inline]
    fn last_output(&self) -> f32 {
        self.out
    }

    fn max_out(&self) -> f32 {
        self.max_out
    }

    fn set_output_limit(&mut self, max_out: f32) {
        self.max_out = max_out;
    }
}

impl Default for IncrementalPid {
    /// 全零增益 + 不限幅的空控制器，再用 setter 细调。
    #[inline]
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, f32::INFINITY, f32::INFINITY)
    }
}
