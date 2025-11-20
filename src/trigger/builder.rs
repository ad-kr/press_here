use crate::{And, Not, TriggerBinding};

pub trait TriggerBindingBuilder: TriggerBinding + Sized {
    /// Returns a new trigger binding that is only active when both this and the given trigger binding are active.
    fn and<TB: TriggerBinding>(self, other: TB) -> And<Self, TB> {
        And(self, other)
    }

    /// Returns a new trigger binding that inverts the state of this trigger binding.
    fn not(self) -> Not<Self> {
        Not(self)
    }
}
