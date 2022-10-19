use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_rapier3d::prelude::*;
use egui::Response;
use emath::Pos2;
use std::{collections::HashSet, sync::Arc};

use crate::db::assets;
use crate::HEIGHT;

#[derive(Default)]
pub struct Images {
    pub img1: String,
    pub img2: String,
    pub img3: String,
}

#[derive(Default)]
pub struct Tags {
    pub tags: HashSet<String>,
}

#[derive(Default)]
pub struct UiState {
    label: String,
    value: f32,
    is_window_open: bool,
}

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

pub fn ui_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut tags: ResMut<Tags>,
    plane_assets: ResMut<assets::PlaneAssets>,
    // You are not required to store Egui texture ids in systems. We store this one here just to
    // demonstrate that rendering by using a texture id of a removed image is handled without
    // making bevy_egui panic.
    mut rendered_texture_id: Local<egui::TextureId>,
    mut rendered_texture_id2: Local<egui::TextureId>,
    mut rendered_texture_id3: Local<egui::TextureId>,
    mut rendered_texture_id4: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    mut is_initialized2: Local<bool>,
    mut is_initialized3: Local<bool>,
    // If you need to access the ids from multiple systems, you can also initialize the `Images`
    // resource while building the app and use `Res<Images>` instead.
    mut images: ResMut<Images>,
    asset_server: Res<AssetServer>,
) {
    // World Builder
    *rendered_texture_id = egui_ctx.add_image(asset_server.load(&images.img1[..]));
    *rendered_texture_id2 = egui_ctx.add_image(asset_server.load(&images.img2[..]));
    *rendered_texture_id3 = egui_ctx.add_image(asset_server.load(&images.img3[..]));
    *rendered_texture_id4 = egui_ctx.add_image(asset_server.load("default_imgs/upload.png"));

    let mut response_bool = false;

    egui::Window::new("World Creator")
        .default_width(200.0)
        .default_height(HEIGHT)
        .default_pos(Pos2 { x: 0.0, y: 25.0 })
        .vscroll(true)
        .show(&egui_ctx.ctx_mut().clone(), |ui| {
            // Searchbar
            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.horizontal(|ui| {
                ui.label("Searchbar: ");
                let response = ui.text_edit_singleline(&mut ui_state.label);
                if response.changed() {
                    response_bool = true;
                }
            });

            // Get search tags
            if response_bool {
                let search = ui_state.label.clone();
                let v: Vec<&str> = search.split(' ').collect();
                println!("PARNIKA {} done {:?}", search, v);
                let mut new_tags: HashSet<String> = HashSet::new();
                for i in v.iter() {
                    new_tags.insert(i.to_string());
                }
                tags.tags = new_tags;
                println!("Saxena {} done {:?}", search, tags.tags);

                // Update images with searches
                let searched_assets = assets::get_assets(plane_assets, tags);
                if !searched_assets.is_empty() {
                    let mut count = 0;
                    for a in searched_assets {
                        let s = format!("{}{}{}", "default_imgs/".to_owned(), a, ".png");
                        println!("String {} {}", a, s);
                        if count == 0 {
                            images.img1 = s;
                        } else if count == 1 {
                            images.img2 = s;
                        } else if count == 2 {
                            images.img3 = s;
                        } else {
                            break;
                        }
                        count += 1;
                    }
                }
            }

            // First Image
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            let response1 = ui.add(egui::widgets::Image::new(
                *rendered_texture_id,
                [256.0, 256.0],
            ));

            // Second Image
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            let response2 = ui.add(egui::widgets::Image::new(
                *rendered_texture_id2,
                [256.0, 256.0],
            ));

            // Third Image
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            let response3 = ui.add(egui::widgets::Image::new(
                *rendered_texture_id3,
                [256.0, 256.0],
            ));

            if response1.clicked() {
                println!("clicked 1 on");
            }
            if response2.clicked() {
                println!("clicked 2 on");
            }
            if response3.clicked() {
                println!("clicked 3 on");
            }

            // Spawn asset shown in image
            if response1.hovered() && !*is_initialized {
                *is_initialized = true;
                println!("1 hovered");
                if let Some(index) = images.img1.find(".") {
                    let name = images.img1[13..index].to_owned();
                    let path = format!("{}{}{}", "default_gltfs/", name, ".glb#Scene0");
                    let player_handle: Handle<Scene> = asset_server.load(&path[..]);

                    commands
                        .spawn_bundle(PbrBundle {
                            // visibility: Visibility {
                            //     is_visible: false,
                            // },
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                                alpha_mode: AlphaMode::Mask(0.5),
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        })
                        .insert_bundle(bevy_mod_picking::PickableBundle::default())
                        .insert(bevy_transform_gizmo::GizmoTransformable)
                        .with_children(|children| {
                            children.spawn_bundle(SceneBundle {
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 0.0), //moves relative to cube pos
                                    scale: Vec3::new(0.5, 0.5, 0.5),
                                    ..default()
                                },
                                scene: player_handle.clone(),
                                ..default()
                            });
                        })
                        // Physics
                        .with_children(|children| {
                            children
                                .spawn_bundle(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                                    material: materials.add(StandardMaterial {
                                        base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                                        alpha_mode: AlphaMode::Mask(0.5),
                                        ..default()
                                    }),
                                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                    ..Default::default()
                                })
                                .insert_bundle(bevy_mod_picking::PickableBundle::default())
                                .insert(bevy_transform_gizmo::GizmoTransformable)
                                .insert(RigidBody::Fixed)
                                .insert(Collider::cuboid(0.25, 0.25, 0.25))
                                .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));
                        });
                }
            }
            if response2.hovered() && !*is_initialized2 {
                *is_initialized2 = true;
                println!("2 hovered");
                if let Some(index) = images.img2.find(".") {
                    let name = images.img2[13..index].to_owned();
                    let path = format!("{}{}{}", "default_gltfs/", name, ".glb#Scene0");
                    let player_handle: Handle<Scene> = asset_server.load(&path[..]);
                    commands
                        .spawn_bundle(PbrBundle {
                            // visibility: Visibility {
                            //     is_visible: false,
                            // },
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                                alpha_mode: AlphaMode::Mask(0.5),
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        })
                        .insert_bundle(bevy_mod_picking::PickableBundle::default())
                        .insert(bevy_transform_gizmo::GizmoTransformable)
                        .with_children(|children| {
                            children.spawn_bundle(SceneBundle {
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 0.0),
                                    scale: Vec3::new(0.5, 0.5, 0.5),
                                    ..default()
                                },
                                scene: player_handle.clone(),
                                ..default()
                            });
                        });
                }
            }
            if response3.hovered() && !*is_initialized3 {
                *is_initialized3 = true;
                println!("3 hovered");
                if let Some(index) = images.img3.find(".") {
                    let name = images.img3[13..index].to_owned();
                    let path = format!("{}{}{}", "default_gltfs/", name, ".glb#Scene0");
                    let player_handle: Handle<Scene> = asset_server.load(&path[..]);
                    commands
                        .spawn_bundle(PbrBundle {
                            // visibility: Visibility {
                            //     is_visible: false,
                            // },
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                                alpha_mode: AlphaMode::Mask(0.5),
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        })
                        .insert_bundle(bevy_mod_picking::PickableBundle::default())
                        .insert(bevy_transform_gizmo::GizmoTransformable)
                        .with_children(|children| {
                            children.spawn_bundle(SceneBundle {
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 0.0),
                                    scale: Vec3::new(0.5, 0.5, 0.5),
                                    ..default()
                                },
                                scene: player_handle.clone(),
                                ..default()
                            });
                        });
                }
            }

            // More Assets Button
            ui.horizontal(|ui| {
                ui.button("More Assets").clicked();
                //    if ui.button("More Assets").clicked() {
                //     println!("more assets")
                //     }
                //     ui.button("Upload Asset").clicked();
            });
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));

            ui.horizontal(|ui| {
                commands
                    .spawn()
                    // add a component
                    .insert(GltfDropTarget)
                    .with_children(|children| {
                        ui.add(egui::widgets::Image::new(
                            *rendered_texture_id4,
                            [128.0, 80.0],
                        ));
                    });
                commands
                    .spawn()
                    // add a component
                    .insert(ImgDropTarget)
                    .with_children(|children| {
                        ui.add(egui::widgets::Image::new(
                            *rendered_texture_id4,
                            [128.0, 80.0],
                        ));
                    });
            });

            // // // Upload Assets
            // commands
            //     .spawn()
            //     // add a component
            //     .insert(GltfDropTarget)
            //     .with_children(|children| {
            //         ui.horizontal(|ui| {
            //             ui.add(egui::widgets::Image::new(
            //                 *rendered_texture_id4,
            //                 [128.0, 80.0],
            //             ));
            //         });
            //     });

            // commands
            //     .spawn()
            //     // add a component
            //     .insert(ImgDropTarget)
            //     .with_children(|children| {
            //         ui.horizontal(|ui| {
            //             ui.add(egui::widgets::Image::new(
            //                 *rendered_texture_id4,
            //                 [128.0, 80.0],
            //             ));
            //         });
            //     });

            // ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            // ui.horizontal(|ui| {
            //     let upload = ui.add(egui::widgets::Image::new(
            //         *rendered_texture_id4,
            //         [128.0, 80.0],
            //     ));
            //     let upload2 = ui.add(egui::widgets::Image::new(
            //         *rendered_texture_id4,
            //         [128.0, 80.0],
            //     ));
            // });
            // if upload.hovered() {
            //     println!("upload");
            // }

            ui.button("Upload Asset").clicked();
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        });

    // Abilities Builder
    egui::SidePanel::right("right_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.heading("Abilities");

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.horizontal(|ui| {
                ui.label("Searchbar: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.add(egui::widgets::Image::new(
                *rendered_texture_id,
                [256.0, 256.0],
            ));

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

            //If you want your panel to be resizable you also need a widget in it that takes up more space as you resize it, such as:
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Insight",
                    "https://github.com/paxsethorld/Insight_World",
                ));
            });
        });

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}

#[derive(Component)]
struct GltfDropTarget;

#[derive(Component)]
struct ImgDropTarget;

pub fn file_drop(
    mut dnd_evr: EventReader<FileDragAndDrop>,
    //query_ui_droptarget: Query<&Interaction, With<MyDropTarget>>,
) {
    for ev in dnd_evr.iter() {
        println!("{:?}", ev);
        if let FileDragAndDrop::DroppedFile { id, path_buf } = ev {
            println!("Dropped file with path: {:?}", path_buf);

            // if id.is_primary() {
            //     // it was dropped over the main window
            // }

            // for interaction in query_ui_droptarget.iter() {
            //     if *interaction == Interaction::Hovered {
            //         // it was dropped over our UI element
            //         // (our UI element is being hovered over)
            //     }
            // }
        }
    }
}
