use std::sync::Arc;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn v2s_channel_mode() -> Arc<dyn Fn(i32) -> String + Send + Sync> {
  Arc::new(move |value| {
    match value {
      0 => "Stereo".to_string(),
      _ => "Ping Pong".to_string()
    }
  })
}

pub fn s2v_channel_mode() -> Arc<dyn Fn(&str) -> Option<i32> + Send + Sync> {
  Arc::new(move |value| {
    match value {
      "Stereo" => Some(0),
      _ => Some(1)
    }
  })
}

pub fn v2s_time_mode() -> Arc<dyn Fn(i32) -> String + Send + Sync> {
  Arc::new(move |value| {
    match value {
      0 => "Repitch".to_string(),
      _ => "Fade".to_string()
    }
  })
}

pub fn s2v_time_mode() -> Arc<dyn Fn(&str) -> Option<i32> + Send + Sync> {
  Arc::new(move |value| {
    match value {
      "Stereo" => Some(0),
      _ => Some(1)
    }
  })
}
