use crate::delay_buffer::DelayBuffer;

pub enum FeedbackSource {
  Internal,
  External
}

pub struct DelayLine {
  buffer: DelayBuffer,

  delay_samples: usize,
  feedback_source: FeedbackSource,
  internal_feedback: f64,
  wet_dry_ratio: f64,
}

impl DelayLine {
  pub fn new(max_delay_samples: usize,
             delay_samples: usize,
             feedback_source: FeedbackSource,
             internal_feedback: f64,
             wet_dry_ratio: f64) -> Self {

    debug_assert!(delay_samples < max_delay_samples);

    DelayLine {
      buffer: DelayBuffer::new(max_delay_samples),
      delay_samples,
      feedback_source,
      internal_feedback,
      wet_dry_ratio,
    }
  }

  pub fn process_with_feedback(&mut self, xn: f64, external_feedback: f64) -> (f64, f64) {
    let delay_signal = self.buffer.read_with_delay(self.delay_samples);

    let internal_feedback = delay_signal * self.internal_feedback;

    let feedback_signal = match self.feedback_source {
      FeedbackSource::Internal => internal_feedback,
      FeedbackSource::External => external_feedback
    };

    self.buffer.write(xn + feedback_signal);

    let wet = self.wet_dry_ratio;
    let dry = 1.0 - self.wet_dry_ratio;
    let yn = delay_signal * wet + xn * dry;

    (yn, internal_feedback)
  }

  pub fn set_delay_samples(&mut self, delay_samples: usize) {
    debug_assert!(delay_samples < self.buffer.capacity());
    self.delay_samples = delay_samples;
  }

  pub fn set_internal_feedback(&mut self, feedback: f64) {
    debug_assert!(feedback >= 0.0 && feedback <= 1.0);
    self.internal_feedback = feedback;
  }

  pub fn set_wet_dry_ratio(&mut self, wet_dry_ratio: f64) {
    debug_assert!(wet_dry_ratio >= 0.0 && wet_dry_ratio <= 1.0);
    self.wet_dry_ratio = wet_dry_ratio;
  }
}
