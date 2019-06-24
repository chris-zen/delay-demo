use std::string::ToString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::utils::atomic::AtomicFloat;

type FormatFn = Fn(f64) -> String + Send + Sync;

pub struct FloatParam {
  name: String,
  format: Arc<FormatFn>,
  value: AtomicFloat,
  modified: AtomicBool,
}

impl FloatParam {
  pub fn new(name: &'static str, format: Arc<FormatFn>, initial_value: f64) -> Self {
    FloatParam {
      name: name.to_string(),
      format,
      value: AtomicFloat::new(initial_value),
      modified: AtomicBool::new(false),
    }
  }

  pub fn format_decimals_3(value: f64) -> String {
    format!("{:.3}", value)
  }

  pub fn format_percentage(value: f64) -> String {
    format!("{:.0}%", value * 100.0)
  }

  pub fn is_modified(&self) -> bool {
    self.modified.load(Ordering::Relaxed)
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_text(&self) -> String {
    (self.format)(self.get())
  }

  pub fn get(&self) -> f64 {
    self.modified.store(true, Ordering::Relaxed);
    self.value.get()
  }

  pub fn set(&self, value: f64) {
    self.value.set(value);
    self.modified.store(true, Ordering::Relaxed);
  }
}
