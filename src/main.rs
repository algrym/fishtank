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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};
use std::ops::Index;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 768;

// Remember, in Bevy's coordinate system the origin is at the center of the screen
const WINDOW_BOTTOM_Y: i32 = WINDOW_HEIGHT / -2;
const WINDOW_BOTTOM_Y_SEAFLOOR: i32 = (WINDOW_BOTTOM_Y as f32 * 0.5) as i32;
const WINDOW_LEFT_X: i32 = WINDOW_WIDTH / -2;
const WINDOW_TOP_Y: i32 = WINDOW_HEIGHT / 2;
const WINDOW_RIGHT_X: i32 = WINDOW_WIDTH / 2;
const STANDARD_Z: f32 = 1.0;

const MIN_NUMBER_FISH: usize = 5;
const MAX_NUMBER_FISH: usize = 20;

const PIXELS_PER_METER: f32 = 100.0;

const BUBBLE_RADIUS: f32 = 15.0;
const BUBBLE_RESTITUTION: f32 = 0.7;
const BUBBLE_GRAVITY: f32 = 0.1;
const BUBBLE_DENSITY: f32 = 0.1;
const BUBBLE_SPAWNS_IN_SECS: u64 = 1;

const FISH_RADIUS: f32 = 32.0;
const FISH_RESTITUTION: f32 = 0.3;
const FISH_MASS: f32 = 5.0;
const FISH_GRAVITY: f32 = 1.5;

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
    _name: String,
}

#[derive(Component)]
struct MobileBubble {}

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
        commands
            .spawn(RigidBody::Dynamic)
            .insert(Name::new(format!("Fish {}", i)))
            .insert(Sleeping::disabled())
            .insert(GravityScale(FISH_GRAVITY))
            .insert(Ccd::enabled())
            .insert(Collider::ball(FISH_RADIUS))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Restitution::coefficient(FISH_RESTITUTION))
            .insert(ColliderMassProperties::Mass(FISH_MASS))
            .insert(Dominance::group(10))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalForce {
                force: Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-25.0..25.0)),
                torque: 0.0,
            })
            .insert(Velocity {
                linvel: Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-25.0..25.0)),
                angvel: 0.0,
            })
            .insert(MobileFish {
                _name: format!("Fish {}", i),
            })
            .insert(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(WINDOW_LEFT_X..WINDOW_RIGHT_X) as f32,
                        rng.gen_range(WINDOW_BOTTOM_Y_SEAFLOOR..WINDOW_TOP_Y) as f32,
                        STANDARD_Z,
                    ),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.sprite.clone(),
                sprite: TextureAtlasSprite {
                    // index: *FISH_OFFSETS.choose(rng).unwrap(),
                    index: *FISH_OFFSETS.index(i % FISH_OFFSETS.len()),
                    ..default()
                },
                ..Default::default()
            });
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("fishBackground.png"),
            ..Default::default()
        })
        .insert(Name::new("Background"));
}

// TODO: add fish logic
fn fish_logic(mut query: Query<(&MobileFish, &mut Velocity, &mut Transform)>) {
    for (fish, fish_velocity, mut fish_transform) in query.iter_mut() {
        debug!("update_fish 🐟{} v{:?}", fish._name, fish_velocity);
        if fish_velocity.linvel.x > 0.0 {
            fish_transform.scale = Vec3::new(1.0, 1.0, 1.0);
        } else if fish_velocity.linvel.x < 0.0 {
            fish_transform.scale = Vec3::new(-1.0, 1.0, 1.0);
        }
    }
}

