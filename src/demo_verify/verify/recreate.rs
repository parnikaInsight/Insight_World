use bevy::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

use crate::verify::{play, animation_helper, movement, player};

use super::movement::FrameTimeDiagnosticsState;

#[derive(Debug, Component)]
pub struct Me;

// Add stationary gltfs.
pub fn create_default_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    let player_handle = asset_server.load("default_characters/sword_jump.glb#Scene0");
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
        // Add player information.
        .insert(player::Player {
            state: player::PlayerState::default(),
            target: player::MovementTarget::default(),
            speed: player::MovementSpeed { speed: 3.0 },
        })
        .id();
    commands.entity(entity_id).insert(Me);

    // Plane
    let trans = Transform::from_xyz(0.0, 0.0, 0.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })), //PLANE_SIZE
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: trans,
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        //half the cube size
        .insert(Collider::cuboid(50.0, 0.0, 50.0))
        .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // green cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(-2.0, 0.5, -3.0),
            ..default()
        })
        // Physics
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(0.25, 0.25, 0.25))
                // Position the collider relative to the rigid-body.
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
        });
    // pink cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(-2.25, 2.5, -3.0),
            ..default()
        })
        // Physics
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(0.25, 0.25, 0.25))
                // Position the collider relative to the rigid-body.
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
        });
    // pink cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.3, 0.0, 0.3).into()),
            transform: Transform::from_xyz(-1.75, 3.5, -3.0),
            ..default()
        })
        // Physics
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(0.25, 0.25, 0.25))
                // Position the collider relative to the rigid-body.
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
        });

    // // Heaven sky orb
    let player_handle2: Handle<Scene> = asset_server.load("nature/heaven/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    // Insert startionary gltf animations.
    commands.insert_resource(play::Animations(vec![
        asset_server.load("default_characters/mutant_roaring.glb#Animation0")
    ]));

    // // Mutant
    let player_handle2: Handle<Scene> =
        asset_server.load("default_characters/mutant_roaring.glb#Scene0");
    commands
        .spawn_bundle(SceneBundle {
            transform: Transform::from_xyz(5.0, 0.0, 0.0)
                //.with_scale(Vec3::new(0.5, 0.5, 1.0))
                .with_rotation(Quat::from_rotation_y((270.0_f32).to_radians())),
            scene: player_handle2.clone(),
            ..default()
        })
        // Physics
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(1.0, 1.0, 1.0))
                // Position the collider relative to the rigid-body.
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
        });

    println!("spawned beginning state");
}

pub fn get_moves() -> Vec<(f64, u8)> {
    // Deserialize from a file, the format is also inferred from the file extension
    let file = File::open("./assets/worlds/demo_moves.txt").unwrap();
    let reader = BufReader::new(file);
    let res: Result<Vec<(f64, u8)>, serde_json::Error> = serde_json::from_reader(reader);

    match res {
        Ok(moves) => {
           // println!("FROM FILE moves: {:?}\n", moves);
            moves
        }
        Err(e) => {
            vec![]
        },
    }
}
