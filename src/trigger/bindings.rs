use crate::{inputs::Inputs, trigger::TriggerBinding};
use bevy::input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton};

impl TriggerBinding for () {
    fn pressed(&mut self, _: &Inputs) -> bool {
        false
    }

    fn just_pressed(&mut self, _: &Inputs) -> bool {
        false
    }

    fn just_released(&mut self, _: &Inputs) -> bool {
        false
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(())
    }
}

impl TriggerBinding for bool {
    fn pressed(&mut self, _: &Inputs) -> bool {
        *self
    }

    fn just_pressed(&mut self, _: &Inputs) -> bool {
        false
    }

    fn just_released(&mut self, _: &Inputs) -> bool {
        false
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(*self)
    }
}

impl TriggerBinding for KeyCode {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.keycodes.pressed(*self)
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.keycodes.just_pressed(*self)
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        inputs.keycodes.just_released(*self)
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(*self)
    }
}

impl TriggerBinding for MouseButton {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.pressed(*self)
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.just_pressed(*self)
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.just_released(*self)
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(*self)
    }
}

impl TriggerBinding for GamepadButton {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.pressed(*self))
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.just_pressed(*self))
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.just_released(*self))
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(*self)
    }
}

impl TriggerBinding for Box<dyn TriggerBinding> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.as_mut().pressed(inputs)
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        self.as_mut().just_pressed(inputs)
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        self.as_mut().just_released(inputs)
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        self.clone()
    }
}
