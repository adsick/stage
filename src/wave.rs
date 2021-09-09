use crate::signal::{Signal, PI, TAU};

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

    //i personally find this funny
    pub fn saw(sr: u32) -> Self {
        let mut samples = Vec::with_capacity(sr as usize);
        for i in 0..sr {
            samples.push(i as f32 / sr as f32);
        }
        Wave { samples }
    }
}

//todo remake it for 'static Wave
pub struct WaveSignal {
    wave: Wave,
    t: f32, // "time"
    pos: f32,
    f: f32,  // "frequency"
    tf: f32, // "target frequency"
    sr: u32, // sample rate
}

impl WaveSignal {
    pub fn sin(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::sin(sr),
            t: 0.0,
            pos: 0.0,
            f,
            tf: f,
            sr,
        }
    }
}

impl Signal for WaveSignal {
    fn sample(&mut self) -> f32 {
        let res = self.wave.samples[self.pos as usize];
        self.pos += self.f;
        self.t += 1.0;
        if self.pos as usize >= self.wave.samples.len() {
            self.pos -= self.wave.samples.len() as f32;
        } else if self.pos < 0.0 {
            self.pos += self.wave.samples.len() as f32;
        }
        // self.f += (self.tf - self.f)/48000.0;
        self.f = self.tf;

        // self.tf += 0.1*(w*w*1.0).sin();

        res
    }
}
