extern crate clap;
extern crate lava_rs;

use clap::{App, Arg};
use lava_rs::player;
use std::thread;
use std::time;

fn main() {
    let matches = App::new("Rust Music Player")
        .version("0.1")
        .author("Marco Qin")
        .about("Nothing but just a player")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input music file to play")
            .required(true)
            .index(1))
        .get_matches();
    let file_name = matches.value_of("INPUT").unwrap();
    println!("Get input file: {}", file_name);
    player::init_player();
    player::load_file(file_name.to_string());
    loop {
        if player::is_stopping() {
            break;
        }
        thread::sleep(time::Duration::new(1, 0));
    }
}
