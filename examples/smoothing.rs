use bevy::prelude::*;
use press_here::{AppExt, Axis, Deadzone, Pair, Smooth};
use std::collections::VecDeque;

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
    mut gizmos: Gizmos,
    unsmoothed: Res<Axis<Unsmoothed>>,
    smoothed: Res<Axis<Smoothed>>,
    mut saved_values: Local<VecDeque<(f32, f32)>>,
) {
    let unsmoothed_value = unsmoothed.value();
    let smoothed_value = smoothed.value();

    saved_values.push_front((unsmoothed_value, smoothed_value));

    let unsmoothed_points = saved_values
        .iter()
        .enumerate()
        .map(|(i, (u, _))| Vec2::new(i as f32 * -5.0, *u * 100.0));

    let smoothed_points = saved_values
        .iter()
        .enumerate()
        .map(|(i, (_, s))| Vec2::new(i as f32 * -5.0, *s * 100.0));

    gizmos.linestrip_2d(unsmoothed_points, Srgba::RED);
    gizmos.linestrip_2d(smoothed_points, Srgba::GREEN);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
