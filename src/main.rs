use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    sprite::*,
    time::common_conditions::on_fixed_timer,
    utils::Duration,
    window::WindowResolution,
};
use rand::Rng;

const TIMESTEP_1_PER_SECOND: u64 = 1;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

// Remember, in Bevy's coordinate system the origin is at the center of the screen
const WINDOW_BOTTOM_Y: i32 = WINDOW_HEIGHT / -2;
const WINDOW_LEFT_X: i32 = WINDOW_WIDTH / -2;
const WINDOW_TOP_Y: i32 = WINDOW_HEIGHT / 2;
const WINDOW_RIGHT_X: i32 = WINDOW_WIDTH / 2;

#[derive(Component)]
struct MobileFish {
    name: String,
}

#[derive(Component)]
struct Location {
    x: i32,
    y: i32,
}

fn spawn_fish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        commands.spawn((
            MobileFish {
                name: i.to_string(),
            },
            Location {
                x: rng.gen_range(WINDOW_LEFT_X..WINDOW_RIGHT_X),
                y: rng.gen_range(WINDOW_BOTTOM_Y..WINDOW_TOP_Y),
            },
        ));
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform {
                translation: Vec3::new(
                    (WINDOW_LEFT_X as f32) + 100.0,
                    (WINDOW_BOTTOM_Y as f32) + 30.0,
                    0.1,
                ),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("dark.png"),
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, -1.0)),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE)) // background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // set up window
            primary_window: Some(Window {
                fit_canvas_to_parent: true, // fill the entire browser window
                resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                resizable: false,
                prevent_default_event_handling: false, // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                title: "Fishtank! - ajw@ajw.io".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_systems((setup, spawn_fish))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // SystemInformationDiagnostics don't work if you're dynamic linking. :|
        .add_plugin(SystemInformationDiagnosticsPlugin::default())
        .add_system(
            show_fish
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_secs(TIMESTEP_1_PER_SECOND))),
        )
        .run();
}
