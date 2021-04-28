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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum LfoType {
    Square,
    Triangle,
    Saw,
    Sine,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Automation {
    Lfo(LfoType, f64, f64, f64, f64, bool),
    None,
}

impl Automation {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }
    pub fn apply(&self, value: &DataHolder, beat: f64) -> Option<DataHolder> {
        if self == &Self::None {
            return None;
        }
        let offset = match *self {
            Self::Lfo(lfo_type, numerator, denominator, phase, amplitude, signed) => {
                let beat_cursor = (beat * numerator / denominator + phase).fract();

                let offset = match lfo_type {
                    LfoType::Square => {
                        if beat_cursor >= 0.5 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    LfoType::Triangle => 1.0 - (beat_cursor * 2.0 - 1.0).abs(),
                    LfoType::Saw => beat_cursor,
                    LfoType::Sine => (beat_cursor * 2.0 * std::f64::consts::PI).sin() * 0.5 + 0.5,
                };

                amplitude * if signed { offset * 2.0 - 1.0 } else { offset }
            }
            _ => unreachable!(),
        };

        match *value {
            DataHolder::Bool(bool_value) => Some(DataHolder::Bool(
                (bool_value || offset > 0.5) && offset > 0.0,
            )),
            DataHolder::Float(float_value) => Some(DataHolder::Float(float_value + offset as f32)),
            DataHolder::Float2(float2_value) => Some(DataHolder::Float2([
                float2_value[0] + offset as f32,
                float2_value[1] + offset as f32,
            ])),
            DataHolder::Float3(float3_value) => Some(DataHolder::Float3([
                float3_value[0] + offset as f32,
                float3_value[1] + offset as f32,
                float3_value[2] + offset as f32,
            ])),
            DataHolder::Float4(float4_value) => Some(DataHolder::Float4([
                float4_value[0] + offset as f32,
                float4_value[1] + offset as f32,
                float4_value[2] + offset as f32,
                float4_value[3] + offset as f32,
            ])),
            DataHolder::Int(int_value) => Some(DataHolder::Int(int_value + offset as i32)),
            DataHolder::Int2(int2_value) => Some(DataHolder::Int2([
                int2_value[0] + offset as i32,
                int2_value[1] + offset as i32,
            ])),
            DataHolder::Int3(int3_value) => Some(DataHolder::Int3([
                int3_value[0] + offset as i32,
                int3_value[1] + offset as i32,
                int3_value[2] + offset as i32,
            ])),
            DataHolder::Int4(int4_value) => Some(DataHolder::Int4([
                int4_value[0] + offset as i32,
                int4_value[1] + offset as i32,
                int4_value[2] + offset as i32,
                int4_value[3] + offset as i32,
            ])),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Speed {
    Fps(f32),
    Fpb(f32),
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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
    pub variables: HashMap<String, (DataHolder, Automation)>,
    #[serde(default)]
    pub precision: BufferPrecision,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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
