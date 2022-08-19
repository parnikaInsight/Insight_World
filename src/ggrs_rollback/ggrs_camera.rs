use bevy::input::mouse::MouseMotion;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::*;


#[derive(Component)]
pub struct MainCamera;

/// set up a simple 3D scene
pub fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let translation = [-2.0f32, 2.0f32, 5.0f32];
       // let translation = [3.0f32, 1.0f32, 1.0f32];
  //  let translation = [0.0f32, 30.0f32, 0.0f32];
    let transform = Transform::from_translation(bevy::math::Vec3::from_slice(&translation))
        .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    let rotation = transform.rotation;
    let mut yaw_pitch = YawPitch::new();
    yaw_pitch.set_rotation_quat(rotation);

    commands.spawn().insert(
        CameraRig::builder()
            .with(Position {
                translation: Vec3::from_slice(&translation),
            })
            .with(Rotation { rotation })
            .with(yaw_pitch)
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build(),
    );

    commands
        .spawn_bundle(Camera3dBundle {
            transform,
            ..Default::default()
        })
        .insert(UiCameraConfig { //idk why not displaying
            show_ui: true,
            ..default()
        })

        .insert_bundle(PickingCameraBundle::default())
        .insert(MainCamera);

    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
pub fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let boost_mult = 5.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    // Q: Is dolly left-handed so z is flipped?
    if keys.pressed(KeyCode::Up) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::Left) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        move_vec.y -= 1.0;
    }

    let boost: f32 = if keys.pressed(KeyCode::LShift) {
        1.
    } else {
        0.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    let mut q1 = query.p1();
    let mut rig = q1.single_mut();

    let move_vec =
        rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost_mult.powf(boost);

    let window = windows.get_primary().unwrap();
    if window.cursor_locked() {
        rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
            -0.1 * delta.x * sensitivity.x,
            -0.1 * delta.y * sensitivity.y,
        );
        rig.driver_mut::<Position>()
            .translate(move_vec * time_delta_seconds * 10.0);
    }

    let transform = rig.update(time_delta_seconds);
    let mut q0 = query.p0();
    let (mut cam, _) = q0.single_mut();

    cam.update(transform);
}
