use bevy::prelude::*;
use press_here::{AppExt, Axis, Pair, Trigger};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_trigger::<Jump>((KeyCode::Space, KeyCode::Enter))
        .add_axis::<Walk>(Pair(KeyCode::KeyA, KeyCode::KeyD))
        .add_systems(Update, test)
        .run();
}

pub struct Jump;
pub struct Walk;

fn test(jump: Res<Trigger<Jump>>, walk: Res<Axis<Walk>>) {
    if jump.just_pressed() {
        println!("Jump!");
    }

    let walk_value = walk.value();
    if walk_value != 0.0 {
        println!("Walking with intensity: {}", walk_value);
    }
}
