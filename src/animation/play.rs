use bevy::prelude::*;

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