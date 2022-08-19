use bevy::math::Quat;
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider, SessionType};
use bevy_mod_picking::*;
use bevy_pbr::PbrBundle;
use bevy_pbr::PointLightBundle;
use bevy_pbr::StandardMaterial;
use bevy_rapier3d::prelude::*;
use bevy_render::color::Color;
use bevy_render::mesh::shape;
use bevy_render::mesh::Mesh;

use bevy_dolly::prelude::*;

use bytemuck::{Pod, Zeroable};
use core::ops::Add;
use ggrs::{
    Config, InputStatus, P2PSession, PlayerHandle, PlayerType, SessionBuilder, SpectatorSession,
    SyncTestSession, UdpNonBlockingSocket,
};
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::{hash::Hash, net::SocketAddr};

use crate::players::{info, movement};

const PLANE_SIZE: f32 = 15.0;
const CUBE_SIZE: f32 = 0.2;
const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rip: ResMut<RollbackIdProvider>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    p2p_session: Option<Res<P2PSession<GGRSConfig>>>,
    synctest_session: Option<Res<SyncTestSession<GGRSConfig>>>,
    spectator_session: Option<Res<SpectatorSession<GGRSConfig>>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE })),
        material: materials.add(PLAYER_COLORS[0.0 as usize].into()),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            ..default()
        },
        ..Default::default()
    });
    
    println!("started setup system");
    let num_players = p2p_session
        .map(|s| s.num_players())
        .or_else(|| synctest_session.map(|s| s.num_players()))
        .or_else(|| spectator_session.map(|s| s.num_players()))
        .expect("No GGRS session found");

    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })), 
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(7.5, 7.5, 7.5)) //half the cube size
        .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

    // player cube - just spawn whatever entity you want, then add a `Rollback` component with a unique id (for example through the `RollbackIdProvider` resource).
    // Every entity that you want to be saved/loaded needs a `Rollback` component with a unique rollback id.
    // When loading entities from the past, this extra id is necessary to connect entities over different game states
    let r = PLANE_SIZE / 4.;

    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    // Insert a resource with the current scene information
    // commands.insert_resource(Animations(vec![
    //     asset_server.load("mixamo/from_blender.glb#Animation0")
    // ]));
   // let player_handle = asset_server.load("mixamo/from_blender.glb#Scene0");

    for handle in 0..num_players {
        let rot = handle as f32 / num_players as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let z = r * rot.sin();

        let mut transform = Transform::default();
        transform.translation.x = x;
        transform.translation.y = CUBE_SIZE / 2.;
        transform.translation.z = z;

        let mut rng = thread_rng();
        let x_loc: f32 = rng.gen();

        let player_handle = asset_server.load("mixamo/from_blender.glb#Scene0");

        let entity_id = commands
            // .spawn_bundle(PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE })),
            //     material: materials.add(PLAYER_COLORS[handle as usize].into()),
            //     transform,
            //     ..Default::default()
            // })
            .spawn_bundle(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(handle as f32, 0.0, -5.0),
                    ..default()
                },
                scene: player_handle.clone(),
                ..default()
            })
            .insert(info::Player {
                handle: handle as u32,
                money: 50,
                bounties: 3,
                friends: HashSet::new(),
                health: 100,
            })
            .insert(info::Velocity::default())
            .insert(info::Information::default())
            .insert_bundle(PickableBundle::default())
            // this component indicates bevy_GGRS that parts of this entity should be saved and loaded
            .insert(Rollback::new(rip.next_id()))
            .insert(RigidBody::Dynamic)
            //.insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_X)
            // .insert(Collider::cuboid(
            //     CUBE_SIZE / 2.0,
            //     CUBE_SIZE / 2.0,
            //     CUBE_SIZE / 2.0,
            // )) //half the cube size
            // .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)))
            .id();

        //insert Me
        let q: usize = query.parse().unwrap();
        if q == handle {
            println!("added me");
            commands.entity(entity_id).insert(Me);

            // let mut yaw_pitch = YawPitch::new();
            // yaw_pitch.set_rotation_quat(Quat::default());

            // let t = Vec3::new(handle as f32, 0.0, 0.0);
            // let camera = CameraRig::builder()
            //     .with(Position::new(t))
            //     .with(Rotation::new(Quat::default()))
            //     .with(Smooth::new_position(1.25).predictive(true))
            //     .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
            //     .with(Smooth::new_position(2.5))
            //     .with(yaw_pitch)
            //     .with(
            //         LookAt::new(t + Vec3::Y)
            //             .tracking_smoothness(1.25)
            //             .tracking_predictive(true),
            //     )
            //     .build();

            // commands.spawn().insert(camera).insert(Rig);

            // let t_cam = Vec3::new(handle as f32, 2.0, 5.0);
            // commands
            //     .spawn_bundle(Camera3dBundle {
            //         transform: Transform{translation: t_cam, ..default()},
            //         ..Default::default()
            //     })
            //     .insert(UiCameraConfig {
            //         //idk why not displaying
            //         show_ui: true,
            //         ..default()
            //     })
            //     .insert_bundle(PickingCameraBundle::default())
            //     .insert(MainCamera);

            // // light
            // commands.spawn_bundle(PointLightBundle {
            //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
            //     ..Default::default()
            // });
        }
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    println!("finished setup system");
}

