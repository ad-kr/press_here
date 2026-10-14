use crate::{TriggerBinding, inputs::Inputs};

/// A modifier that inverts the trigger state.
#[derive(Clone, Copy)]
pub struct Not<T: TriggerBinding>(pub T);

impl<T: TriggerBinding + Clone> TriggerBinding for Not<T> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        !self.0.pressed(inputs)
    }

    fn clone_binding(&self) -> Box<dyn TriggerBinding> {
        Box::new(self.clone())
    }
}
