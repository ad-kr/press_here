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
    fn add_trigger<T: Send + Sync + 'static>(&mut self, binding: impl TriggerBinding) -> &mut Self {
        let trigger = Trigger::<T>::new(binding);

        self.insert_resource(trigger)
            .add_systems(PreUpdate, update_trigger::<T>)
    }

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
    };

    axis.value = axis.binding.value(&inputs).unwrap_or(0.0);
}
