use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::animation::{animation_helper, play};
use crate::ggrs_rollback::network;
use crate::players::info;
use crate::systems::framework;

// Created by player.
// All abilites have these 7 fields. Each ability is its own struct so it can have different
// implementations of movement. Multiple players can use this ability, just change handle.

// User animation: flip; Controlled animation: dance
pub struct Dance_Control_Ability {
    //id: u64, // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by power creator.
impl framework::Power for Dance_Control_Ability {
    fn my_movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Player describes their own animation when using the power.

        // Animate me
        player
            .cross_fade(animations.0[2].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power once then go to idle

        // Put ability into kademlia. Abilites stored in kademlia by nodeid.
    }

    // Implemented by power creator.
    fn effect(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Specifies how a player reacts to this ability being used on them.
        // Program how affected players should be animated.

        player
            .cross_fade(animations.0[3].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power effect once, then go to idle
        p.state.state = info::PlayerStateEnum::IDLE;
    }
}

// User animation: punch; Controlled animation: Translated to the right.
pub struct Punch_Ability {
    //id: u64, // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by power creator.
impl framework::Power for Punch_Ability {
    fn my_movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Player describes their own animation when using the power.

        // Animate me
        player
            .cross_fade(animations.0[4].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power once then go to idle

        // Put ability into kademlia. Abilites stored in kademlia by nodeid.
    }

    // Implemented by power creator.
    fn effect(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Specifies how a player reacts to this ability being used on them.
        // Program how affected players should be animated.

        transform.translation.x += 1.0;
        println!("translated");

        player
            .cross_fade(animations.0[5].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power effect once, then go to idle
        p.state.state = info::PlayerStateEnum::IDLE;
    }
}

// User animation: jump attack; Controlled animation: Health decreased by 10.
pub struct Damage_Ability {
    //id: u64, // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by power creator.
impl framework::Power for Damage_Ability {
    fn my_movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Player describes their own animation when using the power.

        // Animate me
        player
            .cross_fade(animations.0[6].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power once then go to idle

        // Put ability into kademlia. Abilites stored in kademlia by nodeid.
    }

    // Implemented by power creator.
    fn effect(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Specifies how a player reacts to this ability being used on them.
        // Program how affected players should be animated.

        if p.health > 10 {
            p.health -= 10;
            println!("health: {}", p.health);
        }

        player
            .cross_fade(animations.0[7].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power effect once, then go to idle
        p.state.state = info::PlayerStateEnum::IDLE;
    }
}

// User animation: two hands cast spelling; Controlled animation: cubes spawned.
// TODO: Weapons and spawned objects can have their own powers. Ex. if the player touches the cubes,
// they lose health.
pub struct Spawn_Cube_Ability {
    //id: u64, // Ability identifier.
    // pub handle: u32, // Handle of player using this ability.
    // effect: framework::Effect,
    // medium: framework::Medium,
    // power_type: framework::PowerType,
    // affected: framework::Affected,
    // tier: framework::Tier,
}

// Implemented by power creator.
impl framework::Power for Spawn_Cube_Ability {
    fn my_movement(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Player describes their own animation when using the power.

        // Animate me
        player
            .cross_fade(animations.0[8].clone_weak(), Duration::from_secs_f32(0.25))
            .set_speed(1.3);
        p.state.animation = Some(0); //power once then go to idle

        // Put ability into kademlia. Abilites stored in kademlia by nodeid.
    }

    // Implemented by power creator.
    fn effect(
        &self,
        mut p: &mut info::Player,
        mut player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
        transform: &mut Transform,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        animations_resource: &mut ResMut<Assets<AnimationClip>>,
    ) {
        // Specifies how a player reacts to this ability being used on them.
        // Program how affected players should be animated.

        // Spawn dangerous, animated cubes near affected player.

        // The animation API uses the `Name` component to target entities
        let danger_cube = Name::new("cube");

        // Creating the animation
        let mut animation = AnimationClip::default();
        // A curve can modify a single part of a transform, here the translation
        animation.add_curve_to_path(
            EntityPath {
                parts: vec![danger_cube.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
                keyframes: Keyframes::Translation(vec![
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(-1.0, 0.0, 1.0),
                    Vec3::new(-1.0, 0.0, -1.0),
                    Vec3::new(1.0, 0.0, -1.0),
                    // in case seamless looping is wanted, the last keyframe should
                    // be the same as the first one
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
            },
        );

        // Create the animation player, and set it to repeat
        let mut player = AnimationPlayer::default();
        player.play(animations_resource.add(animation)).repeat();

        let mut cube_pos = transform.clone();
        cube_pos.translation.z += 3.0;
        cube_pos.translation.x += 1.0;
        cube_pos.translation.y += 1.0;
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: cube_pos,
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            // .insert(Collider::cuboid(0.5, 0.5, 0.5)) //half the cube size
            // .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)))
            // Add the Name component, and the animation player
            .insert_bundle((danger_cube, player));

        // While Bob uses ability, if this player Alice touches one of Bob's cubes, then Alice falls.
        // TODO: This does not work as intended because the animated cube has a persistent position 
        // at where it was spawned, not where it is in the world during the animation.
        if Vec3::abs_diff_eq(transform.translation, cube_pos.translation, 1.0) {
            p.health -= 1;
            println!("Hit by danger cube!: -10 health");
        }
    }
}
