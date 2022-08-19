use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn play_scene(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
   // mut done: Local<bool>,
) {
    for mut anim in player.iter_mut(){
        anim.play(animations.0[0].clone_weak()).repeat();
        println!("in here");
    }
 }

pub fn create_default_plane(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
){
    commands.insert_resource(Animations(vec![
        asset_server.load("nature/phoenix_bird/scene.gltf#Animation0")
    ]));

    let player_handle1: Handle<Scene> = asset_server.load("nature/phoenix_bird/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, -10.0),
            scale: Vec3::new(0.01, 0.01, 0.01),
            ..default()
        },
        scene: player_handle1.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("nature/heaven/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    println!("created bird");
}
