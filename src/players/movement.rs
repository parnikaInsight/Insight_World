use bevy::animation::Keyframes;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
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
        asset_server.load("mixamo/Shoot_Forward#Animation0"),
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
    names: Query<&Name>,
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

                    let hashmap_curves =
                        assets.get(&animations.0[1].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[1].clone_weak()).unwrap();
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

                    let hashmap_curves =
                        assets.get(&animations.0[2].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[2].clone_weak()).unwrap();
                    // let new_transform =
                    //     get_player_displacement(t.clone(), player, hashmap_curves, animation_clip);
                    // println!("new trans: {}", new_transform.translation);
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

                    let hashmap_curves =
                        assets.get(&animations.0[3].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[3].clone_weak()).unwrap();
                    // let new_transform =
                    //     get_player_displacement(player_ent, /*t.clone(),*/ player, hashmap_curves, animation_clip);
                    // println!("new trans: {}", new_transform.translation);
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

                    let hashmap_curves =
                        assets.get(&animations.0[4].clone_weak()).unwrap().curves();
                    let animation_clip = assets.get(&animations.0[4].clone_weak()).unwrap();
                    let new_transform = get_player_displacement(
                        t.clone(),
                        player_ent,
                        player,
                        hashmap_curves,
                        animation_clip,
                        &names,
                    );
                    println!("new trans: {}", new_transform.translation);
                    t.translation.x = new_transform.translation.y;
                    break;
                }
            }
        }
    }
}

// Cannot use keyframes because must split Quat/Vec3 by timestamp
#[derive(Clone)]
pub enum Num {
    Quat(Quat),
    Vec3(Vec3),
}

#[derive(Clone)]
pub enum KeyframeType {
    Rotation,
    Translation,
    Scale,
}

