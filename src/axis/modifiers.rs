use crate::{AxisBinding, inputs::Inputs};
use bevy::math::Curve;

/// A modifier that applies a curve to the axis value. This is useful for creating non-linear input responses.
#[derive(Clone, Copy)]
pub struct WithCurve<A: AxisBinding, C: Curve<f32>>(pub A, pub C);

impl<A: AxisBinding + Clone, C: Curve<f32> + Clone + Send + Sync + 'static> AxisBinding
    for WithCurve<A, C>
{
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let value = self.0.value(inputs)?;

        self.1.sample(value)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that applies a custom transformation function to the axis value.
///
/// # Examples
/// ```no_run
/// # use bevy::prelude::*;
/// # use press_here::Transformation;
/// let binding = Transformation(GamepadAxis::LeftStickY, |value| value.powi(3));
/// ```
#[derive(Clone, Copy)]
pub struct Transformation<A: AxisBinding, F: Fn(f32) -> f32>(pub A, pub F);

impl<A: AxisBinding + Clone, F: Fn(f32) -> f32 + Clone + Send + Sync + 'static> AxisBinding
    for Transformation<A, F>
{
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let value = self.0.value(inputs)?;

        Some((self.1)(value))
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that multiplies two axis values together.
#[derive(Clone, Copy)]
pub struct Multiply<A: AxisBinding, B: AxisBinding>(pub A, pub B);

impl<A: AxisBinding + Clone, B: AxisBinding + Clone> AxisBinding for Multiply<A, B> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let a = self.0.value(inputs).unwrap_or(0.0);
        let b = self.1.value(inputs).unwrap_or(0.0);

        Some(a * b)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that divides two axis values.
#[derive(Clone, Copy)]
pub struct Divide<A: AxisBinding, B: AxisBinding>(pub A, pub B);

impl<A: AxisBinding + Clone, B: AxisBinding + Clone> AxisBinding for Divide<A, B> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let a = self.0.value(inputs).unwrap_or(0.0);
        let b = self.1.value(inputs).unwrap_or(1.0);

        Some(a / b)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that adds two axis values together.
#[derive(Clone, Copy)]
pub struct Add<A: AxisBinding, B: AxisBinding>(pub A, pub B);

impl<A: AxisBinding + Clone, B: AxisBinding + Clone> AxisBinding for Add<A, B> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let a = self.0.value(inputs).unwrap_or(0.0);
        let b = self.1.value(inputs).unwrap_or(0.0);

        Some(a + b)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that subtracts two axis values.
#[derive(Clone, Copy)]
pub struct Subtract<A: AxisBinding, B: AxisBinding>(pub A, pub B);

impl<A: AxisBinding + Clone, B: AxisBinding + Clone> AxisBinding for Subtract<A, B> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let a = self.0.value(inputs).unwrap_or(0.0);
        let b = self.1.value(inputs).unwrap_or(0.0);

        Some(a - b)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that inverts the axis value.
#[derive(Clone, Copy)]
pub struct Invert<A: AxisBinding>(pub A);

impl<A: AxisBinding + Clone> AxisBinding for Invert<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let value = self.0.value(inputs)?;

        Some(-value)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A modifier that remaps the axis value from one range to another.
///
/// # Examples
/// ```no_run
/// # use press_here::Remap;
/// # let binding = 0.5;
/// let mut remapped = Remap(binding, 0.0, 1.0, -1.0, 1.0);
/// ```
///
/// This is especially useful when using together with the [Deadzone](crate::axis::filters::Deadzone) filter to remap
/// the output range after applying a deadzone.
///
/// ``` no_run
/// # use press_here::{Remap, AxisBindingBuilder};
/// # let binding = 0.5;
/// // Instead of "jumping" from 0.0 to 0.2, the axis will linearly interpolate from 0.0 to 1.0 as the input
/// // moves from 0.2 to 1.0.
/// let mut remapped = binding
///     .deadzone(0.2)
///     .remap(0.2, 1.0, 0.0, 1.0);
/// ```
///
#[derive(Clone, Copy)]
pub struct Remap<A: AxisBinding>(pub A, pub f32, pub f32, pub f32, pub f32);

impl<A: AxisBinding + Clone> AxisBinding for Remap<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let value = self.0.value(inputs)?;

        let (in_min, in_max, out_min, out_max) = (self.1, self.2, self.3, self.4);
        let t = (value - in_min) / (in_max - in_min);
        let value = out_min + t * (out_max - out_min);

        Some(value)
    }
    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}
