use crate::{
    Add, AxisBinding, Divide, Invert, Normalize, RateLimit, Remap, Subtract, TriggerBinding,
    WithTriggerBinding,
    axis::{
        filters::{Deadzone, Smooth},
        modifiers::{Multiply, Transformation, WithCurve},
    },
};
use bevy::math::Curve;

pub trait AxisBindingBuilder: AxisBinding + Sized {
    /// Returns a new axis binding that is only active when the given trigger binding is active.
    fn with_trigger_binding<TB: TriggerBinding>(self, trigger: TB) -> WithTriggerBinding<Self, TB> {
        WithTriggerBinding(self, trigger)
    }

    /// Returns a new axis binding that applies a deadzone filter with the given threshold.
    fn deadzone(self, threshold: f32) -> Deadzone<Self> {
        Deadzone(self, threshold)
    }

    /// Returns a new axis binding that applies a smoothing filter with the given time constant.
    fn smooth(self, tau: f32) -> Smooth<Self> {
        Smooth::new(self, tau)
    }

    // Normalizes this axis value so that the combined magnitude of this and the perpendicular axes is at most 1.0.
    fn normalize<A: AxisBinding>(self, perpendicular: A) -> Normalize<Self, A> {
        Normalize(self, perpendicular)
    }

    /// Returns a new axis binding that applies a rate limit filter with the given maximum rate of change.
    fn limit_rate(self, max_rate: f32) -> RateLimit<Self> {
        RateLimit::new(self, max_rate)
    }

    /// Returns a new axis binding that applies the given curve to the axis value.
    fn with_curve<C: Curve<f32>>(self, curve: C) -> WithCurve<Self, C> {
        WithCurve(self, curve)
    }

    /// Returns a new axis binding that applies the given transformation function to the axis value.
    fn transform<F: Fn(f32) -> f32>(self, func: F) -> Transformation<Self, F> {
        Transformation(self, func)
    }

    /// Returns a new axis binding that multiplies this axis value with another axis value.
    fn mult<A: AxisBinding>(self, other: A) -> Multiply<Self, A> {
        Multiply(self, other)
    }

    /// Returns a new axis binding that divides this axis value by another axis value.
    fn div<A: AxisBinding>(self, other: A) -> Divide<Self, A> {
        Divide(self, other)
    }

    /// Returns a new axis binding that adds another axis value to this axis value.
    fn add<A: AxisBinding>(self, other: A) -> Add<Self, A> {
        Add(self, other)
    }

    /// Return a new axis binding that subtracts another axis value from this axis value.
    fn sub<A: AxisBinding>(self, other: A) -> Subtract<Self, A> {
        Subtract(self, other)
    }

    /// Returns a new axis binding that inverts this axis value.
    fn invert(self) -> Invert<Self> {
        Invert(self)
    }

    /// Remaps the axis value from one range to another.
    fn remap(self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Remap<Self> {
        Remap(self, in_min, in_max, out_min, out_max)
    }
}

impl<A: AxisBinding> AxisBindingBuilder for A {}
