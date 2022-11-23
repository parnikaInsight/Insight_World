use bevy::prelude::*;
use bevy_ggrs::Rollback;

use crate::colliders::collider;
use crate::components::comps;
use crate::ggrs_rollback::network;
use crate::players::info;
use crate::worlds::{player, world_manager};

pub fn create_insight_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut startups: Query<Entity, (With<comps::startup>, With<comps::Meta_Comp>)>,
) {
    if startups.iter().len() == 0 {
        println!("created gray planes");
        //Insight
        let mut insight = world_manager::InsightWorld::new();

        //Default World
        let mut world = world_manager::IWorld::new();
        // Center Plane
        let plane_a = world_manager::IPlane {
            // pub gltfs: Vec<IGltf>,
            x: 0,
            y: 0,
            z: 0,
        };
        // Plane on the right of center plane.
        let plane_b = world_manager::IPlane {
            // pub gltfs: Vec<IGltf>,
            x: 1,
            y: 0,
            z: 0,
        };
        // Plane on top right corner of plane_b.
        let plane_c = world_manager::IPlane {
            // pub gltfs: Vec<IGltf>,
            x: -2,
            y: 0,
            z: -1,
        };

        // Collider testing
        // let handle = asset_server.load("default_gltfs/maple_tree.glb#Scene0");
        //     commands
        //         .spawn_bundle(SceneBundle {
        //             transform: Transform {
        //                 translation: Vec3::new(0.0, 0.0, 0.0),
        //                 scale: Vec3::new(0.1, 0.1, 0.1),
        //                 ..default()
        //             },
        //             scene: handle.clone(),
        //             ..default()
        //         })
        //         .insert(collider::AddCollider::new(false, handle));

        world.add_plane(
            vec![&plane_a, &plane_b, &plane_c],
            commands,
            meshes,
            materials,
        );
    }
}
