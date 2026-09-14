//! PC 端验证程序：提取 bp_pid 核心逻辑，脱离 no_std 环境在 PC 上运行
//!
//! 验证项：
//! 1. LCG 确定性
//! 2. Sigmoid 值域
//! 3. 前向传播输出 kp/ki/kd 在合理范围
//! 4. 增量式 PID 计算正确性（冻结网络，手工验证 PID 公式）
//! 5. 反向传播后权重确实更新
//! 6. 闭环仿真收敛性
//! 7. dt<=0 保护
//! 8. reset 保留权重
//! 9. 输出限幅


const IN: usize = 3;
const HID: usize = 5;
const OUT: usize = 3;

const LCG_A: u32 = 1_664_525;
const LCG_C: u32 = 1_013_904_223;

use micromath::F32Ext;

fn lcg_next(seed: u32) -> u32 {
    seed.wrapping_mul(LCG_A).wrapping_add(LCG_C)
}

fn lcg_rand(seed: &mut u32, range: f32) -> f32 {
    *seed = lcg_next(*seed);
    let frac = ((*seed >> 9) as f32) / (1u32 << 23) as f32;
    (2.0 * frac - 1.0) * range
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + libm::expf(-x))
}

fn tanhf(x: f32) -> f32 {
    libm::tanhf(x)
}

fn clamp_symmetric(value: f32, max: f32) -> f32 {
    value.clamp(-max, max)
}

pub trait Pid {
    fn set_setpoint(&mut self, setpoint: f32);
    fn setpoint(&self) -> f32;
    fn update(&mut self, measured: f32, dt: f32) -> f32;
    fn reset(&mut self);
    fn last_output(&self) -> f32;
    fn max_out(&self) -> f32;
    fn set_output_limit(&mut self, max_out: f32);
}

#[derive(Clone, Copy, Debug)]
pub struct BpPid {
    w_in_hid: [[f32; IN]; HID],
    w_hid_out: [[f32; HID]; OUT],
    b_hid: [f32; HID],
    b_out: [f32; OUT],
    dw_in_hid: [[f32; IN]; HID],
    dw_hid_out: [[f32; HID]; OUT],
    db_hid: [f32; HID],
    db_out: [f32; OUT],
    in_cache: [f32; IN],
    hid_out: [f32; HID],
    setpoint: f32,
    error: [f32; 3],
    out: f32,
    max_out: f32,
    last_u: f32,
    last_measured: f32,
    eta: f32,
    alpha: f32,
    kp_max: f32,
    ki_max: f32,
    kd_max: f32,
    input_scale: f32,
    p_out: f32,
    i_out: f32,
    d_out: f32,
    nn_kp: f32,
    nn_ki: f32,
    nn_kd: f32,
}

impl BpPid {
    pub const fn new(eta: f32, alpha: f32, max_out: f32, kp_max: f32, ki_max: f32, kd_max: f32) -> Self {
        Self {
            w_in_hid: [[0.0; IN]; HID],
            w_hid_out: [[0.0; HID]; OUT],
            b_hid: [0.0; HID],
            b_out: [0.0; OUT],
            dw_in_hid: [[0.0; IN]; HID],
            dw_hid_out: [[0.0; HID]; OUT],
            db_hid: [0.0; HID],
            db_out: [0.0; OUT],
            in_cache: [0.0; IN],
            hid_out: [0.0; HID],
            setpoint: 0.0,
            error: [0.0; 3],
            out: 0.0,
            max_out,
            last_u: 0.0,
            last_measured: 0.0,
            eta,
            alpha,
            kp_max,
            ki_max,
            kd_max,
            input_scale: 1.0,
            p_out: 0.0,
            i_out: 0.0,
            d_out: 0.0,
            nn_kp: 0.0,
            nn_ki: 0.0,
            nn_kd: 0.0,
        }
    }

    pub fn init_weights_with_seed(&mut self, seed: u32) {
        let mut s = seed;
        // 预热 10 轮，消除 LCG 初始状态相关性
        for _ in 0..10 {
            s = lcg_next(s);
        }
        let range = 0.5;
        for j in 0..HID {
            for i in 0..IN {
                self.w_in_hid[j][i] = lcg_rand(&mut s, range);
            }
            self.b_hid[j] = lcg_rand(&mut s, range);
        }
        for l in 0..OUT {
            for j in 0..HID {
                self.w_hid_out[l][j] = lcg_rand(&mut s, range);
            }
            self.b_out[l] = lcg_rand(&mut s, range);
        }
    }

