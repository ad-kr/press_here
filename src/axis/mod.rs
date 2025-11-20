use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use dyn_clone::DynClone;
use std::{any::Any, marker::PhantomData};

pub mod bindings;
pub mod combinators;

dyn_clone::clone_trait_object!(AxisBinding);

pub trait AxisBinding: DynClone + Any + Send + Sync + 'static {
    fn value(&self, inputs: &Inputs) -> Option<f32>;

    /// Clones the inner value and returns it as a boxed trait object.
    fn clone_axis(&self) -> Box<dyn AxisBinding>;

    /// Clones the inner value and returns it as a boxed `Any` trait object.
    fn as_any(&self) -> Box<dyn Any> {
        self.clone_axis() as Box<dyn Any>
    }

    /// If the binding is a "collection binding" (tuple, vec, etc.), this will split the binding into its components and
    /// return a vec of boxed bindings. Otherwise, returns a vector with a single binding.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::{KeyCode, GamepadAxis};
    /// use press_here::AxisBinding;
    ///
    /// let binding = (KeyCode::KeyA, GamepadAxis::LeftStickX);
    /// let components = binding.all_axes();
    ///
    /// assert_eq!(components.len(), 2);
    /// assert_eq!(
    ///     components[0].as_any().downcast_ref::<KeyCode>(),
    ///     Some(&KeyCode::KeyA)
    /// );
    /// assert_eq!(
    ///     components[1].as_any().downcast_ref::<GamepadAxis>(),
    ///     Some(&GamepadAxis::LeftStickX)
    /// );
    /// ```
    fn all_axes(&self) -> Vec<Box<dyn AxisBinding>> {
        vec![self.clone_axis()]
    }
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

    pub fn binding(&self) -> &dyn AxisBinding {
        self.binding.as_ref()
    }

    pub fn set_binding(&mut self, binding: impl AxisBinding) {
        self.binding = Box::new(binding);
    }
}
