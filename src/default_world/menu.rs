use bevy::app::AppLabel;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_dolly::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use egui::Response;
use emath::Pos2;
use futures::select;
use libp2p::kad::Kademlia;
use libp2p::kad::store::MemoryStore;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::path::Path;
use std::sync::Once;
use std::{collections::HashSet, sync::Arc};
static START: Once = Once::new();
use futures::executor::block_on;
use std::thread;
use bevy::app::AppExit;
use bevy::ecs::schedule::ShouldRun;

use crate::GameSender;
use crate::plane_creator::geometry::bevy_ui::CollidableEntity;
use crate::plane_creator::save::save_world;

use super::create_default;

#[derive(Default, Clone)]
pub struct PlaneCreatorState {
    pub bool: bool,
}

pub fn configure_plane_creator_state(mut p_state: ResMut<PlaneCreatorState>) {
    p_state.bool = false;
}

pub fn get_plane_creator_state(mut p_state: ResMut<PlaneCreatorState>) -> ShouldRun {
    if p_state.bool {
        ShouldRun::Yes
    }
    else {
        ShouldRun::No
    }
}

#[derive(Default, Clone)]
pub struct MetaverseState {
    pub bool: bool,
}

pub fn configure_metaverse_state(mut m_state: ResMut<MetaverseState>) {
    m_state.bool = true;
}

pub fn get_metaverse_state(mut m_state: ResMut<MetaverseState>) -> ShouldRun {
    if m_state.bool {
        ShouldRun::Yes
    }
    else {
        ShouldRun::No
    }
}

pub fn ui_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut p_state: ResMut<PlaneCreatorState>,
    mut m_state: ResMut<MetaverseState>,
    assets: Query<(&mut Transform, &CollidableEntity)>,
    game_send: ResMut<GameSender>
    //kademlia: &mut Kademlia<MemoryStore>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Wallet").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            egui::menu::menu_button(ui, "Mode", |ui| {
                if ui.button("World Builder").clicked() {
                    p_state.bool = true;
                    m_state.bool = false;
                    println!("starte: {}", p_state.bool);
                    //std::process::exit(0);
                    
                    // exit.send(AppExit);
                    // App::new().add_plugins(DefaultPlugins).run();
                    
                    // thread::spawn(|| {
                    //     for i in 1..10 {
                    //         println!("hi number {} from the spawned thread!", i);
                    //         //thread::sleep(Duration::from_millis(1));
                    //     }
                    //     App::new()
                    //         // .add_plugins_with(DefaultPlugins, |group| {
                    //         //     group
                    //         //         .disable::<bevy::log::LogPlugin>()
                    //         //         // .disable::<bevy::window::WindowPlugin>()
                    //         //         // .disable::<bevy::winit::WinitPlugin>()
                    //         //         // .disable::<bevy::core::CorePlugin>()
                    //         // })
                    //         // .add_system(hello_world_system)
                    //         .run();
                    // });
                    
                    // thread::spawn(move || {
                    //     block_on(plane_creator).expect("Thread Spawn Error")
                    // });
                }
                // if ui.button("Ability Creator").clicked() {
                //     std::process::exit(0);
                // }
                if ui.button("Metaverse").clicked() {
                    //std::process::exit(0);
                    p_state.bool = false;
                    m_state.bool = true;
                    println!("starte: {}", p_state.bool);
                }
            });
            egui::menu::menu_button(ui, "Creator", |ui| {
                if ui.button("Save").clicked() {
                    save_world::save_scene(assets);
                    println!("saved");
                    //std::process::exit(0);
                }
                if ui.button("Publish").clicked() {
                    //save_world::publish_scene(kademlia);
                    game_send.game_sender.send(String::from("PUBLISH"));
                    println!("you are now a provider for your creation");
                    //std::process::exit(0);
                }
            });
        });
    });
}

fn plane_creator() -> Result<(), Box<dyn Error>> {
    println!("hello world");
    loop {
        App::new().run();
    }
}

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
// pub struct SubAppLabel;

// pub fn switch_app(mut app: &mut App) {
//     let subapp = App::new();
//     app.add_sub_app(SubAppLabel, subapp, move |app_world, subapp| {
//         mem::swap(app_world, &mut subapp.world);
//         START.call_once(|| {
//             subapp.add_plugin(MyApp);
//         });
//         subapp.update();
//         mem::swap(app_world, &mut subapp.world);
//         //app_world = &mut subapp.world;
//     });
// }
// pub struct MyApp;
// impl Plugin for MyApp {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(bevy_mod_picking::DefaultPickingPlugins)
//             .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
//             .add_startup_system(create_default::create_default_plane) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
//             .add_startup_system(setup_camera)
//             .add_system(update_camera);
//     }
// }

