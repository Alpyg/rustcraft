use std::{f32::consts::FRAC_PI_2, f32::consts::SQRT_2};

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::player::{Player, PlayerCamera};

use super::{Acceleration, Velocity};

const MOUSE_SENSITIVITY: f32 = 0.01;
const JUMP_SPEED: f32 = 1.0;

struct Speed;
impl Speed {
    const SPRINT: f32 = 1.3;
    const WALK: f32 = 1.0;
    const SNEAK: f32 = 0.3;
}
struct SpeedFactor;
impl SpeedFactor {
    const DEFAULT: f32 = 0.98;
    const STRAFE: f32 = 1.0;
    const SNEAK_STRAFE: f32 = 0.98 * SQRT_2;
}

struct SlipperinessFactor;
impl SlipperinessFactor {
    const DEFAULT: f32 = 0.6;
    const SLIME: f32 = 0.8;
    const ICE: f32 = 0.98;
    const AIR: f32 = 1.0;
}

#[derive(Component)]
pub struct SurvivalControls;

impl InputContext for SurvivalControls {
    fn context_instance(_world: &World, _entity: Entity) -> ContextInstance {
        let mut ctx = ContextInstance::default();

        ctx.bind::<Move>()
            .to(Cardinal {
                north: KeyCode::KeyE,
                east: KeyCode::KeyS,
                south: KeyCode::KeyD,
                west: KeyCode::KeyF,
            })
            .with_modifiers(DeltaLerp::default());
        ctx.bind::<Look>().to(Input::mouse_motion());
        ctx.bind::<Sprint>()
            .to(KeyCode::ControlLeft)
            .with_conditions(JustPress::default());
        ctx.bind::<Sneak>().to(KeyCode::ShiftLeft);
        ctx.bind::<Jump>().to(KeyCode::Space);

        ctx
    }
}

#[derive(Component)]
pub(super) struct Sprinting;

#[derive(Component)]
pub(super) struct Sneaking;

#[derive(InputAction, Debug)]
#[input_action(output = Vec2)]
struct Move;

#[derive(InputAction, Debug)]
#[input_action(output = Vec2)]
struct Look;

#[derive(InputAction, Debug)]
#[input_action(output = bool)]
struct Sprint;

#[derive(InputAction, Debug)]
#[input_action(output = bool)]
struct Sneak;

#[derive(InputAction, Debug)]
#[input_action(output = bool)]
struct Jump;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<SurvivalControls>()
        .add_observer(handle_move)
        .add_observer(handle_look)
        .add_observer(handle_sprint)
        //.add_observer(handle_sneak)
        .add_observer(handle_jump)
        .add_systems(FixedUpdate, physics);
}

fn handle_move(
    trigger: Trigger<Fired<Move>>,
    mut commands: Commands,
    player: Single<(Entity, &mut Acceleration, Has<Sprinting>, Has<Sneaking>), With<Player>>,
    camera: Single<&GlobalTransform, With<PlayerCamera>>,
) {
    let event = trigger.event();
    let (player_entity, mut acceleration, is_sprinting, is_sneaking) = player.into_inner();
    let camera_transform = camera.into_inner().compute_transform();

    let mut delta = if event.value.length_squared() > 0.01 {
        event.value
            * if is_sneaking {
                Vec2::splat(Speed::SNEAK)
            } else if is_sneaking {
                Vec2::splat(Speed::SPRINT)
            } else {
                Vec2::splat(Speed::WALK)
            }
    } else {
        commands.entity(player_entity).remove::<Sprinting>();
        Vec2::ZERO
    };

    delta *= if delta.x.abs() - delta.y.abs() < 0.01 {
        if is_sneaking {
            SpeedFactor::SNEAK_STRAFE
        } else if is_sprinting {
            SpeedFactor::STRAFE
        } else {
            SpeedFactor::DEFAULT
        }
    } else {
        SpeedFactor::DEFAULT
    };

    // TODO: Add proper slipperiness and effect multipliers
    let slipperiness = SlipperinessFactor::DEFAULT;

    let mut forward = camera_transform.forward().as_vec3();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = camera_transform.right().as_vec3();
    right.y = 0.0;
    right = right.normalize();

    **acceleration = 0.1 * (delta.x * right + delta.y * forward) * (0.6 / slipperiness).powf(3.0);
}

fn handle_look(trigger: Trigger<Fired<Look>>, player: Single<&mut Transform, With<PlayerCamera>>) {
    let event = trigger.event();
    let mut player = player.into_inner();

    let look = event.value;
    if look != Vec2::ZERO {
        let delta_yaw = -look.x * MOUSE_SENSITIVITY;
        let delta_pitch = -look.y * MOUSE_SENSITIVITY;

        let (yaw, pitch, roll) = player.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        player.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn handle_sprint(
    _trigger: Trigger<Started<Sprint>>,
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    let player_entity = player.into_inner();
    commands.entity(player_entity).insert(Sprinting);
}

//fn handle_sneak(
//    _trigger: Trigger<Fired<Sneak>>,
//    mut commands: Commands,
//    player: Single<(Entity, &Transform, &mut TnuaController, Has<Sprinting>), With<Player>>,
//) {
//    let (player_entity, transform, mut controller, is_sprinting) = player.into_inner();
//    controller.action(TnuaBuiltinCrouch::default());
//}

fn handle_jump(_trigger: Trigger<Fired<Jump>>, _player: Single<(), With<Player>>) {}

fn physics(player: Single<(&mut Transform, &Acceleration, &mut Velocity)>) {
    let (mut transform, acceleration, mut velocity) = player.into_inner();

    let momentum = **velocity * 0.91;
    **velocity = momentum + **acceleration;

    transform.translation += **velocity;

    // Drag?
    **velocity *= 0.05;
}
