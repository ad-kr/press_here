use bevy::prelude::*;
use press_here::{
    Add, And, AppExt, AxisBinding, AxisBindingBuilder, AxisVisualizer, Deadzone, Divide, Invert,
    MouseWheel, MouseY, Multiply, Normalize, Not, Pair, RateLimit, Smooth, Subtract,
    Transformation, Trigger, TriggerBinding, WithCurve, WithTriggerBinding,
};
use std::time::Duration;

fn main() {
    // Any type that implements AxisBinding can be used as an axis binding. A lot of bindings can be nested and combined
    // to create more complex behaviors.
    let _super_complex_axis_binding = WithTriggerBinding(
        Smooth::new(
            Deadzone(
                (
                    Pair(KeyCode::KeyA, GamepadAxis::LeftStickX),
                    Multiply(32.0, GamepadAxis::LeftStickX),
                ),
                0.1,
            ),
            0.2,
        ),
        MouseButton::Left,
    );

    // The same complex axis binding can be built using the AxisBindingBuilder trait methods. AxisBindingBuilder is
    // implemented for all AxisBinding types, so all bindings have access to these combinator methods. Triggers have the
    // equivalent TriggerBindingBuilder trait.
    let _super_complex_axis_binding_built_different = (
        Pair(KeyCode::KeyA, GamepadAxis::LeftStickX),
        32.0.mult(GamepadAxis::LeftStickX),
    )
        .deadzone(0.1)
        .smooth(0.2)
        .with_trigger_binding(MouseButton::Left);

    //
    // Bindings:
    //
    App::new()
        .add_plugins(DefaultPlugins)
        //
        // Axis bindings:
        //
        // Basic axis bindings
        .add_axis::<EmptyAxis>(()) // Empty binding that always returns and will not contribute to the axis value.
        .add_axis::<ConstantAxis>(1.0) // AxisBinding is implemented for f32. It's a constant binding that always returns the given value.
        .add_axis::<KeyCodeAxis>(KeyCode::Space) // Binding that returns 1.0 when the specified key is pressed.
        .add_axis::<MouseButtonAxis>(MouseButton::Left) // Binding that returns 1.0 when the specified mouse button is pressed.
        .add_axis::<GamepadButtonAxis>(GamepadButton::South) // Binding that returns the value of the specified gamepad button.
        .add_axis::<GamepadAxisAxis>(GamepadAxis::LeftStickX) // Binding that returns the value of the specified gamepad axis.
        .add_axis::<MouseMovementAxis>(MouseY) // Binding that returns the mouse movement delta. Also works for MouseX.
        .add_axis::<MouseWheelAxis>(MouseWheel::default()) // Binding that returns the mouse wheel scroll delta.
        .add_axis::<BoxedAxis>(Box::new(KeyCode::KeyW) as Box<dyn AxisBinding>) // Box<dyn AxisBinding> also implements the AxisBinding trait.
        // Axis ombinators
        .add_axis::<TupleAxis>((KeyCode::KeyW, GamepadAxis::LeftStickX)) // Tuple of AxisBindings. All active bindings are averaged.
        .add_axis::<VecAxis>(vec![KeyCode::KeyW, KeyCode::ArrowUp]) // Vec of AxisBindings. All active bindings are averaged.
        .add_axis::<PairAxis>(Pair(KeyCode::KeyS, KeyCode::KeyW)) // Pair combinator that uses the first binding for negative direction and the second for positive.
        .add_axis::<WithTriggerAxis>(WithTriggerBinding(MouseY, MouseButton::Left)) // Axis that is only active when the trigger binding is active.
        // Axis filters
        .add_axis::<DeadzoneAxis>(Deadzone(GamepadAxis::LeftStickX, 0.2)) // Deadzone filter that ignores small input values.
        .add_axis::<SmoothAxis>(Smooth::new(GamepadAxis::LeftStickX, 0.1)) // Smooth filter that smooths input values over time.
        .add_axis::<NormalizeAxis>(Normalize(GamepadAxis::LeftStickX, GamepadAxis::LeftStickY)) // Constrain the first given axis to a unit circle when combined with the second axis.
        .add_axis::<RateLimitAxis>(RateLimit::new(GamepadAxis::LeftStickX, 1.0)) // Rate limit filter that limits how quickly the axis value can change over time.
        // Axis modifiers
        .add_axis::<MultiplyAxis>(Multiply(GamepadAxis::LeftStickX, 0.5)) // Modifier that multiplies two axis values together.
        .add_axis::<DivideAxis>(Divide(GamepadAxis::LeftStickX, 2.0)) // Modifier that divides the first axis by the second axis.
        .add_axis::<AddAxis>(Add(GamepadAxis::LeftStickX, 0.5)) // Modifier that adds two axis values together.
        .add_axis::<SubtractAxis>(Subtract(GamepadAxis::LeftStickX, 0.5)) // Modifier that subtracts the second axis from the first axis.
        .add_axis::<InvertAxis>(Invert(GamepadAxis::LeftStickX)) // Invert modifier that negates the axis value.
        .add_axis::<WithCurveAxis>(WithCurve(GamepadAxis::LeftStickX, EaseFunction::BounceIn)) // Modifier that applies a curve to the axis value.
        .add_axis::<TransformationAxis>(Transformation(GamepadAxis::LeftStickX, |v| v * v)) // Modifier that applies a custom transformation function to the axis value.
        //
        // Triggers:
        //
        // Basic trigger bindings
        .add_trigger::<EmptyTrigger>(()) // Empty trigger that is never active.
        .add_trigger::<ConstantTrigger>(true) // Constant trigger that reflects the given boolean value.
        .add_trigger::<KeyCodeTrigger>(KeyCode::Space) // Trigger that is active when the specified key is pressed.
        .add_trigger::<MouseButtonTrigger>(MouseButton::Left) // Trigger that is active when the specified mouse button is pressed.
        .add_trigger::<GamepadButtonTrigger>(GamepadButton::South) // Trigger that is active when the specified gamepad button is pressed.
        .add_trigger::<BoxedTrigger>(Box::new(KeyCode::KeyW) as Box<dyn TriggerBinding>) // Box<dyn TriggerBinding> also implements the TriggerBinding trait.
        // Trigger combinators
        .add_trigger::<TupleTrigger>((KeyCode::KeyW, GamepadButton::South)) // Tuple of TriggerBindings. Active if any binding is active.
        .add_trigger::<VecTrigger>(vec![KeyCode::KeyW, KeyCode::ArrowUp]) // Vec of TriggerBindings. Active if any binding is active.
        .add_trigger::<AndTrigger>(And(KeyCode::KeyW, GamepadButton::South)) // Combinator that is only active if both bindings are active.
        // Trigger modifiers
        .add_trigger::<NotTrigger>(Not(KeyCode::KeyW)) // Modifier that inverts the trigger state.
        .add_systems(
            Update,
            (
                visualize_basic,
                visualize_combinators,
                visualize_filters,
                visualize_modifiers,
                draw_triggers,
            ),
        )
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::WHITE))
        .run();
}

