/// Adapted from vst::util::atomic_float
/// https://github.com/RustAudio/vst-rs/blob/master/src/util/atomic_float.rs

use std::sync::atomic::{AtomicU64, Ordering};

/// Simple atomic floating point variable with relaxed ordering.
///
/// Designed for the common case of sharing parameters between
/// multiple threads when no synchronization or change notification
/// is needed.
pub struct AtomicFloat {
  atomic: AtomicU64,
}

impl AtomicFloat {
  /// New atomic float with initial value `value`.
  pub fn new(value: f64) -> AtomicFloat {
    AtomicFloat {
      atomic: AtomicU64::new(value.to_bits()),
    }
  }

  /// Get the current value of the atomic float.
  pub fn get(&self) -> f64 {
    f64::from_bits(self.atomic.load(Ordering::Relaxed))
  }

  /// Set the value of the atomic float to `value`.
  pub fn set(&self, value: f64) {
    self.atomic.store(value.to_bits(), Ordering::Relaxed)
  }
}
