use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    math::f32::Vec3,
    prelude::*,
    time::common_conditions::on_fixed_timer,
    utils::Duration,
    window::WindowResolution,
};
use rand::Rng;
use rand::seq::SliceRandom;

const TIMESTEP_1_PER_SECOND: u64 = 1;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

// Remember, in Bevy's coordinate system the origin is at the center of the screen
const WINDOW_BOTTOM_Y: i32 = WINDOW_HEIGHT / -2;
const WINDOW_LEFT_X: i32 = WINDOW_WIDTH / -2;
const WINDOW_TOP_Y: i32 = WINDOW_HEIGHT / 2;
const WINDOW_RIGHT_X: i32 = WINDOW_WIDTH / 2;

const SPRITE_SHEET_CELL_WIDTH: f32 = 62.0;
const SPRITE_SHEET_CELL_PADDING: f32 = 2.0;
const SPRITE_SHEET_COLUMNS: usize = 17;
const SPRITE_SHEET_ROWS: usize = 7;

const MAX_NUMBER_FISH: usize = 10;

// Names for all the fish sprite offsets in the texture atlas
const FISH_OFFSET_GREEN: usize = 68;
const FISH_OFFSET_PURPLE: usize = 70;
const FISH_OFFSET_BLUE: usize = 72;
const FISH_OFFSET_ORANGE: usize = 74;
const FISH_OFFSET_PUFFER: usize = 95;
const FISH_OFFSET_EEL: usize = 97;
const FISH_OFFSETS: [usize; 6] = [
    FISH_OFFSET_GREEN,
    FISH_OFFSET_PURPLE,
    FISH_OFFSET_BLUE,
    FISH_OFFSET_ORANGE,
    FISH_OFFSET_PUFFER,
    FISH_OFFSET_EEL,
];

#[derive(Component)]
struct MobileFish {
    name: String,
}

#[derive(Component)]
struct Direction {
    horizontal_speed: f32,
    vertical_speed: f32,
}

fn spawn_fish(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    let texture_handle = asset_server.load("fishTileSheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle,
                                Vec2::new(SPRITE_SHEET_CELL_WIDTH, SPRITE_SHEET_CELL_WIDTH),
                                SPRITE_SHEET_COLUMNS, SPRITE_SHEET_ROWS,
                                Some(Vec2::new(SPRITE_SHEET_CELL_PADDING, SPRITE_SHEET_CELL_PADDING)),
                                None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();
    for i in 0..MAX_NUMBER_FISH {
        info!("spawn_fish {}", i);
        commands.spawn((
            MobileFish {
                name: i.to_string(),
            },
            Direction {
                horizontal_speed: rng.gen_range(-5.0..5.0),
                vertical_speed: rng.gen_range(-5.0..5.0),
            },
            SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(WINDOW_LEFT_X..WINDOW_RIGHT_X) as f32,
                        rng.gen_range(WINDOW_BOTTOM_Y..WINDOW_TOP_Y) as f32,
                        0.1,
                    ),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite { index: *FISH_OFFSETS.choose(&mut rng).unwrap(), ..default() },
                ..Default::default()
            },
        ));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        // Background
        texture: asset_server.load("dark.png"),
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, -1.0)),
        ..Default::default()
    });
}

fn update_fish(mut query: Query<(&MobileFish, &mut Direction, &mut Transform)>) {
    let mut rng = rand::thread_rng();

    for (fish, mut fish_direction, mut fish_transform) in query.iter_mut() {
        debug!(
            "update_fish 🐟{}({}) h{} v{}",
            fish.name,
            fish_transform.translation,
            fish_direction.horizontal_speed,
            fish_direction.vertical_speed
        );

        if fish_direction.horizontal_speed > 0.0 {
            fish_transform.scale = Vec3::new(1.0, 1.0, 1.0);
            fish_direction.horizontal_speed += rng.gen_range(-1.0..1.5);
            if fish_direction.horizontal_speed < 0.0 {
                fish_direction.horizontal_speed = 0.0;
            }
        } else if fish_direction.horizontal_speed < 0.0 {
            fish_transform.scale = Vec3::new(-1.0, 1.0, 1.0);
            fish_direction.horizontal_speed -= rng.gen_range(-1.0..1.5);
            if fish_direction.horizontal_speed > 0.0 {
                fish_direction.horizontal_speed = 0.0;
            }
        } else {
            fish_direction.horizontal_speed += rng.gen_range(-0.5..0.5);
        }

        if fish_direction.vertical_speed > 0.0 {
            fish_direction.vertical_speed += rng.gen_range(-0.5..1.0);
            if fish_direction.vertical_speed < 0.0 {
                fish_direction.vertical_speed = 0.0;
            }
        } else if fish_direction.vertical_speed < 0.0 {
            fish_direction.vertical_speed -= rng.gen_range(-0.5..1.0);
            if fish_direction.vertical_speed > 0.0 {
                fish_direction.vertical_speed = 0.0;
            }
        } else {
            fish_direction.vertical_speed += rng.gen_range(-0.5..0.5);
        }
    }
}

fn move_fish(mut query: Query<(&MobileFish, &mut Direction, &mut Transform)>) {
    for (fish, mut fish_direction, mut fish_transform) in query.iter_mut() {
        debug!(
            "move_fish 🐟{}({}) h{} v{}",
            fish.name,
            fish_transform.translation,
            fish_direction.horizontal_speed,
            fish_direction.vertical_speed
        );
        fish_transform.translation.x += fish_direction.horizontal_speed;
        fish_transform.translation.y += fish_direction.vertical_speed;

        if (fish_transform.translation.x > WINDOW_RIGHT_X as f32)
            || (fish_transform.translation.x < WINDOW_LEFT_X as f32)
        {
            fish_direction.horizontal_speed *= -0.9;
        } else {
            fish_direction.horizontal_speed *= 0.9;
        }

        if (fish_transform.translation.y > WINDOW_TOP_Y as f32)
            || (fish_transform.translation.y < WINDOW_BOTTOM_Y as f32)
        {
            fish_direction.vertical_speed *= -0.9;
        } else {
            fish_direction.vertical_speed *= 0.9;
        }
    }
}

fn spawn_bubble() {
    info!("🫧");
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE)) // background color
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // avoid blurry sprites
                .set(WindowPlugin {
                    // set up window
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true, // fill the entire browser window
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as f32,
                            WINDOW_HEIGHT as f32,
                        ),
                        resizable: false,
                        prevent_default_event_handling: false, // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                        title: "Fishtank! - ajw@ajw.io".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_startup_systems((setup_camera, setup_background, spawn_fish))
        // SystemInformationDiagnostics don't work if you're dynamic linking. :|
        .add_plugin(SystemInformationDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(move_fish)
        .add_system(update_fish)
        .add_system(
            spawn_bubble
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_secs(TIMESTEP_1_PER_SECOND))),
        )
        .run();
}
