use std::collections::HashMap;

use crate::{
    config::filter::FilterMode,
    types::{Automation, BufferPrecision, DataHolder, InputSampler},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RenderStageConfig {
    pub name: String,
    pub filter: String,
    pub filter_mode_params: FilterMode,
    pub inputs: HashMap<String, InputSampler>,
    pub variables: HashMap<String, (DataHolder, Automation, Option<(String, DataHolder)>)>,
    #[serde(default)]
    pub precision: BufferPrecision,
}
