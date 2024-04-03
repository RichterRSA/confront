use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use dev_camera::DevCameraPlugin;
use save_load::SaveLoadPlugin;
use ui::{dev_ui::DevUIPlugin, widgets::CustomUIPlugin};
mod dev_camera;
mod save_load;
mod ui;
mod helpers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::RED))
        .add_systems(Startup, (spawn_block, spawn_light))
        .add_plugins(WorldInspectorPlugin::default()
            .run_if(input_toggle_active(false, KeyCode::Equals)))
        .add_plugins(SaveLoadPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(DevCameraPlugin)
        .add_plugins(CustomUIPlugin)
        .add_plugins(DevUIPlugin)
        .run();
}

fn spawn_block(
    mut commands: Commands,
    ass: Res<AssetServer>,
){
    let my_gltf = ass.load("models/block.glb#Scene0");

    commands.spawn(SceneBundle{
        scene: my_gltf,
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

fn spawn_light(
    mut commands: Commands,
){
    commands.spawn(DirectionalLightBundle{
        transform: Transform::from_rotation(Quat::from_scaled_axis(Vec3 { x: -1., y: 1., z: 0. })),
        directional_light: DirectionalLight {
            illuminance: 10000.,
            ..Default::default()
        },
        ..Default::default()
    });
}