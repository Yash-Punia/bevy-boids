use bevy::prelude::*;
use rand::Rng;

pub fn random_direction_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-1.0..1.0);
    let y: f32 = rng.gen_range(-1.0..1.0);
    Vec2::new(x, y).normalize()
}

pub fn random_vec3_window(window: &Window) -> Vec3 {
    let min_x = -window.width() / 2.0;
    let max_x = window.width() / 2.0;
    let min_y = -window.height() / 2.0;
    let max_y = window.height() / 2.0;
    let x = random_f(min_x, max_x);
    let y = random_f(min_y, max_y);
    Vec3::new(x, y, 0.0)
}

pub fn random_f(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
