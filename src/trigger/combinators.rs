use crate::{inputs::Inputs, trigger::TriggerBinding};
use pastey::paste;

macro_rules! impl_tuple {
    ($($t:expr),*) => {
        paste! {
            impl<$([<T$t>]: TriggerBinding),*> TriggerBinding for ($([<T$t>]),*) {
                fn pressed(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.pressed(inputs))*
                }

                fn just_pressed(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_pressed(inputs))*
                }

                fn just_released(&self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.just_released(inputs))*
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
