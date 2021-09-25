use crate::envelope::Envelope;
use crate::signal::Signal;
use crate::wave::WaveSignal;

//single voice
pub struct Voice {
    vco: Box<dyn Signal>,
    vca: Envelope,
}

impl Voice {
    pub fn sin(f: f32, sr: u32, len: f32) -> Voice {
        let sk = len * sr as f32 / 1000.0; //a convenience for using milliseconds
        let mut vca = Envelope::new(10.0 * sk, 40.0 * sk, 0.0, 0.0 * sk);
        vca.looping = true; //note remove
        let vco = Box::new(WaveSignal::sin(f, sr));
        Voice { vco, vca }
    }

    pub fn saw(f: f32, sr: u32, len: f32) -> Voice {
        let sk = len * sr as f32 / 1000.0; //a convenience for using milliseconds
        let mut vca = Envelope::new(1.0 * sk, 40.0 * sk, 0.0, 0.0 * sk);
        vca.looping = true;

        let vco = Box::new(WaveSignal::saw(f, sr));
        Voice { vco, vca }
    }
    pub fn square(f: f32, sr: u32, len: f32) -> Voice {
        let sk = len * sr as f32 / 1000.0; //a convenience for using milliseconds
        let mut vca = Envelope::new(1.0 * sk, 40.0 * sk, 0.0, 0.0 * sk);
        vca.looping = true;

        let vco = Box::new(WaveSignal::square(f, sr));
        Voice { vco, vca }
    }
    pub fn noise(f: f32, sr: u32, len: f32) -> Voice{
        let sk = len * sr as f32 / 1000.0; //a convenience for using milliseconds
        let mut vca = Envelope::new(0.0 * sk, 40.0 * sk, 0.0, 0.0 * sk);
        vca.looping = true;

        let vco = Box::new(WaveSignal::noise(f, sr));
        Voice { vco, vca }
    }
}

impl Signal for Voice {
    fn sample(&mut self) -> f32 {
        let val = self.vco.sample();
        let vol = self.vca.sample();

        self.vca.mul_len(0.999985); //todo remove
        
        val * vol
    }
}
