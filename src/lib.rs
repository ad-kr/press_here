/*!
# 👉 Press Here 👈

[![Crates.io](https://img.shields.io/crates/v/press_here.svg)](https://crates.io/crates/press_here)
[![docs.rs](https://img.shields.io/docsrs/press_here/latest)](https://docs.rs/press_here/latest)

`press_here` provides simple and modular input handling for the [Bevy](https://docs.rs/bevy/) game engine.

## Example

Setup is quick and easy. Define axis/trigger and configure bindings:

```no_run
# use bevy::prelude::*;
# use press_here::{AppExt, Axis, AxisBindingBuilder, Pair, Trigger};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Space key or south button on the gamepad for jump
        .add_trigger::<Jump>((KeyCode::Space, GamepadButton::South))
        .add_axis::<Walk>((
            // A/D keys for left/right movement
            Pair(KeyCode::KeyA, KeyCode::KeyD),
            // ..or left stick X-axis with deadzone
            GamepadAxis::LeftStickX.deadzone(0.1),
        ))
        .add_systems(Update, update)
        .run();
}

struct Jump;
struct Walk;

fn update(jump: Res<Trigger<Jump>>, walk: Res<Axis<Walk>>) {
    if jump.just_pressed() {
        jump_character();
    }

    let walk_value = walk.value();
    if walk_value != 0.0 {
        walk_character(walk_value);
    }
}

# fn jump_character() {}
# fn walk_character(_value: f32) {}
```

## Super modular

Axis and trigger bindings are modular and can be combined to configure complex input responses. Check out the [all_bindings](https://github.com/ad-kr/press_here/blob/main/examples/all_bindings.rs) example to see all bindings and modfiers in action.

```no_run
# use bevy::prelude::*;
# use press_here::{Deadzone, Pair, Smooth};
let binding = Smooth::new(
    (
        Pair(KeyCode::KeyA, KeyCode::KeyD),
        Deadzone(GamepadAxis::LeftStickX, 0.1),
    ),
    0.2,
);
```

The same binding can be defined using the builder pattern:

```no_run
# use bevy::prelude::*;
# use press_here::{AxisBindingBuilder, Pair};
let binding = (
    Pair(KeyCode::KeyA, KeyCode::KeyD),
    GamepadAxis::LeftStickX.deadzone(0.1),
)
    .smooth(0.2);
```

## Next steps

- Better documentation.
  - Currently most structs have some comments and the crate is rather straight forward, but better examples in comments could be provided.
- Binding information.
  - Add basic info to each binding so that we can know what bindings are set. Right now we would need to iterate over each binding and downcast its type. Not optimal.
- Observer support.
  - Currently you have to set up event triggering yourself. This can be streamlined.
- Tests
  - We could write simple unit tests for each binding. `Inputs`-mocking should be made easier. This would also allow us to make create better doc-comments/tests.
- Referring to existing axes and triggers.
  - It would be nice to be able to refer to other already defined triggers and axes. This would allow us to do something like `.with_trigger::<SomeTrigger>()`.
- Schedule configuration.
  - Right now all checks are happending in the PreUpdate schedule. It might be desirable to configure this.
- Time clock configuration.
  - Currently bindings that rely on time use the `Time<Real>` clock. If a different clock is desired, we have no way to configure that.
- ???

## Bevy compatibility

| Bevy | press_here |
| ---- | ---------- |
| 0.17 | 0.1        |
*/
mod app;
mod axis;
mod inputs;
mod trigger;
mod visualizer;

pub use app::AppExt;
pub use axis::{
    Axis, AxisBinding, bindings::*, builder::*, combinators::*, filters::*, modifiers::*,
};
pub use inputs::Inputs;
pub use trigger::{Trigger, TriggerBinding, builder::*, combinators::*, modifiers::*};
#[cfg(feature = "visualizer")]
pub use visualizer::*;
