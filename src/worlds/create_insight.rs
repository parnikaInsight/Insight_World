use bevy::prelude::*;
use bevy_ggrs::Rollback;

use crate::worlds::{world_manager, player};
use crate::players::info;
use crate::ggrs_rollback::network;

pub fn create_insight_world(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Insight
    let mut insight = world_manager::InsightWorld::new();

    //Default World
    let mut world = world_manager::IWorld::new();
    // Center Plane
    let plane_a =  world_manager::IPlane {
        // pub gltfs: Vec<IGltf>,
        x: 0,
        y: 0,
        z: 0,
    };
    // Plane on the right of center plane.
    let plane_b =  world_manager::IPlane {
        // pub gltfs: Vec<IGltf>,
        x: 1,
        y: 0,
        z: 0,
    };
    // Plane on top right corner of plane_b.
    let plane_c = world_manager::IPlane {
        // pub gltfs: Vec<IGltf>,
        x: 2,
        y: 0,
        z: 1,
    };
    world.add_plane(vec![&plane_a, &plane_b, &plane_c], commands, meshes, materials);
}
