#![allow(dead_code)]

use bevy::prelude::*;
use std::hash::Hash;
use std::collections::HashSet;

use crate::worlds::world_manager;

#[derive(Default, Component, Debug)]
pub struct Player {
    pub handle: u32,
    pub money: usize,
    pub bounties: usize,
    pub friends: HashSet<u32>,
    pub health: usize,
    pub world: u32,
    pub plane: world_manager::IPlane,
    pub state: PlayerState,
    pub target: MovementTarget,
    pub speed: MovementSpeed,
    pub ability_id: u64,
    pub abilities: Vec<u64>,
}

impl Player {
    pub fn add_a_friend(&mut self, friend: u32) {
        self.friends.insert(friend);
    }
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

#[derive(Default, Component)]
pub struct Information {
    pub id: String, 
    pub bounties: u32,
    pub money: f64, 
    pub health: f64, 
}


