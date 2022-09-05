use bevy::prelude::*;
use std::time::Duration;

use crate::animation::play;
use crate::players::info;
use crate::systems::framework;

// Abilites stored in kademlia by nodeid

// Created by player.
// All abilites have these 7 fields. Each ability is its own struct so it can have different 
// implementations of movement. Multiple players can use this ability, just change handle.
pub struct GirlAbility { 
    //id: u64, // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by player.
impl framework::Movement for GirlAbility {
    fn movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
    ) {
        match p.state.state {
            // Player can customize their idle animation.
            info::PlayerStateEnum::IDLE => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 0 {
                    player.play(animations.0[0].clone_weak());
                    p.state.animation = Some(0);
                }
            }
            // Player can customize their moving animation.
            info::PlayerStateEnum::MOVING => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 1 {
                    player
                        .cross_fade(animations.0[1].clone_weak(), Duration::from_secs_f32(0.25))
                        .set_speed(1.3)
                        .repeat();
                    p.state.animation = Some(1);
                }
            }
            // Power
            // Player describes their own animation and how others are affected.
            info::PlayerStateEnum::POWER => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 2 {
                    // Generate ability_id

                    // Animate me
                    player
                        .cross_fade(animations.0[2].clone_weak(), Duration::from_secs_f32(0.25))
                        .set_speed(1.3);
                    p.state.animation = Some(0); //power once then go to idle

                    // Animate others
                    // if player in radius,
                    // change their state to AFFECTED(my_handle, ability_id)
                    // make them fall
                    // put ability into kademlia
                }
            }
            // Power
            // Specifies how player reacts to an ability being used on them by the
            // player specified by the handle.
            // TODO: Should player be able to change how they're affected as a power?
            // TODO: Do you need (handle arg) to know whose ability is affecting you?
            info::PlayerStateEnum::AFFECTED(ability_id) => {
                // request ability from kademlia
            }
        };
    }
}

pub struct NinjaAbility { 
    // id: u64,     // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by player.
impl framework::Movement for NinjaAbility {
    fn movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
    ) {
        match p.state.state {
            // Player can customize their idle animation.
            info::PlayerStateEnum::IDLE => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 0 {
                    player.play(animations.0[0].clone_weak());
                    p.state.animation = Some(0);
                }
            }
            // Player can customize their moving animation.
            info::PlayerStateEnum::MOVING => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 1 {
                    player
                        .cross_fade(animations.0[1].clone_weak(), Duration::from_secs_f32(0.25))
                        .set_speed(1.3)
                        .repeat();
                    p.state.animation = Some(1);
                }
            }
            // Power
            // Player describes their own animation and how others are affected.
            info::PlayerStateEnum::POWER => {
                if p.state.animation.is_none() || p.state.animation.unwrap() != 2 {
                    // Generate ability_id

                    // Animate me
                    player
                        .cross_fade(animations.0[2].clone_weak(), Duration::from_secs_f32(0.25))
                        .set_speed(1.3);
                    p.state.animation = Some(0); //power once then go to idle

                    // Animate others
                    // if player in radius,
                    // change their state to AFFECTED(my_handle, ability_id)
                    // make them fall
                    // put ability into kademlia
                }
            }
            // Power
            // Specifies how player reacts to an ability being used on them by the
            // player specified by the handle.
            // TODO: Should player be able to change how they're affected as a power?
            info::PlayerStateEnum::AFFECTED(ability_id) => {}
        };
    }
}