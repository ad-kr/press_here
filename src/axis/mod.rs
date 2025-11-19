use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use dyn_clone::DynClone;
use std::marker::PhantomData;

pub mod bindings;
pub mod combinators;

pub trait AxisBinding: DynClone + Send + Sync + 'static {
    fn value(&self, inputs: &Inputs) -> Option<f32>;

    /// If the binding is a "collection binding" (tuple, vec, etc.), this will split the binding into its components and
    /// return a vec of boxed bindings. Otherwise, returns None.
    fn split(&self) -> Option<Vec<Box<dyn AxisBinding>>> {
        None
    }
}

dyn_clone::clone_trait_object!(AxisBinding);

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

    pub fn binding(&self) -> &dyn AxisBinding {
        self.binding.as_ref()
    }

    pub fn set_binding(&mut self, binding: impl AxisBinding) {
        self.binding = Box::new(binding);
    }
}
