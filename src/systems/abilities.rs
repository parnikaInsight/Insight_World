use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, PlayerHandle};

use crate::animation::{animation_helper, play};
use crate::players::{info, movement};

pub trait Movement {
    pub fn movement();
}

// Created by player.
pub struct ExampleAbility {
    id: u64, // Ability identifier.
    handle: u32, // Handle of player using this ability.
    effect: Effect,
    medium: Medium,
    power_type: PowerType,
    affected: Affected,
    tier: Tier,
}

// Implemented by player.
impl Movement for ExampleAbility { 
    fn movement(
        animations: Res<play::CharacterAnimations>,
        // assets: Res<Assets<AnimationClip>>,
        mut player: Query<(Entity, &mut AnimationPlayer)>,
        inputs: Res<Vec<(movement::BoxInput, InputStatus)>>,
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
    
            //check that the shooter's parent entity's helper entity has the same id as the 
            // animation_player entity
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
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
                                    .cross_fade(
                                        animations.0[1].clone_weak(),
                                        Duration::from_secs_f32(0.25),
                                    )
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
                                    .cross_fade(
                                        animations.0[2].clone_weak(),
                                        Duration::from_secs_f32(0.25),
                                    )
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
                        info::PlayerStateEnum::AFFECTED(handle, ability_id) => {}
                    };
                }
            }
        }
    }
}

pub enum Tier {
    Basic, // High # clicks, low impact_radius, low impact_extent.
    Intermediate, // High # clicks, high impact_radius, low impact_extent.
    Advanced, // Low # clicks, low impact_radius, high impact_extent.
    God // Low # clicks, high impact_radius, high impact_extent.
}

pub enum Effect {
    Positive, 
    Negative
}

pub enum PowerType {
    HealthManip, // Damage or increase health
    TranformManip, // Change transform 
    AnimationManip, // Change animation
    Object, // Spawn object
}

pub enum Medium {
    Guesture,
    Weapon
}

pub enum Affected {
    Me,
    Other
}