use bevy::prelude::*;

use crate::ggrs_rollback::network;
use super::info;

pub fn fight(
    mut players: Query<(Entity, &Transform, &mut info::Player)>,
    mut me: Query<(Entity, &Transform, &network::Me)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (me_entity, me_transform, me_Me) = me.single_mut();
    for (entity, transform, mut player) in players.iter_mut() {
        if me_transform.translation.distance(transform.translation) < 2.5 && 
        entity.id() != me_entity.id() &&
        keyboard_input.pressed(KeyCode::Return)
        {
            if player.health >= 20 {
                player.health -= 20;
                println!("Damage");
            }
            if player.health == 0 {
                println!("Killed");
            }
            println!("{}", player.health);
        } 
    }
}