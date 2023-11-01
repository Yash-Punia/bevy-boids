mod constants;
mod utils;

use std::f32::consts::PI;

use bevy::prelude::*;
use constants::*;
use utils::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, (
            separation_system.before(move_bird),
            alignment_system.before(move_bird),
            move_bird,
        ))
        .run();
}


#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Close(Vec2);

#[derive(Component)]
struct Alignment {
    average_pos: Vec2,
    average_velocity: Vec2,
    neighbouring: i32
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    // single bird
    // let bird_tex = asset_server.load("textures/bird.png");
    let w = window.single();
    for i in 0..100 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: random_vec3_window(w),
                    rotation: Quat::from_axis_angle(Vec3::Z, random_f(0.0, 2.0 * PI)),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                // texture: bird_tex.clone(),
                ..default()
            },
            Close(Vec2::ZERO),
            Alignment {average_pos: Vec2::ZERO, average_velocity: Vec2::ZERO, neighbouring: 0},
            Velocity(random_direction_vec2() * BIRD_STARTING_SPEED),
        ));
    }
}

fn move_bird(
    time_step: Res<FixedTime>,
    window_query: Query<&Window>,
    mut bird_query: Query<(&mut Transform, &mut Velocity)>) {

    for (mut transform, mut velocity) in &mut bird_query {
        let window = window_query.single();
        let dt = time_step.period.as_secs_f32();

        velocity.x = velocity.x.signum() * velocity.x.abs().clamp(BIRD_MIN_SPEED, BIRD_MAX_SPEED);
        velocity.y = velocity.y.signum() * velocity.y.abs().clamp(BIRD_MIN_SPEED, BIRD_MAX_SPEED);

        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;

        if (transform.translation.x <= -window.width()/2.0 || transform.translation.x >= window.width()/2.0) {
            transform.translation.x *= -1.0;
        }

        if (transform.translation.y <= -window.height()/2.0 || transform.translation.y >= window.height()/2.0) {
            transform.translation.y *= -1.0;
        }
    }
}

fn separation_system(
    mut bird_query: Query<(&mut Transform, &mut Velocity, &mut Close)>,
) {
    let mut combinations = bird_query.iter_combinations_mut();

    while let Some([
        (bird, _bird_vel, mut bird_close),
        (other, _other_vel, mut other_close)
        ]) = combinations.fetch_next() {
        if bird.eq(&other) {
            continue;
        }

        let distance = bird.translation.distance(other.translation);
        if distance < PROTECTED_RANGE {
            bird_close.x += bird.translation.x - other.translation.x;
            bird_close.y += bird.translation.y - other.translation.y;

            other_close.x += other.translation.x - bird.translation.x;
            other_close.y += other.translation.y - bird.translation.y;
        }
    }

    for (_bird, mut velocity, mut close) in &mut bird_query {
        velocity.x += close.x * AVOID_FACTOR;
        velocity.y += close.y * AVOID_FACTOR;

        close.x = 0.0;
        close.y = 0.0;
    }

}

fn alignment_system (
    mut bird_query: Query<(&mut Transform, &mut Velocity, &mut Alignment)>,
) {
    let mut combinations = bird_query.iter_combinations_mut();

    while let Some([
        (bird, bird_vel, mut alignment),
        (other, other_vel, mut other_alignment)
        ]) = combinations.fetch_next() {
        if bird.eq(&other) {
            continue;
        }

        let distance = bird.translation.distance(other.translation);
        if distance < VISIBLE_RANGE && distance > PROTECTED_RANGE {
            alignment.neighbouring += 1;
            other_alignment.neighbouring += 1;
            
            alignment.average_velocity.x += other_vel.x;
            alignment.average_velocity.y += other_vel.y;

            other_alignment.average_velocity.x += bird_vel.x;
            other_alignment.average_velocity.y += bird_vel.y;

            alignment.average_pos.x += other.translation.x;
            alignment.average_pos.y += other.translation.y;

            other_alignment.average_pos.x += bird.translation.x;
            other_alignment.average_pos.y += bird.translation.y;
        }
    }

    for (mut bird, mut velocity, mut alignment) in &mut bird_query {
        if alignment.neighbouring < 1 {
            continue;
        }

        alignment.average_velocity.x = alignment.average_velocity.x / alignment.neighbouring as f32;
        alignment.average_velocity.y = alignment.average_velocity.y / alignment.neighbouring as f32;

        velocity.x += (alignment.average_velocity.x - velocity.x) * MATCHING_FACTOR;
        velocity.y += (alignment.average_velocity.y - velocity.y) * MATCHING_FACTOR;

        alignment.average_pos.x = alignment.average_pos.x / alignment.neighbouring as f32;
        alignment.average_pos.y = alignment.average_pos.y / alignment.neighbouring as f32;

        bird.translation.x += (alignment.average_pos.x - bird.translation.x) * CENTERING_FACTOR;
        bird.translation.y += (alignment.average_pos.y - bird.translation.y) * CENTERING_FACTOR;

        alignment.average_pos = Vec2::ZERO;
        alignment.average_velocity = Vec2::ZERO;
        alignment.neighbouring = 0;
    }
}