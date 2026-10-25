use bevy::{
    ecs::{
        message::MessageReader,
        query::With,
        system::{Query, Res, Single, SystemParam},
    },
    input::{
        ButtonInput,
        gamepad::Gamepad,
        keyboard::KeyCode,
        mouse::{MouseButton, MouseMotion, MouseWheel},
    },
    math::Vec2,
    time::{Real, Time},
    window::{PrimaryWindow, Window},
};

/// A collection of input references for use in input bindings.
pub struct Inputs<'a> {
    pub keycodes: &'a ButtonInput<KeyCode>,
    pub mouse_position: Option<Vec2>,
    pub mouse_buttons: &'a ButtonInput<MouseButton>,
    pub mouse_motion: &'a [&'a MouseMotion],
    pub mouse_wheel: &'a [&'a MouseWheel],
    pub gamepads: &'a [&'a Gamepad],
    pub time: &'a Time<Real>,
}

/// System parameter for accessing input resources.
#[derive(SystemParam)]
pub struct InputsSystemParam<'w, 's> {
    pub keycodes: Res<'w, ButtonInput<KeyCode>>,
    pub gamepads: Query<'w, 's, &'static Gamepad>,
    pub window: Option<Single<'w, 's, &'static Window, With<PrimaryWindow>>>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    pub mouse_motion: MessageReader<'w, 's, MouseMotion>,
    pub mouse_wheel: MessageReader<'w, 's, MouseWheel>,
    pub time: Res<'w, Time<Real>>,
}
