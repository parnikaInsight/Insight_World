use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct CharacterAnimations(pub Vec<Handle<AnimationClip>>);

// Add player animations.
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert a resource with the current scene information
    // TODO: Insert animation rig w/t skin to use between characters
    commands.insert_resource(CharacterAnimations(vec![ 
        // // with girl skin
        // asset_server.load("mixamo/idle.glb#Animation0"), 
        // asset_server.load("mixamo/shoot.glb#Animation0"),
        // asset_server.load("mixamo/flip_uppercut.glb#Animation0"),
        
        // no skin
        asset_server.load("default_characters/idle_breathing.glb#Animation0"), 
        asset_server.load("default_characters/shoot.glb#Animation0"),
        asset_server.load("default_characters/flip_punch.glb#Animation0"),    
        asset_server.load("default_characters/dance.glb#Animation0"),    
        asset_server.load("default_characters/straight_punch.glb#Animation0"),    
        asset_server.load("default_characters/fly_back_death.glb#Animation0"),   
        asset_server.load("default_characters/jump_attack.glb#Animation0"),    
        asset_server.load("default_characters/injured.glb#Animation0"),  
        asset_server.load("default_characters/two_hands_spell.glb#Animation0"), 
        asset_server.load("default_characters/archer.glb#Animation0"),  
        asset_server.load("default_characters/running.glb#Animation0"), 
        asset_server.load("default_characters/mutant_running.glb#Animation0"), 
        asset_server.load("default_characters/sword_run.glb#Animation0"), 
        asset_server.load("default_characters/sword_jump.glb#Animation0"), 
        asset_server.load("default_characters/sword_cast.glb#Animation0"), 
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