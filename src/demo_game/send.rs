use bevy::prelude::*;
use bevy::utils::HashMap;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use serde_json::Deserializer;
use serde::{Deserialize, Serialize};

use crate::MyMoves;

// moves: Vec<(f64, u8)>

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

// 8 move to win
// Last 3 moves meet target difficulty = 51234
pub fn test_hash() {
    let moves: Vec<(f64, u8)> = vec![(0.0,0),(84.0,2),(154.0,0),(186.0,8),(212.0,0),(227.0,8),(239.0,0),(270.0,8)]; 
    let mut input = String::new();
    let moves_len = moves.len();
    let mut h = String::new();
    let new_moves: Vec<(f64, u8)> = moves.into_iter().rev().collect(); 
    for m in new_moves {
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
        if hex_hash <= String::from("51234") {
            h.push_str(&hex_hash[..]);
            println!("Met target difficulty! {}", hex_hash); // Last 3 moves hashed to target difficulty
            println!("moves len: {}", moves_len);
            break;
        }
    }
}