use bevy::prelude::*;
use press_here::{AppExt, AxisVisualizer, Deadzone, Normalize, Pair};

fn main() {
    let x_bindings = (
        Pair(KeyCode::KeyA, KeyCode::KeyD),
        Deadzone(GamepadAxis::LeftStickX, 0.1),
    );
    let y_bindings = (
        Pair(KeyCode::KeyS, KeyCode::KeyW),
        Deadzone(GamepadAxis::LeftStickY, 0.1),
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .add_axis::<X>(x_bindings)
        .add_axis::<Y>(y_bindings)
        .add_axis::<NormalizedX>(Normalize(x_bindings, y_bindings))
        .add_axis::<NormalizedY>(Normalize(y_bindings, x_bindings))
        .add_systems(Update, test)
        .add_systems(Startup, setup)
        .run();
}

pub struct X;
pub struct Y;
pub struct NormalizedX;
pub struct NormalizedY;

fn test(
    mut xy_visualizer: AxisVisualizer<X, Y>,
    mut normalized_visualizer: AxisVisualizer<NormalizedX, NormalizedY>,
) {
    xy_visualizer.axis_circle(Vec2::new(-250.0, 0.0), 200.0, 200.0, Srgba::RED);
    normalized_visualizer.axis_circle(Vec2::new(250.0, 0.0), 200.0, 200.0, Srgba::GREEN);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
