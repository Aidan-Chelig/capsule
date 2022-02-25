

use std::time::Duration;

pub use bevy::prelude::*;
use bevy_framepace::PowerSaver;

mod netclient;
use netclient::NetPlugin;

mod player;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContext};
use bevy_egui::EguiPlugin;
use player::MovementSettings;
use player::PlayerPlugin;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(NetPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EguiPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_plugin(bevy_framepace::FramepacePlugin {
            enabled: true,
            framerate_limit: bevy_framepace::FramerateLimit::Auto,
            warn_on_frame_drop: false,
            safety_margin: std::time::Duration::from_micros(100),
            power_saver: PowerSaver::Enabled(Duration::from_millis(500)),
        })
        .add_startup_system(setup)
        .add_system(ui_example)
        .run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(NetPlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup.system())
        .run();
}



//test
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.7).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
        //material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        material: materials.add(StandardMaterial { base_color: Color::rgb(1.0, 0.0, 0.0), metallic: 0.0, .. Default::default()}),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>
) {
    egui::Area::new("side_panel")
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("world");
            let button = egui::Button::new("Hellow");
            ui.add(button);
        });
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
