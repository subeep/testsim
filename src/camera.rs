use bevy::prelude::*;
use bevy_third_person_camera_2::*;
use crate::player::Player;

#[derive(Component)]
struct PlayerLight;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 1.0))) // Light blue sky color
            .add_systems(Startup, spawn_camera.after(crate::player::spawn_player,))
            .add_systems(Startup, spawn_light.after(crate::player::spawn_player))
            .add_systems(Update, update_light_position);
    }
}

fn spawn_camera(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    // Spawn third person camera targeting the player
    if let Ok(player_entity) = player_query.single() {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ThirdPersonCamera::aimed_at(player_entity),
        ));
    }
}


fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 2000.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
        PlayerLight,
    ));
}

fn update_light_position(
    mut light_query: Query<&mut Transform, (With<PlayerLight>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(mut light_transform) = light_query.single_mut() {
        if let Ok(player_transform) = player_query.single() {
            // Position light above the player
            light_transform.translation = player_transform.translation + Vec3::new(0.0, 5.0, 0.0);
        }
    }
}