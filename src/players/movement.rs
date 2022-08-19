use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider, SessionType};
use bevy_pbr::PbrBundle;
use bevy_pbr::PointLightBundle;
use bevy_pbr::StandardMaterial;
use bevy_rapier3d::prelude::*;
use bevy_render::color::Color;
use bevy_render::mesh::shape;
use bevy_render::mesh::Mesh;
use bytemuck::{Pod, Zeroable};
use ggrs::{
    Config, InputStatus, P2PSession, PlayerHandle, PlayerType, SessionBuilder, SpectatorSession,
    SyncTestSession, UdpNonBlockingSocket,
};
use std::env;
use std::{hash::Hash, net::SocketAddr};

use crate::players::info;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

const MOVEMENT_SPEED: f32 = 0.005;
const MAX_SPEED: f32 = 0.05;
const FRICTION: f32 = 0.9;
const PLANE_SIZE: f32 = 15.0;
const CUBE_SIZE: f32 = 0.2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8,
}

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

// Example system, manipulating a resource, will be added to the rollback schedule.
// Increases the frame count by 1 every update step. If loading and saving resources works correctly,
// you should see this resource rolling back, counting back up and finally increasing by 1 every update step
#[allow(dead_code)]
pub fn increase_frame_system(mut frame_count: ResMut<info::FrameCount>) {
    frame_count.frame += 1;
}

// Example system that moves the cubes, will be added to the rollback schedule.
// Filtering for the rollback component is a good way to make sure your game logic systems
// only mutate components that are being saved/loaded.
#[allow(dead_code)]
pub fn move_cube_system(
    mut query: Query<(&mut Transform, &mut info::Velocity, &info::Player), With<Rollback>>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
) {
    for (mut t, mut v, p) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;
        // set velocity through key presses
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            v.z -= MOVEMENT_SPEED;
        }
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            v.z += MOVEMENT_SPEED;
        }
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            v.x -= MOVEMENT_SPEED;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            v.x += MOVEMENT_SPEED;
        }

        // slow down
        if input & INPUT_UP == 0 && input & INPUT_DOWN == 0 {
            v.z *= FRICTION;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT == 0 {
            v.x *= FRICTION;
        }
        v.y *= FRICTION;

        // constrain velocity
        let mag = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if mag > MAX_SPEED {
            let factor = MAX_SPEED / mag;
            v.x *= factor;
            v.y *= factor;
            v.z *= factor;
        }

        // apply velocity
        t.translation.x += v.x;
        t.translation.y += v.y;
        t.translation.z += v.z;

        // constrain cube to plane
        t.translation.x = t.translation.x.max(-1. * (PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.x = t.translation.x.min((PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.z = t.translation.z.max(-1. * (PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.z = t.translation.z.min((PLANE_SIZE - CUBE_SIZE) * 0.5);
    }
}

//------------------------------------------------------------------------------------------------
pub struct CharacterAnimations(Vec<Handle<AnimationClip>>);

pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                        t.translation.z += 0.2;
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
pub struct AnimationHelperSetup; //marker for parent with animation player child

#[derive(Component)]
pub struct AnimationHelper {
    //contains reference to specific animation player
    pub player_entity: Entity,
}

impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper { player_entity }
    }
}

pub fn setup_helpers(
    //finds all AnimationHelperSetup markers and recursively looks through their children until animation player found
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
                // .remove::<AnimationHelperSetup>()
                .insert(AnimationHelper::new(animation_player)); // This is how I find it later and  what I query for
        }
    }
}

fn find_animation_player_entity(
    parent: Entity,
    children: &Query<&Children>,
    players: &Query<&AnimationPlayer>,
) -> Option<Entity> {
    if let Ok(candidates) = children.get(parent) {
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                if players.get(candidate).is_ok() {
                    return Some(candidate);
                } else if let Ok(new) = children.get(candidate) {
                    next_candidates.extend(new.iter());
                }
            }
        }
    }
    None
}
