use std::sync::Arc;

pub fn v2s_f32_synced_time() -> Arc<dyn Fn(i32) -> String + Send + Sync> {
  Arc::new(move |value| {
    match value {
      0 => "1/32",
      1 => "1/16T",
      2 => "1/32.",
      3 => "1/16",
      4 => "1/8T",
      5 => "1/16.",
      6 => "1/8",
      7 => "1/4T",
      8 => "1/8.",
      9 => "1/4",
      10 => "1/2T",
      11 => "1/4.",
      12 => "1/2",
      13 => "1T",
      14 => "1/2.",
      15 => "1",
      _ => "",
    }
    .to_string()
  })
}

pub fn s2v_f32_synced_time() -> Arc<dyn Fn(&str) -> Option<i32> + Send + Sync> {
  Arc::new(|string| match string {
    "1/32" => Some(0),
    "1/16T" => Some(1),
    "1/32." => Some(2),
    "1/16" => Some(3),
    "1/8T" => Some(4),
    "1/16." => Some(5),
    "1/8" => Some(6),
    "1/4T" => Some(7),
    "1/8." => Some(8),
    "1/4" => Some(9),
    "1/2T" => Some(10),
    "1/4." => Some(11),
    "1/2" => Some(12),
    "1T" => Some(13),
    "1/2." => Some(14),
    "1" => Some(15),
    _ => None,
  })
}
