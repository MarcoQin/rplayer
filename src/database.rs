extern crate fs_extra;

use self::fs_extra::dir::{create, DirOptions, get_dir_content2};
use std::env;
use std::path::{Path, PathBuf};

use work_space;
pub fn hello() {
    println!("hello");
    let current_dir = work_space::get_current_work_dir();
    println!("current dir: {:?}", current_dir);
}
