use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use egui::Response;
use std::{collections::HashSet, sync::Arc};

use crate::db::assets;

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

    let mut response_bool = false;
    egui::SidePanel::left("left_panel")
        .default_width(200.0)
        .show(&egui_ctx.ctx_mut().clone(), |ui| {
            // Title
            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.heading("World Creator");

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

            // Spawn asset shown in image
            if response1.hovered() && !*is_initialized {
                *is_initialized = true;
                println!("1 DOUBLE CLICKED");
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
            if response2.hovered() && !*is_initialized2 {
                *is_initialized2 = true;
                println!("2 DOUBLE CLICKED");
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
                println!("3 DOUBLE CLICKED");
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

            // Next Button
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            ui.horizontal(|ui| {
                ui.button("Next").clicked();
            });
            ui.allocate_space(egui::Vec2::new(1.0, 20.0));

            //If you want your panel to be resizable you also need a widget in it that takes up more space as you resize it, such as:
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Insight",
                    "https://github.com/paxsethorld/Insight_World",
                ));
            });
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

    egui::Window::new("Window")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });
}
