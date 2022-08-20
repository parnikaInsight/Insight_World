//use std::collections::HashMap;
use bevy::utils::hashbrown::HashMap;
use bevy::prelude::*;
use bevy_ggrs::Rollback;
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, PlayerHandle};

use crate::players::info;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8, // List of inputs: up, down, left, right.
}

// Handles one movement.
pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    BoxInput { inp: input }
}
pub struct CharacterAnimations(Vec<Handle<AnimationClip>>);

// Add player animations.
pub fn setup_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Insert a resource with the current scene information
    commands.insert_resource(CharacterAnimations(vec![
        asset_server.load("mixamo/from_blender.glb#Animation0"),
        asset_server.load("mixamo/walk_forward.glb#Animation0"),
        asset_server.load("mixamo/backward.glb#Animation0"),
        asset_server.load("mixamo/left.glb#Animation0"),
        asset_server.load("mixamo/right_crouch.glb#Animation0"),
    ]));
}

pub fn animate_moving_player(
    animations: Res<CharacterAnimations>,
    assets: Res<Assets<AnimationClip>>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &info::Player,
        &AnimationHelper,
    )>,
    mut done: Local<bool>,
) {
    // Update all players.
    for (e, children, mut t, p, helper) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;
        // println!("start {}", t.translation);
        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            println!("pressed W");

            // Check that player's helper entity has the same id as the AnimationPlayer entity.
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    println!("start{}\n {}", t.translation, player_ent.id());
                    player.play(animations.0[1].clone_weak());

                    let hashmap_curves = assets.get(&animations.0[1].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[1].clone_weak()).unwrap();
                    let new_transform = get_player_displacement(t.clone(), player, hashmap_curves, animation_clip);
                    println!("new trans: {}", new_transform.translation);
                    //update t
                    //t.translation.z = new_transform.translation.y;

                    // // let hashmap_curves =
                    // //         assets.get(&animations.0[1].clone_weak()).unwrap().curves();
                    // // let (path, curves) = hashmap_curves[0];
                    // // let step_start = match curve
                    // //     .keyframe_timestamps
                    // //     .binary_search_by(|probe| probe.partial_cmp(&elapsed).unwrap())
                    // // {
                    // //     Ok(i) => i,
                    // //     Err(0) => {
                    // //         println!("cont 1");
                    // //         continue;
                    // //     } // this curve isn't started yet
                    // //     Err(n) if n > curve.keyframe_timestamps.len() - 1 => {
                    // //         *done == true;
                    // //         continue;
                    // //     } // this curve is finished
                    // //     Err(i) => i - 1,
                    // // };

                    // //println!("Player animation W");
                    // // t.translation.z += 0.1;

                    // //let curves = assets.get(&animations.0[1].clone_weak()).unwrap().curves();
                    // //t = get_player_displacement(t, player, assets, animations);

                    // if (*done == true) {
                    //     let hashmap_curves =
                    //         assets.get(&animations.0[1].clone_weak()).unwrap().curves();
                    //     for (path, curves) in hashmap_curves {
                    //         //println!("variblecurve");
                    //         for curve in curves {
                    //             //println!("curve");
                    //             // Some curves have only one keyframe used to set a transform.
                    //             if curve.keyframe_timestamps.len() == 1 {
                    //                 println!("one");
                    //                 match &curve.keyframes {
                    //                     Keyframes::Rotation(keyframes) => t.rotation = keyframes[0],
                    //                     Keyframes::Translation(keyframes) => {
                    //                         t.translation = keyframes[0];
                    //                     }
                    //                     Keyframes::Scale(keyframes) => t.scale = keyframes[0],
                    //                 }
                    //                 continue;
                    //             }
                    //             //println!("after");

                    //             let mut elapsed = player.elapsed();
                    //             // Assumes animation of one movement is not repeated.
                    //             // Else, need AnimationClip's "repeat" private field
                    //             let animation_clip =
                    //                 assets.get(&animations.0[1].clone_weak()).unwrap();
                    //             if elapsed < 0.0 {
                    //                 elapsed += animation_clip.duration();
                    //             }
                    //             println!("elapsed");

                    //             // Find the current keyframe
                    //             // PERF: finding the current keyframe can be optimised

                    //             let step_start = 0;
                    //             // let step_start = match curve
                    //             //     .keyframe_timestamps
                    //             //     .binary_search_by(|probe| probe.partial_cmp(&elapsed).unwrap())
                    //             // {
                    //             //     Ok(i) => i,
                    //             //     Err(0) => {
                    //             //         println!("cont 1");
                    //             //         continue}, // this curve isn't started yet
                    //             //     Err(n) if n > curve.keyframe_timestamps.len() - 1 => {
                    //             //         println!("cont 2");
                    //             //         continue}, // this curve is finished
                    //             //     Err(i) => i - 1,
                    //             // };
                    //             println!("stepstart {}", step_start);

                    //             let ts_start = curve.keyframe_timestamps[step_start];
                    //             let ts_end = curve.keyframe_timestamps[step_start + 1];
                    //             let lerp = (elapsed - ts_start) / (ts_end - ts_start);
                    //             println!("current");

                    //             // Apply the keyframe
                    //             match &curve.keyframes {
                    //                 Keyframes::Rotation(keyframes) => {
                    //                     let rot_start = keyframes[step_start];
                    //                     let mut rot_end = keyframes[step_start + 1];
                    //                     // Choose the smallest angle for the rotation
                    //                     if rot_end.dot(rot_start) < 0.0 {
                    //                         rot_end = -rot_end;
                    //                     }
                    //                     // Rotations are using a spherical linear interpolation
                    //                     t.rotation =
                    //                         rot_start.normalize().slerp(rot_end.normalize(), lerp);
                    //                     println!("rotated");
                    //                 }
                    //                 Keyframes::Translation(keyframes) => {
                    //                     let translation_start = keyframes[step_start];
                    //                     let translation_end = keyframes[step_start + 1];
                    //                     let result = translation_start.lerp(translation_end, lerp);
                    //                     t.translation = result;
                    //                     println!("translated");
                    //                 }
                    //                 Keyframes::Scale(keyframes) => {
                    //                     let scale_start = keyframes[step_start];
                    //                     let scale_end = keyframes[step_start + 1];
                    //                     let result = scale_start.lerp(scale_end, lerp);
                    //                     t.scale = result;
                    //                     println!("scaled");
                    //                 }
                    //             }
                    //             println!("matched");
                    //         }
                    //     }
                    // }
                    // //----------------------------------------------------------------
                    // println!("trans {}", t.translation);
                    // break;
                }
            }
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            // println!("pressed S");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[2].clone_weak());
                    //println!("Player animation S");
                    //t.translation.z -= 0.1;

                    let hashmap_curves = assets.get(&animations.0[2].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[2].clone_weak()).unwrap();
                    let new_transform = get_player_displacement(t.clone(), player, hashmap_curves, animation_clip);
                    println!("new trans: {}", new_transform.translation);
                }
            }
        }
        // A
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            //println!("pressed A");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[3].clone_weak());
                    //println!("Player animation A");
                    //t.translation.x += 0.1;

                    let hashmap_curves = assets.get(&animations.0[3].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[3].clone_weak()).unwrap();
                    let new_transform = get_player_displacement(t.clone(), player, hashmap_curves, animation_clip);
                    println!("new trans: {}", new_transform.translation);
                }
            }
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            // println!("pressed D");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[4].clone_weak());
                    // println!("Player animation D");
                    // t.translation.x -= 0.1;

                    let hashmap_curves = assets.get(&animations.0[4].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[4].clone_weak()).unwrap();
                    let new_transform = get_player_displacement(t.clone(), player, hashmap_curves, animation_clip);
                    println!("new trans: {}", new_transform.translation);
                    t.translation.x = new_transform.translation.y;
                }
            }
        }
    }
}

