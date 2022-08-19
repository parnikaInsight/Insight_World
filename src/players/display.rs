use crate::{players::info};
use bevy::prelude::*;
//use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_picking::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};
use std::{
    marker::{PhantomData, PhantomPinned},
    thread, time,
};

#[derive(Component)]
pub struct UICamera;

/// Spawn the UI camera
pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component)]
pub struct InfoDisplay;

pub fn click_for_display(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    asset_server: Res<AssetServer>,
    players: Query<(Entity, &Transform, &mut info::Player)>,
    mut query: Query<(Entity, With<InfoDisplay>)>, //there should only be one info display at a time
) {
    let sprite_handle: Handle<Image> = asset_server.load("branding/icon.png");

    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                //spawn sprite bundle with transparent sprite background overlaid with text specific to player
                if matches!(e, HoverEvent::JustEntered(_)) {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    let text_style = TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::WHITE,
                    };
                    let text_alignment = TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    };

                    if let HoverEvent::JustEntered(player) = e {
                        for i in players.iter() {
                            if i.0.id() == player.id() {
                                let id: String = i.2.handle.to_string();
                                let money = i.2.money.to_string();
                                let bounties = i.2.bounties.to_string();
                                let health = i.2.health.to_string();
                                commands
                                    .spawn_bundle(Text2dBundle {
                                        text: Text::from_section(
                                            String::from("Id: ") + &*id + 
                                                &*String::from("\nHealth: ") + &*health +
                                                &*String::from("\nMoney: $") + &*money +
                                                &*String::from("\nBounties: ") + &*bounties ,
                                            text_style.clone(),
                                        ).with_alignment(text_alignment),
                                        ..default()
                                    })
                                    .insert(InfoDisplay);
                            }
                        }
                    }
                    println!("spawning sprite");
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.0, 0.0, 1.0, 0.7),
                                ..default()
                            },
                            texture: sprite_handle.clone(),
                            ..default()
                        })
                        .insert(InfoDisplay);
                } else {
                    //despawn or make invisible
                    for q in query.iter() {
                        commands.entity(q.0).despawn();
                    }
                }
            }
            _ => info!("nothing"),
        }
    }
}

