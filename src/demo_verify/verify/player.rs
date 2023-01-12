#![allow(dead_code)]

use bevy::prelude::*;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Default, Component, Debug)]
pub struct Player {
    pub state: PlayerState,
    pub target: MovementTarget,
    pub speed: MovementSpeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayerStateEnum {
    IDLE,
    MOVING,
    POWER,
    AFFECTED(u64), // Handle of player that is using their ability on you.
}

#[derive(Component, Debug)]
pub struct PlayerState {
    pub state: PlayerStateEnum,
    pub animation: Option<usize>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { 
            state: PlayerStateEnum::IDLE,
            animation: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct MovementTarget {
    pub current_target: Option<Vec3>,
}

impl Default for MovementTarget {
    fn default() -> Self {
        return Self {
           // current_target: Some(Vec3::ZERO),
           current_target: None,
        };
    }
}

// Components that should be saved/loaded need to implement the `Reflect` trait
#[derive(Component, Default, Debug)]
pub struct MovementSpeed{
    pub speed: f32,
}