// #[derive(Component)]
// pub struct MainCamera; // Dolly fly camera

// pub fn setup_camera(
//     mut commands: Commands,
//     mut windows: ResMut<Windows>,
// ) {
//     // Camera Setup

//     let translation = [-2.0f32, 2.0f32, 5.0f32];
//     let transform = Transform::from_translation(bevy::math::Vec3::from_slice(&translation))
//         .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);
//     let rotation = transform.rotation;
//     let mut yaw_pitch = YawPitch::new();
//     yaw_pitch.set_rotation_quat(rotation);

//     // Insert camera rig to control camera movement.
//     // Camera added separately.
//     commands.spawn().insert(
//         CameraRig::builder()
//             .with(Position {
//                 translation: transform.translation,
//             })
//             .with(Rotation { rotation })
//             .with(yaw_pitch)
//             .with(Smooth::new_position_rotation(1.0, 1.0))
//             .build(),
//     );

//     // Create camera.
//     commands
//         .spawn_bundle(Camera3dBundle {
//             transform,
//             ..Default::default()
//         })
//         .insert(UiCameraConfig { //Currently not displaying
//             show_ui: true,
//             ..default()
//         })

//         .insert_bundle(PickingCameraBundle::default())
//         .insert(MainCamera)
//         .insert(bevy_transform_gizmo::GizmoPickSource::default());

//     // Directional 'sun' light.
//     commands.spawn_bundle(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             illuminance: 32000.0,
//             ..default()
//         },
//         transform: Transform {
//             translation: Vec3::new(0.0, 2.0, 0.0),
//             rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
//             ..default()
//         },
//         ..default()
//     });

//     // let mut window = windows.get_primary_mut().unwrap();
//     // if window.cursor_locked() {
//     //     println!("changed to unlocked");
//     //     toggle_grab_cursor(window);
//     // }
// }
// pub fn update_camera(
//     time: Res<Time>,
//     keys: Res<Input<KeyCode>>,
//     mut windows: ResMut<Windows>,
//     mut mouse_motion_events: EventReader<MouseMotion>,
//     mut query: ParamSet<(
//         Query<(&mut Transform, With<MainCamera>)>,
//         Query<&mut CameraRig>,
//     )>,
// ) {
//     let time_delta_seconds: f32 = time.delta_seconds();
//     let boost_mult = 5.0f32;
//     let sensitivity = Vec2::splat(1.0);

//     let mut move_vec = Vec3::ZERO;

//     // Camera Movement
//     if keys.pressed(KeyCode::Up) {
//         move_vec.z -= 1.0;
//     }
//     if keys.pressed(KeyCode::Down) {
//         move_vec.z += 1.0;
//     }
//     if keys.pressed(KeyCode::Left) {
//         move_vec.x -= 1.0;
//     }
//     if keys.pressed(KeyCode::Right) {
//         move_vec.x += 1.0;
//     }

//     if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
//         move_vec.y += 1.0;
//     }
//     if keys.pressed(KeyCode::Q) {
//         move_vec.y -= 1.0;
//     }
//     let window = windows.get_primary_mut().unwrap();
//     if keys.just_pressed(KeyCode::RShift){
//         //println!("Rshift pressed");
//         toggle_grab_cursor(window);
//     }

//     // Camera Thrust
//     let boost: f32 = if keys.pressed(KeyCode::LShift) {
//         1.
//     } else {
//         0.
//     };

//     // Locked Camera Rotation
//     let mut delta = Vec2::ZERO;
//     for event in mouse_motion_events.iter() {
//         delta += event.delta;
//     }

//     let mut q1 = query.p1();
//     let mut rig = q1.single_mut();

//     let move_vec =
//         rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost_mult.powf(boost);

//     // If locked, rotate camera. Else, move camera.
//     let window = windows.get_primary().unwrap();
//     if window.cursor_locked() {
//         //println!("Cursor locked");
//         rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
//             -0.1 * delta.x * sensitivity.x,
//             -0.1 * delta.y * sensitivity.y,
//         );
//         rig.driver_mut::<Position>()
//             .translate(move_vec * time_delta_seconds * 10.0);
//     }

//     // Update rig and camera postion.
//     let transform = rig.update(time_delta_seconds);
//     let mut q0 = query.p0();
//     let (mut cam, _) = q0.single_mut();

//     cam.update(transform);
// }

// /// Grabs/ungrabs mouse cursor
// fn toggle_grab_cursor(window: &mut Window) {
//     window.set_cursor_lock_mode(!window.cursor_locked());
//     //println!("Toggling cursor: {}", window.cursor_locked());
//     window.set_cursor_visibility(!window.cursor_visible());
// }