#[derive(Component)]
pub struct Rig;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Component)]
pub struct Me;

/// You need to define a config struct to bundle all the generics of GGRS. You can safely ignore `State` and leave it as u8 for all GGRS functionality.
/// TODO: Find a way to hide the state type.
#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = movement::BoxInput;
    type State = u8;
    type Address = SocketAddr;
}

// create a GGRS session /
pub fn create_ggrs_session() -> Result<SessionBuilder<GGRSConfig>, Box<dyn std::error::Error>> {
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(2)
        .with_max_prediction_window(12) // (optional) set max prediction window
        .with_input_delay(2); // (optional) set input delay for the local player

    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("Searching for {}", query);

    sess_build = sess_build.add_player(PlayerType::Local, query.parse().unwrap())?;
    if query == "0" {
        let player_addr: &String = &String::from("127.0.0.1:7001");
        let remote_addr: SocketAddr = player_addr.parse()?; //receive addr of discovered peers
        sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), 1)?;
    } else {
        let player_addr: &String = &String::from("127.0.0.1:7000");
        let remote_addr: SocketAddr = player_addr.parse()?; //receive addr of discovered peers
        sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), 0)?;
    }

    Ok(sess_build)
}

pub fn start_ggrs_session(
    sess_build: SessionBuilder<GGRSConfig>,
) -> Result<P2PSession<GGRSConfig>, Box<dyn std::error::Error>> {
    // start the GGRS session

    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("Searching for {}", query);

    //let socket = UdpNonBlockingSocket::bind_to_port("/ip4/0.0.0.0/udp/0/quic")?;
    let sess: P2PSession<GGRSConfig>;
    if query == "0" {
        let socket = UdpNonBlockingSocket::bind_to_port(7000)?;
        sess = sess_build.start_p2p_session(socket)?;
    } else {
        let socket = UdpNonBlockingSocket::bind_to_port(7001)?;
        sess = sess_build.start_p2p_session(socket)?;
    }

    Ok(sess)
}

//movement-------------------------------------------------------------------
// pub struct Animations(Vec<Handle<AnimationClip>>); // breaks when in character.rs says resource not found, need to clean this file

// pub fn animate(animations: Res<Animations>, mut player: Query<&mut AnimationPlayer>) {
//     for mut player in &mut player {
//         player.play(animations.0[0].clone_weak()).repeat();
//         println!("Player animation from separate fn")
//     }
// }
// Once the scene is loaded, start the animation
pub fn move_player(
    // animations: Res<Animations>,
    // mut player: Query<&mut AnimationPlayer>,
    inputs: Res<Vec<(movement::BoxInput, InputStatus)>>,
    mut query: Query<(&mut Transform, &info::Player), With<Rollback>>,
) {
    for (mut t, p) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;
        // set velocity through key presses

        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            // for mut player in &mut player {
            //     player.play(animations.0[0].clone_weak());
            //     println!("Player animation")
            // }
            t.translation.z += 0.1;
            //t.rotation = Add::add(t.rotation, Quat::from_euler(EulerRot::YZX, 0.75, 0.0, 0.0));
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            // for mut player in &mut player {
            //     player.play(animations.0[0].clone_weak());
            //     println!("Player animation")
            // }
            t.translation.z -= 0.1;
        }
        // A
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            // for mut player in &mut player {
            //     player.play(animations.0[0].clone_weak());
            //     println!("Player animation")
            // }
            t.translation.x += 0.1;
            //t.rotation = Add::add(t.rotation, Quat::from_euler(EulerRot::YZX, 0.25, 0.0, 0.0));
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            // for mut player in &mut player {
            //     player.play(animations.0[0].clone_weak());
            //     println!("Player animation")
            // }
            t.translation.x -= 0.1;
            //t.rotation = Add::add(t.rotation, Quat::from_euler(EulerRot::YZX, 0.25, 0.0, 0.0));
        }
    }
}

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
