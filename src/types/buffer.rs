pub struct Buffer {
    pub dimensions: Vec<usize>,
    pub data: Option<Vec<u8>>,
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
