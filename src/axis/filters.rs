use crate::{AxisBinding, inputs::Inputs};

/// A filter that only allows axis values that exceed a certain deadzone threshold.
#[derive(Clone, Copy)]
pub struct Deadzone<A: AxisBinding>(pub A, pub f32);

impl<A: AxisBinding + Clone> AxisBinding for Deadzone<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let value = self.0.value(inputs)?;

        if value.abs() < self.1 {
            None
        } else {
            Some(value)
        }
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// A filter that smooths axis values using [exponential smoothing][https://en.wikipedia.org/wiki/Exponential_smoothing#Basic_(simple)_exponential_smoothing].
///
/// `tau` is the time constant that controls the amount of smoothing. Small `tau` values result in less smoothing (more
/// responsive), while large `tau` values result in more smoothing (less responsive).
#[derive(Clone, Copy)]
pub struct Smooth<A: AxisBinding> {
    pub binding: A,
    pub tau: f32,
    previous_value: f32,
}

impl<A: AxisBinding> Smooth<A> {
    pub fn new(binding: A, tau: f32) -> Self {
        Self {
            binding,
            tau,
            previous_value: 0.0,
        }
    }
}

impl<A: AxisBinding + Clone> AxisBinding for Smooth<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let target = self.binding.value(inputs).unwrap_or(0.0);
        let dt = inputs.time.delta_secs();

        let alpha = 1.0 - (-dt / self.tau).exp();

        let value = self.previous_value + alpha * (target - self.previous_value);
        self.previous_value = value;

        Some(value)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}
