pub struct MovingMin {
  current_min: f32,
  next_min: f32,
  next_hold_length: u32,
  hold_index: u32,
  hold_length: u32,
  limit: f32,
}

impl MovingMin {
  pub fn new(sample_rate: f32, attack_time: f32, hold_time: f32, limit: f32) -> Self {
    let hold_length = ((attack_time + hold_time) * 0.001 * sample_rate - 1.0) as u32;

    Self {
      current_min: limit,
      next_min: limit,
      next_hold_length: 0,
      hold_index: hold_length,
      hold_length,
      limit,
    }
  }

  /// If gain_reduction is below current current_min, then replace current_min immediately. <br />
  /// Else if hold_time has passed, then reset min gain_reduction that occured during the hold_time window and reset the hold_time to when the next peak occured in the hold_time window. <br />
  /// Else, keep the current_min value untouched for hold_time.
  pub fn process(&mut self, input: f32) -> f32 {
    if input < self.current_min {
      self.current_min = input;
      self.next_min = self.limit;
      self.hold_index = self.hold_length;
    } else {
      if self.hold_index == 0 {
        if self.next_min < self.limit {
          self.hold_index = self.next_hold_length;
        } else {
          self.hold_index = self.hold_length;
        }

        self.current_min = self.next_min;
        self.next_min = self.limit;
      } else {
        self.hold_index -= 1;
      }

      if input < self.limit {
        self.next_hold_length = self.hold_length - self.hold_index - 1;
      }
      self.next_min = input.min(self.next_min);
    }

    self.current_min
  }
}

#[cfg(test)]
mod tests {
  use super::MovingMin;

  #[test]
  fn should_hold_minimum_for_attack_time_in_ms() {
    let mut moving_min = MovingMin::new(1000., 4., 0., 1.);

    assert_eq!(moving_min.process(10.0), 1.);

    assert_eq!(moving_min.process(0.7), 0.7);
    assert_eq!(moving_min.process(1.2), 0.7);
    assert_eq!(moving_min.process(1.2), 0.7);
    assert_eq!(moving_min.process(1.2), 0.7);

    assert_eq!(moving_min.process(1.2), 1.0);
  }

  #[test]
  fn should_hold_new_minimums_that_occur_in_the_hold_window() {
    let mut moving_min = MovingMin::new(1000., 4., 0., 1.);

    // test 1
    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(0.8), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);

    assert_eq!(moving_min.process(1.6), 0.8);
    assert_eq!(moving_min.process(1.5), 1.0);

    // test 2
    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(0.8), 0.3);
    assert_eq!(moving_min.process(0.5), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);

    assert_eq!(moving_min.process(1.6), 0.5);
    assert_eq!(moving_min.process(1.6), 0.5);
    assert_eq!(moving_min.process(1.5), 1.0);

    // test 3
    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(0.8), 0.3);
    assert_eq!(moving_min.process(0.5), 0.3);
    assert_eq!(moving_min.process(0.6), 0.3);

    assert_eq!(moving_min.process(1.6), 0.5);
    assert_eq!(moving_min.process(1.5), 0.5);
    assert_eq!(moving_min.process(1.5), 0.5);
    assert_eq!(moving_min.process(1.5), 1.0);

    // test 4
    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(1.8), 0.3);
    assert_eq!(moving_min.process(1.5), 0.3);
    assert_eq!(moving_min.process(0.5), 0.3);

    assert_eq!(moving_min.process(1.6), 0.5);
    assert_eq!(moving_min.process(0.8), 0.5);
    assert_eq!(moving_min.process(0.6), 0.5);

    assert_eq!(moving_min.process(1.6), 0.6);
    assert_eq!(moving_min.process(1.6), 0.6);
    assert_eq!(moving_min.process(1.6), 0.6);
    assert_eq!(moving_min.process(1.6), 1.0);
  }

  #[test]
  fn should_hold_minimum_for_attack_plus_hold_time_in_ms() {
    let mut moving_min = MovingMin::new(1000., 4., 2., 1.);

    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(0.8), 0.3);
    assert_eq!(moving_min.process(1.2), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);
    assert_eq!(moving_min.process(1.3), 0.3);

    assert_eq!(moving_min.process(1.6), 0.8);
    assert_eq!(moving_min.process(1.5), 1.0);
  }

  #[test]
  fn should_not_exceed_limit() {
    let mut moving_min = MovingMin::new(1000., 4., 0., 0.5);

    assert_eq!(moving_min.process(10.0), 0.5);

    assert_eq!(moving_min.process(0.3), 0.3);
    assert_eq!(moving_min.process(1.2), 0.3);
    assert_eq!(moving_min.process(1.2), 0.3);
    assert_eq!(moving_min.process(1.2), 0.3);

    assert_eq!(moving_min.process(1.2), 0.5);
  }
}
