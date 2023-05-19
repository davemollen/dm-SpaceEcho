pub struct BoxcarFilter {
  buffer: Vec<f32>,
  write_pointer: usize,
  previous_sum: f32,
}

impl BoxcarFilter {
  pub fn new(length: usize) -> Self {
    Self {
      buffer: vec![0.0; length],
      write_pointer: 0,
      previous_sum: 0.,
    }
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }

  fn write(&mut self, value: f32) {
    self.buffer[self.write_pointer] = value;
    self.write_pointer = self.wrap(self.write_pointer + 1);
  }

  fn get_oldest_buffer_entry(&self) -> f32 {
    self.buffer[self.write_pointer]
  }

  pub fn run(&mut self, input: f32) -> f32 {
    let n = self.buffer.len();
    let sum = input + self.previous_sum - self.get_oldest_buffer_entry();
    self.previous_sum = sum;
    self.write(input);

    sum / n as f32
  }
}
