use bevy::prelude::*;

use crate::worlds::world_manager;
use crate::players::info;
use crate::ggrs_rollback::network;

#[derive(Debug)]
pub struct PlayerWorldInfo {
    pub transform: Transform, 
    pub plane: world_manager::IPlane, 
}

// Player World Loc = transform (can also determine plane level and index by rounding down)
pub fn get_world_loc(
    mut query: Query<
        (
            &mut Transform,
            &info::Player,
            &network::Me,
        )
    >,
) {
    let (transform, _, _) = query.single_mut();
    let pos = transform.translation;
    let x = round_down(pos.x);
    let y = pos.y as i32;
    let z = round_down(pos.y);
    let plane = world_manager::IPlane::new(x, y, z);
    let info = PlayerWorldInfo {
        transform: transform.clone(),
        plane
    };
    println!("player info: {:?}", info);
}

pub fn round_down(mut coord: f32) -> u32 {
    coord = coord.floor() + 0.5;
    while coord % 7.5 != 0.0 {
        coord = coord - 1.0;
    }
    if (coord + 7.5) % 15.0 != 0.0 {
        coord = coord - 7.5;
    }
    let res = (coord + 7.5) / 15.0;
    res as u32
}