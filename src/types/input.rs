#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum InputSampler {
    Nearest(String),
    Linear(String),
    Mipmaps(String),
}
