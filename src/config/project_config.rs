use std::collections::hash_map::HashMap;
use std::path::PathBuf;

use crate::config::server_config::ServerConfig;
use crate::DataHolder;
use crate::DataRange;

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
    pub locked_speed: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq)]
pub enum Speed {
    Fps(f32),
    Beats(f32),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum InputConfig {
    Video {
        path: String,
        width: usize,
        height: usize,
        speed: Speed,
    },
    Picture {
        path: String,
        width: usize,
        height: usize,
    },
    Cam {
        path: String,
        width: usize,
        height: usize,
    },
    Midi {
        name: String,
    },
}

impl InputConfig {
    pub fn is_video(&self) -> bool {
        matches!(self, InputConfig::Video { .. })
    }

    pub fn is_picture(&self) -> bool {
        matches!(self, InputConfig::Picture { .. })
    }

    pub fn is_cam(&self) -> bool {
        matches!(self, InputConfig::Cam { .. })
    }

    pub fn is_midi(&self) -> bool {
        matches!(self, InputConfig::Midi { .. })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BufferPrecision {
    U8,
    F16,
    F32,
}

impl Default for BufferPrecision {
    fn default() -> Self {
        Self::U8
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SampledInput {
    Nearest(String),
    Linear(String),
    Mipmaps(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RenderStageConfig {
    pub name: String,
    pub filter: String,
    pub filter_mode_params: FilterMode,
    pub inputs: HashMap<String, SampledInput>,
    pub variables: HashMap<String, DataHolder>,
    #[serde(default)]
    pub precision: BufferPrecision,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FilterMode {
    Rectangle(f32, f32, f32, f32),
    Particles(usize),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FilterConfig {
    pub mode: FilterMode,
    pub inputs: Vec<String>,
    pub vertex_shader: Vec<String>,
    pub fragment_shader: Vec<String>,
    pub variables: HashMap<String, (DataHolder, DataRange)>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub bpm: f32,
    pub view: ViewConfig,
    pub server: ServerConfig,
    pub inputs: HashMap<String, InputConfig>,
    pub render_chain: Vec<RenderStageConfig>,
    pub final_stage: RenderStageConfig,
}
