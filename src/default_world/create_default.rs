use bevy::prelude::*;
use bevy_dolly::prelude::CameraRig;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_rapier3d::prelude::*;

use crate::animation::play;
use crate::components::comps::{self, Meta_Comp, PC_Comp};
use crate::default_world::menu;
use crate::players::info::Player;

pub fn despawn_meta(mut commands: Commands, query: Query<Entity, With<Meta_Comp>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
        println!("despawnd");
    }
}

pub fn despawn_pc(mut commands: Commands, query: Query<Entity, With<PC_Comp>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
        // println!("despawnd");
    }
}

// Add stationary gltfs.
pub fn create_default_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut startups: Query<Entity, (With<comps::startup>, With<comps::PC_Comp>)>,
) {
    if startups.iter().len() == 0 {
        println!("sky");

        // gray cube
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), //PLANE_SIZE
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(comps::PC_Comp)
            .insert(comps::startup)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5, 0.5)) //half the cube size
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));
        // pink cube
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), //PLANE_SIZE
                //material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_xyz(3.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(comps::PC_Comp)
            .insert(comps::startup)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5, 0.5)) //half the cube size
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

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

        // // Heaven sky orb
        let player_handle2: Handle<Scene> = asset_server.load("nature/heaven/scene.gltf#Scene0");
        commands
            .spawn_bundle(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                scene: player_handle2.clone(),
                ..default()
            })
            .insert(comps::PC_Comp)
            .insert(comps::startup);

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
}
