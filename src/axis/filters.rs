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

/// A filter that smooths axis values using
/// [exponential smoothing](https://en.wikipedia.org/wiki/Exponential_smoothing#Basic_(simple)_exponential_smoothing).
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

/// A filter that normalizes the axis value so that the combined magnitude of the two axes is at most 1.0.
///
/// The first supplied axis is the one being normalized, and the second is the perpendicular axis used to calculate the
/// magnitude.
#[derive(Clone, Copy)]
pub struct Normalize<A: AxisBinding, Perpendicular: AxisBinding>(pub A, pub Perpendicular);

impl<A: AxisBinding + Clone, Perpendicular: AxisBinding + Clone> AxisBinding
    for Normalize<A, Perpendicular>
{
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let x = self.0.value(inputs).unwrap_or(0.0);
        let y = self.1.value(inputs).unwrap_or(0.0);

        let magnitude = (x * x + y * y).sqrt();
        if magnitude > 1.0 {
            Some(x / magnitude)
        } else {
            Some(x)
        }
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// Limits the rate of change of an axis value to a maximum delta per second.
///
/// # Examples
/// ```no_run
/// # use press_here::RateLimit;
/// # let binding = 1.0;
/// let maximum_rate = 0.5; // units per second
/// let mut limited_axis = RateLimit::new(binding, maximum_rate);
/// ```
#[derive(Clone, Copy)]
pub struct RateLimit<A: AxisBinding> {
    pub binding: A,
    pub max_rate: f32,
    previous_value: f32,
}

impl<A: AxisBinding> RateLimit<A> {
    pub fn new(binding: A, max_rate: f32) -> Self {
        Self {
            binding,
            max_rate,
            previous_value: 0.0,
        }
    }
}

impl<A: AxisBinding + Clone> AxisBinding for RateLimit<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let target = self.binding.value(inputs)?;
        let dt = inputs.time.delta_secs();

        let max_delta = self.max_rate * dt;
        let delta = (target - self.previous_value).clamp(-max_delta, max_delta);

        let value = self.previous_value + delta;
        self.previous_value = value;

        Some(value)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}
