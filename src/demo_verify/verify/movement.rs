use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, PlayerHandle};
use std::time::Duration;

use super::{animation_helper, play, player};
use super::recreate::{get_moves, Me};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const POWER: u8 = 1 << 4;

// Translation and animation given inputs
pub fn translate(
    frame_count: Res<FrameTimeDiagnosticsState>,
    mut ani_players: Query<(Entity, &mut AnimationPlayer)>,
    mut query: Query<(Entity, &mut Transform, &animation_helper::AnimationHelper, &mut player::Player,)>, //With<Me>>,
    time: Res<Time>,
    mut target_rot: Local<Quat>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
    animations: Res<play::CharacterAnimations>,
    mut animations_resource: ResMut<Assets<AnimationClip>>,
) {
    let moves = get_moves();
    let input_len = moves.len();
    let turn_speed: f32 = 15.0;
    let res = query.get_single_mut();
    match res {
        Ok((e, mut t, anim, mut p)) => {
            for (player_ent, mut player) in &mut ani_players {
                //AnimationPlayer
                if anim.player_entity.id() == player_ent.id() {
                    let current_frame = frame_count.frame_count;
                    let mut prev_input = 0;
                    // let mut count = 0;
                    // let mut last_frame = 0.0;
                    for (frame, input) in moves.iter() {
                        // count += 1;
                        // if count == input_len {
                        //     last_frame = *frame;
                        // }
                        println!("frame: {}", *frame);
                        //t.translation += Vec3::new(1.0, 0.0, 0.0);
                        //println!("move");
                        if *frame <= current_frame {
                            prev_input = *input;
                        } else {
                            break;
                        }
                    }
                    if let Some((last_frame, last_input)) = moves.last().copied() {
                        if current_frame <= last_frame {
                            let mut direction = Vec3::default();

                            // Power
                            if prev_input & POWER != 0 {
                                p.state.state = player::PlayerStateEnum::POWER;
                                // //power animation
                                // let girl_ability = Sword_Ability {};
                                // girl_ability.my_movement(&mut player, animations.clone(), &mut t, &mut commands,  &mut meshes, &mut materials, &mut animations_resource, &mut asset_server);
                       
                            }

                            // W
                            if prev_input & INPUT_UP != 0 && prev_input & INPUT_DOWN == 0 {
                                direction.z -= 1.0;
                            }
                            // S
                            if prev_input & INPUT_UP == 0 && prev_input & INPUT_DOWN != 0 {
                                direction.z += 1.0;
                            }
                            // A
                            if prev_input & INPUT_LEFT != 0 && prev_input & INPUT_RIGHT == 0 {
                                direction.x -= 1.0;
                            }
                            // D
                            if prev_input & INPUT_LEFT == 0 && prev_input & INPUT_RIGHT != 0 {
                                direction.x += 1.0;
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

                                p.state.state = player::PlayerStateEnum::MOVING;
                                // player
                                // .cross_fade(
                                //     animations.0[12].clone_weak(),
                                //     Duration::from_secs_f32(0.25),
                                // )
                                // .set_speed(1.3)
                                // .repeat();
                            }
                            else {
                               // player.play(animations.0[0].clone_weak());
                                //     if let Some(current_target) = None {
                                //         t.translation = current_target;
                                //     }
                                p.state.state = player::PlayerStateEnum::IDLE;
                            }

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
        _ => {}
    }
}

pub fn animate_moving_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
    animations: Res<play::CharacterAnimations>,
    mut ani_players: Query<(Entity, &mut AnimationPlayer)>,
    mut query: Query<(
        Entity,
        &Children,
        &mut Transform,
        &mut player::Player,
        &animation_helper::AnimationHelper,
    )>,
    mut animations_resource: ResMut<Assets<AnimationClip>>,
) {
    for (e, children, mut t, mut p, helper) in query.iter_mut() {
        //check that the shooter's parent entity's helper entity has the same id as the animation_player entity
        for (player_ent, mut player) in &mut ani_players {
            //AnimationPlayer
            if helper.player_entity.id() == player_ent.id() {
                // ability_id and abilities changed during game
                //insert systems::abilities::movement for the ability_id currently in use by player p, else use default movement

                match p.state.state {
                    // How my player moves when idle. (TODO: This can be customized.)
                    player::PlayerStateEnum::IDLE => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 0 {
                            player.play(animations.0[0].clone_weak());
                            p.state.animation = Some(0);
                        }
                    }
                    // How my player moves when moving. (TODO: This can be customized.)
                    player::PlayerStateEnum::MOVING => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 1 {
                            player
                                .cross_fade(
                                    animations.0[12].clone_weak(),
                                    Duration::from_secs_f32(0.25),
                                )
                                .set_speed(1.3)
                                .repeat();
                            p.state.animation = Some(1);
                        }
                    }
                    // How my player moves when using a specified power.
                    // Get ability in use by my player and change my movement accordingly.
                    // Change status of players in my radius to AFFECTED.
                    // Else, use default ability. (TODO: Shift this code to a default ability specified in systems::abilities)
                    player::PlayerStateEnum::POWER => {
                        if p.state.animation.is_none() || p.state.animation.unwrap() != 2 {
                            // Change my player's movement according to p.ability_id 
                           // let girl_ability = abilities::Sword_Ability {};
                            let girl_ability = Sword_Ability {};
                            girl_ability.my_movement(&mut p, &mut player, animations.clone(), &mut t, &mut commands,  &mut meshes, &mut materials, &mut animations_resource, &mut asset_server);
                        }
                    }
                    // How my player reacts to having the specified power used on them.
                    player::PlayerStateEnum::AFFECTED(ability_id) => {
                        // Request ability from kademlia by ability_id.
                        // Animate player according to "effect" implementation of that power.
                        // A player cannot change how they're affected as a power, but they can create a power to counter.
                        // TODO: Do you need (handle arg) to know whose ability is affecting you?

                        let girl_effect = Sword_Ability {};
                        girl_effect.effect(&mut p, &mut player, animations.clone(), &mut t, &mut commands, &mut meshes, &mut materials, &mut animations_resource, &mut asset_server);
                    }
                };
            }
        }
    }
}

// Get locations of all entities after win and perform checksum

// Frame
pub struct FrameTimeDiagnosticsState {
    pub frame_count: f64,
}

pub fn inc_frame(mut frame: ResMut<FrameTimeDiagnosticsState>) {
    frame.frame_count += 1.0;
}


// Sword ability

pub trait Power {
    fn my_movement(
        &self,
        p: &mut player::Player,
        player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands, 
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
        asset_server: &mut Res<AssetServer>,
    );

    fn effect(
        &self,
        p: &mut player::Player,
        player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands, 
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
        asset_server: &mut Res<AssetServer>,
    );
}

pub struct Sword_Ability {
}
// Implemented by power creator.
impl Power for Sword_Ability {
    fn my_movement(
        &self,
        mut p: &mut player::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        // Player describes their own animation when using the power.

        // Animate me
        player
            .cross_fade(animations.0[14].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        //p.state.animation = Some(0); //power once then go to idle

        // Put ability into kademlia. Abilites stored in kademlia by nodeid.
    }

    // Implemented by power creator.
    fn effect(
        &self,
        mut p: &mut player::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        // Specifies how a player reacts to this ability being used on them.
        // Program how affected players should be animated.

        player
            .cross_fade(animations.0[3].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        // p.state.animation = Some(0); //power effect once, then go to idle
        // p.state.state = info::PlayerStateEnum::IDLE;
    }
}