use bevy::pbr::PbrBundle;
use bevy::pbr::PointLightBundle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh::shape;
use bevy::render::mesh::Mesh;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use ggrs::{
    Config, P2PSession, PlayerType, SessionBuilder, SpectatorSession, SyncTestSession,
    UdpNonBlockingSocket,
};
use std::collections::HashSet;
use std::env;
use std::net::SocketAddr;

use crate::animation::animation_helper;
use crate::players::{info, movement};
use crate::worlds::world_manager;

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
    //start creating p2p session
    let num_players = p2p_session
        .map(|s| s.num_players())
        .or_else(|| synctest_session.map(|s| s.num_players()))
        .or_else(|| spectator_session.map(|s| s.num_players()))
        .expect("No GGRS session found");

    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    // Add player scene.
    let mut player_handle = asset_server.load("mixamo/shoot.glb#Scene0");

    // Players identified in ggrs by handles starting from 0.
    for handle in 0..num_players {
        if handle == 1 {
            // TODO
            player_handle = asset_server.load("mixamo/ninja_tpose.glb#Scene0");
        }
        else {
            player_handle = asset_server.load("mixamo/shoot.glb#Scene0");
        }
        let entity_id = commands
            // Create player.
            .spawn_bundle(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(handle as f32, 0.0, -5.0),
                    ..default()
                },
                scene: player_handle.clone(),
                ..default()
            })
            
            // Add player information.
            .insert(info::Player {
                handle: handle as u32,
                money: 50,
                bounties: 3,
                friends: HashSet::new(),
                health: 100,
                world: 0,
                plane: world_manager::IPlane::new(0, 0, 0),
                state: info::PlayerState::default(),
                target: info::MovementTarget::default(),
                speed: info::MovementSpeed { speed: 3.0 },
            })
            .insert(info::Information::default())
            .insert_bundle(PickableBundle::default()) // Player can be clicked.
            
            // Indicates bevy_GGRS that this entity should be saved and loaded.
            .insert(Rollback::new(rip.next_id()))
            
            // Physics
            .insert(LockedAxes::ROTATION_LOCKED) 
            .insert(RigidBody::Dynamic)
            .with_children(|children| {
                children.spawn()
                    .insert(Collider::cuboid(0.5, 1.0, 0.5))
                    // Position the collider relative to the rigid-body.
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
            })
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)))
            
            // Animation Helper
            .insert(animation_helper::AnimationHelperSetup)
            .id();

        // Insert my player.
        let q: usize = query.parse().unwrap();
        if q == handle {
            commands.entity(entity_id).insert(Me);
        }
    }
    println!("setup system");
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

// create a GGRS session (only runs locally for now)
pub fn create_ggrs_session() -> Result<SessionBuilder<GGRSConfig>, Box<dyn std::error::Error>> {
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(2)
        .with_max_prediction_window(12) // (optional) set max prediction window
        .with_input_delay(2); // (optional) set input delay for the local player

    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    sess_build = sess_build.add_player(PlayerType::Local, query.parse().unwrap())?;
    if query == "0" {
        let player_addr: &String = &String::from("127.0.0.1:7001");
        // Should receive addresses of discovered peers
        let remote_addr: SocketAddr = player_addr.parse()?;
        sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), 1)?;
    } else {
        let player_addr: &String = &String::from("127.0.0.1:7000");
        // Should receive addresses of discovered peers
        let remote_addr: SocketAddr = player_addr.parse()?;
        sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), 0)?;
    }

    Ok(sess_build)
}

// Start the GGRS session for my port.
pub fn start_ggrs_session(
    sess_build: SessionBuilder<GGRSConfig>,
) -> Result<P2PSession<GGRSConfig>, Box<dyn std::error::Error>> {
    // Read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

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
