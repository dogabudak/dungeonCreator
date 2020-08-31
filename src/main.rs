extern crate rand;
extern crate sha2;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod room;
mod level;
mod draw;
mod roomscorridors;
mod bsp;

use sha2::{ Sha256, Digest };
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use std::thread;
// use std::time::Duration;

use draw::{ draw };
use bsp::{ BspLevel };

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

enum Algorithm {
    Bsp,
}

fn main() {

    let child = thread::spawn(move || -> String {
        println!(" THREAD !! ");

        let seed: String = create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>());

        let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
        let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

        let board_width = 100;
        let board_height = 100;

        let level = BspLevel::new(board_width, board_height, &seed, &mut rng);
        println!("{}", level);

        draw(&level, ".", "level").unwrap();
        let serialised = serde_json::to_string(&level).unwrap();
        println!("{}", serialised);
        serialised
    });

    println!("Final sum result: {}", child.join().unwrap());
}