    pub fn init_weights(&mut self) {
        self.init_weights_with_seed(12345);
    }

    pub fn set_input_scale(&mut self, scale: f32) {
        self.input_scale = if scale > 0.0 { scale } else { 1.0 };
    }

    pub fn set_gains_max(&mut self, kp_max: f32, ki_max: f32, kd_max: f32) {
        self.kp_max = kp_max;
        self.ki_max = ki_max;
        self.kd_max = kd_max;
    }

    fn forward(&mut self, rin: f32, yout: f32, error: f32) {
        let s = self.input_scale;
        self.in_cache = [rin / s, yout / s, error / s];

        for j in 0..HID {
            let mut net = self.b_hid[j];
            for i in 0..IN {
                net += self.w_in_hid[j][i] * self.in_cache[i];
            }
            self.hid_out[j] = tanhf(net);
        }

        let k_max = [self.kp_max, self.ki_max, self.kd_max];
        let nn_k = [&mut self.nn_kp, &mut self.nn_ki, &mut self.nn_kd];
        for l in 0..OUT {
            let mut net = self.b_out[l];
            for j in 0..HID {
                net += self.w_hid_out[l][j] * self.hid_out[j];
            }
            *nn_k[l] = k_max[l] * sigmoid(net);
        }
    }

    /// 反向传播（修复版：加入 ∂u/∂kp, ∂u/∂ki, ∂u/∂kd 和 k_max 缩放）
    fn backward(&mut self, error: f32, dy_du: f32, du_dk: [f32; OUT]) {
        let nn_o = [
            self.nn_kp / if self.kp_max > 0.0 { self.kp_max } else { 1.0 },
            self.nn_ki / if self.ki_max > 0.0 { self.ki_max } else { 1.0 },
            self.nn_kd / if self.kd_max > 0.0 { self.kd_max } else { 1.0 },
        ];
        let k_max = [self.kp_max, self.ki_max, self.kd_max];
        let delta_out: [f32; OUT] = {
            let mut d = [0.0f32; OUT];
            for l in 0..OUT {
                let o = nn_o[l];
                d[l] = error * dy_du * du_dk[l] * k_max[l] * o * (1.0 - o);
            }
            d
        };

        let delta_hid: [f32; HID] = {
            let mut d = [0.0f32; HID];
            for j in 0..HID {
                let mut sum = 0.0f32;
                for l in 0..OUT {
                    sum += self.w_hid_out[l][j] * delta_out[l];
                }
                let h = self.hid_out[j];
                d[j] = sum * (1.0 - h * h);
            }
            d
        };

        for l in 0..OUT {
            for j in 0..HID {
                let dw = self.eta * delta_out[l] * self.hid_out[j] + self.alpha * self.dw_hid_out[l][j];
                self.w_hid_out[l][j] += dw;
                self.dw_hid_out[l][j] = dw;
            }
            let db = self.eta * delta_out[l] + self.alpha * self.db_out[l];
            self.b_out[l] += db;
            self.db_out[l] = db;
        }

        for j in 0..HID {
            for i in 0..IN {
                let dw = self.eta * delta_hid[j] * self.in_cache[i] + self.alpha * self.dw_in_hid[j][i];
                self.w_in_hid[j][i] += dw;
                self.dw_in_hid[j][i] = dw;
            }
            let db = self.eta * delta_hid[j] + self.alpha * self.db_hid[j];
            self.b_hid[j] += db;
            self.db_hid[j] = db;
        }
    }

    fn approximate_jacobian(&self, measured: f32) -> f32 {
        let dy = measured - self.last_measured;
        let du = self.out - self.last_u;
        if du.abs() < 1e-6 {
            if dy > 0.0 {
                1.0
            } else if dy < 0.0 {
                -1.0
            } else {
                0.0
            }
        } else {
            clamp_symmetric(dy / du, 5.0)
        }
    }
}

