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
