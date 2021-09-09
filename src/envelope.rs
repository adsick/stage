//trait Envelope {...}

///a basic linear envelope
pub struct Envelope {
    //consider changing these to pub
    a: f32, //attack
    d: f32, //decay
    s: f32, //sustain
    r: f32, //release

    pub looping: bool, //is it a looping one or not

    t: f32, //current "position" in the envelope
}

impl Envelope {
    pub fn new(a: f32, d: f32, s: f32, r: f32) -> Self {
        Envelope {
            a,
            d,
            s,
            r,
            looping: false,
            t: 0.0,
        }
    }

    pub fn mul_len(&mut self, factor: f32) {
        self.a *= factor;
        self.d *= factor;
        self.r *= factor;
    }

    pub fn sample(&mut self) -> f32 {
        let Envelope {
            a,
            d,
            s,
            r,
            looping,
            t,
        } = self;

        if *t < *a {
            *t += 1.0;
            return (*t) / (*a);
        } else if *t < *a + *d {
            *t += 1.0;
            return 1.0 + (*s - 1.0) * (*t - *a) / (*d); //todo test
        }
        //need to handle sustain somehow...
        else if *t < *a + *d + *r {
            *t += 1.0;

            return *s * (*r + *a + *d - *t) / (*r);
        } else {
            if *looping {
                *t = 0.0; //here we go again
            }
            0.0
        }
    }
}