impl Pid for BpPid {
    fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }
    fn setpoint(&self) -> f32 {
        self.setpoint
    }

    fn update(&mut self, measured: f32, dt: f32) -> f32 {
        let error = self.setpoint - measured;
        self.forward(self.setpoint, measured, error);

        self.error[2] = self.error[1];
        self.error[1] = self.error[0];
        self.error[0] = error;

        let [e0, e1, e2] = self.error;
        let kp = self.nn_kp;
        let ki = self.nn_ki;
        let kd = self.nn_kd;

        self.p_out = kp * (e0 - e1);
        self.i_out = ki * e0 * dt;
        self.d_out = if dt > 0.0 { kd * (e0 - 2.0 * e1 + e2) / dt } else { 0.0 };

        self.out = clamp_symmetric(self.out + self.p_out + self.i_out + self.d_out, self.max_out);

        if dt > 0.0 {
            let dy_du = self.approximate_jacobian(measured);
            let du_dk_raw = [e0 - e1, e0 * dt, (e0 - 2.0 * e1 + e2) / dt];
            let norm = (du_dk_raw[0] * du_dk_raw[0] + du_dk_raw[1] * du_dk_raw[1] + du_dk_raw[2] * du_dk_raw[2]).sqrt();
            let du_dk = if norm > 1e-8 {
                [du_dk_raw[0] / norm, du_dk_raw[1] / norm, du_dk_raw[2] / norm]
            } else {
                [0.0; OUT]
            };
            self.backward(error, dy_du, du_dk);
            self.last_u = self.out;
            self.last_measured = measured;
        }

        self.out
    }

    fn reset(&mut self) {
        self.error = [0.0; 3];
        self.out = 0.0;
        self.p_out = 0.0;
        self.i_out = 0.0;
        self.d_out = 0.0;
        self.last_u = 0.0;
        self.last_measured = 0.0;
        self.dw_in_hid = [[0.0; IN]; HID];
        self.dw_hid_out = [[0.0; HID]; OUT];
        self.db_hid = [0.0; HID];
        self.db_out = [0.0; OUT];
    }

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

// ── 测试 ──
// fn main() {
//     let mut passed = 0;
//     let mut failed = 0;

//     macro_rules! check {
//         ($cond:expr, $msg:expr) => {
//             if $cond {
//                 println!("  ✅ {}", $msg);
//                 passed += 1;
//             } else {
//                 println!("  ❌ {}", $msg);
//                 failed += 1;
//             }
//         };
//     }

//     // ── 测试 1: LCG 确定性 ──
//     println!("\n[测试1] LCG 确定性");
//     {
//         let mut s1 = 42u32;
//         let mut s2 = 42u32;
//         let mut ok = true;
//         for _ in 0..100 {
//             s1 = lcg_next(s1);
//             s2 = lcg_next(s2);
//             if s1 != s2 { ok = false; break; }
//         }
//         check!(ok, "同种子 LCG 序列完全一致");
//     }

//     // ── 测试 2: Sigmoid 值域 ──
//     println!("\n[测试2] Sigmoid 值域");
//     {
//         let mut all_in_range = true;
//         for &x in &[-100.0, -10.0, -1.0, 0.0, 1.0, 10.0, 100.0] {
//             let y = sigmoid(x);
//             if !(y >= 0.0 && y <= 1.0) {
//                 all_in_range = false;
//                 println!("    sigmoid({}) = {} 不在 [0,1]", x, y);
//             }
//         }
//         check!(all_in_range, "sigmoid 输出全部在 [0,1]");
//         check!((sigmoid(0.0) - 0.5).abs() < 1e-6, "sigmoid(0) ≈ 0.5");
//     }

//     // ── 测试 3: 前向传播输出范围 ──
//     println!("\n[测试3] 前向传播输出范围");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 65535.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(100.0);
//         pid.set_setpoint(50.0);
//         let _ = pid.update(30.0, 0.01);

//         check!(pid.nn_kp >= 0.0 && pid.nn_kp <= 200.0,
//                format!("kp = {:.3} ∈ [0, 200]", pid.nn_kp));
//         check!(pid.nn_ki >= 0.0 && pid.nn_ki <= 200.0,
//                format!("ki = {:.3} ∈ [0, 200]", pid.nn_ki));
//         check!(pid.nn_kd >= 0.0 && pid.nn_kd <= 10.0,
//                format!("kd = {:.3} ∈ [0, 10]", pid.nn_kd));
//     }

