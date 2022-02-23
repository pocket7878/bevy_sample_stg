/*
 * ボスの位置の移動のための線形、イージングの補完を行うを関数
 */
use bevy::prelude::*;

pub fn easing_vec3_linear_interpolation(t: f32, b: Vec3, c: Vec3, d: f32) -> Vec3 {
    interpolate_vec3(t, b, c, d, easing_linear_interpolation)
}

pub fn easing_vec3_ease_in_out_interpolation(t: f32, b: Vec3, c: Vec3, d: f32) -> Vec3 {
    interpolate_vec3(t, b, c, d, easing_ease_in_out_interpolation)
}

/* f32の補完の関数をVec3の各要素に適用する */
fn interpolate_vec3<F>(t: f32, b: Vec3, c: Vec3, d: f32, interpolate_function: F) -> Vec3
where
    F: Fn(f32, f32, f32, f32) -> f32,
{
    Vec3::new(
        interpolate_function(t, b.x, c.x, d),
        interpolate_function(t, b.y, c.y, d),
        interpolate_function(t, b.z, c.z, d),
    )
}

fn easing_ease_in_out_interpolation(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut tt = t / (d / 2.0);
    if tt < 1. {
        return c / 2.0 * tt * tt + b;
    }
    tt -= 1.;
    -c / 2.0 * (tt * (tt - 2.) - 1.) + b
}

fn easing_linear_interpolation(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * t / d + b
}
