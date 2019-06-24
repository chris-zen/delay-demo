use wasm_bindgen::prelude::*;

use delay::stereo_cross_feedback_delay::StereoCrossFeedbackDelay;

#[wasm_bindgen]
pub struct Delay(StereoCrossFeedbackDelay);

#[wasm_bindgen]
impl Delay {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: f64) -> Self {
        Delay(StereoCrossFeedbackDelay::new(sample_rate))
    }

    pub fn process(
        &mut self,
        in_left: &[f32],
        in_right: &[f32],
        out_left: &mut [f32],
        out_right: &mut [f32],
    ) {
        self.0.process_begins();
        for i in 0..in_left.len() {
            let (left, right) = self.0.process(in_left[i] as f64, in_right[i] as f64);
            out_left[i] = left as f32;
            out_right[i] = right as f32;
        }
    }

    pub fn set_delay_seconds(&mut self, value: f64) {
        self.0.clone_params().delay_seconds.set(value);
    }

    pub fn get_delay_seconds(&mut self) -> f64 {
        self.0.clone_params().delay_seconds.get()
    }

    pub fn set_feedback(&mut self, value: f64) {
        self.0.clone_params().feedback.set(value)
    }

    pub fn get_feedback(&mut self) -> f64 {
        self.0.clone_params().feedback.get()
    }

    pub fn set_wet_dry_ratio(&mut self, value: f64) {
        self.0.clone_params().wet_dry_ratio.set(value)
    }

    pub fn get_wet_dry_ratio(&mut self) -> f64 {
        self.0.clone_params().wet_dry_ratio.get()
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
