use bevy::{asset::AssetServerSettings, prelude::*, window::PresentMode, winit::WinitSettings, render::primitives::Aabb};
use bevy_egui::{egui, EguiContext, EguiPlugin};
//use bevy::render::primitives::Aabb;

mod geometry;
use geometry::{my_plane, bevy_ui};
mod camera;
use camera::pan_orbit;
mod save;
mod db;
use db::assets;

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
            width: 1600.0,
            height: 1000.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        // AssetServerSettings must be inserted before adding the AssetPlugin or DefaultPlugins.
        // Tell the asset server to watch for asset changes on disk:
        .insert_resource (AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(bevy_ui::Images {
            img1: "default_imgs/emu.png".to_owned(), 
            img2: "default_imgs/tiger.png".to_owned(), 
            img3: "default_imgs/soccer_ball.png".to_owned()
})
        .init_resource::<bevy_ui::UiState>()
        //.init_resource::<bevy_ui::Images>()
        .init_resource::<bevy_ui::Tags>()
        .init_resource::<assets::PlaneAssets>()

    //Plugins
        .add_plugins(DefaultPlugins) //disable log and winit plugin when put into subapp 
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.

    //Startup Systems
       // .add_system(mouse_events::print_mouse_events_system)
        .add_startup_system(my_plane::setup_plane)
        .add_startup_system(pan_orbit::spawn_camera)
        .add_startup_system(assets::default_assets)
        .add_startup_system(bevy_ui::configure_visuals)
        .add_startup_system(bevy_ui::configure_ui_state)

    //Systems
        .add_system(pan_orbit::pan_orbit_camera)
        //.add_system(lets_get_ass)
        //.add_system(save::save::save_scene)
        .add_system(bevy_ui::ui_example)
        .run();
}


fn lets_get_ass(
    mut ass: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    //as_mesh: ResMut<Assets<Mesh>>,
) {
    match ass.get_mut(&asset_server.load("default_gltfs/shiba_inu.glb#Scene0")) {
        Some(res) => {
            //println!("{:?}", res.world.component_id::<Aabb>());
            //println!("{:?}", res.world.component_id::<Handle<Mesh>>());
            let mut query_one = res.world.query::<(&Aabb)>();
            //let mut query_two = res.world.query::<(&Handle<Mesh>)>();
            for c in query_one.iter(&res.world) {
                println!("{:?}", c);
            }
        }
        None => println!("hello"),
    }
}