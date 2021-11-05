use std::collections::HashMap;

use crate::types::{DataHolder, DataRange};

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