pub fn get_player_displacement(
    mut transform: Transform,
    player_entity: Entity,
    player: Mut<AnimationPlayer>,
    hashmap_curves: &HashMap<EntityPath, Vec<VariableCurve>>,
    animation_clip: &AnimationClip,
    names: &Query<&Name>, //path.parts.contains(&player_entity) returns error without this
) -> Transform {
    // For a player's singular animation, get a hashmap with key being timestamp and value
    // being all the keyframes at that timestamp from the animation clip's different variable curves
    let mut transforms: HashMap<String, Vec<(KeyframeType, Num)>> = HashMap::new();
    let mut longest_time: Vec<f32> = vec![0.0];
    // hashmap_curves is animation_clip.curves which is a hashmap
    // Key: EntityPath, or list of players
    // Value: vector of variable curves that were added to the animation clip for those players.
    for (path, curves) in hashmap_curves {
        if let Ok(name) = names.get(player_entity) {
            // If EntityPath contains the player
            if path.parts.contains(name) {
                // Variable curves corresponding to EntityPath.
                for curve in curves {
                    // //one keyframe
                    // if curve.keyframe_timestamps.len() == 1 {
                    //     match &curve.keyframes {
                    //         Keyframes::Rotation(keyframes) => transform.rotation = keyframes[0],
                    //         Keyframes::Translation(keyframes) => {
                    //             transform.translation = keyframes[0];
                    //         }
                    //         Keyframes::Scale(keyframes) => transform.scale = keyframes[0],
                    //     }
                    //     continue;
                    // }

                    // // Apply the keyframe
                    // let len = curve.keyframe_timestamps.len();
                    // let elapsed = player.elapsed();
                    // for index in 0..(len-1) {
                    //     let ts_start = curve.keyframe_timestamps[index];
                    //     let ts_end = curve.keyframe_timestamps[index + 1];
                    //     let lerp = (elapsed - ts_start) / (ts_end - ts_start);

                    //     match &curve.keyframes {
                    //         Keyframes::Rotation(keyframes) => {
                    //             let rot_start = keyframes[index];
                    //             let mut rot_end = keyframes[index + 1];
                    //             // Choose the smallest angle for the rotation
                    //             if rot_end.dot(rot_start) < 0.0 {
                    //                 rot_end = -rot_end;
                    //             }
                    //             // Rotations are using a spherical linear interpolation
                    //             transform.rotation =
                    //                 rot_start.normalize().slerp(rot_end.normalize(), lerp);
                    //         }
                    //         Keyframes::Translation(keyframes) => {
                    //             let translation_start = keyframes[index];
                    //             let translation_end = keyframes[index + 1];
                    //             let result = translation_start.lerp(translation_end, lerp);
                    //             transform.translation = result;
                    //         }
                    //         Keyframes::Scale(keyframes) => {
                    //             let scale_start = keyframes[index];
                    //             let scale_end = keyframes[index + 1];
                    //             let result = scale_start.lerp(scale_end, lerp);
                    //             transform.scale = result;
                    //         }
                    //     }
                    // }




                    // IMPORTANT: START HERE 
                    // organize transforms not only by timestamp but also which variable curve the Quat/Vec3
                    // is a part of. That way when updating transform, you know when t and t+1 correspond
                    // to same variable curve.
                    let mut count = 0; // helps get keyframe at timestamp
                    let keyframes = &curve.keyframes;
                    for time in &curve.keyframe_timestamps {
                        if transforms.contains_key(&time.to_string()) {
                            let mut v = (*transforms.get(&time.to_string()).unwrap()).clone();
                            match keyframes {
                                Keyframes::Rotation(i) => {
                                    let elem = i[count];
                                    let add = Num::Quat(elem);
                                    let appending = (KeyframeType::Rotation, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                                Keyframes::Translation(i) => {
                                    let elem = i[count];
                                    let add = Num::Vec3(elem);
                                    let appending = (KeyframeType::Translation, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                                Keyframes::Scale(i) => {
                                    let elem = i[count];
                                    let add = Num::Vec3(elem);
                                    let appending = (KeyframeType::Scale, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                            }
                        } else {
                            let mut v = Vec::new();
                            match keyframes {
                                Keyframes::Rotation(i) => {
                                    let elem = i[count];
                                    let add = Num::Quat(elem);
                                    let appending = (KeyframeType::Rotation, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                                Keyframes::Translation(i) => {
                                    let elem = i[count];
                                    let add = Num::Vec3(elem);
                                    let appending = (KeyframeType::Translation, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                                Keyframes::Scale(i) => {
                                    let elem = i[count];
                                    let add = Num::Vec3(elem);
                                    let appending = (KeyframeType::Scale, add);
                                    v.push(appending);
                                    transforms.insert(time.to_string(), v);
                                }
                            }
                        }
                        if count > longest_time.len() {
                            longest_time.push(*time);
                        }
                        count += 1;
                    }
     

    //update t
    let mut count = 0;
    let mut elapsed = player.elapsed();
    for i in longest_time {
        let mut v_count = 0;
        let v = *transforms.get(&i.to_string()).unwrap();
        let ts_start = i;
        let ts_end = longest_time[count];
        let lerp = (elapsed - ts_start) / (ts_end - ts_start);
        for (keyframe_type, num) in v.split_off(v.len() - 1) {
            match keyframe_type {
                KeyframeType::Rotation => {
                    match num {
                        Num::Quat(q) => {
                            let rot_start = q;
                            let (tup_key, tup_num) = v[v_count + 1];
                            match tup_key {
                                KeyframeType::Rotation => {
                                    match num {}
                                }
                            }

                            let mut rot_end = v[v_count + 1];
                            // Choose the smallest angle for the rotation
                            if rot_end.dot(rot_start) < 0.0 {
                                rot_end = -rot_end;
                            }
                            // Rotations are using a spherical linear interpolation
                            transform.rotation =
                                rot_start.normalize().slerp(rot_end.normalize(), lerp);
                        },
                        Num::Vec3(v) => continue,
                    }
                },
                _ => continue
                // KeyframeType::Translation => {},
                // KeyframeType::Scale => {},
            }
            v_count += 1;
        }
        count += 1;
    }
}
}
}
}
transform
}


//--------------------------------------------------------------------------------------------------------------------
//since this is a hashmap and theres no order of curves, players of ggrs move different distances
//also distance calculated via curves is incorrect
// right now only D displaces
// ex. pressing D and W shifts the player right and then moves forward
// rather than moving right, returning to original position, and then moving forward

// pub fn get_player_displacement(
//     // mut transform: Transform,
//     player_entity: Entity,
//     player: Mut<AnimationPlayer>,
//     hashmap_curves: &HashMap<EntityPath, Vec<VariableCurve>>,
//     animation_clip: &AnimationClip,

//     names: Query<&Name>,
//     mut transforms: Query<&mut Transform>,
//     children: Query<&Children>,
//     // assets: Res<Assets<AnimationClip>>,
//     // animations: Res<CharacterAnimations>,
// ) -> Transform {
//     //let mut transform = t.clone();
//     // Get AnimationClip curves from passing AnimationClip handle to assets.
//     //let hashmap_curves = assets.get(&animations.0[1].clone_weak()).unwrap().curves();
//     // rewrite this method taking into account key timeframes-- animation player is a system that animates starting from where entity is in animation path
//     // in one go, im trying to find where entity will be after entire animation complete

//     //order of for loop unknown since hashmap
//     'entity: for (path, curves) in hashmap_curves {
//         // PERF: finding the target entity can be optimised
//         let mut current_entity = player_entity;
//         // Ignore the first name in the entity path, it is the root node which we already have since we query for names
//         for part in path.parts.iter().skip(1) {
//             let mut found = false;
//             // Get all entities in the path following current_entity
//             if let Ok(children) = children.get(current_entity) {
//                 for child in children.deref() {
//                     if let Ok(name) = names.get(*child) {
//                         if name == part {
//                             // Found a children with the right name, continue to the next part
//                             current_entity = *child;
//                             found = true;
//                             break;
//                         }
//                     }
//                 }
//             }
//             if !found {
//                 warn!("Entity not found for path {:?} on part {:?}", path, part);
//                 continue 'entity;
//             }
//         }

//         if let Ok(mut transform) = transforms.get_mut(current_entity) {
//             for curve in curves {
//                 // Some curves have only one keyframe used to set a transform.
//                 if curve.keyframe_timestamps.len() == 1 {
//                     match &curve.keyframes {
//                         Keyframes::Rotation(keyframes) => transform.rotation = keyframes[0],
//                         Keyframes::Translation(keyframes) => {
//                             transform.translation = keyframes[0];
//                         }
//                         Keyframes::Scale(keyframes) => transform.scale = keyframes[0],
//                     }
//                     continue;
//                 }

//                 let mut elapsed = player.elapsed();
//                 // Assumes animation of one movement is not repeated.
//                 // Else, need AnimationClip's "repeat" private field
//                 //let animation_clip = assets.get(&animations.0[1].clone_weak()).unwrap();
//                 if elapsed < 0.0 {
//                     elapsed += animation_clip.duration();
//                 }

//                 // Find the current keyframe
//                 // PERF: finding the current keyframe can be optimised
//                 let step_start = 0;
//                 // let step_start = match curve
//                 //     .keyframe_timestamps
//                 //     .binary_search_by(|probe| probe.partial_cmp(&elapsed).unwrap())
//                 // {
//                 //     Ok(i) => i,
//                 //     Err(0) => continue, // this curve isn't started yet
//                 //     Err(n) if n > curve.keyframe_timestamps.len() - 1 => continue, // this curve is finished
//                 //     Err(i) => i - 1,
//                 // };
//                 let ts_start = curve.keyframe_timestamps[step_start];
//                 let ts_end = curve.keyframe_timestamps[step_start + 1];
//                 let lerp = (elapsed - ts_start) / (ts_end - ts_start);

//                 // Apply the keyframe
//                 match &curve.keyframes {
//                     Keyframes::Rotation(keyframes) => {
//                         let rot_start = keyframes[step_start];
//                         let mut rot_end = keyframes[step_start + 1];
//                         // Choose the smallest angle for the rotation
//                         if rot_end.dot(rot_start) < 0.0 {
//                             rot_end = -rot_end;
//                         }
//                         // Rotations are using a spherical linear interpolation
//                         transform.rotation = rot_start.normalize().slerp(rot_end.normalize(), lerp);
//                     }
//                     Keyframes::Translation(keyframes) => {
//                         let translation_start = keyframes[step_start];
//                         let translation_end = keyframes[step_start + 1];
//                         let result = translation_start.lerp(translation_end, lerp);
//                         transform.translation = result;
//                     }
//                     Keyframes::Scale(keyframes) => {
//                         let scale_start = keyframes[step_start];
//                         let scale_end = keyframes[step_start + 1];
//                         let result = scale_start.lerp(scale_end, lerp);
//                         transform.scale = result;
//                     }
//                 }
//             }
//         }
//     }
//     // t = transform;
//     transform
//     //return Mut<'a, transform>;
// }

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
