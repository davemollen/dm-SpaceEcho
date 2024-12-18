use std::fmt;
use std::ops;
use std::slice;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SmoothStatus {
    Inactive,
    Active,
    Deactivating
}

impl SmoothStatus {
    #[inline]
    fn is_active(&self) -> bool {
        self != &SmoothStatus::Inactive
    }
}

pub struct SmoothOutput<'a> {
    pub values: &'a [f32],
    pub status: SmoothStatus
}

impl<'a> SmoothOutput<'a> {
    #[inline]
    pub fn is_smoothing(&self) -> bool {
        self.status.is_active()
    }
}

impl<'a, I> ops::Index<I> for SmoothOutput<'a>
    where I: slice::SliceIndex<[f32]>
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.values[idx]
    }
}

pub struct Smooth {
    output: [f32; crate::MAX_BLOCKSIZE],
    input: f32,

    status: SmoothStatus,

    a: f32,
    b: f32,
    last_output: f32
}

impl Smooth {
    pub fn new(input: f32) -> Self {
        Self {
            status: SmoothStatus::Inactive,
            input,
            output: [input; crate::MAX_BLOCKSIZE],

            a: 1.,
            b: 0.,
            last_output: input
        }
    }

    pub fn reset(&mut self, val: f32)
    {
        *self = Self {
            a: self.a,
            b: self.b,

            ..Self::new(val)
        };
    }

    pub fn set(&mut self, val: f32) {
        self.input = val;
        self.status = SmoothStatus::Active;
    }

    #[inline]
    pub fn dest(&self) -> f32 {
        self.input
    }

    #[inline]
    pub fn output(&self) -> SmoothOutput {
        SmoothOutput {
            values: &self.output,
            status: self.status
        }
    }

    #[inline]
    pub fn current_value(&self) -> SmoothOutput {
        SmoothOutput {
            values: slice::from_ref(&self.last_output),
            status: self.status
        }
    }

    pub fn update_status_with_epsilon(&mut self) -> SmoothStatus {
        let status = self.status;

        match status {
            SmoothStatus::Active => {
                if (self.input - self.output[0]).abs() < f32::EPSILON {
                    self.reset(self.input);
                    self.status = SmoothStatus::Deactivating;
                }
            },

            SmoothStatus::Deactivating =>
                self.status = SmoothStatus::Inactive,

            _ => ()
        };

        self.status
    }

    pub fn process(&mut self, nframes: usize) {
        if self.status != SmoothStatus::Active {
            return
        }

        let nframes = nframes.min(crate::MAX_BLOCKSIZE);
        let input = self.input * self.a;

        self.output[0] = input + (self.last_output * self.b);

        for i in 1..nframes {
            self.output[i] = input + (self.output[i - 1] * self.b);
        }

        self.last_output = self.output[nframes - 1];
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

impl Smooth {
    pub fn set_speed_ms(&mut self, sample_rate: f32, ms: f32) {
        self.b = (-1.0f32 / (ms * (sample_rate / 1000.0f32))).exp();
        self.a = 1.0f32 - self.b;
    }

    #[inline]
    pub fn update_status(&mut self) -> SmoothStatus {
        self.update_status_with_epsilon()
    }
}

impl<T> From<T> for Smooth<T>
    where T: Float + fmt::Display
{
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T, I> ops::Index<I> for Smooth<T>
    where I: slice::SliceIndex<[T]>,
          T: Float
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.output[idx]
    }
}