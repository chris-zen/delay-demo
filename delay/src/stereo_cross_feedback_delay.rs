use std::sync::Arc;

use crate::delay_line::{DelayLine, FeedbackSource};
use crate::utils::atomic::AtomicFloat;
use crate::utils::params::FloatParam;

const DEFAULT_DELAY_SECONDS: f64 = 0.50;
const DEFAULT_FEEDBACK: f64 = 0.50;
const DEFAULT_WET_DRY_RATIO: f64 = 0.50;

pub struct StereoCrossFeedbackParams {
  pub delay_seconds: FloatParam,
  pub feedback: FloatParam,
  pub wet_dry_ratio : FloatParam,
}

impl Default for StereoCrossFeedbackParams {
  fn default() -> Self {
    StereoCrossFeedbackParams {
      delay_seconds: FloatParam::new("Delay", Arc::new(FloatParam::format_decimals_3), DEFAULT_DELAY_SECONDS),
      feedback: FloatParam::new("Feedback", Arc::new(FloatParam::format_percentage), DEFAULT_FEEDBACK),
      wet_dry_ratio: FloatParam::new("Wet/Dry", Arc::new(FloatParam::format_percentage), DEFAULT_WET_DRY_RATIO),
    }
  }
}

pub struct StereoCrossFeedbackDelay {
  params: Arc<StereoCrossFeedbackParams>,
  sample_rate: AtomicFloat,
  left_delay_line: DelayLine,
  right_delay_line: DelayLine,
  left_feedback: f64,
  right_feedback: f64,
}

impl StereoCrossFeedbackDelay {
  pub fn new(sample_rate: f64) -> Self {
    let params = StereoCrossFeedbackParams::default();
    let left_delay_line = Self::new_delay_line(sample_rate, &params);
    let right_delay_line = Self::new_delay_line(sample_rate, &params);

    StereoCrossFeedbackDelay {
      params: Arc::new(params),
      sample_rate: AtomicFloat::new(sample_rate),
      left_delay_line,
      right_delay_line,
      left_feedback: 0.0,
      right_feedback: 0.0,
    }
  }

  fn new_delay_line(sample_rate: f64, params: &StereoCrossFeedbackParams) -> DelayLine {
    let max_delay_samples = sample_rate as usize + 1;
    let delay_samples = (sample_rate * params.delay_seconds.get()) as usize;
    DelayLine::new(max_delay_samples,
                   delay_samples,
                   FeedbackSource::External,
                   params.feedback.get(),
                   params.wet_dry_ratio.get())
  }

  pub fn clone_params(&self) -> Arc<StereoCrossFeedbackParams> {
    Arc::clone(&self.params)
  }

  pub fn set_sample_rate(&mut self, sample_rate: f64) {
    if self.sample_rate.get() != sample_rate {
      self.sample_rate.set(sample_rate);
      let params = &self.params;
      self.left_delay_line = Self::new_delay_line(sample_rate, &params);
      self.right_delay_line = Self::new_delay_line(sample_rate, &params);
    }
  }

  pub fn process_begins(&mut self) {
    if self.params.delay_seconds.is_modified() {
      let delay_samples = (self.params.delay_seconds.get() * self.sample_rate.get()) as usize;
      self.left_delay_line.set_delay_samples(delay_samples);
      self.right_delay_line.set_delay_samples(delay_samples);
    }

    if self.params.feedback.is_modified() {
      let feedback = self.params.feedback.get();
      self.left_delay_line.set_internal_feedback(feedback);
      self.right_delay_line.set_internal_feedback(feedback);
    }

    if self.params.wet_dry_ratio.is_modified() {
      let wet_dry_ratio = self.params.wet_dry_ratio.get();
      self.left_delay_line.set_wet_dry_ratio(wet_dry_ratio);
      self.right_delay_line.set_wet_dry_ratio(wet_dry_ratio);
    }
  }

  pub fn process(&mut self, in_left_sample: f64, in_right_sample: f64) -> (f64, f64) {
    let (yn_left, left_feedback) = self.left_delay_line.process_with_feedback(in_left_sample, self.right_feedback);
    let (yn_right, right_feedback) = self.right_delay_line.process_with_feedback(in_right_sample, self.left_feedback);
    self.left_feedback = left_feedback;
    self.right_feedback = right_feedback;
    (yn_left, yn_right)
  }
}
