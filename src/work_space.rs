extern crate fs_extra;

use self::fs_extra::dir::{create, DirOptions, get_dir_content2};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct WorkSpace {
    home_dir: PathBuf,
}

impl WorkSpace {
    fn init() -> Self {
        let work_dir = init_work_dir();
        Self {home_dir: work_dir}
    }
}

lazy_static! {
    static ref WORKSPACE: WorkSpace = WorkSpace::init();
}

pub fn get_current_work_dir() -> PathBuf {
    WORKSPACE.home_dir.clone()
}

fn init_work_dir() -> PathBuf {
    let work_dir;
    match env::home_dir() {
        Some(path) => {
            work_dir = path.join(".rplayer/");
        }
        None => {
            println!("Impossible to get your home dir! Use temp file instead.");
            work_dir = Path::new("/tmp/.rplayer/").to_path_buf();
        }
    }
    if !work_dir.exists() {
        println!("dir not exists! create one!");
        match create(work_dir.clone(), false) {
            Ok(_) => println!("create work dir success"),
            Err(error) => println!("create dir error: {}", error),
        }
    }
    println!("init work dir success: {}", work_dir.display());
    work_dir
}