use crate::{inputs::Inputs, trigger::TriggerBinding};
use bevy::input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton};

impl TriggerBinding for KeyCode {
    fn pressed(&self, inputs: &Inputs) -> bool {
        inputs.keycodes.pressed(*self)
    }

    fn just_pressed(&self, inputs: &Inputs) -> bool {
        inputs.keycodes.just_pressed(*self)
    }

    fn just_released(&self, inputs: &Inputs) -> bool {
        inputs.keycodes.just_released(*self)
    }
}

impl TriggerBinding for MouseButton {
    fn pressed(&self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.pressed(*self)
    }

    fn just_pressed(&self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.just_pressed(*self)
    }

    fn just_released(&self, inputs: &Inputs) -> bool {
        inputs.mouse_buttons.just_released(*self)
    }
}

impl TriggerBinding for GamepadButton {
    fn pressed(&self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.pressed(*self))
    }

    fn just_pressed(&self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.just_pressed(*self))
    }

    fn just_released(&self, inputs: &Inputs) -> bool {
        inputs.gamepads.iter().any(|pad| pad.just_released(*self))
    }
}
