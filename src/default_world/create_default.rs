use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::animation::play;

// Add stationary gltfs.
pub fn create_default_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // plane
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })), //PLANE_SIZE
    //         material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
    //         ..Default::default()
    //     })
    //     .insert(RigidBody::Fixed)
    //     .insert(Collider::cuboid(7.5, 7.5, 7.5)) //half the cube size
    //     .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

    // // Light
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });

    // // play_scene needs this
    // // Insert startionary gltf animations.
    // commands.insert_resource(play::Animations(vec![
    //     asset_server.load("nature/phoenix_bird/scene.gltf#Animation0")
    // ]));

    // // Load gltf.
    // let player_handle1: Handle<Scene> = asset_server.load(
    //     "nature/phoenix_bird/scene.gltf#Scene0"
    // );

    // // Spawning SceneBundle automatically adds AnimationPlayer.
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 5.0, -10.0),
    //         scale: Vec3::new(0.01, 0.01, 0.01),
    //         ..default()
    //     },
    //     scene: player_handle1.clone(),
    //     ..default()
    // });

    // green cube
    commands.spawn_bundle(PbrBundle {
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
    commands.spawn_bundle(PbrBundle {
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
    commands.spawn_bundle(PbrBundle {
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
    let player_handle2: Handle<Scene> = asset_server.load("default_characters/mutant_roaring.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
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

    // // Dome
    // let player_handle3: Handle<Scene> = asset_server.load("nature/parasol/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         scale: Vec3::new(0.001, 0.001, 0.001),
    //         ..default()
    //     },
    //     scene: player_handle3.clone(),
    //     ..default()
    // });
}
