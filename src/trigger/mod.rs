use crate::inputs::Inputs;
use bevy::ecs::resource::Resource;
use dyn_clone::DynClone;
use std::{any::Any, fmt::Debug, marker::PhantomData};

mod bindings;
pub mod builder;
pub mod combinators;
pub mod modifiers;

dyn_clone::clone_trait_object!(TriggerBinding);

pub trait TriggerBinding:
    AsDynTriggerBinding + DynClone + Debug + Any + Send + Sync + 'static
{
    fn pressed(&mut self, inputs: &Inputs) -> bool;

    /// Clones the inner value and returns it as a boxed `Any` trait object.
    fn as_any(&self) -> &dyn Any {
        self.as_dyn()
    }

    /// If the binding is a "collection binding" (tuple, vec, etc.), this will split the binding
    /// into its components and return a vec of boxed bindings. Otherwise, returns a vector with a
    /// single binding.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::{KeyCode, MouseButton};
    /// use press_here::TriggerBinding;
    ///
    /// let binding = (KeyCode::KeyA, MouseButton::Right);
    /// let components = binding.inner_bindings();
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
    fn inner_bindings(&self) -> Vec<Box<dyn TriggerBinding>> {
        vec![dyn_clone::clone_box(self.as_dyn())]
    }
}

pub trait AsDynTriggerBinding {
    fn as_dyn(&self) -> &dyn TriggerBinding;
}

impl<T: TriggerBinding + 'static> AsDynTriggerBinding for T {
    fn as_dyn(&self) -> &dyn TriggerBinding {
        self
    }
}

/// A resource that holds the current state and binding for a trigger.
///
/// # Examples
/// Reading the state:
/// ```no_run
/// # use bevy::prelude::*;
/// # use press_here::Trigger;
/// # struct MyTrigger;
/// fn system(trigger: Res<Trigger<MyTrigger>>) {
///   let is_pressed = trigger.pressed();
/// }
/// ```
/// Modifying the binding:
/// ```no_run
/// # use bevy::prelude::*;
/// # use press_here::Trigger;
/// # struct MyTrigger;
/// fn system(mut trigger: ResMut<Trigger<MyTrigger>>) {
///    trigger.set_binding(KeyCode::Space);
/// }
/// ```
#[derive(Resource)]
pub struct Trigger<T> {
    trigger: PhantomData<T>,
    pub(crate) pressed: bool,
    pub(crate) pressed_last_frame: bool,
    pub(crate) binding: Box<dyn TriggerBinding>,
}

impl<T> Trigger<T> {
    pub fn new(binding: impl TriggerBinding + 'static) -> Self {
        Self {
            trigger: PhantomData,
            pressed: false,
            pressed_last_frame: false,
            binding: Box::new(binding),
        }
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn just_pressed(&self) -> bool {
        self.pressed && !self.pressed_last_frame
    }

    pub fn just_released(&self) -> bool {
        !self.pressed && self.pressed_last_frame
    }

    pub fn binding(&self) -> &dyn TriggerBinding {
        self.binding.as_ref()
    }

    pub fn set_binding(&mut self, binding: impl TriggerBinding) {
        self.binding = Box::new(binding);
    }
}
