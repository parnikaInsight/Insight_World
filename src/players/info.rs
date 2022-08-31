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

// Components that should be saved/loaded need to implement the `Reflect` trait
#[derive(Default, Reflect, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Component)]
pub struct Information {
    pub id: String, 
    pub bounties: u32,
    pub money: f64, 
    pub health: f64, 
}


