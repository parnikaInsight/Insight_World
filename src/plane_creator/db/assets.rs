use bevy::prelude::shape::Plane;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::geometry::bevy_ui;

#[derive(Default, Clone)]
pub struct PlaneAssets {
    // Asset filename : Tags
    pub assets: HashMap<String, HashSet<String>>,
}

pub fn default_assets(mut commands: Commands) {
    let mut assets = HashMap::new();

    let mut tags = HashSet::new();
    tags.insert(String::from("chocolate"));
    tags.insert(String::from("cupcake"));
    tags.insert(String::from("desert"));
    assets.insert(String::from("chocolate_cupcake"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("emu"));
    tags.insert(String::from("animal"));
    assets.insert(String::from("emu"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("tree"));
    tags.insert(String::from("nature"));
    assets.insert(String::from("maple_tree"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("pool"));
    tags.insert(String::from("ball"));
    assets.insert(String::from("pool_ball"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("crypto"));
    tags.insert(String::from("japanese"));
    tags.insert(String::from("dog"));
    assets.insert(String::from("shiba_inu"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("ball"));
    tags.insert(String::from("soccer"));
    tags.insert(String::from("sport"));
    assets.insert(String::from("soccer_ball"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("tiger"));
    tags.insert(String::from("animal"));
    assets.insert(String::from("tiger"), tags);

    let mut tags = HashSet::new();
    tags.insert(String::from("victorian"));
    tags.insert(String::from("street"));
    tags.insert(String::from("lamp"));
    assets.insert(String::from("victorian_street_lamp"), tags);

    commands.insert_resource(PlaneAssets { assets });
}

pub fn get_assets(
   // plane_assets: ResMut<PlaneAssets>,
   // search_tags: ResMut<bevy_ui::Tags>,
    plane_assets: PlaneAssets,
    search_tags: bevy_ui::Tags,

) -> HashSet<String> {
    let mut tagged_assets: HashSet<String> = HashSet::new();
    for (filename, asset_tags) in plane_assets.assets.iter() {
        for t in search_tags.tags.iter() {
            if asset_tags.contains(t) {
                tagged_assets.insert(filename.clone());
            }
        }
    }
    println!("Tagged assets: {:?}", tagged_assets);
    tagged_assets
}

pub fn update_assets(
    mut plane_assets: &mut PlaneAssets,
    filename: String,
    tags: HashSet<String>,
) {
    plane_assets.assets.insert(filename, tags);
}

pub fn delete_asset(
    mut plane_assets: &mut PlaneAssets,
    filename: String,
) {
    if plane_assets.assets.contains_key(&filename){
        plane_assets.assets.remove(&filename);
    }
}