struct EmptyAxis;
struct ConstantAxis;
struct KeyCodeAxis;
struct MouseButtonAxis;
struct GamepadButtonAxis;
struct GamepadAxisAxis;
struct MouseMovementAxis;
struct MouseWheelAxis;
struct BoxedAxis;

struct TupleAxis;
struct VecAxis;
struct PairAxis;
struct WithTriggerAxis;

struct DeadzoneAxis;
struct SmoothAxis;
struct NormalizeAxis;
struct RateLimitAxis;

struct MultiplyAxis;
struct DivideAxis;
struct AddAxis;
struct SubtractAxis;
struct InvertAxis;
struct WithCurveAxis;
struct TransformationAxis;

struct EmptyTrigger;
struct ConstantTrigger;
struct KeyCodeTrigger;
struct MouseButtonTrigger;
struct GamepadButtonTrigger;
struct BoxedTrigger;

struct TupleTrigger;
struct VecTrigger;
struct AndTrigger;

struct NotTrigger;

#[allow(clippy::too_many_arguments)]
fn visualize_basic(
    mut empty: AxisVisualizer<EmptyAxis>,
    mut constant: AxisVisualizer<ConstantAxis>,
    mut keycode: AxisVisualizer<KeyCodeAxis>,
    mut mouse_button: AxisVisualizer<MouseButtonAxis>,
    mut gamepad_button: AxisVisualizer<GamepadButtonAxis>,
    mut gamepad_axis: AxisVisualizer<GamepadAxisAxis>,
    mut mouse_movement: AxisVisualizer<MouseMovementAxis>,
    mut mouse_wheel: AxisVisualizer<MouseWheelAxis>,
    mut boxed: AxisVisualizer<BoxedAxis>,
) {
    graph(&mut empty, 0, 0, SCALE);
    graph(&mut constant, 1, 0, SCALE);
    graph(&mut keycode, 2, 0, SCALE);
    graph(&mut mouse_button, 3, 0, SCALE);
    graph(&mut gamepad_button, 4, 0, SCALE);
    graph(&mut gamepad_axis, 5, 0, SCALE);
    graph(&mut mouse_movement, 6, 0, 1.0);
    graph(&mut mouse_wheel, 7, 0, 1.0);
    graph(&mut boxed, 8, 0, SCALE);
}

