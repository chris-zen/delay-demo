use std::sync::Arc;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::plugin_main;

use delay::stereo_cross_feedback_delay::{StereoCrossFeedbackDelay, StereoCrossFeedbackParams};

enum Params {
  Delay,
  Feedback,
  WetDry
}

impl Params {
  fn from_index(index: i32) -> Self {
    match index {
      0 => Params::Delay,
      1 => Params::Feedback,
      2 => Params::WetDry,
      _ => unreachable!()
    }
  }

  fn count() -> i32 { 3 }
}

struct DelayPluginParameters {
  params: Arc<StereoCrossFeedbackParams>
}

impl PluginParameters for DelayPluginParameters {
  fn get_parameter_text(&self, index: i32) -> String {
    match Params::from_index(index) {
      Params::Delay => self.params.delay_seconds.get_text(),
      Params::Feedback => self.params.feedback.get_text(),
      Params::WetDry => self.params.wet_dry_ratio.get_text(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match Params::from_index(index) {
      Params::Delay => self.params.delay_seconds.get_name(),
      Params::Feedback => self.params.feedback.get_name(),
      Params::WetDry => self.params.wet_dry_ratio.get_name(),
    }.clone()
  }

  fn get_parameter(&self, index: i32) -> f32 {
    match Params::from_index(index) {
      Params::Delay => self.params.delay_seconds.get() as f32,
      Params::Feedback => self.params.feedback.get() as f32,
      Params::WetDry => self.params.wet_dry_ratio.get() as f32,
    }
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match Params::from_index(index) {
      Params::Delay => self.params.delay_seconds.set(val as f64),
      Params::Feedback => self.params.feedback.set(val as f64),
      Params::WetDry => self.params.wet_dry_ratio.set(val as f64),
    }
  }
}

pub struct StereoDelayPlugin {
  effect: StereoCrossFeedbackDelay,
}

impl Default for StereoDelayPlugin {
  fn default() -> Self {
    StereoDelayPlugin {
      effect: StereoCrossFeedbackDelay::new(44100.0)
    }
  }
}

impl Plugin for StereoDelayPlugin {
  fn get_info(&self) -> Info {
    Info {
      name: "Stereo Delay".to_string(),
      vendor: "Christian".to_string(),
      unique_id: 1358,
      inputs: 2,
      outputs: 2,
      category: Category::Effect,
      parameters: Params::count(),

      ..Default::default()
    }
  }

  fn set_sample_rate(&mut self, rate: f32) {
    self.effect.set_sample_rate(rate as f64);
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

    self.effect.process_begins();

    let (input_channels, output_channels) = buffer.split();

    let (input_left_buffer, input_right_buffer) = (input_channels.get(0), input_channels.get(1));
    let (output_left_buffer, output_right_buffer) = (output_channels.get_mut(0), output_channels.get_mut(1));

    let input_buffer_interleaved = input_left_buffer.into_iter().zip(input_right_buffer);
    let output_buffer_interleaved = output_left_buffer.into_iter().zip(output_right_buffer);

    let all_buffers_interleaved = input_buffer_interleaved.zip(output_buffer_interleaved);

    for ((in_left_sample, in_right_sample), (out_left_sample, out_right_sample)) in all_buffers_interleaved {
      let (left, right) = self.effect.process(*in_left_sample as f64, *in_right_sample as f64);
      *out_left_sample = left as f32;
      *out_right_sample = right as f32;
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::new(DelayPluginParameters {
      params: self.effect.clone_params()
    })
  }
}

plugin_main!(StereoDelayPlugin);
