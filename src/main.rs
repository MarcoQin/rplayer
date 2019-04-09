extern crate clap;
extern crate ctrlc;
extern crate lava_rs;
#[macro_use]
extern crate lazy_static;
extern crate rustyline;

use clap::{App, Arg, SubCommand};
use lava_rs::player;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

mod database;
mod work_space;

fn main() {
    database::hello();
    let matches = App::new("Rust Music Player")
        .version("0.1")
        .author("Marco Qin")
        .about("Nothing but just a player")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input music file to play")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("c")
                .short("c")
                .long("control")
                .multiple(false)
                .help("Set whether need to control player by command"),
        )
        .get_matches();
    let file_name = matches.value_of("INPUT").unwrap();
    println!("Get input file: {}", file_name);

    let command_parser = App::new("command")
        .version(".01")
        .subcommand(
            SubCommand::with_name("add").arg(Arg::with_name("path").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("volume").arg(Arg::with_name("volume").required(true).index(1)),
        )
        .subcommand(SubCommand::with_name("pause"));

    let use_control;

    match matches.occurrences_of("c") {
        0 => {
            use_control = false;
        }
        1 => {
            use_control = true;
        }
        _ => {
            use_control = false;
        }
    }

    player::play(file_name.to_string());
    let mut cmd: String = "".to_string();

    if !use_control {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
        while running.load(Ordering::SeqCst) {
            if player::stopping() {
                break;
            }
            thread::sleep(time::Duration::new(1, 0));
        }
    } else {
        let history_file = work_space::get_current_work_dir().join("cmd_history.txt");
        let history_file_path = history_file.to_str().unwrap();
        let mut rl = Editor::<()>::new();
        if rl.load_history(history_file_path).is_err() {
            println!("No previous history");
        }
        loop {
            if player::stopping() {
                println!("no more music to play, exit");
                break;
            }
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    println!("get command: {}", line);
                    rl.add_history_entry(line.as_ref());
                    let line = line.trim().replace("\\", "");
                    if line == "q" {
                        break;
                    }
                    let mut arg_vet = vec!["command"];
                    cmd.clone_from(&line);
                    if let Some(idx) = cmd.find(" ") {
                        let c = cmd.split_at(idx);
                        arg_vet.push(c.0);
                        arg_vet.push(c.1.trim());
                        match command_parser.clone().get_matches_from_safe(arg_vet) {
                            Ok(sub_commands) => {
                                if let Some(sub_m) = sub_commands.subcommand_matches("add") {
                                    if sub_m.is_present("path") {
                                        let path = sub_m.value_of("path").unwrap();
                                        let real_path = Path::new(path);
                                        if real_path.exists() && real_path.is_file() {
                                            player::play(path.to_string());
                                        }
                                    }
                                } else if let Some(sub_m) =
                                    sub_commands.subcommand_matches("volume")
                                {
                                    if sub_m.is_present("volume") {
                                        let volume = sub_m.value_of("volume").unwrap();
                                        match volume.parse::<i32>() {
                                            Ok(real_volume) => {
                                                let real_volume = if real_volume > 100 {
                                                    100
                                                } else {
                                                    real_volume
                                                };
                                                let real_volume =
                                                    if real_volume < 0 { 0 } else { real_volume };
                                                player::set_volume_to(real_volume);
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                println!("error {:?}", err);
                            }
                        }
                    } else {
                        arg_vet.push(cmd.as_str());
                        match command_parser.clone().get_matches_from_safe(arg_vet) {
                            Ok(sub_commands) => {
                                if let Some(_) = sub_commands.subcommand_matches("pause") {
                                    player::pause();
                                }
                            }
                            Err(err) => {
                                println!("error {:?}", err);
                            }
                        }
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
        rl.save_history(history_file_path).unwrap();
    }
}
