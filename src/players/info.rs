#![allow(dead_code)]

use bevy::prelude::*;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Default, Component, Debug)]
pub struct Player {
    pub handle: u32,
    pub money: usize,
    pub bounties: usize,
    pub friends: HashSet<u32>,
    pub health: usize,
    pub world: usize,
}

impl Player {
    pub fn add_a_friend(&mut self, friend: u32) {
        self.friends.insert(friend);
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


