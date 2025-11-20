use bevy::prelude::*;
use press_here::{AppExt, Axis, AxisBindingBuilder, Pair, Trigger};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Space key or gamepad south button for jump
        .add_trigger::<Jump>((KeyCode::Space, GamepadButton::South))
        .add_axis::<Walk>((
            // A/D keys for left/right movement
            Pair(KeyCode::KeyA, KeyCode::KeyD),
            // Left stick X-axis with deadzone
            GamepadAxis::LeftStickX.deadzone(0.1),
        ))
        .add_systems(Update, update)
        .run();
}

struct Jump;
struct Walk;

fn update(jump: Res<Trigger<Jump>>, walk: Res<Axis<Walk>>) {
    if jump.just_pressed() {
        jump_character();
    }

    let walk_value = walk.value();
    if walk_value != 0.0 {
        walk_character(walk_value);
    }
}

fn jump_character() {
    println!("Character jumped!");
}

fn walk_character(value: f32) {
    println!("Character walking with intensity: {}", value);
}
