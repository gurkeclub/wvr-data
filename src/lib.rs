extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;

pub mod config;
pub mod shader;

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq)]
pub enum DataRange {
    IntRange(i64, i64, i64),
    FloatRange(f64, f64, f64),
    ColorRange,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataHolder {
    Float(f32),
    Float2([f32; 2]),
    Float3([f32; 3]),
    Float4([f32; 4]),
    FloatArray(Vec<f32>),

    Int(i32),
    Int2([i32; 2]),
    Int3([i32; 3]),
    Int4([i32; 4]),
    IntArray(Vec<i32>),

    Mat2([[f32; 2]; 2]),
    Mat3([[f32; 3]; 3]),
    Mat4([[f32; 4]; 4]),

    Bool(bool),
    BoolArray(Vec<bool>),

    ByteArray(Vec<u8>),

    Texture(((u32, u32), Vec<u8>)),
}

pub struct Buffer {
    pub dimensions: Vec<usize>,
    pub data: Option<Vec<u8>>,
}

pub trait InputProvider {
    fn set_name(&mut self, name: &str);
    fn provides(&self) -> Vec<String>;
    fn get(&mut self, uniform_name: &str, invalidate: bool) -> Option<DataHolder>;

    fn set_property(&mut self, property: &str, value: &DataHolder);
    fn set_beat(&mut self, _bpm: f64, _sync: bool) {}
    fn set_time(&mut self, _time: f64, _sync: bool) {}
    fn stop(&mut self) {}
}

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
