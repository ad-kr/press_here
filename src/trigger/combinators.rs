use crate::{inputs::Inputs, trigger::TriggerBinding};
use pastey::paste;

/// A combinator that returns true only if both trigger bindings are pressed.
#[derive(Clone, Copy, Debug)]
pub struct And<T1: TriggerBinding, T2: TriggerBinding>(pub T1, pub T2);

impl<T1: TriggerBinding + Clone, T2: TriggerBinding + Clone> TriggerBinding for And<T1, T2> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.0.pressed(inputs) && self.1.pressed(inputs)
    }
}

impl<T: TriggerBinding + Clone> TriggerBinding for Vec<T> {
    fn pressed(&mut self, inputs: &Inputs) -> bool {
        self.iter_mut().any(|binding| binding.pressed(inputs))
    }

    fn inner_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
        self.iter()
            .map(|b| dyn_clone::clone_box(b.as_dyn()))
            .collect()
    }
}

macro_rules! impl_tuple {
    ($($t:expr),*) => {
        paste! {
            impl<$([<T$t>]: TriggerBinding + Clone),*> TriggerBinding for ($([<T$t>]),*) {
                fn pressed(&mut self, inputs: &Inputs) -> bool {
                    false $(|| self.$t.pressed(inputs))*
                }


                fn inner_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
                    vec![$(dyn_clone::clone_box(self.$t.as_dyn())),*]
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
