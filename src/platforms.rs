use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_BOTTOM_Y;

const COLOR_PLATFORM: Color = Color::rgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0), 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PlatformBundle::new(0.0, Vec3::new(100.0, 50.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(100.0, 100.0, 1.0)));
    commands.spawn(PlatformBundle::new(200.0, Vec3::new(100.0, 150.0, 1.0)));
    commands.spawn(PlatformBundle::new(300.0, Vec3::new(100.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(400.0, Vec3::new(100.0, 250.0, 1.0)));
    commands.spawn(PlatformBundle::new(500.0, Vec3::new(100.0, 300.0, 1.0)));
}
