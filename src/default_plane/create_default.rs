use bevy::prelude::*;

#[derive(Debug)]
pub struct Animations(Vec<Handle<AnimationClip>>);

// Play stationary gltf animations.
pub fn play_scene(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut anim in player.iter_mut(){
        anim.play(animations.0[0].clone_weak()).repeat();
        println!("in here");
    }
 }

// Add startionary gltfs.
pub fn create_default_plane(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
){
    // Insert startionary gltf animations.
    commands.insert_resource(Animations(vec![
        asset_server.load("nature/phoenix_bird/scene.gltf#Animation0")
    ]));

    // Load gltf.
    let player_handle1: Handle<Scene> = asset_server.load(
        "nature/phoenix_bird/scene.gltf#Scene0"
    );
    // Spawning SceneBundle automatically adds AnimationPlayer.
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, -10.0),
            scale: Vec3::new(0.01, 0.01, 0.01),
            ..default()
        },
        scene: player_handle1.clone(),
        ..default()
    });

    // Heaven sky orb
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

    // Dome
    let player_handle3: Handle<Scene> = asset_server.load("default/parasol/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(0.001, 0.001, 0.001),
            ..default()
        },
        scene: player_handle3.clone(),
        ..default()
    });
}
