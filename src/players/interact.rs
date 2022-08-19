use crate::players::info;
use crate::ggrs_rollback::network;
use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use std::env;
use std::vec::Vec;

use super::info::Player;

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Hover! {:?}", e),
            PickingEvent::Clicked(e) => info!("Click! {:?}", e),
        }
    }
}

pub fn add_friend(
    mut events: EventReader<PickingEvent>,
    mut players: Query<(Entity, &Transform, &mut info::Player)>,
    mut me: Query<(Entity, &network::Me)>,
) {
    //i need to do this better
    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let my_handle = &args[1];

    //println!("in add_friend");
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                let mut added: bool = false;
                let mut id: u32 = 90;
                //spawn sprite bundle with transparent sprite background overlaid with text specific to player
                
                for (entity, transform, mut player) in players.iter_mut() {
                    //if entity is the clicked one and isn't me
                    if entity.id() == e.id() && entity.id() != me.single_mut().0.id(){
                        id = player.handle;
                        added = true;
                    }
                    //if entity is me
                    if entity.id() == me.single_mut().0.id() && added == true{
                        player.add_a_friend(id.to_owned());
                        added = false;
                        println!("added friend: {:?}", id);
                    }
                }
                // do again in case u passed urself before passing friend, do better
                for (entity, transform, mut player) in players.iter_mut() {
                    //if entity is the clicked one
                    if entity.id() == e.id() && entity.id() != me.single_mut().0.id(){
                        id = player.handle;
                        added = true;
                    }
                    //if entity is me
                    if entity.id() == me.single_mut().0.id() && added == true{
                        player.add_a_friend(id.to_owned());
                        added = false;
                        println!("added friend: {:?}", id);
                    }
                }
            }
            _ => info!(""),
        }
    }
}
