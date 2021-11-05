extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;

pub mod config;
pub mod shader;
pub mod types;

pub fn get_data_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("club", "gurke", "wvr") {
        Path::new(&proj_dirs.data_local_dir()).to_path_buf()
    } else {
        panic!("Can't load default directories info, this might be a platform specific issue");
    }
}

pub fn get_libs_path() -> PathBuf {
    get_data_path().join("libs")
}

pub fn get_filters_path() -> PathBuf {
    get_data_path().join("filters")
}
