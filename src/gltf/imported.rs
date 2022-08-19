use crate::players::{info, movement};
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, Rollback};
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("mixamo/from_blender.glb#Animation0")
    ]));

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    //Character

    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(2.0, 0.0, 0.0),
            ..default()
        },
        scene: asset_server.load("mixamo/from_blender.glb#Scene0"),
        ..default()
    });
}


// Once the scene is loaded, start the animation
pub fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            let time_elapsed = player.play(animations.0[0].clone_weak()).repeat().elapsed();
            println!("time: {}", time_elapsed);
            *done = true;
            println!("Animation");
        }
    }
}