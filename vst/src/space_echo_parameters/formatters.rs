use std::sync::Arc;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn v2s_f32_percentage(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value * 100.0))
}

pub fn s2v_f32_percentage() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(|string| {
    string
      .trim_end_matches(&[' ', '%'])
      .parse()
      .ok()
      .map(|x: f32| x / 100.0)
  })
}

pub fn v2s_f32_hz_then_khz(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| {
    if value < 1000.0 {
      format!("{value:.digits$} Hz")
    } else {
      format!("{:.digits$} kHz", value / 1000.0, digits = digits.max(1))
    }
  })
}

pub fn s2v_f32_hz_then_khz() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(move |string| {
    let string = string.trim();
    let mut segments = string.split(',');
    let segments = (segments.next(), segments.next(), segments.next());

    let frequency_segment = segments.0?;
    let cleaned_string = frequency_segment
      .trim_end_matches([' ', 'k', 'K', 'h', 'H', 'z', 'Z'])
      .parse()
      .ok();
    match frequency_segment.get(frequency_segment.len().saturating_sub(3)..) {
      Some(unit) if unit.eq_ignore_ascii_case("khz") => cleaned_string.map(|x| x * 1000.0),
      _ => cleaned_string,
    }
  })
}