use crate::signal::{Signal, PI, TAU};
use crate::wave::Wave;
pub struct WaveTable {
    waves: Vec<Wave>,

    sr: u32,
}

impl WaveTable{
    //sin tri saw square
    pub fn classic(sr: u32)->Self{
        todo!()
    }
}

pub struct WaveTableSignal{
    wavetable: WaveTable,
    sample_position: f32, //vertical position (which sample)
    wave_position: f32, //horisontal position (which wave)
    f: f32,

}

impl WaveTableSignal{

}