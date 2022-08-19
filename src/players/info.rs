use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider, SessionType};
use bevy_pbr::PbrBundle;
use bevy_pbr::PointLightBundle;
use bevy_pbr::StandardMaterial;
use bevy_render::color::Color;
use bevy_render::mesh::shape;
use bevy_render::mesh::Mesh;
use bytemuck::{Pod, Zeroable};
use ggrs::{
    Config, InputStatus, P2PSession, PlayerHandle, PlayerType, SessionBuilder, SpectatorSession,
    SyncTestSession, UdpNonBlockingSocket,
};
use bevy_rapier3d::prelude::*;
use std::env;
use std::{hash::Hash, net::SocketAddr};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Default, Component, Debug)]
pub struct Player {
    pub handle: u32,
    pub money: usize,
    pub bounties: usize,
    pub friends: HashSet<u32>,
    pub health: usize,
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

// You can also register resources. If your Component / Resource implements Hash, you can make use of `#[reflect(Hash)]`
// in order to allow a GGRS `SyncTestSession` to construct a checksum for a world snapshot
#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