// TODO: add fish forces
fn _fish_forces(mut query: Query<&mut ExternalForce, With<MobileFish>>) {
    info!("fish_forces {:?}", query);
    for nudge in query.iter_mut() {
        info!("fish_forces nudge {:?}", nudge);
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
    // Pick a random fish
    let Some((fish_transform, _)) = query.iter().choose(&mut rng) else { return; };
    info!("🫧🐟{} #{}", fish_transform.translation, query.iter().len());

    // Spawn a bubble
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Name::new("Bubble"))
        .insert(Sleeping::disabled())
        .insert(GravityScale(BUBBLE_GRAVITY))
        .insert(Ccd::enabled())
        .insert(Collider::ball(BUBBLE_RADIUS))
        .insert(*fish_transform)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(BUBBLE_RESTITUTION))
        .insert(ColliderMassProperties::Density(BUBBLE_DENSITY))
        .insert(Dominance::group(0))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 1.0),
            torque: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 2.0),
            angvel: 0.0,
        })
        .insert(SpriteSheetBundle {
            transform: *fish_transform,
            texture_atlas: texture_atlas_handle.sprite.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..Default::default()
        })
        .insert(animation_indices)
        .insert(AnimationTimer(Timer::from_seconds(
            rng.gen_range(1.0..3.0),
            TimerMode::Repeating,
        )))
        .insert(MobileBubble {});
}

fn bubble_reaper(
    mut commands: Commands,
    mut query: Query<(Entity, &MobileBubble, &mut Transform)>,
) {
    for (bubble_entity, _bubble, bubble_transform) in query.iter_mut() {
        // despawn bubbles when they get just past the edge of the screen
        if bubble_transform.translation.x <= WINDOW_LEFT_X as f32 - (BUBBLE_RADIUS * 2.0)
            || bubble_transform.translation.y >= WINDOW_RIGHT_X as f32 + (BUBBLE_RADIUS * 2.0)
            || bubble_transform.translation.y >= WINDOW_TOP_Y as f32 + (BUBBLE_RADIUS * 2.0)
            || bubble_transform.translation.y
                <= WINDOW_BOTTOM_Y_SEAFLOOR as f32 - (BUBBLE_RADIUS * 2.0)
        {
            commands.entity(bubble_entity).despawn();
        }
    }
}

// apply a random "nudge" to the bubbles
fn bubble_forces(mut query: Query<&mut ExternalForce, With<MobileBubble>>) {
    debug!("bubble_forces {:?}", query);
    for mut nudge in query.iter_mut() {
        debug!("bubble_forces nudge {:?}", nudge);
        nudge.force = Vec2::new(thread_rng().gen_range(-5.0..5.0), 1.0);
        nudge.torque = 0.0;
    }
}

fn fish_constraints(
    mut query_fish: Query<(&Transform, &mut Velocity, &mut ExternalForce), With<MobileFish>>,
) {
    for (fish_transform, mut fish_velocity, mut fish_external_force) in query_fish.iter_mut() {
        if fish_transform.translation.x >= WINDOW_RIGHT_X as f32 + (FISH_RADIUS * 2.0)
            || fish_transform.translation.x <= WINDOW_LEFT_X as f32 - (FISH_RADIUS * 2.0)
        {
            fish_velocity.linvel.x *= -0.9;
            fish_external_force.force.x *= -0.9;
        }

        if fish_transform.translation.y >= WINDOW_TOP_Y as f32 + (FISH_RADIUS * 2.0)
            || fish_transform.translation.y <= WINDOW_BOTTOM_Y_SEAFLOOR as f32 - (FISH_RADIUS * 2.0)
        {
            fish_velocity.linvel.y *= -0.9;
            fish_external_force.force.y *= -0.9;
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
                        title: format!("Fishtank! v{} - ajw@ajw.io", env!("CARGO_PKG_VERSION")),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Ensure assets are loaded before using them
        .init_collection::<FishSpriteSheet>()
        .add_startup_systems((setup_camera, setup_background, spawn_fish))
        // Load diagnostic plugins
        // SystemInformationDiagnostics don't work if you're dynamic linking. :|
        .add_plugin(SystemInformationDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Load physics plugins
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(
            // Bubbles only get spawned on a scheduled timer
            spawn_bubble
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_secs(BUBBLE_SPAWNS_IN_SECS))),
        )
        .add_system(bubble_reaper)
        .add_system(bubble_forces)
        .add_system(fish_logic)
        .add_system(fish_constraints)
        .add_system(animate_sprite)
        .run();
}
