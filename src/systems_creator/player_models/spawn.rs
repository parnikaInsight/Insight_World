use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashSet;
use std::env;

use package::src::animation::animation_helper;
use crate::players::{info, movement};
use super::worlds::world_manager;

pub fn setup_my_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // read cmd line argument to determine player
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    // Add player scene.
    let mut player_handle = asset_server.load("mixamo/shoot.glb#Scene0");

    let entity_id = commands
        // Create player.
        .spawn_bundle(SceneBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -5.0),
                ..default()
            },
            scene: player_handle.clone(),
            ..default()
        })

        // Add player information.
        .insert(info::Player {
            handle: 0,
            money: 50,
            bounties: 3,
            friends: HashSet::new(),
            health: 100,
            world: 0,
            plane: world_manager::IPlane::new(0, 0, 0),
            state: info::PlayerState::default(),
            target: info::MovementTarget::default(),
            speed: info::MovementSpeed { speed: 3.0 },
            ability_id: 0,
            abilities: Vec::new(),
        })
        .insert(info::Information::default())
        .insert_bundle(PickableBundle::default()) // Player can be clicked.
        
        // Physics
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(0.5, 1.0, 0.5))
                // Position the collider relative to the rigid-body.
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
        })
        .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)))
        
        // Animation Helper
        .insert(animation_helper::AnimationHelperSetup)
        .id();
}