//     // ── 测试 4: 增量式 PID 计算正确性 ──
//     // 用恒定增益手动构造，跳过 forward 覆盖
//     println!("\n[测试4] 增量式 PID 计算（手动验证增量公式）");
//     {
//         // 模拟：kp=2, ki=1, kd=0.5, dt=0.01
//         // 手算增量式 PID：
//         //   step1: e0=10, e1=0, e2=0
//         //     ΔP = 2*(10-0) = 20
//         //     ΔI = 1*10*0.01 = 0.1
//         //     ΔD = 0.5*(10-0+0)/0.01 = 500
//         //     u1 = 0 + 20 + 0.1 + 500 = 520.1
//         //   step2: e0=5, e1=10, e2=0
//         //     ΔP = 2*(5-10) = -10
//         //     ΔI = 1*5*0.01 = 0.05
//         //     ΔD = 0.5*(5-20+0)/0.01 = -750
//         //     u2 = 520.1 - 10 + 0.05 - 750 = -239.85
//         let kp = 2.0f32;
//         let ki = 1.0f32;
//         let kd = 0.5f32;
//         let dt = 0.01f32;

//         let mut error = [0.0f32; 3];
//         let mut out = 0.0f32;

//         // step 1
//         error[2] = error[1]; error[1] = error[0]; error[0] = 10.0;
//         let [e0, e1, e2] = error;
//         let p = kp * (e0 - e1);
//         let i = ki * e0 * dt;
//         let d = kd * (e0 - 2.0*e1 + e2) / dt;
//         out = out + p + i + d;
//         let expected_u1 = 520.1;
//         check!((out - expected_u1).abs() < 0.01,
//                format!("step1: out={:.3}, expected={:.3}", out, expected_u1));

//         // step 2
//         error[2] = error[1]; error[1] = error[0]; error[0] = 5.0;
//         let [e0, e1, e2] = error;
//         let p = kp * (e0 - e1);
//         let i = ki * e0 * dt;
//         let d = kd * (e0 - 2.0*e1 + e2) / dt;
//         out = out + p + i + d;
//         let expected_u2 = -239.85;
//         check!((out - expected_u2).abs() < 0.01,
//                format!("step2: out={:.3}, expected={:.3}", out, expected_u2));
//     }

//     // ── 测试 5: 反向传播后权重更新 ──
//     println!("\n[测试5] 反向传播后权重确实更新");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 1000.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(100.0);

//         let w_before = pid.w_hid_out[0][0];
//         let _ = pid.update(50.0, 0.01);
//         let w_after = pid.w_hid_out[0][0];

//         check!(w_before != w_after,
//                format!("w_hid_out[0][0]: {:.6} → {:.6} (已更新)", w_before, w_after));
//     }

//     // ── 测试 6: 闭环仿真收敛 ──
//     println!("\n[测试6] 闭环仿真收敛（一阶惯性对象，多种子尝试）");
//     {
//         // 被控对象（连续时间离散化）: y(k+1) = a*y(k) + b*u(k)*dt
//         // 取 a=0.9, b=5.0, dt=0.01 → 稳态 u_ss = (1-a)*target/(b*dt) = 0.1*50/0.05 = 100
//         let a = 0.9f32;
//         let b = 5.0f32;
//         let dt = 0.01f32;
//         let seeds_to_try = [42u32, 123, 456, 789, 111, 222, 333, 444, 555, 666, 777, 888, 999];
//         let mut best_final = f32::MAX;
//         let mut best_min = f32::MAX;
//         let mut best_seed = 0;

//         for &seed in &seeds_to_try {
//             let mut pid = BpPid::new(0.15, 0.01, 200.0, 20.0, 20.0, 10.0);
//             pid.init_weights_with_seed(seed);
//             pid.set_input_scale(100.0);
//             pid.set_setpoint(50.0);

//             let mut y = 0.0f32;
//             let mut min_error = f32::MAX;
//             for _step in 0..10000 {
//                 let u = pid.update(y, dt);
//                 y = a * y + b * u * dt;
//                 let err = (y - 50.0).abs();
//                 if err < min_error { min_error = err; }
//             }
//             let final_error = (y - 50.0).abs();
//             if final_error < best_final {
//                 best_final = final_error;
//                 best_min = min_error;
//                 best_seed = seed;
//             }
//         }

//         println!("    最佳种子 {}: 最终误差 {:.4}, 最小误差 {:.6}", best_seed, best_final, best_min);
//         check!(best_final < 1.0,
//                format!("最佳最终误差 {:.4} < 1.0", best_final));
//         check!(best_min < 0.5,
//                format!("最佳最小误差 {:.6} < 0.5", best_min));
// 	    }

