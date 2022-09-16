#![allow(dead_code)]

use bevy::prelude::*;
use bevy_dolly::prelude::*;
//use bevy_egui::EguiPlugin;
use bevy_ggrs::{GGRSPlugin, SessionType};
//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

mod animation;
mod default_world;
mod ggrs_rollback;
mod players;
mod worlds;
mod systems;

use animation::{animation_helper, play};
use default_world::create_default;
use ggrs_rollback::{ggrs_camera, network};
use players::{info, movement, physics};
use worlds::{create_insight, player};

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";
const ROLLBACK_DEFAULT2: &str = "rollback_default2"; 
// cargo run -- --local-port 7000 --players localhost 127.0.0.1:7001
// cargo run -- --local-port 7001 --players 127.0.0.1:7000 localhost
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a GGRS session.
    let sess_build = network::create_ggrs_session().unwrap();

    // Start the GGRS session.
    let sess = network::start_ggrs_session(sess_build).unwrap();

    let mut app = App::new();
    // GGRS Configuration
    GGRSPlugin::<network::GGRSConfig>::new()
        // Define frequency of rollback game logic update.
        .with_update_frequency(FPS)
        // Define system that returns inputs given a player handle, so GGRS can send the inputs.
        .with_input_system(movement::input)
        // Register types of components and resources you want to be rolled back.
        .register_rollback_type::<Transform>()
        //.register_rollback_type::<info::Velocity>()
        // These systems will be executed as part of the advance frame update.
        .with_rollback_schedule(Schedule::default()
            .with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel().with_system(movement::translate_player),
            )
            .with_stage_after(
                ROLLBACK_DEFAULT,
                ROLLBACK_DEFAULT2,
                SystemStage::parallel().with_system(movement::animate_moving_player),
            )
        )
        .build(&mut app);

    // GGRS Setup
    app // Add your GGRS session.
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession);

    //General Setup
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            // This must come before default plugin.
            width: 800.,
            height: 800.,
            title: "InsightWorld".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_plugin(DollyCursorGrab)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default());

    // Camera
    app.add_startup_system(ggrs_camera::setup_camera)
        .add_system(ggrs_camera::update_camera);

    // Setup Players
    app.add_startup_system(network::setup_system) // Start p2p session and add players.
        .add_startup_system(play::setup_character) // Insert player animations.
        .add_system(animation_helper::setup_helpers); // Find AnimationHelperSetup markers for players.

    // // Create default plane.
    // app.add_startup_system(create_default::create_default_plane);

    app.add_startup_system(create_insight::create_insight_world);

    // Play stationary animations
    //  .add_system(play::play_scene);

    //egui
    // app.add_plugin(EguiPlugin)
    //     .add_plugin(WorldInspectorPlugin::new()); // Records all assets.

    app.run();

    Ok(())
}
