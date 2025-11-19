use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use std::marker::PhantomData;

pub mod bindings;
pub mod combinators;

pub trait AxisBinding: Send + Sync + 'static {
    fn value(&self, inputs: &Inputs) -> Option<f32>;
}

#[derive(Resource)]
pub struct Axis<A> {
    axis: PhantomData<A>,
    pub(crate) value: f32,
    pub(crate) binding: Box<dyn AxisBinding>,
}

impl<A> Axis<A> {
    pub fn new(binding: impl AxisBinding + 'static) -> Self {
        Self {
            axis: PhantomData,
            value: 0.0,
            binding: Box::new(binding),
        }
    }

    /// Get current value of the axis.
    pub fn value(&self) -> f32 {
        self.value
    }
}
