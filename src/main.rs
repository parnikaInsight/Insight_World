#![allow(dead_code)]

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_dolly::prelude::*;
//use bevy_egui::EguiPlugin;
use bevy_ggrs::{GGRSPlugin, SessionType};
//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

mod animation;
mod colliders;
mod default_world;
mod ggrs_rollback;
mod players;
mod systems;
mod worlds;

use animation::{animation_helper, play};
use colliders::collider;
use default_world::create_default;
use ggrs_rollback::{follow_camera, ggrs_camera, network};
use players::movement::FrameTimeDiagnosticsState;
use players::{info, movement, physics};
use worlds::{create_insight, player};

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";
const ROLLBACK_DEFAULT2: &str = "rollback_default2";
// cargo run -- --local-port 7000 --players localhost 127.0.0.1:7001
// cargo run -- --local-port 7001 --players 127.0.0.1:7000 localhost
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Create a GGRS session.
    // let sess_build = network::create_ggrs_session().unwrap();

    // // Start the GGRS session.
    // let sess = network::start_ggrs_session(sess_build).unwrap();

    let mut app = App::new();
    // // GGRS Configuration
    // GGRSPlugin::<network::GGRSConfig>::new()
    //     // Define frequency of rollback game logic update.
    //     .with_update_frequency(FPS)
    //     // Define system that returns inputs given a player handle, so GGRS can send the inputs.
    //     .with_input_system(movement::input)
    //     // Register types of components and resources you want to be rolled back.
    //     .register_rollback_type::<Transform>()
    //     //.register_rollback_type::<info::Velocity>()
    //     // These systems will be executed as part of the advance frame update.
    //     .with_rollback_schedule(
    //         Schedule::default()
    //             .with_stage(
    //                 ROLLBACK_DEFAULT,
    //                 SystemStage::parallel().with_system(movement::translate_player),
    //             )
    //             .with_stage_after(
    //                 ROLLBACK_DEFAULT,
    //                 ROLLBACK_DEFAULT2,
    //                 SystemStage::parallel().with_system(movement::animate_moving_player),
    //             ),
    //     )
    //     .build(&mut app);

    // // GGRS Setup
    // app // Add your GGRS session.
    //     .insert_resource(sess)
    //     .insert_resource(SessionType::P2PSession);

    //General Setup
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            // This must come before default plugin.
            width: 1000.,
            height: 800.,
            title: "InsightWorld".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_plugin(DollyCursorGrab)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(collider::ColliderBuilderPlugin::default());

    // Camera
    app.add_startup_system(ggrs_camera::setup_camera)
        .add_system(ggrs_camera::update_camera);

    // // Follow Camera (uncomment in network.rs)
    // app.add_system(follow_camera::update_camera) //puts camera behind player
    //     .add_system(follow_camera::frame); //follows player

    // Setup Players
    app.add_startup_system(network::setup_system) // Start p2p session and add players.
        .add_startup_system(play::setup_character) // Insert player animations.
        .add_system(animation_helper::setup_helpers); // Find AnimationHelperSetup markers for players.

    // Single player movement
    app.add_startup_system(starting_inputs);
    app.add_system(movement::input.after(network::setup_system));
    app.add_system(movement::translate_player.after(movement::input));
    app.add_system(movement::animate_moving_player.after(movement::translate_player));
    app.insert_resource(MyMoves{moves: vec![(0.0, 0)]});
    app.insert_resource(PrevInput{prev_input: 0});
    app.insert_resource(FrameTimeDiagnosticsState{frame_count: 0.0});
    app.add_system(movement::inc_frame);

    // // Create default plane.
    app.add_startup_system(create_default::create_default_plane);

    app.add_startup_system(create_insight::create_insight_world);

    // Play stationary animations
    app.add_system(play::play_scene);

    //egui
    // app.add_plugin(EguiPlugin)
    //     .add_plugin(WorldInspectorPlugin::new()); // Records all assets.

    app.run();

    Ok(())
}

pub fn starting_inputs (mut commands: Commands) {
    let input = 0;
    let vec = vec![movement::BoxInput { inp: input }];
    commands.insert_resource(vec);
}

#[derive(Default, Debug)]
pub struct MyMoves{
    moves: Vec<(f64, u8)>
} // (frame, move)

#[derive(Default, Debug)]
pub struct PrevInput{
    prev_input: u8
} //