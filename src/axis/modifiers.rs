use crate::{AxisBinding, inputs::Inputs};

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
