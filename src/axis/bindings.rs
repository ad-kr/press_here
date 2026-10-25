use crate::{axis::AxisBinding, inputs::Inputs};
use bevy::input::{
    gamepad::{GamepadAxis, GamepadButton},
    keyboard::KeyCode,
    mouse::{MouseButton, MouseScrollUnit},
};
use std::any::Any;

impl AxisBinding for () {
    fn value(&mut self, _: &Inputs) -> Option<f32> {
        None
    }
}

impl AxisBinding for f32 {
    fn value(&mut self, _: &Inputs) -> Option<f32> {
        Some(*self)
    }
}

impl AxisBinding for KeyCode {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        inputs.keycodes.pressed(*self).then_some(1.0)
    }
}

impl AxisBinding for MouseButton {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        inputs.mouse_buttons.pressed(*self).then_some(1.0)
    }
}

impl AxisBinding for GamepadButton {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        inputs.gamepads.iter().find_map(|pad| pad.get(*self))
    }
}

/// Binds the X-axis of mouse movement as an axis input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseX {
    /// Mouse movement difference since the last frame, in physical pixels
    Delta,

    /// Mouse position from center of the window, in physical pixels. The position is taken from the
    /// [bevy::window::PrimaryWindow]'s cursor position. Value will be [None] if the cursor is
    /// outside the window.
    Position,
}

impl AxisBinding for MouseX {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        match self {
            MouseX::Delta => {
                if inputs.mouse_motion.is_empty() {
                    return None;
                }

                let sum = inputs.mouse_motion.iter().map(|m| m.delta.x).sum();
                Some(sum)
            }
            MouseX::Position => {
                let mouse_position = inputs.mouse_position?;
                Some(mouse_position.x)
            }
        }
    }
}

/// Binds the Y-axis of mouse movement as an axis input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseY {
    /// Mouse movement difference since the last frame, in physical pixels
    Delta,

    /// Mouse position from center of the window, in physical pixels. The position is taken from the
    /// [bevy::window::PrimaryWindow]'s cursor position. Value will be [None] if the cursor is
    /// outside the window.
    Position,
}

impl AxisBinding for MouseY {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        match self {
            MouseY::Delta => {
                if inputs.mouse_motion.is_empty() {
                    return None;
                }

                let sum = inputs.mouse_motion.iter().map(|m| m.delta.y).sum();
                Some(sum)
            }
            MouseY::Position => {
                let mouse_position = inputs.mouse_position?;
                Some(mouse_position.y)
            }
        }
    }
}

/// Binds mouse wheel movement as an axis input.
#[derive(Clone, Copy, Debug)]
pub struct MouseWheel {
    pub px_per_line: f32,
}

impl Default for MouseWheel {
    fn default() -> Self {
        Self { px_per_line: 16.0 }
    }
}

impl AxisBinding for MouseWheel {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
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
}

impl AxisBinding for GamepadAxis {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        for pad in inputs.gamepads {
            if let Some(value) = pad.get(*self) {
                return Some(value);
            }
        }
        None
    }
}

impl AxisBinding for Box<dyn AxisBinding> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        self.as_mut().value(inputs)
    }

    fn as_any(&self) -> &dyn Any {
        // We forward the inner AxisBinding's as_any method, to not expose the Box itself.
        (**self).as_any()
    }

    fn inner_bindings(&self) -> Vec<Box<dyn AxisBinding>> {
        // We forward the inner AxisBinding's inner_bindings method, to not expose the Box itself.
        (**self).inner_bindings()
    }
}
