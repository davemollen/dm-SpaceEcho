include!(concat!(env!("OUT_DIR"), "/wave_table.rs"));
mod allpass_filter;
mod average;
mod biquad_filter;
mod dc_block;
mod delay_line;
mod delta;
mod float_ext;
mod grains;
mod lfo;
mod mix;
mod one_pole_filter;
mod pan;
mod phasor;
mod reverb;
mod reverse;
mod shimmer;
mod tap;
mod taps;
mod tilt_filter;
mod wave_table;

pub const MIN_PREDELAY: f32 = 7.;
pub const MAX_PREDELAY: f32 = 500.;
pub const MIN_SIZE: f32 = 1.;
pub const MAX_SIZE: f32 = 500.;
pub const MAX_DEPTH: f32 = 3.;
pub use self::reverb::Reverb;
