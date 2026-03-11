#![allow(unused)]
mod asin_tab;
mod cos_tab;
mod sin_table;
use asin_tab::ASIN_TAB;
use cos_tab::COS_TAB;
use num_traits::Float;
use sin_table::SIN;

pub const PI: f32 = std::f32::consts::PI;
pub const HALF_PI: f32 = std::f32::consts::FRAC_2_PI;
pub const TWO_PI: f32 = std::f32::consts::TAU;
pub const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / std::f32::consts::PI;
pub const EPSILON: f32 = 1.0E-5;
pub const SQRT_OF_TWO: f32 = std::f32::consts::SQRT_2;
const SIN_SCALE: f64 = 10430.378350470453;
const MULTIPLY_DE_BRUIJN_BIT_POSITION: [i32; 32] = [
    0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8, 31, 27, 13, 23, 21, 19, 16, 7, 26,
    12, 18, 6, 11, 5, 10, 9,
];
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

pub fn equal<T: Float>(a: T, b: T) -> bool {
    let eps = T::from(1e-5).unwrap();
    (b - a).abs() < eps
}

pub fn positive_modulo<T: Float>(input: T, r#mod: T) -> T {
    (input % r#mod + r#mod) % r#mod
}

pub fn pack_degrees(angle: f32) -> i8 {
    (angle * 256.0 / 360.0).floor() as i8
}

pub fn unpack_degress(rot: i8) -> f32 {
    rot as f32 * 360.0 / 256.0
}

pub fn wrap_degress<T: Float>(angle: T) -> T {
    let deg180 = T::from(180).unwrap();
    let deg360 = T::from(360).unwrap();
    let a: T = angle % deg360;

    match a {
        x if x >= deg180 => x - deg360,
        x if x < -deg180 => x + deg360,
        x => x,
    }
}
