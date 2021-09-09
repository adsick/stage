use crate::signal::{Signal, PI, TAU};
use crate::wave::Wave;
pub struct WaveTable {
    waves: Vec<Wave>,
    sample_rate: u32,
}
