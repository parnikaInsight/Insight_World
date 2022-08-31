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
const POWER: u8 = 1 << 4;

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
    if keyboard_input.pressed(KeyCode::LShift) {
        input |= POWER;
    }

    BoxInput { inp: input }
}

pub fn animate_moving_player(
    animations: Res<play::CharacterAnimations>,
    // assets: Res<Assets<AnimationClip>>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
    //inputs: Res<Vec<BoxInput>>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &mut info::Player,
        &animation_helper::AnimationHelper,
    )>,
) {
    for (e, children, mut t, mut p, helper) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;

        //check that the shooter's parent entity's helper entity has the same id as the animation_player entity
        for (player_ent, mut player) in &mut player {
            if helper.player_entity.id() == player_ent.id() {
                match p.state.state {
                    info::PlayerStateEnum::IDLE => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 0 {
                            player.play(animations.0[0].clone_weak());
                            p.state.animation = Some(0);
                        }
                    }
                    info::PlayerStateEnum::MOVING => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 1 {
                            player
                                .cross_fade(
                                    animations.0[1].clone_weak(),
                                    Duration::from_secs_f32(0.25),
                                )
                                .set_speed(1.3)
                                .repeat();
                            p.state.animation = Some(1);
                        }
                    }
                    info::PlayerStateEnum::POWER => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 2 {
                            player
                                .cross_fade(
                                    animations.0[2].clone_weak(),
                                    Duration::from_secs_f32(0.25),
                                )
                                .set_speed(1.3);
                            p.state.animation = Some(0); //power once then go to idle
                        }
                    }
                };
            }
        }
    }
}

pub fn translate_player(
    animations: Res<play::CharacterAnimations>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &mut info::Player,
        &animation_helper::AnimationHelper,
    )>,
    time: Res<Time>,
    mut target_rot: Local<Quat>,
) {
    let turn_speed: f32 = 15.0;

    for (e, children, mut t, mut p, helper) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;
        //check that the shooter's parent entity's helper entity has the same id as the animation_player entity
        for (player_ent, mut player) in &mut player {
            if helper.player_entity.id() == player_ent.id() {
                let mut direction = Vec3::default();

                // Power
                if input & POWER != 0 {
                    p.state.state = info::PlayerStateEnum::POWER;
                    break;
                }

                let mut turn_bool = false;
                // W
                if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
                    direction.z -= 1.0;
                    turn_bool = true;
                }
                // S
                if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
                    direction.z += 1.0;
                    turn_bool = true;
                }
                // A
                if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
                    direction.x -= 1.0;
                    turn_bool = true;
                }
                // D
                if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
                    direction.x += 1.0;
                    turn_bool = true;
                }

                if direction.length() > time.delta_seconds() * p.speed.speed {
                    let normalized_dir = direction.normalize();
                    t.translation += normalized_dir * p.speed.speed * time.delta_seconds();

                    // Rotation
                    let angle = normalized_dir.angle_between(Vec3::new(0.0, 0.0, 1.0));
                    *target_rot = Quat::from_rotation_y(if normalized_dir.x > 0.0 {
                        angle
                    } else {
                        -angle
                    });

                    p.state.state = info::PlayerStateEnum::MOVING;
                } else {
                    if let Some(current_target) = p.target.current_target {
                        t.translation = current_target;
                    }
                    p.state.state = info::PlayerStateEnum::IDLE;
                }
                if turn_bool { // Bool prevents synched player rotation
                    let angle_to_target = t.rotation.angle_between(*target_rot);
                    if angle_to_target > 0.0 {
                        let t2 = turn_speed / angle_to_target;
                        t.rotation = t
                            .rotation
                            .slerp(*target_rot, 1.0_f32.min(t2 * time.delta_seconds()));
                    }
                }
            }
        }
    }
}
