use std::f64::consts::PI;

pub enum Input {
    Square,
    Sin,
    // Triangle,
    // Saw
}

impl Input {
    pub fn get_sample(&self, phase: f64) -> f64 {
        match self {
            Input::Square => if phase < 0.5 {-1.} else {1.},
            Input::Sin => (phase * 2. * PI).sin(),
            // Input::Triangle => todo!(),
            // Input::Saw => todo!(),
        }
    }
}