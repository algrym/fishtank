use bevy::prelude::*;

#[derive(Component)]
struct MobileFish {
    name: String,
}

#[derive(Component)]
struct Location {
    x: i32,
    y: i32,
}

fn add_fish(mut commands: Commands) {
    for i in 0..9 {
        commands.spawn((MobileFish { name: i.to_string() }, Location { x: i, y: i }));
    }
}

fn show_fish(query: Query<(&MobileFish, &Location)>) {
    for (fish, loc) in &query {
        println!("Fish {} at {},{}", fish.name, loc.x, loc.y);
    }
}

fn main() {
    App::new()
        .add_startup_system(add_fish)
        .add_system(show_fish)
        .run();
}