use std::ops::{Add, Div, Mul, Sub};

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

    String(String),

    Texture(((u32, u32), Vec<u8>)),
    SrgbTexture(((u32, u32), Vec<u8>)),
}

impl<'a> Mul for &'a DataHolder {
    type Output = DataHolder;

    fn mul(self, other: Self) -> DataHolder {
        match (self, other) {
            (DataHolder::Float(value_left), DataHolder::Float(value_right)) => {
                DataHolder::Float(value_left * value_right)
            }
            (DataHolder::Float2(value_left), DataHolder::Float2(value_right)) => {
                DataHolder::Float2([
                    value_left[0] * value_right[0],
                    value_left[1] * value_right[1],
                ])
            }
            (DataHolder::Float3(value_left), DataHolder::Float3(value_right)) => {
                DataHolder::Float3([
                    value_left[0] * value_right[0],
                    value_left[1] * value_right[1],
                    value_left[2] * value_right[2],
                ])
            }
            (DataHolder::Float4(value_left), DataHolder::Float4(value_right)) => {
                DataHolder::Float4([
                    value_left[0] * value_right[0],
                    value_left[1] * value_right[1],
                    value_left[2] * value_right[2],
                    value_left[3] * value_right[3],
                ])
            }

            (DataHolder::Int(value_left), DataHolder::Int(value_right)) => {
                DataHolder::Int(value_left * value_right)
            }
            (DataHolder::Int2(value_left), DataHolder::Int2(value_right)) => DataHolder::Int2([
                value_left[0] * value_right[0],
                value_left[1] * value_right[1],
            ]),
            (DataHolder::Int3(value_left), DataHolder::Int3(value_right)) => DataHolder::Int3([
                value_left[0] * value_right[0],
                value_left[1] * value_right[1],
                value_left[2] * value_right[2],
            ]),
            (DataHolder::Int4(value_left), DataHolder::Int4(value_right)) => DataHolder::Int4([
                value_left[0] * value_right[0],
                value_left[1] * value_right[1],
                value_left[2] * value_right[2],
                value_left[3] * value_right[3],
            ]),

            (DataHolder::Bool(value_left), DataHolder::Bool(value_right)) => {
                DataHolder::Bool(*value_left && *value_right)
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> Add for &'a DataHolder {
    type Output = DataHolder;

    fn add(self, other: Self) -> DataHolder {
        match (self, other) {
            (DataHolder::Float(value_left), DataHolder::Float(value_right)) => {
                DataHolder::Float(value_left + value_right)
            }
            (DataHolder::Float2(value_left), DataHolder::Float2(value_right)) => {
                DataHolder::Float2([
                    value_left[0] + value_right[0],
                    value_left[1] + value_right[1],
                ])
            }
            (DataHolder::Float3(value_left), DataHolder::Float3(value_right)) => {
                DataHolder::Float3([
                    value_left[0] + value_right[0],
                    value_left[1] + value_right[1],
                    value_left[2] + value_right[2],
                ])
            }
            (DataHolder::Float4(value_left), DataHolder::Float4(value_right)) => {
                DataHolder::Float4([
                    value_left[0] + value_right[0],
                    value_left[1] + value_right[1],
                    value_left[2] + value_right[2],
                    value_left[3] + value_right[3],
                ])
            }

            (DataHolder::Int(value_left), DataHolder::Int(value_right)) => {
                DataHolder::Int(value_left + value_right)
            }
            (DataHolder::Int2(value_left), DataHolder::Int2(value_right)) => DataHolder::Int2([
                value_left[0] + value_right[0],
                value_left[1] + value_right[1],
            ]),
            (DataHolder::Int3(value_left), DataHolder::Int3(value_right)) => DataHolder::Int3([
                value_left[0] + value_right[0],
                value_left[1] + value_right[1],
                value_left[2] + value_right[2],
            ]),
            (DataHolder::Int4(value_left), DataHolder::Int4(value_right)) => DataHolder::Int4([
                value_left[0] + value_right[0],
                value_left[1] + value_right[1],
                value_left[2] + value_right[2],
                value_left[3] + value_right[3],
            ]),

            (DataHolder::Bool(value_left), DataHolder::Bool(value_right)) => {
                DataHolder::Bool(*value_left || *value_right)
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> Sub for &'a DataHolder {
    type Output = DataHolder;

    fn sub(self, other: Self) -> DataHolder {
        match (self, other) {
            (DataHolder::Float(value_left), DataHolder::Float(value_right)) => {
                DataHolder::Float(value_left - value_right)
            }
            (DataHolder::Float2(value_left), DataHolder::Float2(value_right)) => {
                DataHolder::Float2([
                    value_left[0] - value_right[0],
                    value_left[1] - value_right[1],
                ])
            }
            (DataHolder::Float3(value_left), DataHolder::Float3(value_right)) => {
                DataHolder::Float3([
                    value_left[0] - value_right[0],
                    value_left[1] - value_right[1],
                    value_left[2] - value_right[2],
                ])
            }
            (DataHolder::Float4(value_left), DataHolder::Float4(value_right)) => {
                DataHolder::Float4([
                    value_left[0] - value_right[0],
                    value_left[1] - value_right[1],
                    value_left[2] - value_right[2],
                    value_left[3] - value_right[3],
                ])
            }

            (DataHolder::Int(value_left), DataHolder::Int(value_right)) => {
                DataHolder::Int(value_left - value_right)
            }
            (DataHolder::Int2(value_left), DataHolder::Int2(value_right)) => DataHolder::Int2([
                value_left[0] - value_right[0],
                value_left[1] - value_right[1],
            ]),
            (DataHolder::Int3(value_left), DataHolder::Int3(value_right)) => DataHolder::Int3([
                value_left[0] - value_right[0],
                value_left[1] - value_right[1],
                value_left[2] - value_right[2],
            ]),
            (DataHolder::Int4(value_left), DataHolder::Int4(value_right)) => DataHolder::Int4([
                value_left[0] - value_right[0],
                value_left[1] - value_right[1],
                value_left[2] - value_right[2],
                value_left[3] - value_right[3],
            ]),

            (DataHolder::Bool(value_left), DataHolder::Bool(value_right)) => {
                DataHolder::Bool(*value_left && !value_right)
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> Div for &'a DataHolder {
    type Output = DataHolder;

    fn div(self, other: Self) -> DataHolder {
        match (self, other) {
            (DataHolder::Float(value_left), DataHolder::Float(value_right)) => {
                DataHolder::Float(value_left / value_right)
            }
            (DataHolder::Float2(value_left), DataHolder::Float2(value_right)) => {
                DataHolder::Float2([
                    value_left[0] / value_right[0],
                    value_left[1] / value_right[1],
                ])
            }
            (DataHolder::Float3(value_left), DataHolder::Float3(value_right)) => {
                DataHolder::Float3([
                    value_left[0] / value_right[0],
                    value_left[1] / value_right[1],
                    value_left[2] / value_right[2],
                ])
            }
            (DataHolder::Float4(value_left), DataHolder::Float4(value_right)) => {
                DataHolder::Float4([
                    value_left[0] / value_right[0],
                    value_left[1] / value_right[1],
                    value_left[2] / value_right[2],
                    value_left[3] / value_right[3],
                ])
            }

            (DataHolder::Int(value_left), DataHolder::Int(value_right)) => {
                DataHolder::Int(value_left / value_right)
            }
            (DataHolder::Int2(value_left), DataHolder::Int2(value_right)) => DataHolder::Int2([
                value_left[0] / value_right[0],
                value_left[1] / value_right[1],
            ]),
            (DataHolder::Int3(value_left), DataHolder::Int3(value_right)) => DataHolder::Int3([
                value_left[0] / value_right[0],
                value_left[1] / value_right[1],
                value_left[2] / value_right[2],
            ]),
            (DataHolder::Int4(value_left), DataHolder::Int4(value_right)) => DataHolder::Int4([
                value_left[0] / value_right[0],
                value_left[1] / value_right[1],
                value_left[2] / value_right[2],
                value_left[3] / value_right[3],
            ]),

            (DataHolder::Bool(value_left), DataHolder::Bool(_)) => DataHolder::Bool(*value_left),
            _ => unimplemented!(),
        }
    }
}
