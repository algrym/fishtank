use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin,
                 LogDiagnosticsPlugin,
                 SystemInformationDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
};
use rand::Rng;

const _TIMESTEP_1_PER_SECOND: f32 = 1.0;

#[derive(Component)]
struct MobileFish {
    name: String,
}

#[derive(Component)]
struct Location {
    x: i32,
    y: i32,
}

fn spawn_fish(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        commands.spawn((MobileFish { name: i.to_string() }, Location { x: rng.gen_range(0..9), y: rng.gen_range(0..9) }));
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        });
    }
}

fn show_fish(query: Query<(&MobileFish, &Location)>) {
    for (fish, loc) in &query {
        info!("🐟{}({},{}) ", fish.name, loc.x, loc.y);
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.8))) // background color
        .add_plugins(DefaultPlugins.set(WindowPlugin { // set up window
            primary_window: Some(Window {
                fit_canvas_to_parent: true, // fill the entire browser window
                prevent_default_event_handling: false, // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                title: "Fishtank! - ajw@ajw.io".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_systems((setup, spawn_fish))
        .add_system(show_fish) // TODO: these run too frequently

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // SystemInformationDiagnostics don't work if you're dynamic linking. :|
        .add_plugin(SystemInformationDiagnosticsPlugin::default())

        .run();
}