fn visualize_combinators(
    mut tuple: AxisVisualizer<TupleAxis>,
    mut vec: AxisVisualizer<VecAxis>,
    mut pair: AxisVisualizer<PairAxis>,
    mut with_trigger: AxisVisualizer<WithTriggerAxis>,
) {
    graph(&mut tuple, 0, 1, SCALE);
    graph(&mut vec, 1, 1, SCALE);
    graph(&mut pair, 2, 1, SCALE);
    graph(&mut with_trigger, 3, 1, 1.0);
}

fn visualize_filters(
    mut deadzone: AxisVisualizer<DeadzoneAxis>,
    mut smooth: AxisVisualizer<SmoothAxis>,
    mut normalize: AxisVisualizer<NormalizeAxis>,
    mut rate_limit: AxisVisualizer<RateLimitAxis>,
) {
    graph(&mut deadzone, 0, 2, SCALE);
    graph(&mut smooth, 1, 2, SCALE);
    graph(&mut normalize, 2, 2, SCALE);
    graph(&mut rate_limit, 3, 2, SCALE);
}

fn visualize_modifiers(
    mut multiply: AxisVisualizer<MultiplyAxis>,
    mut divide: AxisVisualizer<DivideAxis>,
    mut add: AxisVisualizer<AddAxis>,
    mut subtract: AxisVisualizer<SubtractAxis>,
    mut invert: AxisVisualizer<InvertAxis>,
    mut with_curve: AxisVisualizer<WithCurveAxis>,
    mut transformation: AxisVisualizer<TransformationAxis>,
) {
    graph(&mut multiply, 0, 3, SCALE);
    graph(&mut divide, 1, 3, SCALE);
    graph(&mut add, 2, 3, SCALE);
    graph(&mut subtract, 3, 3, SCALE);
    graph(&mut invert, 4, 3, SCALE);
    graph(&mut with_curve, 5, 3, SCALE);
    graph(&mut transformation, 6, 3, SCALE);
}

#[allow(clippy::too_many_arguments)]
fn draw_triggers(
    mut gizmos: Gizmos,
    empty: Res<Trigger<EmptyTrigger>>,
    constant: Res<Trigger<ConstantTrigger>>,
    keycode: Res<Trigger<KeyCodeTrigger>>,
    mouse_button: Res<Trigger<MouseButtonTrigger>>,
    gamepad_button: Res<Trigger<GamepadButtonTrigger>>,
    boxed: Res<Trigger<BoxedTrigger>>,
    tuple: Res<Trigger<TupleTrigger>>,
    vec: Res<Trigger<VecTrigger>>,
    and: Res<Trigger<AndTrigger>>,
    not: Res<Trigger<NotTrigger>>,
) {
    draw_trigger(&mut gizmos, &empty, 0);
    draw_trigger(&mut gizmos, &constant, 1);
    draw_trigger(&mut gizmos, &keycode, 2);
    draw_trigger(&mut gizmos, &mouse_button, 3);
    draw_trigger(&mut gizmos, &gamepad_button, 4);
    draw_trigger(&mut gizmos, &boxed, 5);
    draw_trigger(&mut gizmos, &tuple, 6);
    draw_trigger(&mut gizmos, &vec, 7);
    draw_trigger(&mut gizmos, &and, 8);
    draw_trigger(&mut gizmos, &not, 9);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

const SCALE: f32 = 32.0;
const MAX_COLUMNS: u32 = 9;
const MAX_ROWS: u32 = 4;
const TRIGGER_COUNT: u32 = 10;

fn graph<A: Send + Sync + 'static>(visualizer: &mut AxisVisualizer<A>, x: i32, y: i32, scale: f32) {
    let timespan = Duration::from_secs(5);
    let margin = 32.0;
    let size = 64.0;

    let width = MAX_COLUMNS as f32 * (size + margin) - margin;
    let height = MAX_ROWS as f32 * (size + margin) - margin;

    let pos = Vec2::new(
        -width / 2.0 + x as f32 * (size + margin) + size / 2.0,
        height / 2.0 - y as f32 * (size + margin) - size / 2.0,
    );
    let color = Color::srgb(
        x as f32 / MAX_COLUMNS as f32,
        y as f32 / MAX_ROWS as f32,
        1.0 - (x as f32 / MAX_COLUMNS as f32),
    );

    visualizer.graph_x(timespan, pos, scale, Vec2::splat(size), color);
}

fn draw_trigger<T: Send + Sync + 'static>(
    gizmos: &mut Gizmos,
    trigger: &Res<Trigger<T>>,
    index: u32,
) {
    let is_pressed = trigger.pressed();
    let color = if is_pressed {
        Srgba::RED
    } else {
        Srgba::gray(0.4)
    };

    let size = 8.0;
    let margin = 32.0;

    let width = TRIGGER_COUNT as f32 * (size + margin) - margin;

    let pos = Vec2::new(
        -width / 2.0 + index as f32 * (size + margin) + size / 2.0,
        -240.0,
    );

    gizmos.circle_2d(pos, size, color);
}
