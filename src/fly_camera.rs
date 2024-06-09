use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct FlyCamera {
    pub accel: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_backward: KeyCode,
    pub key_forward: KeyCode,
    pub key_jump: KeyCode,
    pub key_sneak: KeyCode,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            accel: 1.5,
            max_speed: 0.5,
            friction: 1.0,
            sensitivity: 10.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            key_left: KeyCode::KeyS,
            key_right: KeyCode::KeyF,
            key_backward: KeyCode::KeyD,
            key_forward: KeyCode::KeyE,
            key_jump: KeyCode::Space,
            key_sneak: KeyCode::ShiftLeft,
        }
    }
}

pub struct FlyCameraPlugin;
impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (movement, look));
    }
}

fn movement(
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    for (mut options, mut transform) in query.iter_mut() {
        let (x, z, y) = (
            movement_axis(&kb, options.key_right, options.key_left),
            movement_axis(&kb, options.key_backward, options.key_forward),
            movement_axis(&kb, options.key_jump, options.key_sneak),
        );

        let rotation = transform.rotation;
        let accel: Vec3 =
            strafe_vector(&rotation) * x + forward_vector(&rotation) * z + Vec3::Y * y;
        let accel: Vec3 = if accel.length() != 0.0 {
            accel.normalize() * options.accel
        } else {
            Vec3::ZERO
        };

        let friction: Vec3 = if options.velocity.length() != 0.0 {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::ZERO
        };

        options.velocity += accel * time.delta_seconds();

        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity =
            if (options.velocity + delta_friction).signum() != options.velocity.signum() {
                Vec3::ZERO
            } else {
                options.velocity + delta_friction
            };

        transform.translation += options.velocity;
    }
}

fn look(
    time: Res<Time>,
    mut mouse_ev: EventReader<MouseMotion>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    for (mut options, mut transform) in query.iter_mut() {
        let mut delta: Vec2 = Vec2::ZERO;
        for ev in mouse_ev.read() {
            delta += ev.delta;
        }
        if delta.is_nan() {
            return;
        }

        options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
        options.pitch += delta.y * options.sensitivity * time.delta_seconds();

        options.pitch = options.pitch.clamp(-89.9, 89.9);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, options.yaw.to_radians())
            * Quat::from_axis_angle(-Vec3::X, options.pitch.to_radians());
    }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    let f = rotation.mul_vec3(Vec3::Z).normalize();
    Vec3::new(f.x, 0.0, f.z).normalize()
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_vector(rotation))
        .normalize()
}

fn movement_axis(kb: &Res<ButtonInput<KeyCode>>, plus: KeyCode, minus: KeyCode) -> f32 {
    let mut axis = 0.0;
    if kb.pressed(plus) {
        axis += 1.0;
    }
    if kb.pressed(minus) {
        axis -= 1.0;
    }
    axis
}
