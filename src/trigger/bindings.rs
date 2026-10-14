use crate::{inputs::Inputs, trigger::TriggerBinding};
use bevy::input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton};
use std::any::Any;

impl TriggerBinding for () {
    fn pressed(&mut self, _: &Inputs) -> bool {
        false
    }
}

impl TriggerBinding for bool {
    fn pressed(&mut self, _: &Inputs) -> bool {
        *self
    }
}

impl TriggerBinding for KeyCode {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.keycodes.pressed(*self)
    }
}

impl TriggerBinding for MouseButton {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.pressed(*self)
    }
}

impl TriggerBinding for GamepadButton {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.pressed(*self))
    }
}

impl TriggerBinding for Box<dyn TriggerBinding> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.as_mut().pressed(inputs)
    }

    fn as_any(&self) -> &dyn Any {
        // We forward the inner TriggerBindings's as_any method, to not expose the Box itself.
        (**self).as_any()
    }

    fn inner_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
        // We forward the inner TriggerBinding's inner_bindings method, to not expose the Box itself.
        (**self).inner_bindings()
    }
}
