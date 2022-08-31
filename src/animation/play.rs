use bevy::prelude::*;

pub struct CharacterAnimations(pub Vec<Handle<AnimationClip>>);

// Add player animations.
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(CharacterAnimations(vec![
        asset_server.load("mixamo/idle.glb#Animation0"),
        asset_server.load("mixamo/shoot.glb#Animation0"),
        asset_server.load("mixamo/flip_uppercut.glb#Animation0"),
    ]));
}

#[derive(Debug)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

// Play stationary gltf animations.
pub fn play_scene(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut anim in player.iter_mut(){
        anim.play(animations.0[0].clone_weak()).repeat();
    }
 }