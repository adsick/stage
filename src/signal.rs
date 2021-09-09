pub use std::f32::consts::{PI, TAU};
pub mod helpers;

//note: consider renaming

///anything that can be sampled
pub trait Signal: Send {
    fn sample(&mut self) -> f32;

    ///this in theory should return the number of samples written but imma not sure if we need that
    fn samples(&mut self, buf: &mut [f32]) -> usize {
        for v in buf.iter_mut() {
            *v = self.sample();
        }
        0
    }
}

#[deprecated]
#[derive(Debug)]
pub struct Sine {
    pub f: f32, //frequency
    pub a: f32, //amplitude
    phs: f32,   //phase shift
    t: f32,     //'time'
    sr: f32,    //sample rate
}

impl Default for Sine {
    fn default() -> Self {
        Self {
            f: 440.0,
            a: 1.0,
            phs: 0.0,
            t: 0.0,
            sr: 48000.0,
        }
    }
}

impl Sine {
    pub fn new(f: f32, a: f32, sr: f32) -> Self {
        Sine {
            f,
            a,
            sr,
            ..Default::default()
        }
    }
}

impl Signal for Sine {
    fn sample(&mut self) -> f32 {
        self.t += 1.0 / self.sr; //todo replace it for precomputed version (?)
        self.a * (TAU * self.f * self.t + self.phs).sin()
    }
}
