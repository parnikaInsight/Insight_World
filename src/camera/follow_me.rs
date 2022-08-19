use bevy_dolly::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_ggrs::Rollback;
use bevy_mod_picking::*;

use crate::players::{info, movement};
use crate::ggrs_rollback::network;

pub fn frame(
    mut q: Query<(&mut CameraRig, &network::Rig)>,
    mut me: Query<(&mut Transform, &info::Player, &network::Me), With<Rollback>>,
) {
    let (mut t, p, m) = me.single_mut();
    println!("Me: {:?}", t);
    let (mut camera, rig) = q.single_mut();
    camera.driver_mut::<Position>().translation = t.translation;
    camera.driver_mut::<Rotation>().rotation = t.rotation;
    camera.driver_mut::<LookAt>().target = t.translation + Vec3::Y;
}

pub fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(
        Query<(&mut Transform, With<network::MainCamera>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let boost_mult = 5.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    // Q: Is dolly left-handed so z is flipped?
    // if keys.pressed(KeyCode::Up) {
    //     move_vec.z -= 1.0;
    // }
    // if keys.pressed(KeyCode::Down) {
    //     move_vec.z += 1.0;
    // }
    // if keys.pressed(KeyCode::Left) {
    //     move_vec.x -= 1.0;
    // }
    // if keys.pressed(KeyCode::Right) {
    //     move_vec.x += 1.0;
    // }

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


