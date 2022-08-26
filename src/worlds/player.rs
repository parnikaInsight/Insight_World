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
// No error yet if player walking on nonexistent plane.
pub fn get_world_loc(
    mut query: Query<
        (
            &mut Transform,
            &mut info::Player,
            &network::Me,
        )
    >,
) {
    let (transform, mut player, _) = query.single_mut();
    let pos = transform.translation;
    let x = round_down(pos.x);
    let y = pos.y as i32;
    let z = round_down(pos.z);
    let plane = world_manager::IPlane::new(x, y, z);
    let info = PlayerWorldInfo {
        transform: transform.clone(),
        plane
    };
    player.plane = plane;
    println!("player info: {:?}", info.plane);
}

pub fn round_down(mut coord: f32) -> i32 {
    coord = coord.floor() + 0.5;
    while coord % 7.5 != 0.0 {
        coord = coord - 1.0;
    }
    if (coord + 7.5) % 15.0 != 0.0 {
        coord = coord - 7.5;
    }
    let res = (coord + 7.5) / 15.0;
    res as i32
}