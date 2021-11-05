use super::DataHolder;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum LfoType {
    Square,
    Triangle,
    Saw,
    Sine,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Lfo {
    pub lfo_type: LfoType,
    pub numerator: f64,
    pub denominator: f64,
    pub phase: f64,
    pub amplitude: f64,
    pub signed: bool,
}

impl Lfo {
    pub fn get_amplitude(&self, beat: f64) -> f64 {
        let beat_cursor = (beat * self.numerator / self.denominator + self.phase).fract();
        let value = match self.lfo_type {
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

        let value = if self.signed {
            value * 2.0 - 1.0
        } else {
            value
        };

        value * self.amplitude
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
