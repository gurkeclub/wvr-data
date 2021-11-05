use std::collections::hash_map::HashMap;
use std::path::PathBuf;

use crate::config::server::ServerConfig;

use crate::types::{Automation, DataHolder};

use super::input::InputConfig;
use super::rendering::RenderStageConfig;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ViewConfig {
    pub width: i64,
    pub height: i64,
    pub fullscreen: bool,
    pub target_fps: f32,
    pub dynamic: bool,
    pub vsync: bool,
    pub screenshot_path: PathBuf,
    pub screenshot: bool,
    pub screenshot_frame_count: i64,
    pub locked_speed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub bpm: f32,
    pub view: ViewConfig,
    pub server: ServerConfig,
    pub variables: HashMap<String, (DataHolder, Automation)>,
    pub inputs: HashMap<String, InputConfig>,
    pub render_chain: Vec<RenderStageConfig>,
    pub final_stage: RenderStageConfig,
}
