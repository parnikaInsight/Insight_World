use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_rapier3d::prelude::*;
use egui::Response;
use emath::Pos2;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{collections::HashSet, sync::Arc};

pub fn ui_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_ctx: ResMut<EguiContext>,
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
                    //std::process::exit(0);
                    App::new().add_plugins(DefaultPlugins).run();
                }
                // if ui.button("Ability Creator").clicked() {
                //     std::process::exit(0);
                // }
                if ui.button("Metaverse").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}
