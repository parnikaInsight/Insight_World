use bevy::prelude::*;
use bevy::utils::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use serde_json::Deserializer;
use serde::{Deserialize, Serialize};

use crate::plane_creator::geometry::bevy_ui::{CollidableEntity, MyCollider};

// TODO: Update only when "save" is pressed. 

#[derive(Serialize, Deserialize, Default, Debug)]
struct WorldDataStructures {
    world_assets: HashMap<String, String>
}

// conflicitng queries - try putting in ParamSet
pub fn save_scene(
    assets: Query<(&mut Transform, &CollidableEntity)>,
    // colliders: Query<&mut Transform, With<MyCollider>> //is pbr transform same as cuboid size?
) {
    let mut my_assets = WorldDataStructures::default();
    for (transform, collidable_entity) in assets.iter(){
        my_assets.world_assets.insert(collidable_entity.assetID.clone(), format!("{:?}", transform));
       // println!("name: {}, transform: {:?}", collidable_entity.assetID, transform);
    }
    let world_name = "experiment_world"; // Hash of World/ Name given by creator
    let pathname = format!("{}{}{}", "./assets/worlds/", world_name, ".txt");
    let path = Path::new(&pathname);
    let mut file = fs::File::create(path).unwrap();
    let j = serde_json::to_string(&my_assets).unwrap();
    fs::write(pathname, j);
}

pub fn recreate_scene(mut commands: Commands) {
    // Deserialize from a file, the format is also inferred from the file extension
    let file = File::open("./assets/worlds/experiment_world.txt").unwrap();
    let reader = BufReader::new(file);
    let my_world_res: Result<WorldDataStructures, serde_json::Error> = serde_json::from_reader(reader);   
    //println!("JSON World: {:?}", my_world_res);
    
    match my_world_res {
        Ok(world) => (), //println!("good"), // spawn these assets in game world
        Err(e) => (), // println!("bad from plane_creator::save::save_world::recreate_scene:"),
    }
}


