use bevy::{math::VectorSpace, prelude::*};
use press_here::{AppExt, AxisVisualizer, Deadzone, Pair, Smooth};
use std::time::Duration;

fn main() {
    let bindings = (
        Pair(KeyCode::KeyA, KeyCode::KeyD),
        Deadzone(GamepadAxis::LeftStickX, 0.1),
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .add_axis::<Unsmoothed>(bindings)
        .add_axis::<Smoothed>(Smooth::new(bindings, 0.1))
        .add_systems(Startup, setup)
        .add_systems(Update, test)
        .run();
}

pub struct Unsmoothed;
pub struct Smoothed;

fn test(
    mut unsmoothed_visualizer: AxisVisualizer<Unsmoothed>,
    mut smoothed_visualizer: AxisVisualizer<Smoothed>,
) {
    unsmoothed_visualizer.graph_x(
        Duration::from_secs_f32(5.0),
        Vec2::ZERO,
        100.0,
        Vec2::new(400.0, 200.0),
        Srgba::RED,
    );
    smoothed_visualizer.graph_x(
        Duration::from_secs_f32(5.0),
        Vec2::ZERO,
        100.0,
        Vec2::new(400.0, 200.0),
        Srgba::GREEN,
    );
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
