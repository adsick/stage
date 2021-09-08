pub use std::f32::consts::{PI, TAU};

pub trait Signal: Send {
    fn sample(&mut self) -> f32;
}

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

pub mod helpers {
    use super::Sine;

    pub struct SineFabric {
        pub sr: f32,
    }

    impl SineFabric {
        pub fn new(sr: f32) -> Self {
            SineFabric { sr }
        }

        pub fn f(&self, f: f32) -> Sine {
            Sine {
                f,
                a: 1.0,
                phs: 0.0,
                t: 0.0,
                sr: self.sr,
            }
        }
        pub fn fa(&self, f: f32, a: f32) -> Sine {
            Sine {
                f,
                a,
                phs: 0.0,
                t: 0.0,
                sr: self.sr,
            }
        }
        pub fn fv(&self, fs: &[f32]) -> Vec<Sine> {
            fs.iter().map(|f| self.f(*f)).collect()
        }
        pub fn fav(&self, fas: &[(f32, f32)]) -> Vec<Sine> {
            fas.iter().map(|(f, a)| self.fa(*f, *a)).collect()
        }
    }
}
