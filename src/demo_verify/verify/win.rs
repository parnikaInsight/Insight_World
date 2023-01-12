use base16ct;
use bevy::ecs::bundle;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier3d::prelude::*;
use rand::seq::index;
use sha2::{Digest, Sha256};
use std::{thread, time::Duration};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use serde_json::Deserializer;
use serde::{Deserialize, Serialize};

use crate::{MyMoves, WinningMoves};

use super::player;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component, Default)]
pub struct WinText;

// A unit struct to help identify the color-changing Text component
#[derive(Component, Default)]
pub struct HashText;

pub fn display_hash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hash: String,
    choose: usize,
    moves: Vec<(f64, u8)>,
) {
    // Won and met target difficulty
    if choose == 0 {
        // Send moves and hash to script
        save_moves(moves);
        
        // Display mining message
        let win_entity_id = commands
            .spawn_bundle(
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(" You won! Choose content to mine! \n Hash: {}", hash),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                        //..Default::default()
                    },
                )
                // Set the alignment of the Text
                .with_text_alignment(TextAlignment::BOTTOM_LEFT),
            )
            .id();
        commands.entity(win_entity_id).insert(WinText::default());
    } 
    // Display try again message
    else {
        let hash_entity_id = commands
            .spawn_bundle(
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    " Objective completed! Try again to mine!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                        //..Default::default()
                    },
                )
                // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_LEFT),
            )
            .id();
        commands.entity(hash_entity_id).insert(HashText::default());
    }
}

pub fn hash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    moves: Vec<(f64, u8)>,
    difficulty: String,
) {
    let mut input = String::new();
    let moves_len = moves.len();
    let mut choose: usize = 1;
    let mut h = String::new();
    for m in moves.clone() {
        // create a Sha256 object
        let mut hasher = Sha256::new();

        let (frame, move1) = m;
        let string_convert = format!("{}{} ", frame, move1);
        input.push_str(&string_convert[..]);

        // write input message
        hasher.update(input.clone());

        // read hash digest and consume hasher
        let result = hasher.finalize();
        //println!("hash: {:?}", result);

        let hex_hash = base16ct::lower::encode_string(&result);
        println!("Hex-encoded hash: {}", hex_hash);

        // If meet target difficulty:
        //      1.) display message "Objective completed and difficulty met! Add content to mine!"
        //      2.) send moves to script
        if hex_hash <= difficulty {
            h.push_str(&hex_hash[..]);
            choose = 0;
            println!("Met target difficulty!");
            println!("moves len: {}", moves_len);
            break;
        }
    }
    display_hash(commands, asset_server, h, choose, moves.clone().into_iter().rev().collect());
}

pub fn objective_completion(
    mut query: Query<(&mut Transform, &mut player::Player)>,
    mut my_moves: ResMut<MyMoves>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alr_won: Local<bool>,
    mut alr_won2: Local<bool>,
    mut winning_moves: ResMut<WinningMoves>,
) {
    let m_moves = my_moves.moves.clone();
    // Win Condition: Knight collides with mutant
    let win_bool = objective_win(query, alr_won, my_moves, winning_moves);

    // Hash moves (start from 1 move till completion, then 2 moves ... 50 moves) and display hashes
    let rev_moves: Vec<(f64, u8)> = m_moves.into_iter().rev().collect();
    if win_bool && !*alr_won2 {
        *alr_won2 = true;
        println!("here");
        hash(commands, asset_server, rev_moves, String::from("51234"));
    }
}

pub fn objective_win(
    mut query: Query<(&mut Transform, &mut player::Player)>,
    mut alr_won: Local<bool>,
    my_moves: ResMut<MyMoves>,
    mut winning_moves: ResMut<WinningMoves>,
) -> bool {
    let (transform, player) = query.single_mut();

    // Win Condition: Knight collides with mutant
    let mutant_transform = Transform::from_xyz(5.0, 0.0, 0.0)
        .with_rotation(Quat::from_rotation_y((270.0_f32).to_radians()));
    if Vec3::abs_diff_eq(transform.translation, mutant_transform.translation, 1.5) && !*alr_won {
        winning_moves.moves = my_moves.moves.clone();
        *alr_won = true;
        println!("IN CONTACT");
        return true;
    }
    false
}

pub fn save_moves(
    moves: Vec<(f64, u8)>,
) {
    let world_name = "demo_moves"; // Hash of World/ Name given by creator
    let pathname = format!("{}{}{}", "./assets/worlds/", world_name, ".txt");
    let path = Path::new(&pathname);
    let mut file = fs::File::create(path).unwrap();
    let j = serde_json::to_string(&moves).unwrap();
    fs::write(pathname, j);
}