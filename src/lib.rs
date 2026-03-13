mod asin_tab;
mod cos_tab;
mod sin_table;

use asin_tab::ASIN_TAB;
use cos_tab::COS_TAB;
use num_traits::{Euclid, Float, FromPrimitive, Signed};
use sin_table::SIN;
use std::f32::consts::PI;
use std::f64::consts::PI as PI_64;

pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;
const SIN_SCALE: f64 = 10430.378350470453;
const ONE_SIXTH: f64 = 0.16666666666666666;
const FRAC_BIAS: f64 = f64::from_bits(4805340802404319232u64);

pub fn sin(x: f64) -> f32 {
    let idx = (((x * SIN_SCALE) as i64 as u64) & 65535) as usize;
    SIN[idx]
}

pub fn cos(x: f64) -> f32 {
    let idx = (((x * SIN_SCALE + 16384.0) as i64 as u64) & 65535) as usize;
    SIN[idx]
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn floor<T: Float>(v: T) -> T {
    let _ = v;
    todo!()
}

pub fn lfloor(v: f64) -> i64 {
    let _ = v;
    todo!()
}

pub fn abs<T: Signed>(v: T) -> T {
    v.abs()
}

pub fn ceil<T: Float>(v: T) -> i32 {
    let _ = v;
    todo!()
}

pub fn ceil_long(v: f64) -> i64 {
    let _ = v;
    todo!()
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    num_traits::clamp(value, min, max)
}

pub fn clamped_lerp<T: Float>(factor: T, min: T, max: T) -> T {
    let _ = (factor, min, max);
    todo!()
}

pub fn abs_max<T: Signed + PartialOrd>(a: T, b: T) -> T {
    let abs_a = a.abs();
    let abs_b = b.abs();
    if abs_a > abs_b { abs_a } else { abs_b }
}

pub fn chessboard_distance(x0: i32, z0: i32, x1: i32, z1: i32) -> i32 {
    abs_max(x1 - x0, z1 - z0)
}

pub fn floor_div(a: i32, b: i32) -> i32 {
    a.div_euclid(b)
}

pub fn equal<T: Float>(a: T, b: T) -> bool {
    let eps = T::from(1e-5).unwrap();
    (b - a).abs() < eps
}

pub fn positive_modulo<T: Euclid>(input: T, r#mod: T) -> T {
    input.rem_euclid(&r#mod)
}

pub fn is_miltiple_of(dividend: i32, divisor: i32) -> bool {
    dividend % divisor == 0
}

pub fn pack_degrees(angle: f32) -> i8 {
    (angle * 256.0 / 360.0).floor() as i8
}

pub fn unpack_degress(rot: i8) -> f32 {
    rot as f32 * 360.0 / 256.0
}

pub fn wrap_degrees<T>(angle: T) -> T
where
    T: Signed + Copy + FromPrimitive + PartialOrd,
{
    let deg180 = T::from_i32(180).unwrap();
    let deg360 = T::from_i32(360).unwrap();
    let a: T = angle % deg360;

    match a {
        x if x >= deg180 => x - deg360,
        x if x < -deg180 => x + deg360,
        x => x,
    }
}

pub fn degrees_difference(from_angle: f32, to_angle: f32) -> f32 {
    wrap_degrees(to_angle - from_angle)
}

pub fn degrees_difference_abs(angle_a: f32, angle_b: f32) -> f32 {
    degrees_difference(angle_a, angle_b).abs()
}

pub fn rotate_if_necessary(base_angle: f32, target_angle: f32, max_angle_diff: f32) -> f32 {
    let _ = (base_angle, target_angle, max_angle_diff);
    todo!()
}

pub fn approach(current: f32, target: f32, increment: f32) -> f32 {
    let _ = (current, target, increment);
    todo!()
}

pub fn approach_degress(current: f32, target: f32, increment: f32) -> f32 {
    let _ = (current, target, increment);
    todo!()
}

pub fn get_int(input: &str, default: i32) -> i32 {
    input.parse::<i32>().unwrap_or(default)
}

pub fn smallest_encompassing_power_of_two(input: i32) -> i32 {
    let _ = input;
    todo!()
}

pub fn is_power_of_two(input: u32) -> bool {
    input.is_power_of_two()
}

pub fn ceillog2(input: u32) -> u32 {
    if is_power_of_two(input) {
        log2(input)
    } else {
        log2(input) + 1
    }
}

pub fn log2(input: u32) -> u32 {
    input.ilog2()
}

pub fn frac<T: Float>(num: T) -> T {
    let _ = num;
    todo!()
}

pub fn inverse_lerp<T: Float>(value: T, min: T, max: T) -> T {
    let _ = (value, min, max);
    todo!()
}

pub fn atan2(y: f64, x: f64) -> f64 {
    let mut x = x;
    let mut y = y;
    let d2 = x * x + y * y;

    if d2.is_nan() {
        return f64::NAN;
    }

    let neg_y = y < 0.0;
    if neg_y {
        y = -y;
    }

    let neg_x = x < 0.0;
    if neg_x {
        x = -x;
    }

    let steep = y > x;
    if steep {
        (x, y) = (y, x)
    }

    let rinv = fast_inv_sqrt(d2);

    x *= rinv;
    y *= rinv;

    let yp = FRAC_BIAS + y;
    let index = (yp.to_bits() & 0xFFFF_FFFF) as usize;
    let phi = ASIN_TAB[index];
    let c_phi = COS_TAB[index];
    let s_phi = yp - FRAC_BIAS;

    let sd = y * c_phi - x * s_phi;

    let d = (6.0 + sd * sd) * sd * ONE_SIXTH;

    let mut theta = phi + d;

    if steep {
        theta = PI_64 / 2.0 - theta
    }

    if neg_x {
        theta = PI_64 - theta
    }

    if neg_y {
        theta = -theta
    }

    theta
}

pub fn inv_sqrt<T: Float>(x: T) -> T {
    x.sqrt().recip()
}

pub fn fast_inv_sqrt(x: f64) -> f64 {
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 6910469410427058090_u64 - (i >> 1);
    let y = f64::from_bits(i);

    y * (1.5 - xhalf * y * y)
}

pub fn fast_inv_cube_root(x: f32) -> f32 {
    let _ = x;
    todo!()
}

pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> i32 {
    let _ = (hue, saturation, value);
    todo!()
}

pub fn hsv_to_argb(hue: f32, saturation: f32, value: f32, alpha: i32) -> i32 {
    let _ = (hue, saturation, value, alpha);
    todo!()
}

pub fn lerp_int(alpha1: f32, p0: i32, p1: i32) -> i32 {
    let _ = (alpha1, p0, p1);
    todo!()
}

pub fn lerp_discrete(alpha1: f32, p0: i32, p1: i32) -> i32 {
    let _ = (alpha1, p0, p1);
    todo!()
}

pub fn lerp<T: Float>(alpha1: T, p0: T, p1: T) -> T {
    p0 + alpha1 * (p1 - p0)
}

pub fn lerp2(alpha1: f64, alpha2: f64, x00: f64, x10: f64, x01: f64, x11: f64) -> f64 {
    lerp(alpha2, lerp(alpha1, x00, x10), lerp(alpha1, x01, x11))
}

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    alpha1: f64,
    alpha2: f64,
    alpha3: f64,
    x000: f64,
    x100: f64,
    x010: f64,
    x110: f64,
    x001: f64,
    x101: f64,
    x011: f64,
    x111: f64,
) -> f64 {
    lerp(
        alpha3,
        lerp2(alpha1, alpha2, x000, x100, x010, x110),
        lerp2(alpha1, alpha2, x001, x101, x011, x111),
    )
}

pub fn catmullrom(alpha: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let _ = (alpha, p0, p1, p2, p3);
    todo!()
}

pub fn smoothstep(x: f64) -> f64 {
    let _ = x;
    todo!()
}

pub fn smoothstep_derivative(x: f64) -> f64 {
    let _ = x;
    todo!()
}

pub fn sign(number: f64) -> i32 {
    number.signum() as i32
}

pub fn rot_lerp<T>(a: T, from: T, to: T) -> T
where
    T: Signed + Copy + FromPrimitive + PartialOrd + Float,
{
    from + a * wrap_degrees(to - from)
}

pub fn rot_lerp_rad(a: f32, from: f32, to: f32) -> f32 {
    let diff = (to - from + PI).rem_euclid(2.0 * PI) - PI;
    from + a * diff
}

pub fn triangle_wave(index: f32, period: f32) -> f32 {
    let _ = (index, period);
    todo!()
}

pub fn clamped_map<T: Float>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T {
    clamped_lerp(inverse_lerp(value, from_min, from_max), to_min, to_max)
}

pub fn map<T: Float>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T {
    lerp(inverse_lerp(value, from_min, from_max), to_min, to_max)
}

pub fn round_toward(input: i32, multiple: i32) -> i32 {
    positive_ceil_div(input, multiple) * multiple
}

pub fn positive_ceil_div(input: i32, divisor: i32) -> i32 {
    -floor_div(-input, divisor)
}

pub fn length_squared<T: Float>(x: T, y: T) -> T {
    x * x + y * y
}

pub fn length<T: Float>(x: T, y: T) -> T {
    length_squared(x, y).sqrt()
}

pub fn length_squared_3<T: Float>(x: T, y: T, z: T) -> T {
    x * x + y * y + z * z
}

pub fn length_3<T: Float>(x: T, y: T, z: T) -> T {
    length_squared_3(x, y, z).sqrt()
}

pub fn quantize(value: f64, quantize_resolution: f64) -> f64 {
    floor(value / quantize_resolution) * quantize_resolution
}
