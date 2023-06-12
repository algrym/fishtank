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
use bevy_asset_loader::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::{seq::IteratorRandom, seq::SliceRandom, thread_rng, Rng};
use bevy_rapier2d::prelude::*;

const BUBBLE_SPAWNS_IN_SECS: u64 = 2;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 768;

// Remember, in Bevy's coordinate system the origin is at the center of the screen
const WINDOW_BOTTOM_Y: i32 = WINDOW_HEIGHT / -2;
const WINDOW_BOTTOM_Y_SEAFLOOR: i32 = (WINDOW_BOTTOM_Y as f32 * 0.6) as i32;
const WINDOW_LEFT_X: i32 = WINDOW_WIDTH / -2;
const WINDOW_TOP_Y: i32 = WINDOW_HEIGHT / 2;
const WINDOW_RIGHT_X: i32 = WINDOW_WIDTH / 2;

const MIN_NUMBER_FISH: usize = 5;
const MAX_NUMBER_FISH: usize = 20;

const BUBBLE_RISE_SPEED: f32 = 1.0;

// Names for all the fish sprite offsets in the texture atlas
const FISH_OFFSET_GREEN: usize = 69;
const FISH_OFFSET_PURPLE: usize = 71;
const FISH_OFFSET_BLUE: usize = 73;
const FISH_OFFSET_ORANGE: usize = 75;
const FISH_OFFSET_PUFFER: usize = 96;
const FISH_OFFSET_EEL: usize = 98;
const DECOR_OFFSET_BUBBLE_BIG_OPEN: usize = 117;
const DECOR_OFFSET_BUBBLE_SMALL_FILLED: usize = 118;
const _DECOR_OFFSET_BUBBLE_SMALL_OPEN: usize = 119; // TODO: Fix runtime crash when accessing element 119

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
struct MobileBubble {}

#[derive(Component)]
struct Direction {
    horizontal_speed: f32,
    vertical_speed: f32,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(AssetCollection, Resource)]
struct FishSpriteSheet {
    // sadly, the "derive" crashes if I use the const's.
    #[asset(texture_atlas(
    tile_size_x = 63.0,
    tile_size_y = 63.0,
    columns = 17,
    rows = 7,
    padding_x = 1.0,
    padding_y = 1.0
    ))]
    #[asset(path = "fishTileSheet.png")]
    sprite: Handle<TextureAtlas>,
}

fn spawn_fish(mut commands: Commands, texture_atlas_handle: Res<FishSpriteSheet>) {
    info!(
        "spawn_fish: bottom_y={} seafloor={}",
        WINDOW_BOTTOM_Y, WINDOW_BOTTOM_Y_SEAFLOOR
    );
    let mut rng = thread_rng();
    for i in 0..rng.gen_range(MIN_NUMBER_FISH..MAX_NUMBER_FISH) {
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
                        rng.gen_range(WINDOW_BOTTOM_Y_SEAFLOOR..WINDOW_TOP_Y) as f32,
                        0.1,
                    ),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.sprite.clone(),
                sprite: TextureAtlasSprite {
                    index: *FISH_OFFSETS.choose(&mut rng).unwrap(),
                    ..default()
                },
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
        texture: asset_server.load("fishBackground.png"),
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, -1.0)),
        ..Default::default()
    });
}

fn fish_logic(mut query: Query<(&MobileFish, &mut Direction, &mut Transform)>) {
    let mut rng = thread_rng();

    for (fish, mut fish_direction, mut fish_transform) in query.iter_mut() {
        debug!(
            "update_fish 🐟{}@({}) s{} h{} v{}",
            fish.name,
            fish_transform.translation,
            fish_transform.scale,
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

        if fish_transform.translation.x > WINDOW_RIGHT_X as f32 {
            fish_transform.translation.x = WINDOW_RIGHT_X as f32;
            fish_direction.horizontal_speed *= -0.9;
        } else if fish_transform.translation.x < WINDOW_LEFT_X as f32 {
            fish_transform.translation.x = WINDOW_LEFT_X as f32;
            fish_direction.horizontal_speed *= -0.9;
        } else {
            fish_direction.horizontal_speed *= 0.9;
        }

        if fish_transform.translation.y > WINDOW_TOP_Y as f32 {
            fish_transform.translation.y = WINDOW_TOP_Y as f32;
            fish_direction.vertical_speed *= -0.9;
        } else if fish_transform.translation.y < WINDOW_BOTTOM_Y_SEAFLOOR as f32 {
            fish_transform.translation.y = WINDOW_BOTTOM_Y_SEAFLOOR as f32;
            fish_direction.vertical_speed *= -0.9;
        } else {
            fish_direction.vertical_speed *= 0.9;
        }
    }
}

fn spawn_bubble(
    mut commands: Commands,
    texture_atlas_handle: Res<FishSpriteSheet>,
    query: Query<(&Transform, With<MobileFish>)>,
) {
    let mut rng = thread_rng();
    let animation_indices = AnimationIndices {
        first: DECOR_OFFSET_BUBBLE_SMALL_FILLED,
        last: DECOR_OFFSET_BUBBLE_BIG_OPEN,
    };
    let Some((fish_transform, _fish)) = query.iter().choose(&mut rng) else { return; };
    info!("🫧🐟{} #{}", fish_transform.translation, query.iter().len());

    commands.spawn((
        MobileBubble {},
        SpriteSheetBundle {
            transform: *fish_transform,
            texture_atlas: texture_atlas_handle.sprite.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(rng.gen_range(1.0..3.0), TimerMode::Repeating)),
    ));
}

fn move_bubble(mut commands: Commands, mut query: Query<(Entity, &MobileBubble, &mut Transform)>) {
    for (bubble_entity, _bubble, mut bubble_transform) in query.iter_mut() {
        bubble_transform.translation.x += thread_rng().gen_range(-2.0..2.0);
        bubble_transform.translation.y += BUBBLE_RISE_SPEED + thread_rng().gen_range(-1.0..1.0);

        if bubble_transform.translation.y > WINDOW_TOP_Y as f32 {
            commands.entity(bubble_entity).despawn();
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else if indices.first < indices.last {
                sprite.index + 1
            } else if indices.first > indices.last {
                sprite.index - 1
            } else {
                // I'm not sure we can get here, but eh.
                sprite.index
            };
        }
    }
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
        .init_collection::<FishSpriteSheet>()
        .add_startup_systems((setup_camera, setup_background, spawn_fish))
        // SystemInformationDiagnostics don't work if you're dynamic linking. :|
        .add_plugin(SystemInformationDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vect::NEG_Y,
            ..Default::default()
        })
        // .add_plugin(WorldInspectorPlugin::new())
        .add_system(animate_sprite)
        .add_system(move_fish)
        .add_system(fish_logic)
        .add_system(
            spawn_bubble
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_secs(BUBBLE_SPAWNS_IN_SECS))),
        )
        .add_system(move_bubble)
        .run();
}
