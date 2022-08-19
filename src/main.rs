use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, SessionType};
use camera::follow_me;
use ggrs::{P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use std::env;
use std::net::SocketAddr;

use bevy::input::mouse::MouseMotion;
use bevy_dolly::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod gltf;
use gltf::imported;

mod ggrs_rollback;
mod players;
use ggrs_rollback::{ggrs_camera, network};
use players::{display, fight, info, interact, movement};

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";

// cargo run -- --local-port 7000 --players localhost 127.0.0.1:7001
// cargo run -- --local-port 7001 --players 127.0.0.1:7000 localhost
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a GGRS session
    let sess_build = network::create_ggrs_session().unwrap();

    // start the GGRS session
    let sess = network::start_ggrs_session(sess_build).unwrap();

    let mut app = App::new();
    GGRSPlugin::<network::GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(movement::input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_type::<Transform>()
        .register_rollback_type::<info::Velocity>()
        .register_rollback_type::<info::FrameCount>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(network::move_player)
                    //.with_system(movement::move_cube_system)
                    .with_system(movement::increase_frame_system),
                //.with_system(pcg_city::buildings::spawn_buildings), //i think spawning can't be done in rollback
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    // continue building/running the app like you normally would
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            //must come before default plugin
            width: 1700.,
            height: 1000.,
            title: "Insight".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(network::setup_system)
        // add your GGRS session
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession)
        // register a resource that will be rolled back
        .insert_resource(info::FrameCount { frame: 0 })
        //my code
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DollyCursorGrab)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(ggrs_camera::setup_camera)
        .add_system(ggrs_camera::update_camera);

    //art
    app.add_startup_system(imported::create_default_plane) //animation of gltf not yet updated in rollback
        .add_system(imported::play_scene);

    // .add_system(follow_me::update_camera) //puts camera behind player
    // .add_system(follow_me::frame); //follows player

    //egui
    app.add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new());

    app.run();

    Ok(())
}
