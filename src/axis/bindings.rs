use crate::{axis::AxisBinding, inputs::Inputs};
use bevy::input::{
    gamepad::{GamepadAxis, GamepadButton},
    keyboard::KeyCode,
    mouse::{MouseButton, MouseScrollUnit},
};

impl AxisBinding for () {
    fn value(&self, _inputs: &Inputs) -> Option<f32> {
        None
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(())
    }
}

impl AxisBinding for KeyCode {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        inputs.keycodes.pressed(*self).then_some(1.0)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

impl AxisBinding for MouseButton {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        inputs.mouse_buttons.pressed(*self).then_some(1.0)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

impl AxisBinding for GamepadButton {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        inputs.gamepads.iter().find_map(|pad| pad.get(*self))
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
pub struct MouseX;

impl AxisBinding for MouseX {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        if inputs.mouse_motion.is_empty() {
            return None;
        }

        let sum = inputs.mouse_motion.iter().map(|m| m.delta.x).sum();
        Some(sum)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
pub struct MouseY;

impl AxisBinding for MouseY {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        if inputs.mouse_motion.is_empty() {
            return None;
        }

        let sum = inputs.mouse_motion.iter().map(|m| m.delta.y).sum();
        Some(sum)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
pub struct MouseWheel {
    pub px_per_line: f32,
}

impl Default for MouseWheel {
    fn default() -> Self {
        Self { px_per_line: 16.0 }
    }
}

impl AxisBinding for MouseWheel {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        if inputs.mouse_wheel.is_empty() {
            return None;
        }

        let sum = inputs
            .mouse_wheel
            .iter()
            .map(|e| match e.unit {
                MouseScrollUnit::Line => e.y * self.px_per_line,
                MouseScrollUnit::Pixel => e.y,
            })
            .sum();

        Some(sum)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

impl AxisBinding for GamepadAxis {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        for pad in inputs.gamepads {
            if let Some(value) = pad.get(*self) {
                return Some(value);
            }
        }
        None
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(*self)
    }
}

impl AxisBinding for Box<dyn AxisBinding> {
    fn value(&self, inputs: &Inputs) -> Option<f32> {
        self.as_ref().value(inputs)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        self.clone()
    }
}
