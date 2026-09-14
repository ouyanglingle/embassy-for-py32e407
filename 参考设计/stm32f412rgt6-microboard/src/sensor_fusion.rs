//! 传感器融合模块：IMU（加速度计+陀螺仪）+ 磁力计 → MotionFX 9 轴姿态解算。
//!
//! 本模块负责：
//! - 定义任务间通信通道（`OUTPUT_CHANNEL`、`MAG_CHANNEL`）
//! - 以 10ms 周期执行 IMU 读取 + 磁力计低通滤波 + MotionFX 更新
//! - 以 20ms 周期独立读取磁力计

use crate::board::{ImuSpi, MagSpi, MotorA, MotorB};
use crate::lsm6dsrtr::Lsm6dsrtr;
use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Instant, Ticker};

/// 磁力计一阶低通系数（越小越平滑、滞后越大）。10ms 周期下 α=0.5 时间常数 ~0.02s。
const MAG_LPF_ALPHA: f32 = 0.5;

/// 输出数据通道：imu_update_task → telemetry / main
/// 传输 (mag_z: f32, yaw: f32)
pub static OUTPUT_CHANNEL: Channel<CriticalSectionRawMutex, (f32, f32), 1> = Channel::new();

/// 磁力计数据通道：mag_task → imu_update_task
pub static MAG_CHANNEL: Channel<CriticalSectionRawMutex, (f32, f32, f32), 2> = Channel::new();

/// 所有需要移到 RTC 中断执行器 imu_update_task 的资源
pub struct ImuTaskResources {
    pub imu: Lsm6dsrtr<ImuSpi>,
    pub calib: crate::mag_calib::MagCalib,
    pub mfx: crate::motionfx::MotionFx,
    pub mag_lpf: Option<(f32, f32, f32)>,
    pub latest_mag: Option<(f32, f32, f32)>,
    pub prev: Instant,
    pub motor_a: MotorA,
    pub motor_b: MotorB,
}

// 单核 Cortex-M4F 上跨执行器传递原始指针类型是安全的
unsafe impl Send for ImuTaskResources {}

/// 磁力计任务资源包装（单核安全）
pub struct MagTaskResources {
    pub mag: crate::mlx90393::Mlx90393<MagSpi>,
}

unsafe impl Send for MagTaskResources {}

/// 传感器融合主任务：10ms 周期读取 IMU + 磁力计 → MotionFX 更新 → 输出姿态
#[embassy_executor::task]
pub async fn imu_update_task(mut res: ImuTaskResources) {
    let mut ticker_10ms = Ticker::every(Duration::from_millis(10));
    loop {
        ticker_10ms.next().await;
        let start = Instant::now();
        // 非阻塞读取最新磁力计数据（没有新数据就复用上一次的）
        if let Ok(m) = MAG_CHANNEL.try_receive() {
            res.latest_mag = Some(m);
        }
        let (mag_x, mag_y, mag_z) = match res.latest_mag {
            Some(m) => m,
            None => continue,
        };

        // 读取加速度计（g）
        let (ax_g, ay_g, az_g) = match res.imu.read_accel_g().await {
            Ok(a) => (a.x, a.y, a.z),
            Err(_) => continue,
        };

        // 读取陀螺仪（°/s）
        let (gx_dps, gy_dps, gz_dps) = match res.imu.read_gyro_dps().await {
            Ok(g) => (g.x, g.y, g.z),
            Err(_) => continue,
        };

        let now = Instant::now();
        let dt_s = now.duration_since(res.prev).as_millis() as f32 * 0.001;
        res.prev = now;

        // 校准 + 低通磁力计 → ÷50 归一化（*0.02） → 喂 MotionFX
        let (mx0, my0, mz0) = res.calib.apply(mag_x, mag_y, mag_z);
        let lpf = res.mag_lpf.get_or_insert((mx0, my0, mz0));
        lpf.0 += MAG_LPF_ALPHA * (mx0 - lpf.0);
        lpf.1 += MAG_LPF_ALPHA * (my0 - lpf.1);
        lpf.2 += MAG_LPF_ALPHA * (mz0 - lpf.2);

        let input = crate::motionfx::MFX_input_t {
            mag: [lpf.0 * 0.02, lpf.1 * 0.02, lpf.2 * 0.02],
            acc: [ax_g, ay_g, az_g],
            gyro: [gx_dps, gy_dps, gz_dps],
        };
        res.mfx.update(&input, dt_s);
        let yaw = res.mfx.output().rotation[0];

        // 推送数据到通道
        let _ = OUTPUT_CHANNEL.try_send((mag_z, yaw));

        // 电机速度环控制
        res.motor_a.update(0.01f32);
        res.motor_b.update(0.01f32);
        
        info!("motor_a: {}", res.motor_a.pid);
        let elapsed = start.elapsed();
        // defmt::info!("exec time: {} us", elapsed.as_micros());
    }
}

/// 磁力计独立读取任务：20ms 周期轮询 MLX90393
#[embassy_executor::task]
pub async fn mag_task(mut resource: MagTaskResources) {
    let mut ticker_20ms = Ticker::every(Duration::from_millis(20));
    loop {
        ticker_20ms.next().await;
        if let Ok(m) = resource.mag.read_magnetic().await {
            MAG_CHANNEL.send((m.x, m.y, m.z)).await;
        }
    }
}
