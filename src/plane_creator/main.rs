use bevy::{prelude::*, window::PresentMode};

mod geometry;
use geometry::my_plane;

mod camera;
use camera::pan_orbit;

fn main() {
    let mut app = bevy::app::App::new(); //new vs empty //bevy::App has more trait implementations than bevy_app
   // app.add_event::<mouse_events::MyCursorMoved>() //never used
    //Events

    //Resources
    app
        //Window: event loops, changing contexts
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.4)))
        .insert_resource(Msaa { samples: 4 }) //remove jaggedness
        .insert_resource(WindowDescriptor { //must come before DefaultPlugins
            title: "InsightWorld Plane Creator".to_string(),
            width: 1000.0,
            height: 800.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })

    //Plugins
        .add_plugins(DefaultPlugins) //disable log and winit plugin when put into subapp 
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.

    //Startup Systems
       // .add_system(mouse_events::print_mouse_events_system)
        .add_startup_system(my_plane::setup_plane)
        .add_startup_system(pan_orbit::spawn_camera)

    //Systems
        .add_system(pan_orbit::pan_orbit_camera)
        .add_system(my_plane::add_block)
        .run();
}
