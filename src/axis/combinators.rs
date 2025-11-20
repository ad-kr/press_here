use crate::{axis::AxisBinding, inputs::Inputs, trigger::TriggerBinding};
use pastey::paste;

/// A pair of axis binding, where the first axis is used for the negative direction and the second axis is used for the
/// positive direction.
#[derive(Clone, Copy)]
pub struct Pair<A1: AxisBinding, A2: AxisBinding>(pub A1, pub A2);

impl<A1: AxisBinding + Clone, A2: AxisBinding + Clone> AxisBinding for Pair<A1, A2> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let negative = self.0.value(inputs);
        let positive = self.1.value(inputs);

        if negative.is_none() && positive.is_none() {
            return None;
        }

        let negative = negative.unwrap_or(0.0);
        let positive = positive.unwrap_or(0.0);

        Some(positive - negative)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

/// An axis binding that is only active when the given trigger binding is active.
#[derive(Clone, Copy)]
pub struct WithTriggerBinding<A: AxisBinding, T: TriggerBinding>(pub A, pub T);

impl<A: AxisBinding + Clone, T: TriggerBinding + Clone> AxisBinding for WithTriggerBinding<A, T> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        if !self.1.pressed(inputs) {
            return None;
        }

        self.0.value(inputs)
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }
}

impl<A: AxisBinding + Clone> AxisBinding for Vec<A> {
    fn value(&mut self, inputs: &Inputs) -> Option<f32> {
        let all = self
            .iter_mut()
            .filter_map(|binding| binding.value(inputs))
            .collect::<Vec<_>>();
        let sum = all.iter().sum::<f32>();
        let count = all.len();

        if count > 0 {
            Some(sum / count as f32)
        } else {
            None
        }
    }

    fn clone_axis(&self) -> Box<dyn AxisBinding> {
        Box::new(self.clone())
    }

    fn all_axes(&self) -> Vec<Box<dyn AxisBinding>> {
        self.iter().map(|b| b.clone_axis()).collect()
    }
}

macro_rules! impl_tuple {
    ($($a:expr),*) => {
        paste! {
            impl<$([<A$a>]: AxisBinding + Clone),*> AxisBinding for ($([<A$a>]),*) {
                fn value(&mut self, inputs: &Inputs) -> Option<f32> {
                    let all = [$(self.$a.value(inputs)),*]
                        .iter()
                        .filter_map(|v| *v)
                        .collect::<Vec<_>>();
                    let sum = all.iter().sum::<f32>();
                    let count = all.len();

                    if count > 0 {
                        Some(sum / count as f32)
                    } else {
                        None
                    }
                }

                fn clone_axis(&self) -> Box<dyn AxisBinding> {
                    Box::new(self.clone())
                }

                fn all_axes(&self) -> Vec<Box<dyn AxisBinding>> {
                    vec![$(self.$a.clone_axis()),*]
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