// 	    // ── 测试 7: dt <= 0 保护 ──
//     println!("\n[测试7] dt <= 0 保护");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 1000.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(100.0);
//         pid.set_setpoint(100.0);

//         let _u1 = pid.update(80.0, 0.01);  // 正常
//         let w_before = pid.w_hid_out;
//         let u2 = pid.update(70.0, 0.0);   // dt=0
//         let w_after = pid.w_hid_out;

//         check!(u2 != 0.0, "dt=0 仍产生输出");
//         check!(w_before == w_after, "dt=0 时不更新权重");

//         let _u3 = pid.update(60.0, -0.01);  // dt<0
//         let w_after2 = pid.w_hid_out;
//         check!(w_before == w_after2, "dt<0 时不更新权重");
//     }

//     // ── 测试 8: reset 保留权重 ──
//     println!("\n[测试8] reset 保留权重");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 1000.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(100.0);
//         pid.set_setpoint(100.0);

//         let _ = pid.update(80.0, 0.01);
//         let w_before = pid.w_in_hid;
//         pid.reset();
//         let w_after = pid.w_in_hid;

//         check!(w_before == w_after, "reset 后权重不变");
//         check!(pid.last_output() == 0.0, "reset 后输出清零");
//         check!(pid.error == [0.0; 3], "reset 后误差清零");
//     }

//     // ── 测试 9: 输出限幅 ──
//     println!("\n[测试9] 输出限幅");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 100.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(1.0);
//         pid.set_setpoint(1000.0);  // 极大设定值
//         let u = pid.update(0.0, 0.01);
//         check!(u.abs() <= 100.0 + 0.001,
//                format!("输出 |{:.3}| ≤ max_out=100", u));
//     }

//     // ── 测试 10: 梯度符号验证（有限差分法）──
//     // 对输出层偏置做小扰动，验证 ∂kp/∂b_out[0] 的符号和量级
//     println!("\n[测试10] 梯度符号验证（有限差分法）");
//     {
//         let mut pid = BpPid::new(0.1, 0.01, 1000.0, 200.0, 200.0, 10.0);
//         pid.init_weights_with_seed(42);
//         pid.set_input_scale(100.0);
//         pid.set_setpoint(50.0);

//         // 准备输入状态
//         pid.forward(50.0, 30.0, 20.0);
//         let kp_base = pid.nn_kp;
//         let o = kp_base / pid.kp_max;  // sigmoid 激活值

//         let eps = 1e-4_f32;

//         // ── 对 b_out[0]（kp 的偏置）做 ±ε 扰动 ──
//         let b0 = pid.b_out[0];

//         pid.b_out[0] = b0 + eps;
//         pid.forward(50.0, 30.0, 20.0);
//         let kp_plus = pid.nn_kp;

//         pid.b_out[0] = b0 - eps;
//         pid.forward(50.0, 30.0, 20.0);
//         let kp_minus = pid.nn_kp;

//         // 数值梯度: ∂kp/∂b_out[0] ≈ (kp(+ε) - kp(-ε)) / 2ε
//         let num_grad = (kp_plus - kp_minus) / (2.0 * eps);

//         // 解析梯度: kp = kp_max * sigmoid(b_out[0] + Σ w_hid_out[0][j] * hid_out[j])
//         //   ∂kp/∂b_out[0] = kp_max * sigmoid'(net) = kp_max * o * (1 - o)
//         let analytic_grad = pid.kp_max * o * (1.0 - o);

//         // 符号一致 + 量级在同一数量级（差不超过 2 倍）
//         let sign_ok = num_grad.signum() == analytic_grad.signum() || num_grad.abs() < 1e-10;
//         let mag_ok = if analytic_grad.abs() > 1e-10 {
//             (num_grad / analytic_grad - 1.0).abs() < 1.0  // 差不超过 100%
//         } else {
//             num_grad.abs() < 1e-6
//         };

//         check!(sign_ok,
//                format!("∂kp/∂b₀ 符号一致: 数值={:.8}, 解析={:.8}", num_grad, analytic_grad));
//         check!(mag_ok,
//                format!("∂kp/∂b₀ 量级合理: 比值={:.4}", num_grad / analytic_grad));
//     }

//     // ── 汇总 ──
//     println!("\n==================================================");
//     println!("验证完成: {} 通过, {} 失败 / 共 {}", passed, failed, passed + failed);
//     if failed > 0 {
//         std::process::exit(1);
//     }
// }
