//delete this

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::geometry::bevy_ui::{CollidableEntity, MyCollider};

// TODO: Update only when "save" is pressed. 

// conflicitng queries - try putting in ParamSet
pub fn save_scene(
    assets: Query<(&mut Transform, &CollidableEntity)>,
    colliders: Query<&mut Transform, With<MyCollider>> //is pbr transform same as cuboid size?
) {
    for (transform, collidable_entity) in assets.iter(){
        println!("transform: {:?}", transform);
        println!("name: {}", collidable_entity.assetID);
    }

    for transform in colliders.iter(){
        println!("collider transform: {:?}", transform);
    }
}

pub fn recreate_scene() {
    // let path = Path::new("./assets/default_imgs/whale.jpg");
    // let mut file = File::create(path).unwrap();
    // let res = std::fs::copy("/Users/parnikasaxena/Downloads/whale.jpg", path);
}


