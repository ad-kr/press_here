use bevy::prelude::*;
use press_here::{AppExt, AxisVisualizer, Deadzone, Pair};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_axis::<AxisX>((
            Pair(KeyCode::KeyA, KeyCode::KeyD),
            Deadzone(GamepadAxis::LeftStickX, 0.1),
        ))
        .add_axis::<AxisY>((
            Pair(KeyCode::KeyS, KeyCode::KeyW),
            Deadzone(GamepadAxis::LeftStickY, 0.1),
        ))
        .add_systems(Update, test)
        .add_systems(Startup, setup)
        .run();
}

pub struct AxisX;
pub struct AxisY;

fn test(mut visualizer: AxisVisualizer<AxisX, AxisY>) {
    visualizer.graph_x(
        Duration::from_secs_f32(4.0),
        Vec2::ZERO,
        100.0,
        Vec2::splat(200.0),
        Srgba::GREEN,
    );
    visualizer.graph_y(
        Duration::from_secs_f32(4.0),
        Vec2::ZERO,
        100.0,
        Vec2::splat(200.0),
        Srgba::BLUE,
    );
    visualizer.axis_circle(Vec2::ZERO, 200.0, 200.0, Srgba::RED);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
