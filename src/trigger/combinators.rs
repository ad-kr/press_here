use crate::{inputs::Inputs, trigger::TriggerBinding};
use pastey::paste;

impl<T: TriggerBinding + Clone> TriggerBinding for Vec<T> {
    fn pressed(&self, inputs: &Inputs) -> bool {
        self.iter().any(|binding| binding.pressed(inputs))
    }

    fn just_pressed(&self, inputs: &Inputs) -> bool {
        self.iter().any(|binding| binding.just_pressed(inputs))
    }

    fn just_released(&self, inputs: &Inputs) -> bool {
        self.iter().any(|binding| binding.just_released(inputs))
    }

    fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
        Box::new(self.clone())
    }

    fn all_triggers(&self) -> Vec<Box<dyn TriggerBinding>> {
        self.iter().map(|b| b.clone_trigger()).collect()
    }
}

macro_rules! impl_tuple {
    ($($t:expr),*) => {
        paste! {
            impl<$([<T$t>]: TriggerBinding + Clone),*> TriggerBinding for ($([<T$t>]),*) {
                fn pressed(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.pressed(inputs))*
                }

                fn just_pressed(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_pressed(inputs))*
                }

                fn just_released(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_released(inputs))*
                }

                fn clone_trigger(&self) -> Box<dyn TriggerBinding> {
                    Box::new(self.clone())
                }

                fn all_triggers(&self) -> Vec<Box<dyn TriggerBinding>> {
                    vec![$(self.$t.clone_trigger()),*]
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
