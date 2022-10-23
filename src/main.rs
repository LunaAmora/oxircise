#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use oxilib::log;

fn main() {
    #[cfg(wasm)]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            fit_canvas_to_parent: true,
            width: 800.0,
            height: 600.0,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("emoji-dzuk/emoji/ferris_64.png"),
        ..default()
    });

    #[cfg(wasm)]
    log!("The wasm setup is done!");

    #[cfg(native)]
    log!("The non-wasm setup is done!");
}
