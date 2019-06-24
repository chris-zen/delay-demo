pub struct DelayBuffer {
  buffer: Vec<f64>,
  index: usize,
}

impl DelayBuffer {
  pub fn new(capacity: usize) -> Self {
    DelayBuffer {
      buffer: vec![0.0; capacity],
      index: 0,
    }
  }

  pub fn capacity(&self) -> usize {
    self.buffer.capacity()
  }

  pub fn write(&mut self, value: f64) {
    self.buffer[self.index] = value;
    self.index = (self.index + 1) % self.buffer.capacity();
  }

  pub fn read_with_delay(&self, delay: usize) -> f64 {
    let offset = if delay >= self.index {
      self.buffer.capacity() + self.index - 1 - delay
    }
    else {
      self.index - 1 - delay
    };
    self.buffer[offset]
  }
}

#[cfg(test)]
mod test {
  use super::DelayBuffer;

  #[test]
  fn test_new() {
    let delay_line = DelayBuffer::new(3);
    assert_eq!(delay_line.buffer.capacity(), 3);
    assert_eq!(delay_line.buffer.len(), 3);
    assert_eq!(delay_line.index, 0);
  }

  #[test]
  fn test_write_no_crossing() {
    let mut delay_line = DelayBuffer::new(3);
    delay_line.write(1.0);
    delay_line.write(2.0);
    delay_line.write(3.0);
    assert_eq!(delay_line.buffer, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_write_crossing_end() {
    let mut delay_line = DelayBuffer::new(3);
    delay_line.write(1.0);
    delay_line.write(2.0);
    delay_line.write(3.0);
    delay_line.write(4.0);
    assert_eq!(delay_line.buffer, vec![4.0, 2.0, 3.0]);
  }

  #[test]
  fn test_read_no_crossing() {
    let mut delay_line = DelayBuffer::new(3);
    delay_line.write(1.0);
    delay_line.write(2.0);
    assert_eq!(delay_line.read_with_delay(0), 2.0);
    assert_eq!(delay_line.read_with_delay(1), 1.0);
  }

  #[test]
  fn test_read_crossing() {
    let mut delay_line = DelayBuffer::new(3);
    for i in 0..5 {
      delay_line.write(i as f64);
    }
    for i in 0..3 {
      assert_eq!(delay_line.read_with_delay(i), 4.0 - i as f64);
    }
  }
}
