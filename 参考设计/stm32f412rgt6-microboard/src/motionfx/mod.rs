//! Rust FFI wrapper for ST Microelectronics MotionFX sensor fusion library.
//!
//! MotionFX implements a Kalman filter for 9-axis (acc+gyro+mag) orientation
//! estimation, with automatic gyro bias estimation and magnetic calibration.
//!
//! This module provides safe Rust bindings to the precompiled `libMotionFX.a`.
//!
//! # Units
//! - Accel: g
//! - Gyro: °/s
//! - Mag: µT/50 (the library scales internally so that Earth field ≈ 1.0)
//!
//! # Quick start
//! ```ignore
//! let mut mfx = MotionFx::new();
//! mfx.init();
//! loop {
//!     let input = MFX_input_t { mag: [mx/50.0, ..], acc: [ax, ay, az], gyro: [gx, gy, gz] };
//!     mfx.update(&input, dt_s);
//!     let out = mfx.output();
//!     info!("yaw={} pitch={} roll={}", out.rotation[0], out.rotation[1], out.rotation[2]);
//! }
//! ```

use core::ffi::c_void;

// ── Static state buffer (allocated once, shared across all instances) ──────
// MotionFX uses a void* state pointer; the actual memory must remain valid
// for the lifetime of the engine. This buffer is sized generously.
const MFX_STATE_BUF_SIZE: usize = 16384;
static mut MFX_STATE_BUF: [u8; MFX_STATE_BUF_SIZE] = [0u8; MFX_STATE_BUF_SIZE];

// ── C-compatible types matching motion_fx.h ──────────────────────────────

