use bevy::{prelude::*, DefaultPlugins};
use bevy_third_person_camera_2::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;


mod camera;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)              // instantiate if needed: PlayerPlugin::new()
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin::default())
        .run();
}