//since this is a hashmap and theres no order of curves, players of ggrs move different distances
//also distance calculated via curves is incorrect
// right now only D displaces
// ex. pressing D and W shifts the player right and then moves forward 
// rather than moving right, returning to original position, and then moving forward
pub fn get_player_displacement<'a>(
    mut transform: Transform,
    player: Mut<AnimationPlayer>,
    hashmap_curves: &HashMap<EntityPath, Vec<VariableCurve>>,
    animation_clip: &AnimationClip,
    // assets: Res<Assets<AnimationClip>>,
    // animations: Res<CharacterAnimations>,
) -> Transform
{
    //let mut transform = t.clone();
    // Get AnimationClip curves from passing AnimationClip handle to assets.
    //let hashmap_curves = assets.get(&animations.0[1].clone_weak()).unwrap().curves();
    for (path, curves) in hashmap_curves {
        for curve in curves {
            // Some curves have only one keyframe used to set a transform.
            if curve.keyframe_timestamps.len() == 1 {
                match &curve.keyframes {
                    Keyframes::Rotation(keyframes) => transform.rotation = keyframes[0],
                    Keyframes::Translation(keyframes) => {
                        transform.translation = keyframes[0];
                    }
                    Keyframes::Scale(keyframes) => transform.scale = keyframes[0],
                }
                continue;
            }

            let mut elapsed = player.elapsed();
            // Assumes animation of one movement is not repeated.
            // Else, need AnimationClip's "repeat" private field
            //let animation_clip = assets.get(&animations.0[1].clone_weak()).unwrap();
            if elapsed < 0.0 {
                elapsed += animation_clip.duration();
            }

            // Find the current keyframe
            // PERF: finding the current keyframe can be optimised
            let step_start = 0;
            // let step_start = match curve
            //     .keyframe_timestamps
            //     .binary_search_by(|probe| probe.partial_cmp(&elapsed).unwrap())
            // {
            //     Ok(i) => i,
            //     Err(0) => continue, // this curve isn't started yet
            //     Err(n) if n > curve.keyframe_timestamps.len() - 1 => continue, // this curve is finished
            //     Err(i) => i - 1,
            // };
            let ts_start = curve.keyframe_timestamps[step_start];
            let ts_end = curve.keyframe_timestamps[step_start + 1];
            let lerp = (elapsed - ts_start) / (ts_end - ts_start);

            // Apply the keyframe
            match &curve.keyframes {
                Keyframes::Rotation(keyframes) => {
                    let rot_start = keyframes[step_start];
                    let mut rot_end = keyframes[step_start + 1];
                    // Choose the smallest angle for the rotation
                    if rot_end.dot(rot_start) < 0.0 {
                        rot_end = -rot_end;
                    }
                    // Rotations are using a spherical linear interpolation
                    transform.rotation = rot_start.normalize().slerp(rot_end.normalize(), lerp);
                }
                Keyframes::Translation(keyframes) => {
                    let translation_start = keyframes[step_start];
                    let translation_end = keyframes[step_start + 1];
                    let result = translation_start.lerp(translation_end, lerp);
                    transform.translation = result;
                }
                Keyframes::Scale(keyframes) => {
                    let scale_start = keyframes[step_start];
                    let scale_end = keyframes[step_start + 1];
                    let result = scale_start.lerp(scale_end, lerp);
                    transform.scale = result;
                }
            }
        }
    }
    // t = transform;
    transform
    //return Mut<'a, transform>;
}

