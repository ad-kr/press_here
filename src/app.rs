use crate::{
    axis::{Axis, AxisBinding},
    inputs::{Inputs, InputsSystemParam},
    trigger::{Trigger, TriggerBinding},
};
use bevy::{
    app::{App, PreUpdate},
    ecs::system::ResMut,
};

pub trait AppExt {
    fn add_trigger<T: Send + Sync + 'static>(&mut self, binding: impl TriggerBinding) -> &mut Self;
    fn add_axis<A: Send + Sync + 'static>(&mut self, binding: impl AxisBinding) -> &mut Self;
}

impl AppExt for App {
    /// Adds a trigger to the app with the given binding. This will insert the trigger as a resource and set up the
    /// necessary systems to update it.
    ///
    /// # Examples
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use press_here::AppExt;
    /// App::new()
    ///    .add_trigger::<MyTrigger>(KeyCode::Space)
    ///    .add_trigger::<OtherTrigger>(GamepadButton::South);;
    ///
    /// # struct MyTrigger;
    /// # struct OtherTrigger;
    /// ```
    fn add_trigger<T: Send + Sync + 'static>(&mut self, binding: impl TriggerBinding) -> &mut Self {
        let trigger = Trigger::<T>::new(binding);

        self.insert_resource(trigger)
            .add_systems(PreUpdate, update_trigger::<T>)
    }

    /// Adds an axis to the app with the given binding. This will insert the axis as a resource and set up the necessary
    /// systems to update it.
    ///
    /// # Examples
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use press_here::{AppExt, Pair, AxisBindingBuilder};
    /// App::new()
    ///   .add_axis::<MyAxis>((
    ///       Pair(KeyCode::KeyA, KeyCode::KeyD),
    ///       GamepadAxis::LeftStickX.deadzone(0.1),
    ///   ));
    ///
    /// # struct MyAxis;
    /// ```
    fn add_axis<A: Send + Sync + 'static>(&mut self, binding: impl AxisBinding) -> &mut Self {
        let axis = Axis::<A>::new(binding);

        self.insert_resource(axis)
            .add_systems(PreUpdate, update_axis::<A>)
    }
}

fn update_trigger<T: Send + Sync + 'static>(
    mut trigger: ResMut<Trigger<T>>,
    mut raw_inputs: InputsSystemParam,
) {
    let gamepads = raw_inputs.gamepads.iter().collect::<Vec<_>>();
    let mouse_motion = raw_inputs.mouse_motion.read().collect::<Vec<_>>();
    let mouse_wheel = raw_inputs.mouse_wheel.read().collect::<Vec<_>>();

    let inputs = Inputs {
        keycodes: &raw_inputs.keycodes,
        mouse_buttons: &raw_inputs.mouse_buttons,
        mouse_motion: mouse_motion.as_slice(),
        mouse_wheel: mouse_wheel.as_slice(),
        gamepads: gamepads.as_slice(),
        time: &raw_inputs.time,
    };

    trigger.pressed = trigger.binding.pressed(&inputs);
    trigger.just_pressed = trigger.binding.just_pressed(&inputs);
    trigger.just_released = trigger.binding.just_released(&inputs);
}

fn update_axis<A: Send + Sync + 'static>(
    mut axis: ResMut<Axis<A>>,
    mut raw_inputs: InputsSystemParam,
) {
    let gamepads = raw_inputs.gamepads.iter().collect::<Vec<_>>();
    let mouse_motion = raw_inputs.mouse_motion.read().collect::<Vec<_>>();
    let mouse_wheel = raw_inputs.mouse_wheel.read().collect::<Vec<_>>();

    let inputs = Inputs {
        keycodes: &raw_inputs.keycodes,
        mouse_buttons: &raw_inputs.mouse_buttons,
        mouse_motion: mouse_motion.as_slice(),
        mouse_wheel: mouse_wheel.as_slice(),
        gamepads: gamepads.as_slice(),
        time: &raw_inputs.time,
    };

    axis.value = axis.binding.value(&inputs).unwrap_or(0.0);
}
