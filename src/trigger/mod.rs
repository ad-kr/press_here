use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use dyn_clone::DynClone;
use std::marker::PhantomData;

mod bindings;
mod combinators;

pub trait TriggerBinding: DynClone + Send + Sync + 'static {
    fn pressed(&self, inputs: &Inputs) -> bool;
    fn just_pressed(&self, inputs: &Inputs) -> bool;
    fn just_released(&self, inputs: &Inputs) -> bool;

    /// If the binding is a "collection binding" (tuple, vec, etc.), this will split the binding into its components and
    /// return a vec of boxed bindings. Otherwise, returns None.
    fn split(&self) -> Option<Vec<Box<dyn TriggerBinding>>> {
        None
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
