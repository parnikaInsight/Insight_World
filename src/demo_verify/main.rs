use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_dolly::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_ggrs::{GGRSPlugin, SessionType};
//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

mod verify;
use verify::{recreate, camera, play, animation_helper, movement, win};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mystruct = RapierConfiguration {
        gravity: Vect::Y * -9.81,
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Fixed {
            dt: 0.1,
            substeps: 1,
        },
        scaled_shape_subdivision: 10,
       
    };
    //General Setup
    app.insert_resource(Msaa { samples: 4 })
    .insert_resource(mystruct)
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
        .add_plugin(EguiPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_plugin(DollyCursorGrab)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default());

    // Camera
    app.add_startup_system(camera::setup_camera)
        .add_system(camera::update_camera);

    // Create default plane.
    app.add_startup_system(recreate::create_default_plane);

    // Setup Players
    app.add_startup_system(play::setup_character) // Insert player animations.
        .add_system(animation_helper::setup_helpers); // Find AnimationHelperSetup markers for players.

    // Play stationary animations
    app.add_system(play::play_scene);

    // Frame time
    app.insert_resource(movement::FrameTimeDiagnosticsState{frame_count: 0.0});
    app.add_system(movement::inc_frame);


    // // Single player movement
    app.add_system(movement::translate);
    app.add_system(movement::animate_moving_player);

    app.insert_resource(MyMoves{moves: vec![(0.0, 0)]});
    app.insert_resource(WinningMoves{moves: vec![(0.0, 0)]});
    // app.insert_resource(PrevInput{prev_input: 0});

    app.add_system(win::objective_completion.after(movement::translate));


    app.run();

    Ok(())
}

#[derive(Default, Debug)]
pub struct MyMoves{
    moves: Vec<(f64, u8)>
} // (frame, move)

#[derive(Default, Debug)]
pub struct WinningMoves{
    moves: Vec<(f64, u8)>
} // (frame, move)

