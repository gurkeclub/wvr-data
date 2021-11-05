use crate::types::Speed;

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
