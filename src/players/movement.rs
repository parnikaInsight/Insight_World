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
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
    // assets: Res<Assets<AnimationClip>>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
    mut query: Query<
        (
            Entity,
            &Children,
            &mut Transform,
            &info::Player,
            &AnimationHelper,
        ),
        With<Rollback>,
    >,
) {
    for (e, children, mut t, p, helper) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;

        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            println!("pressed W");
            println!("{}", t.translation);

            //check that the shooter's parent entity's helper entity has the same id as the animation_player entity

            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[1].clone_weak());
                    println!("Player animation W");
                    t.translation.z += 0.1;

                    // let a: &Assets<AnimationClip>;
                    // let animation_clip = Assets::get(&animations.0[1].clone_weak());
                }
            }
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            println!("pressed S");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[2].clone_weak());
                    println!("Player animation S");
                    t.translation.z -= 0.1;
                }
            }
        }
        // A
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            println!("pressed A");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[3].clone_weak());
                    println!("Player animation A");
                    t.translation.x += 0.1;
                }
            }
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            println!("pressed D");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[4].clone_weak());
                    println!("Player animation D");
                    t.translation.x -= 0.1;
                }
            }
        }
    }
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
