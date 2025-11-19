use bevy::prelude::*;
use press_here::{AppExt, Axis, Pair, Trigger};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_trigger::<ChangeBinding>(KeyCode::Space)
        .add_axis::<SomeAxis>((Pair(KeyCode::KeyA, KeyCode::KeyD), GamepadAxis::LeftStickX))
        .add_systems(Update, change_axis_binding)
        .add_systems(Update, print_axis)
        .run();
}

pub struct ChangeBinding;
pub struct SomeAxis;

fn change_axis_binding(trigger: Res<Trigger<ChangeBinding>>, mut axis: ResMut<Axis<SomeAxis>>) {
    if trigger.just_pressed() {
        let current = axis.binding();
        let Some(mut all) = current.split() else {
            return;
        };

        all[0] = Box::new(Pair(KeyCode::ArrowLeft, KeyCode::ArrowRight));

        axis.set_binding(all);
    }
}

fn print_axis(axis: Res<Axis<SomeAxis>>) {
    println!("Axis value: {}", axis.value());
}
