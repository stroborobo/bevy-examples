use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    render::color::Color,
    sprite::collide_aabb::collide,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
    utils::*,
    DefaultPlugins,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct SlowZone {
    pub velocity_modifier: f32,
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0, 250.0, 0.0),
                scale: Vec3::new(100.0, 100.0, 00.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.7, 0.1, 0.1),
                ..default()
            },
            ..default()
        },
        SlowZone {
            velocity_modifier: 0.8,
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(20.0, 20.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        Velocity(Vec2::new(0.0, 0.0)),
        Player,
        Collider,
    ));
}

fn input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = query.single_mut();
    let diff = 5.0;
    if keyboard_input.pressed(KeyCode::Space) {
        player_velocity.x = 0.0;
        player_velocity.y = 0.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.y += diff;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.y -= diff;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.x -= diff;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.x += diff;
    }
}

fn slowzone(
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    slowzone_query: Query<(&Transform, &SlowZone)>,
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();
    for (transform, slowzone) in &slowzone_query {
        let maybe_collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(_) = maybe_collision {
            player_velocity.y *= slowzone.velocity_modifier;
            player_velocity.x *= slowzone.velocity_modifier;
        }
    }
}

fn movement(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (input, slowzone, movement).chain());
    }
}
