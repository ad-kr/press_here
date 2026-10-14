use crate::{inputs::Inputs, trigger::TriggerBinding};
use pastey::paste;

/// A combinator that returns true only if both trigger bindings are pressed.
#[derive(Clone, Copy)]
pub struct And<T1: TriggerBinding, T2: TriggerBinding>(pub T1, pub T2);

impl<T1: TriggerBinding + Clone, T2: TriggerBinding + Clone> TriggerBinding for And<T1, T2> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.0.pressed(inputs) && self.1.pressed(inputs)
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        let is_pressed = self.pressed(inputs);
        let was_pressed = self.0.just_pressed(inputs) || self.1.just_pressed(inputs);
        is_pressed && was_pressed
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        let is_released = !self.pressed(inputs);

        // "just released" for "And" requires that both triggers were pressed before, so we check
        // for that as well
        let first_was_already_pressed = self.0.pressed(inputs) && !self.0.just_pressed(inputs);
        let second_was_already_pressed = self.1.pressed(inputs) && !self.1.just_pressed(inputs);

        let first_was_pressed = first_was_already_pressed || self.0.just_released(inputs);
        let second_was_pressed = second_was_already_pressed || self.1.just_released(inputs);

        is_released && first_was_pressed && second_was_pressed
    }

    fn clone_binding(&self) -> Box<dyn TriggerBinding> {
        Box::new(self.clone())
    }
}

impl<T: TriggerBinding + Clone> TriggerBinding for Vec<T> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.iter_mut().any(|binding| binding.pressed(inputs))
    }

    fn just_pressed(&mut self, inputs: &Inputs) -> bool {
        self.iter_mut().any(|binding| binding.just_pressed(inputs))
    }

    fn just_released(&mut self, inputs: &Inputs) -> bool {
        self.iter_mut().any(|binding| binding.just_released(inputs))
    }

    fn clone_binding(&self) -> Box<dyn TriggerBinding> {
        Box::new(self.clone())
    }

    fn all_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
        self.iter().map(|b| b.clone_binding()).collect()
    }
}

macro_rules! impl_tuple {
    ($($t:expr),*) => {
        paste! {
            impl<$([<T$t>]: TriggerBinding + Clone),*> TriggerBinding for ($([<T$t>]),*) {
                fn pressed(&mut self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.pressed(inputs))*
                }

                fn just_pressed(&mut self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_pressed(inputs))*
                }

                fn just_released(&mut self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_released(inputs))*
                }

                fn clone_binding(&self) -> Box<dyn TriggerBinding> {
                    Box::new(self.clone())
                }

                fn all_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
                    vec![$(self.$t.clone_binding()),*]
                }
            }
        }
    };
}

impl_tuple!(0, 1);
impl_tuple!(0, 1, 2);
impl_tuple!(0, 1, 2, 3);
impl_tuple!(0, 1, 2, 3, 4);
impl_tuple!(0, 1, 2, 3, 4, 5);
impl_tuple!(0, 1, 2, 3, 4, 5, 6);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
