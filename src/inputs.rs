use bevy::{
    ecs::{
        message::MessageReader,
        system::{Query, Res, SystemParam},
    },
    input::{
        ButtonInput,
        gamepad::Gamepad,
        keyboard::KeyCode,
        mouse::{MouseButton, MouseMotion, MouseWheel},
    },
    time::{Real, Time},
};

pub struct Inputs<'a> {
    pub keycodes: &'a ButtonInput<KeyCode>,
    pub mouse_buttons: &'a ButtonInput<MouseButton>,
    pub mouse_motion: &'a [&'a MouseMotion],
    pub mouse_wheel: &'a [&'a MouseWheel],
    pub gamepads: &'a [&'a Gamepad],
    pub time: &'a Time<Real>,
}

#[derive(SystemParam)]
pub struct InputsSystemParam<'w, 's> {
    pub keycodes: Res<'w, ButtonInput<KeyCode>>,
    pub gamepads: Query<'w, 's, &'static Gamepad>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    pub mouse_motion: MessageReader<'w, 's, MouseMotion>,
    pub mouse_wheel: MessageReader<'w, 's, MouseWheel>,
    pub time: Res<'w, Time<Real>>,
}
