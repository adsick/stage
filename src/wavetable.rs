use crate::signal::{Signal, PI, TAU};

pub struct WaveTable {
    waves: Vec<Wave>,
    sample_rate: u32,
}

//tothink oversampling (2x, 3x, 4x...)
pub struct Wave {
    pub samples: Vec<f32>,
}

impl Wave {
    pub fn sin(sr: u32) -> Self {
        let mut samples = Vec::with_capacity(sr as usize);
        for i in 0..sr {
            samples.push((i as f32 * TAU / sr as f32).sin());
        }
        Wave { samples }
    }
}

//todo remake it for 'static Wave
pub struct WaveSignal {
    wave: Wave,
    t: f32,  // "time"
    f: f32,  // "frequency"
    tf: f32, // "target frequency"
    sr: u32, // sample rate
}

impl WaveSignal {
    pub fn sin(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::sin(sr),
            t: 0.0,
            f,
            tf: 2.0*f, //todo remove 2.0*
            sr,
        }
    }
}

impl Signal for WaveSignal {
    fn sample(&mut self) -> f32 {
        let res = self.wave.samples[self.t as usize];
        self.t += self.f;
        if self.t as usize >= self.wave.samples.len() {
            self.t -= self.wave.samples.len() as f32;
        } else if self.t < 0.0 {
            self.t += self.wave.samples.len() as f32;
        }
        self.f += (self.tf - self.f)/44100.0;
        
        res
    }
}

impl Signal for WaveTable {
    fn sample(&mut self) -> f32 {
        todo!()
    }
}

