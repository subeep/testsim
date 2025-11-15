use bevy::prelude::*;
use bevy_third_person_camera_2::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera.after(crate::player::spawn_player));
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