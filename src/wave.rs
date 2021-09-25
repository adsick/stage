use crate::signal::{Signal, PI, TAU};
use rand::prelude::*;
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
            samples.push(1.0 - (2.0 * i as f32)/sr as f32);
        }
        Wave { samples }
    }

    //i know this is dumb
    pub fn square(sr: u32) -> Self{
        let mut samples = Vec::with_capacity(sr as usize);
        for i in 0..sr {
            samples.push(if i < sr/2 {-1.0} else {1.0});
        }
        Wave { samples }
    }

    pub fn noise(sr: u32) ->Self{
        let sr = sr as usize;
        let mut rng = thread_rng();
        let mut samples = vec![0.0; sr];
        let mut curr = 0.0;
        
        for i in 0..sr as usize{
            curr += rng.gen_range(-0.3..=0.3);
            // samples[i] = rng.gen_range(-1.0..=1.0);
            samples[i] = curr;
            curr*=0.99; //dampening
        }
        Wave{samples}
    }
}

//todo remake it for 'static Wave
pub struct WaveSignal {
    wave: Wave,
    t: f32, // "time"
    position: f32,
    f: f32,  // "frequency"
    tf: f32, // "target frequency"
    sr: u32, // sample rate
}

// impl Default for WaveSignal{
//     fn default() -> Self {
//         Self { wave: Wave{ samples: vec![]}, t: 0.0, position: 0.0, f: 440.0, tf: 440.0, sr: 44100 }
//     }
// }

impl WaveSignal {
    pub fn sin(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::sin(sr),
            t: 0.0,
            position: 0.0,
            f,
            tf: f,
            sr,
        }
    }

    pub fn saw(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::saw(sr),
            t: 0.0,
            position: 0.0,
            f,
            tf: f,
            sr,
        }
    }

    pub fn square(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::square(sr),
            t: 0.0,
            position: 0.0,
            f,
            tf: f,
            sr, 
        }
    }

    pub fn noise(f: f32, sr: u32) -> Self {
        WaveSignal {
            wave: Wave::noise(sr),
            t: 0.0,
            position: 0.0,
            f,
            tf: f,
            sr, 
        }
    }

}

impl Signal for WaveSignal {
    fn sample(&mut self) -> f32 {
        let res = self.wave.samples[self.position as usize];
        self.position += self.f;
        self.t += 1.0;
        if self.position as usize >= self.wave.samples.len() {
            self.position -= self.wave.samples.len() as f32;
        } else if self.position < 0.0 {
            self.position += self.wave.samples.len() as f32;
        }
        // self.f += (self.tf - self.f)/48000.0;
        self.f = self.tf;

        // self.tf += 0.1*(w*w*1.0).sin();

        res
    }
}
