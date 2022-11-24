use bevy::app::AppLabel;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_dolly::prelude::*;
use bevy_egui::egui::menu;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use crossbeam_channel::Receiver;
use egui::Response;
use emath::Pos2;
use futures::select;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Kademlia;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{mem, time};
use std::path::Path;
use std::sync::Once;
use std::{collections::HashSet, sync::Arc};
static START: Once = Once::new();
use bevy::app::AppExit;
use bevy::ecs::schedule::ShouldRun;
use futures::executor::block_on;
use std::thread;

use crate::plane_creator::geometry::bevy_ui::CollidableEntity;
use crate::plane_creator::save::save_world;
use crate::{GameReceiver, GameSender};

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
    } else {
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
    } else {
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
    game_send: ResMut<GameSender>, //kademlia: &mut Kademlia<MemoryStore>,
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
                    //Send notification from my game to my network about the new update (go to kademlia.rs)
                    game_send.game_sender.send(String::from("PUBLISH"));
                    println!("you are becoming a provider for your creation");
                    //std::process::exit(0);
                }
            });
        });
    });
}

// pub fn push_notif(
//     game_receiver: ResMut<GameReceiver>,
//     mut update: ResMut<MetaverseUpdate>,
// ) {
//     let res = game_receiver.game_receiver.try_recv();
//     match res {
//         // in kademlia handle_input_line
//         Ok(string) => {
//             if string == "NOW_PROV" {
//                 update.update = 1; // creation ID hardcoded for no
//                 println!("push notification");
//             }
//         }
//         Err(_) => (),
//     }
// }

// // You can also register resources. If your Component / Resource implements Hash, you can make use of `#[reflect(Hash)]`
// // in order to allow a GGRS `SyncTestSession` to construct a checksum for a world snapshot
// #[derive(Default, Reflect, Hash, Component)]
// #[reflect(Hash)]
// pub struct MetaverseUpdate {
//     pub update: u32, // creation ID
// }

// pub fn check_res_changed(
//     my_res: Res<MetaverseUpdate>,
//     mut commands: Commands,
//     asset_server: ResMut<AssetServer>,
// ) {
//     if my_res.is_changed() {
//         // do something
//         println!("res changed {}", my_res.update);
//         let entity = commands
//             .spawn_bundle(NodeBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(20.0), Val::Percent(40.0)),
//                     justify_content: JustifyContent::SpaceBetween,
//                     ..default()
//                 },
//                 ..default()
//             })
//             .with_children(|parent| {
//                 // text
//                 parent.spawn_bundle(
//                     TextBundle::from_section(
//                         format!("Text Example: {}", my_res.update.to_string()),
//                         TextStyle {
//                             //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             font_size: 30.0,
//                             color: Color::WHITE,
//                             ..default()
//                         },
//                     )
//                     .with_style(Style {
//                         margin: UiRect::all(Val::Px(5.0)),
//                         ..default()
//                     }),
//                 );
//             }).id();
//         println!("metaverse update notif");
//         let ten_millis = time::Duration::from_secs(10);
//         let now = time::Instant::now();
//         thread::sleep(ten_millis);
//         commands.entity(entity).despawn_recursive();
//         println!("update despawend");
//     }
// }
