extern crate clap;
extern crate ctrlc;
extern crate lava_rs;
extern crate rustyline;

use clap::{App, Arg};
use lava_rs::player;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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
        .arg(Arg::with_name("c")
            .short("c")
            .long("control")
            .multiple(false)
            .help("Set whether need to control player by command"))
        .get_matches();
    let file_name = matches.value_of("INPUT").unwrap();
    println!("Get input file: {}", file_name);

    let use_control;

    match matches.occurrences_of("c") {
        0 => { use_control = false; }
        1 => { use_control = true; }
        _ => { use_control = false; }
    }

    player::init_player();
    player::load_file(file_name.to_string());

    if !use_control {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
        while running.load(Ordering::SeqCst) {
            if player::is_stopping() {
                break;
            }
            thread::sleep(time::Duration::new(1, 0));
        }
    } else {
        let mut rl = Editor::<()>::new();
        loop {
            if player::is_stopping() {
                println!("no more music to play, exit");
                break;
            }
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    println!("get command: {}", line);
                    if line == "q" {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("EOF");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