pub type MFXState_t = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFX_knobs_t {
    pub ATime: f32, // 加速度计数据融合收敛时间系数
    pub MTime: f32, // 磁力计数据融合收敛时间系数
    pub FrTime: f32, // 存在外部加速度干扰时，加速度计数据融合收敛时间系数
    pub LMode: u8, // 陀螺仪零偏校准模式：1-静态校准，2-动态校准
    pub gbias_mag_th_sc: f32,// 陀螺仪零偏磁力计阈值缩放系数
    pub gbias_acc_th_sc: f32,  // 陀螺仪零偏加速度计阈值缩放系数
    pub gbias_gyro_th_sc: f32, // 陀螺仪零偏陀螺仪自身阈值缩放系数
    pub modx: u8,// 数据降采样倍率：手机/平板场景设为1，嵌入式设备建议大于等于1
    pub acc_orientation: [i8; 4],// 加速度计坐标轴方向映射配置
    pub gyro_orientation: [i8; 4],// 陀螺仪坐标轴方向映射配置
    pub mag_orientation: [i8; 4],// 磁力计坐标轴方向映射配置
    pub output_type: i32,// 输出坐标系：0-北东地(NED)，1-东北天(ENU)
    pub start_automatic_gbias_calculation: i32,// 使能自动陀螺仪零偏计算（非0开启，0关闭）
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFX_input_t {
    pub mag: [f32; 3],
    pub acc: [f32; 3],
    pub gyro: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFX_output_t {
    pub rotation: [f32; 3],   // yaw, pitch, roll (degrees)
    pub quaternion: [f32; 4], // [q0, q1, q2, q3]
    pub gravity: [f32; 3],    // device-frame gravity
    pub linear_acceleration: [f32; 3],
    pub heading: f32,    // magnetic heading (degrees)
    pub headingErr: f32, // heading error (degrees)
}

// ── MotionFX MagCal C-compatible types ────────────────────────────────────

/// Calibration quality returned by MotionFX_MagCal_getParams.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFX_MagCal_quality_t {
    Unknown = 0,
    Poor = 1,
    Ok = 2,
    Good = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFX_MagCal_input_t {
    pub mag: [f32; 3],   // Uncalibrated mag [uT/50]
    pub time_stamp: i32, // Timestamp [ms]
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFX_MagCal_output_t {
    pub hi_bias: [f32; 3], // Hard iron offset array [uT/50]
    pub cal_quality: i32,  // MFX_MagCal_quality_t (C enum → i32)
}

// ── FFI declarations (module-private) ─────────────────────────────────────

#[link(name = "MotionFX")]
unsafe extern "C" {
    fn MotionFX_GetStateSize() -> usize;
    fn MotionFX_initialize(mfxstate_pt: MFXState_t);
    fn MotionFX_setKnobs(mfxstate_pt: MFXState_t, knobs: *const MFX_knobs_t);
    fn MotionFX_getKnobs(mfxstate_pt: MFXState_t, knobs: *mut MFX_knobs_t);
    fn MotionFX_getStatus_6X(mfxstate_pt: MFXState_t) -> i32;
    fn MotionFX_getStatus_9X(mfxstate_pt: MFXState_t) -> i32;
    fn MotionFX_enable_6X(mfxstate_pt: MFXState_t, enable: i32);
    fn MotionFX_enable_9X(mfxstate_pt: MFXState_t, enable: i32);
    fn MotionFX_setGbias(mfxstate_pt: MFXState_t, gbias: *const f32);
    fn MotionFX_getGbias(mfxstate_pt: MFXState_t, gbias: *mut f32);
    fn MotionFX_update(
        mfxstate_pt: MFXState_t,
        data_out: *mut MFX_output_t,
        data_in: *mut MFX_input_t,
        eml_deltatime: *mut f32,
        eml_q_update: *mut f32,
    );
    fn MotionFX_propagate(
        mfxstate_pt: MFXState_t,
        data_out: *mut MFX_output_t,
        data_in: *mut MFX_input_t,
        eml_deltatime: *mut f32,
    );

    // ── Magnetic calibration ──────────────────────────────────────────
    fn MotionFX_MagCal_init(sampletime: i32, enable: u16);
    fn MotionFX_MagCal_run(data_in: *mut MFX_MagCal_input_t);
    fn MotionFX_MagCal_getParams(data_out: *mut MFX_MagCal_output_t);

    // ── Version ───────────────────────────────────────────────────────
    fn MotionFX_GetLibVersion(version: *mut i8) -> u8;
}

// ── Compile-time ABI assertions ───────────────────────────────────────────
const _: () = {
    assert!(core::mem::size_of::<MFX_knobs_t>() == 52);
    assert!(core::mem::size_of::<MFX_input_t>() == 36);
    assert!(core::mem::size_of::<MFX_output_t>() == 60);
    assert!(core::mem::size_of::<MFX_MagCal_input_t>() == 16);
    assert!(core::mem::size_of::<MFX_MagCal_output_t>() == 16);
};

// ── Public MagCal helper functions ────────────────────────────────────────

/// Initialize (or disable) the MotionFX magnetic calibration sub-system.
///
/// * `sampletime_ms` — period in milliseconds between successive
///   [`mag_cal_run`] calls
/// * `enable` — 1 to enable, 0 to disable
///
/// # Safety
/// This function calls the C FFI `MotionFX_MagCal_init`. It is safe as long
/// as no other MotionFX MagCal functions are called concurrently.
pub unsafe fn mag_cal_init(sampletime_ms: i32, enable: u16) {
    // SAFETY: caller must ensure no concurrent MagCal access
    unsafe {
        MotionFX_MagCal_init(sampletime_ms, enable);
    }
}

/// Feed one uncalibrated magnetometer sample to the MotionFX MagCal engine.
///
/// The `mag` field of `data_in` must be in **µT/50** units.
///
/// # Safety
/// Must not be called concurrently with other MagCal functions.
pub unsafe fn mag_cal_run(data_in: &mut MFX_MagCal_input_t) {
    // SAFETY: caller must ensure no concurrent MagCal access
    unsafe {
        MotionFX_MagCal_run(data_in as *mut MFX_MagCal_input_t);
    }
}

/// Retrieve the hard-iron bias and calibration quality from the MagCal engine.
///
/// Call this after collecting enough samples (usually 20+ seconds of data
/// with the device rotated through various orientations).
///
/// # Safety
/// Must not be called concurrently with other MagCal functions.
pub unsafe fn mag_cal_get_params(data_out: &mut MFX_MagCal_output_t) {
    // SAFETY: caller must ensure no concurrent MagCal access
    unsafe {
        MotionFX_MagCal_getParams(data_out as *mut MFX_MagCal_output_t);
    }
}

// ── Safe Rust wrapper ────────────────────────────────────────────────────

pub struct MotionFx {
    state: MFXState_t,
    output: MFX_output_t,
}

impl MotionFx {
    /// Create a new uninitialized instance.
    pub fn new() -> Self {
        Self {
            state: core::ptr::null_mut(),
            output: unsafe { core::mem::zeroed() },
        }
    }

    /// Initialize the MotionFX engine: allocate state memory, set default
    /// knobs, and enable 9-axis (acc + gyro + mag) fusion.
    ///
    /// # Panics
    /// Panics if the required state memory exceeds the static buffer size.
    pub fn init(&mut self) {
        unsafe {
            let req = MotionFX_GetStateSize();
            assert!(
                req <= MFX_STATE_BUF_SIZE as _,
                "MotionFX state requires {} bytes but buffer is {}",
                req,
                MFX_STATE_BUF_SIZE,
            );

            self.state = core::ptr::addr_of_mut!(MFX_STATE_BUF).cast::<c_void>();
            MotionFX_initialize(self.state);

            // Configure knobs: first get defaults, then override
            let mut knobs: MFX_knobs_t = core::mem::zeroed();
            MotionFX_getKnobs(self.state, &mut knobs);

            // 打印库默认 knobs 参数
            defmt::info!(
                "MFX knobs defaults: ATime={} MTime={} FrTime={} LMode={} modx={} gbias_mag={} gbias_acc={} gbias_gyro={}",
                knobs.ATime,
                knobs.MTime,
                knobs.FrTime,
                knobs.LMode,
                knobs.modx,
                knobs.gbias_mag_th_sc,
                knobs.gbias_acc_th_sc,
                knobs.gbias_gyro_th_sc,
            );

            // Orientation codes (ASCII char: 'n','w','u','s','e' mapped to signed bytes)
            // IKS01A2 board: acc/gyro = NWU, mag = NEU
            knobs.acc_orientation = ['n' as i8, 'w' as i8, 'u' as i8, 0];
            knobs.gyro_orientation = ['n' as i8, 'w' as i8, 'u' as i8, 0];
            knobs.mag_orientation = ['n' as i8, 'e' as i8, 'u' as i8, 0];

            // knobs.MTime = 0.1;
            // knobs.FrTime = 0.3;
            // knobs.output_type = 1;
            // knobs.LMode = 1;
            knobs.modx = 2;

            // 打印赋值后的 knobs 参数
            defmt::info!(
                "MFX knobs applied: ATime={} MTime={} FrTime={} LMode={} modx={} gbias_mag={} gbias_acc={} gbias_gyro={}",
                knobs.ATime,
                knobs.MTime,
                knobs.FrTime,
                knobs.LMode,
                knobs.modx,
                knobs.gbias_mag_th_sc,
                knobs.gbias_acc_th_sc,
                knobs.gbias_gyro_th_sc,
            );

            MotionFX_setKnobs(self.state, &knobs);
            MotionFX_enable_6X(self.state, 0);
            MotionFX_enable_9X(self.state, 1);
        }
    }

    /// Run one step of the Kalman filter: propagate (predict) then update (correct).
    ///
    /// * `input` — sensor data in **µT/50** (mag), **g** (acc), **°/s** (gyro)
    /// * `dt_s` — elapsed time since last update in seconds
    pub fn update(&mut self, input: &MFX_input_t, dt_s: f32) {
        if dt_s <= 0.0 || dt_s.is_nan() || dt_s.is_infinite() {
            return;
        }
        // The library requires both propagate (prediction step) and update
        // (measurement incorporation) to be called each frame, with the same
        // data_out and data_in pointers passed to both.
        let mut input_copy = *input;
        let dt = dt_s;
        unsafe {
            MotionFX_propagate(
                self.state,
                &mut self.output as *mut MFX_output_t,
                &mut input_copy as *mut MFX_input_t,
                &dt as *const f32 as *mut f32,
            );
            MotionFX_update(
                self.state,
                &mut self.output as *mut MFX_output_t,
                &mut input_copy as *mut MFX_input_t,
                &dt as *const f32 as *mut f32,
                core::ptr::null_mut(), // eml_q_update = NULL
            );
        }
    }

    /// Read-only access to the most recent fusion output.
    pub fn output(&self) -> &MFX_output_t {
        &self.output
    }

    /// Mutable access to the most recent fusion output (e.g. for logging).
    pub fn output_mut(&mut self) -> &mut MFX_output_t {
        &mut self.output
    }
}

impl Default for MotionFx {
    fn default() -> Self {
        Self::new()
    }
}
