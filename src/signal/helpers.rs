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
