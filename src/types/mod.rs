use anyhow::Result;

pub mod automation;
pub mod buffer;
pub mod data;
pub mod input;

pub use automation::*;
pub use buffer::*;
pub use data::*;
pub use input::*;

pub trait InputProvider {
    fn set_name(&mut self, name: &str);
    fn provides(&self) -> Vec<String>;
    fn get(&mut self, uniform_name: &str, invalidate: bool) -> Option<DataHolder>;

    fn set_property(&mut self, property: &str, value: &DataHolder);
    fn set_beat(&mut self, _bpm: f64, _sync: bool) {}
    fn set_time(&mut self, _time: f64, _sync: bool) {}
    fn stop(&mut self) -> Result<()> {
        Ok(())
    }
    fn pause(&mut self) -> Result<()> {
        Ok(())
    }
    fn play(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Speed {
    Fps(f32),
    Fpb(f32),
}
