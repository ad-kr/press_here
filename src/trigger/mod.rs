use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use dyn_clone::DynClone;
use std::{any::Any, marker::PhantomData};

mod bindings;
pub mod combinators;

pub trait TriggerBinding: DynClone + Any + Send + Sync + 'static {
    fn pressed(&mut self, inputs: &Inputs) -> bool;
    fn just_pressed(&mut self, inputs: &Inputs) -> bool;
    fn just_released(&mut self, inputs: &Inputs) -> bool;

    /// Clones the inner value and returns it as a boxed trait object.
    fn clone_trigger(&self) -> Box<dyn TriggerBinding>;

    /// Clones the inner value and returns it as a boxed `Any` trait object.
    fn as_any(&self) -> Box<dyn Any> {
        self.clone_trigger() as Box<dyn Any>
    }

    /// If the binding is a "collection binding" (tuple, vec, etc.), this will split the binding into its components and
    /// return a vec of boxed bindings. Otherwise, returns a vector with a single binding.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::{KeyCode, MouseButton};
    /// use press_here::TriggerBinding;
    ///
    /// let binding = (KeyCode::KeyA, MouseButton::Right);
    /// let components = binding.all_triggers();
    ///
    /// assert_eq!(components.len(), 2);
    /// assert_eq!(
    ///     components[0].as_any().downcast_ref::<KeyCode>(),
    ///     Some(&KeyCode::KeyA)
    /// );
    /// assert_eq!(
    ///     components[1].as_any().downcast_ref::<MouseButton>(),
    ///     Some(&MouseButton::Right)
    /// );
    /// ```
    fn all_triggers(&self) -> Vec<Box<dyn TriggerBinding>> {
        vec![self.clone_trigger()]
    }
}

dyn_clone::clone_trait_object!(TriggerBinding);

#[derive(Resource)]
pub struct Trigger<T> {
    trigger: PhantomData<T>,
    pub(crate) pressed: bool,
    pub(crate) just_pressed: bool,
    pub(crate) just_released: bool,
    pub(crate) binding: Box<dyn TriggerBinding>,
}

impl<T> Trigger<T> {
    pub fn new(binding: impl TriggerBinding + 'static) -> Self {
        Self {
            trigger: PhantomData,
            pressed: false,
            just_pressed: false,
            just_released: false,
            binding: Box::new(binding),
        }
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn just_pressed(&self) -> bool {
        self.just_pressed
    }

    pub fn just_released(&self) -> bool {
        self.just_released
    }

    pub fn binding(&self) -> &dyn TriggerBinding {
        self.binding.as_ref()
    }

    pub fn set_binding(&mut self, binding: impl TriggerBinding) {
        self.binding = Box::new(binding);
    }
}
