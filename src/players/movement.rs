#![allow(dead_code)]

use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, PlayerHandle};
use std::time::Duration;

use crate::animation::{animation_helper, play};
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
pub fn input(keyboard_input: Res<Input<KeyCode>>, mut commands: Commands) {
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

    let vec = vec![BoxInput { inp: input }];
    commands.insert_resource(vec);
}

pub fn animate_moving_player(
    animations: Res<play::CharacterAnimations>,
    // assets: Res<Assets<AnimationClip>>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    //inputs: Res<Vec<(BoxInput, InputStatus)>>,
    inputs: Res<Vec<BoxInput>>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &mut info::Player,
        &animation_helper::AnimationHelper,
    )>,
) {
    for (e, children, mut t, mut p, helper) in query.iter_mut() {
        //let input = inputs[p.handle as usize].0.inp;
        let input = inputs[p.handle as usize].inp;

        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            //println!("pressed W");
            //println!("{}", t.translation);

            //check that the shooter's parent entity's helper entity has the same id as the animation_player entity

            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    match p.state.state {
                        info::PlayerStateEnum::IDLE => {
                            println!("idle");
                            if p.state.animation.is_none() || p.state.animation.unwrap() != 0 {
                                println!("none123");
                                player.play(animations.0[5].clone_weak()).repeat();
                                println!("none");

                                p.state.animation = Some(0);
                            }
                        }
                        info::PlayerStateEnum::MOVING => {
                            println!("moving heyyyyyyyyyyyyyy");
                        }
                    };

                    //player.play(animations.0[5].clone_weak()).repeat();

                    // player.play(animations.0[6].clone_weak());
                    // println!("anim");
                    // player//cross fade requires animation before it?
                    //     .cross_fade(
                    //         animations.0[5].clone_weak(),
                    //         Duration::from_secs_f32(0.25),
                    //     )
                    //     .set_speed(1.3)
                    //     .repeat();

                    //println!("Player animation W");
                    //t.translation.z += 0.1;

                    // let a: &Assets<AnimationClip>;
                    // let animation_clip = Assets::get(&animations.0[1].clone_weak());
                }
            }
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            //println!("pressed S");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[2].clone_weak());
                    //println!("Player animation S");
                    // t.translation.z -= 0.1;
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
                }
            }
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            //println!("pressed D");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[4].clone_weak());
                    //println!("Player animation D");
                    //t.translation.x -= 0.1;
                }
            }
        }
    }
}

pub fn translate_player(
    animations: Res<play::CharacterAnimations>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    //inputs: Res<Vec<(BoxInput, InputStatus)>>,
    inputs: Res<Vec<BoxInput>>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &info::Player,
        &animation_helper::AnimationHelper,
        &info::Velocity,
    )>,
    time: Res<Time>,
) {
    for (e, children, mut t, p, helper, speed) in query.iter_mut() {
        let input = inputs[p.handle as usize].inp;

        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            //check that the shooter's parent entity's helper entity has the same id as the animation_player entity

            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    println!("Player trans W");
                    // let mut direction = Vec3::default();
                    // direction.z += 1.0;
                    // if direction.length() > time.delta_seconds() * speed.z {
                    //     let normalized_dir = direction.normalize();
                    //     t.translation += normalized_dir * speed.z * time.delta_seconds();
                    // } else {
                    //     t.translation = Vec3::ZERO;
                    // }

                    t.translation.z += 0.1;
                }
            }
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            //println!("pressed S");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    //println!("Player animation S");
                    t.translation.z -= 0.1;
                }
            }
        }
        // A
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            //println!("pressed A");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    //println!("Player animation A");
                    t.translation.x += 0.1;
                }
            }
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            //println!("pressed D");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    //println!("Player animation D");
                    t.translation.x -= 0.1;
                }
            }
        }
    }
}