#[derive(Debug, Component)]
pub struct AnimationHelperSetup; // Marker for parent with animation player child.

#[derive(Component)]
pub struct AnimationHelper {
    // Contains reference to specific animation player.
    pub player_entity: Entity,
}

impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper { player_entity }
    }
}

pub fn setup_helpers(
    // Finds all AnimationHelperSetup markers.
    // Recursively looks through their children until animation player found.
    mut commands: Commands,
    to_setup: Query<Entity, With<AnimationHelperSetup>>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
) {
    for host_entity in to_setup.iter() {
        if let Some(animation_player) =
            find_animation_player_entity(host_entity, &children, &players)
        {
            commands
                .entity(host_entity)
                // This is how to find the animation player later
                .insert(AnimationHelper::new(animation_player));
        }
    }
}

fn find_animation_player_entity(
    parent: Entity,
    children: &Query<&Children>,
    players: &Query<&AnimationPlayer>,
) -> Option<Entity> {
    if let Ok(candidates) = children.get(parent) {
        // Collect all children.
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                // Return child if it is the entity with an AnimationPlayer component.
                if players.get(candidate).is_ok() {
                    return Some(candidate);
                }
                // Else recursively get children and add to candidates list.
                else if let Ok(new) = children.get(candidate) {
                    next_candidates.extend(new.iter());
                }
            }
        }
    }
    None
}
