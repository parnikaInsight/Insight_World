use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AnimationHelperSetup; // Marker for parent with animation player child.

#[derive(Component)]
pub struct AnimationHelper {
    // Contains reference to specific animation player.
    pub player_entity: Entity,
}

impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper { player_entity }
    }
}

pub fn setup_helpers(
    // Finds all AnimationHelperSetup markers.
    // Recursively looks through their children until animation player found.
    mut commands: Commands,
    to_setup: Query<Entity, With<AnimationHelperSetup>>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
) {
    for host_entity in to_setup.iter() {
        if let Some(animation_player) =
            find_animation_player_entity(host_entity, &children, &players)
        {
            commands
                .entity(host_entity)
                // This is how to find the animation player later 
                .insert(AnimationHelper::new(animation_player)); 
        }
    }
}

fn find_animation_player_entity(
    parent: Entity,
    children: &Query<&Children>,
    players: &Query<&AnimationPlayer>,
) -> Option<Entity> {
    if let Ok(candidates) = children.get(parent) {
        // Collect all children.
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                // Return child if it is the entity with an AnimationPlayer component.
                if players.get(candidate).is_ok() {
                    return Some(candidate);
                } 
                // Else recursively get children and add to candidates list.
                else if let Ok(new) = children.get(candidate) {
                    next_candidates.extend(new.iter());
                }
            }
        }
    }
    None
}