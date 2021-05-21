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
    pub screenshot_frame_count: i64,
    pub locked_speed: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Lfo {
    Square(f64, f64, f64, f64, bool),
    Triangle(f64, f64, f64, f64, bool),
    Saw(f64, f64, f64, f64, bool),
    Sine(f64, f64, f64, f64, bool),
}

impl Lfo {
    pub fn get_amplitude(&self, beat: f64) -> f64 {
        match *self {
            Self::Square(numerator, denominator, phase, amplitude, signed) => {
                let beat_cursor = (beat * numerator / denominator + phase).fract();
                let value = if beat_cursor >= 0.5 { 1.0 } else { 0.0 };

                amplitude * if signed { value * 2.0 - 1.0 } else { value }
            }
            Self::Triangle(numerator, denominator, phase, amplitude, signed) => {
                let beat_cursor = (beat * numerator / denominator + phase).fract();
                let value = 1.0 - (beat_cursor * 2.0 - 1.0).abs();

                amplitude * if signed { value * 2.0 - 1.0 } else { value }
            }
            Self::Saw(numerator, denominator, phase, amplitude, signed) => {
                let beat_cursor = (beat * numerator / denominator + phase).fract();
                let value = beat_cursor;

                amplitude * if signed { value * 2.0 - 1.0 } else { value }
            }
            Self::Sine(numerator, denominator, phase, amplitude, signed) => {
                let beat_cursor = (beat * numerator / denominator + phase).fract();
                let value = (beat_cursor * 2.0 * std::f64::consts::PI).sin() * 0.5 + 0.5;

                amplitude * if signed { value * 2.0 - 1.0 } else { value }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Automation {
    Lfo(Lfo),
    Lfo2d(Lfo, Lfo),
    Lfo3d(Lfo, Lfo, Lfo),
    Lfo4d(Lfo, Lfo, Lfo, Lfo),
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
            Self::Lfo(lfo) => vec![lfo.get_amplitude(beat)],
            Self::Lfo2d(lfo_x, lfo_y) => vec![lfo_x.get_amplitude(beat), lfo_y.get_amplitude(beat)],
            Self::Lfo3d(lfo_x, lfo_y, lfo_z) => vec![
                lfo_x.get_amplitude(beat),
                lfo_y.get_amplitude(beat),
                lfo_z.get_amplitude(beat),
            ],
            Self::Lfo4d(lfo_x, lfo_y, lfo_z, lfo_w) => vec![
                lfo_x.get_amplitude(beat),
                lfo_y.get_amplitude(beat),
                lfo_z.get_amplitude(beat),
                lfo_w.get_amplitude(beat),
            ],
            _ => unreachable!(),
        };

        match *value {
            DataHolder::Bool(bool_value) => Some(DataHolder::Bool(
                (bool_value || offset[0] > 0.5) && offset[0] > 0.0,
            )),
            DataHolder::Float(float_value) => {
                Some(DataHolder::Float(float_value + offset[0] as f32))
            }
            DataHolder::Float2(float2_value) => {
                if offset.len() == 2 {
                    Some(DataHolder::Float2([
                        float2_value[0] + offset[0] as f32,
                        float2_value[1] + offset[1] as f32,
                    ]))
                } else {
                    Some(DataHolder::Float2([
                        float2_value[0] + offset[0] as f32,
                        float2_value[1] + offset[0] as f32,
                    ]))
                }
            }
            DataHolder::Float3(float3_value) => {
                if offset.len() == 3 {
                    Some(DataHolder::Float3([
                        float3_value[0] + offset[0] as f32,
                        float3_value[1] + offset[1] as f32,
                        float3_value[2] + offset[2] as f32,
                    ]))
                } else {
                    Some(DataHolder::Float3([
                        float3_value[0] + offset[0] as f32,
                        float3_value[1] + offset[0] as f32,
                        float3_value[2] + offset[0] as f32,
                    ]))
                }
            }
            DataHolder::Float4(float4_value) => {
                if offset.len() == 4 {
                    Some(DataHolder::Float4([
                        float4_value[0] + offset[0] as f32,
                        float4_value[1] + offset[1] as f32,
                        float4_value[2] + offset[2] as f32,
                        float4_value[3] + offset[3] as f32,
                    ]))
                } else {
                    Some(DataHolder::Float4([
                        float4_value[0] + offset[0] as f32,
                        float4_value[1] + offset[0] as f32,
                        float4_value[2] + offset[0] as f32,
                        float4_value[3] + offset[0] as f32,
                    ]))
                }
            }
            DataHolder::Int(int_value) => Some(DataHolder::Int(int_value + offset[0] as i32)),
            DataHolder::Int2(int2_value) => {
                if offset.len() == 2 {
                    Some(DataHolder::Int2([
                        int2_value[0] + offset[0] as i32,
                        int2_value[1] + offset[1] as i32,
                    ]))
                } else {
                    Some(DataHolder::Int2([
                        int2_value[0] + offset[0] as i32,
                        int2_value[1] + offset[0] as i32,
                    ]))
                }
            }
            DataHolder::Int3(int3_value) => {
                if offset.len() == 3 {
                    Some(DataHolder::Int3([
                        int3_value[0] + offset[0] as i32,
                        int3_value[1] + offset[1] as i32,
                        int3_value[2] + offset[2] as i32,
                    ]))
                } else {
                    Some(DataHolder::Int3([
                        int3_value[0] + offset[0] as i32,
                        int3_value[1] + offset[0] as i32,
                        int3_value[2] + offset[0] as i32,
                    ]))
                }
            }
            DataHolder::Int4(int4_value) => {
                if offset.len() == 4 {
                    Some(DataHolder::Int4([
                        int4_value[0] + offset[0] as i32,
                        int4_value[1] + offset[0] as i32,
                        int4_value[2] + offset[0] as i32,
                        int4_value[3] + offset[0] as i32,
                    ]))
                } else {
                    Some(DataHolder::Int4([
                        int4_value[0] + offset[0] as i32,
                        int4_value[1] + offset[1] as i32,
                        int4_value[2] + offset[2] as i32,
                        int4_value[3] + offset[3] as i32,
                    ]))
                }
            }
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
