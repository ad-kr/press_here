use crate::{TriggerBinding, inputs::Inputs};

/// A modifier that inverts the trigger state.
#[derive(Clone, Copy)]
pub struct Not<T: TriggerBinding>(pub T);

impl<T: TriggerBinding + Clone> TriggerBinding for Not<T> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        !self.0.pressed(inputs)
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        !self.0.just_pressed(inputs)
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        !self.0.just_released(inputs)
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(self.clone())
    }
}
