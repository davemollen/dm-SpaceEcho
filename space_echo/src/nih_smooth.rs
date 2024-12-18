use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
pub use atomic_float::AtomicF32;

#[derive(Debug, Clone)]
pub enum SmoothingStyle {
    Linear(f32),
    Logarithmic(f32),
    Exponential(f32),
}

#[derive(Debug)]
pub struct Smoother<T: Smoothable> {
    pub style: SmoothingStyle,
    steps_left: AtomicI32,
    step_size: AtomicF32,
    current: AtomicF32,
    target: T::Atomic,
}

pub struct SmootherIter<'a, T: Smoothable> {
    smoother: &'a Smoother<T>,
}

impl SmoothingStyle {
    #[inline]
    pub fn num_steps(&self, sample_rate: f32) -> u32 {
        match self {
            Self::OversamplingAware(oversampling_times, style) => {
                style.num_steps(sample_rate * oversampling_times.load(Ordering::Relaxed))
            }

            Self::None => 1,
            Self::Linear(time) | Self::Logarithmic(time) | Self::Exponential(time) => {
                (sample_rate * time / 1000.0).round() as u32
            }
        }
    }
    
    #[inline]
    pub fn step_size(&self, start: f32, target: f32, num_steps: u32) -> f32 {
        match self {
            Self::Linear(_) => (target - start) / (num_steps as f32),
            Self::Logarithmic(_) => {
                ((target / start) as f64).powf((num_steps as f64).recip()) as f32
            }
            Self::Exponential(_) => 0.0001f64.powf((num_steps as f64).recip()) as f32,
        }
    }
    
    #[inline]
    pub fn next(&self, current: f32, target: f32, step_size: f32) -> f32 {
        match self {
            Self::Linear(_) => current + step_size,
            Self::Logarithmic(_) => current * step_size,
            Self::Exponential(_) => (current * step_size) + (target * (1.0 - step_size)),
        }
    }
}

impl Default for Smoother {
    fn default() -> Self {
        Self {
            style: SmoothingStyle::Exponential(200.),
            steps_left: i32,
            step_size: Default::default(),
            current: f32,
            target: Default::default(),
        }
    }
}

impl Iterator for Smoother {
    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(self.smoother.next())
    }
}

impl Clone for SmootherIter {
    fn clone(&self) -> Self {
        
        
        Self {
            style: self.style.clone(),
            steps_left: AtomicI32::new(self.steps_left.load(Ordering::Relaxed)),
            step_size: AtomicF32::new(self.step_size.load(Ordering::Relaxed)),
            current: AtomicF32::new(self.current.load(Ordering::Relaxed)),
            target: T::atomic_new(T::atomic_load(&self.target)),
        }
    }
}

impl<T: Smoothable> Smoother<T> {
    
    pub fn new(style: SmoothingStyle) -> Self {
        Self {
            style,
            ..Default::default()
        }
    }

    
    pub fn none() -> Self {
        Default::default()
    }

    
    
    #[inline]
    pub fn steps_left(&self) -> i32 {
        self.steps_left.load(Ordering::Relaxed)
    }

    
    
    #[inline]
    pub fn is_smoothing(&self) -> bool {
        self.steps_left() > 0
    }

    
    
    
    #[inline]
    pub fn iter(&self) -> SmootherIter<T> {
        SmootherIter { smoother: self }
    }

    
    pub fn reset(&self, value: T) {
        T::atomic_store(&self.target, value);
        self.current.store(value.to_f32(), Ordering::Relaxed);
        self.steps_left.store(0, Ordering::Relaxed);
    }

    
    pub fn set_target(&self, sample_rate: f32, target: T) {
        T::atomic_store(&self.target, target);

        let steps_left = self.style.num_steps(sample_rate) as i32;
        self.steps_left.store(steps_left, Ordering::Relaxed);

        let current = self.current.load(Ordering::Relaxed);
        let target_f32 = target.to_f32();
        self.step_size.store(
            if steps_left > 0 {
                self.style.step_size(current, target_f32, steps_left as u32)
            } else {
                0.0
            },
            Ordering::Relaxed,
        );
    }

    
    
    
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn next(&self) -> T {
        let target = T::atomic_load(&self.target);

        
        
        if self.steps_left.load(Ordering::Relaxed) > 0 {
            let current = self.current.load(Ordering::Relaxed);
            let target_f32 = target.to_f32();
            let step_size = self.step_size.load(Ordering::Relaxed);

            
            
            
            
            
            let old_steps_left = self.steps_left.fetch_sub(1, Ordering::Relaxed);
            let new = if old_steps_left == 1 {
                self.steps_left.store(0, Ordering::Relaxed);
                target_f32
            } else {
                self.style.next(current, target_f32, step_size)
            };
            self.current.store(new, Ordering::Relaxed);

            T::from_f32(new)
        } else {
            target
        }
    }

    
    
    
    
    #[inline]
    pub fn next_step(&self, steps: u32) -> T {

        let target = T::atomic_load(&self.target);

        if self.steps_left.load(Ordering::Relaxed) > 0 {
            let current = self.current.load(Ordering::Relaxed);
            let target_f32 = target.to_f32();
            let step_size = self.step_size.load(Ordering::Relaxed);

            
            
            
            
            
            let old_steps_left = self.steps_left.fetch_sub(steps as i32, Ordering::Relaxed);
            let new = if old_steps_left <= steps as i32 {
                self.steps_left.store(0, Ordering::Relaxed);
                target_f32
            } else {
                self.style.next_step(current, target_f32, step_size, steps)
            };
            self.current.store(new, Ordering::Relaxed);

            T::from_f32(new)
        } else {
            target
        }
    }

    
    
    
    
    pub fn previous_value(&self) -> T {
        T::from_f32(self.current.load(Ordering::Relaxed))
    }

    
    
    
    
    
    
    
    
    
    pub fn next_block(&self, block_values: &mut [T], block_len: usize) {
        self.next_block_exact(&mut block_values[..block_len])
    }

    
    pub fn next_block_exact(&self, block_values: &mut [T]) {
        let target = T::atomic_load(&self.target);

        
        
        
        
        
        let steps_left = self.steps_left.load(Ordering::Relaxed) as usize;
        let num_smoothed_values = block_values.len().min(steps_left);
        if num_smoothed_values > 0 {
            let mut current = self.current.load(Ordering::Relaxed);
            let target_f32 = target.to_f32();
            let step_size = self.step_size.load(Ordering::Relaxed);

            if num_smoothed_values == steps_left {
                
                
                block_values[..num_smoothed_values - 1].fill_with(|| {
                    current = self.style.next(current, target_f32, step_size);
                    T::from_f32(current)
                });

                
                
                current = target_f32.to_f32();
                block_values[num_smoothed_values - 1] = target;
            } else {
                block_values[..num_smoothed_values].fill_with(|| {
                    current = self.style.next(current, target_f32, step_size);
                    T::from_f32(current)
                });
            }

            block_values[num_smoothed_values..].fill(target);

            self.current.store(current, Ordering::Relaxed);
            self.steps_left
                .fetch_sub(num_smoothed_values as i32, Ordering::Relaxed);
        } else {
            block_values.fill(target);
        }
    }

    
    
    
    
    
    pub fn next_block_mapped(
        &self,
        block_values: &mut [T],
        block_len: usize,
        f: impl FnMut(usize, f32) -> T,
    ) {
        self.next_block_exact_mapped(&mut block_values[..block_len], f)
    }

    
    
    pub fn next_block_exact_mapped(
        &self,
        block_values: &mut [T],
        mut f: impl FnMut(usize, f32) -> T,
    ) {
        
        
        let target_f32 = T::atomic_load(&self.target).to_f32();

        let steps_left = self.steps_left.load(Ordering::Relaxed) as usize;
        let num_smoothed_values = block_values.len().min(steps_left);
        if num_smoothed_values > 0 {
            let mut current = self.current.load(Ordering::Relaxed);
            let step_size = self.step_size.load(Ordering::Relaxed);

            
            if num_smoothed_values == steps_left {
                for (idx, value) in block_values
                    .iter_mut()
                    .enumerate()
                    .take(num_smoothed_values - 1)
                {
                    current = self.style.next(current, target_f32, step_size);
                    *value = f(idx, current);
                }

                current = target_f32.to_f32();
                block_values[num_smoothed_values - 1] = f(num_smoothed_values - 1, target_f32);
            } else {
                for (idx, value) in block_values
                    .iter_mut()
                    .enumerate()
                    .take(num_smoothed_values)
                {
                    current = self.style.next(current, target_f32, step_size);
                    *value = f(idx, current);
                }
            }

            for (idx, value) in block_values
                .iter_mut()
                .enumerate()
                .skip(num_smoothed_values)
            {
                *value = f(idx, target_f32);
            }

            self.current.store(current, Ordering::Relaxed);
            self.steps_left
                .fetch_sub(num_smoothed_values as i32, Ordering::Relaxed);
        } else {
            for (idx, value) in block_values.iter_mut().enumerate() {
                *value = f(idx, target_f32);
            }
        }
    }
}

impl Smoothable for f32 {
    type Atomic = AtomicF32;

    #[inline]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline]
    fn from_f32(value: f32) -> Self {
        value
    }

    #[inline]
    fn atomic_new(value: Self) -> Self::Atomic {
        AtomicF32::new(value)
    }

    #[inline]
    fn atomic_load(this: &Self::Atomic) -> Self {
        this.load(Ordering::Relaxed)
    }

    #[inline]
    fn atomic_store(this: &Self::Atomic, value: Self) {
        this.store(value, Ordering::Relaxed)
    }
}

impl Smoothable for i32 {
    type Atomic = AtomicI32;

    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline]
    fn from_f32(value: f32) -> Self {
        value.round() as i32
    }

    #[inline]
    fn atomic_new(value: Self) -> Self::Atomic {
        AtomicI32::new(value)
    }

    #[inline]
    fn atomic_load(this: &Self::Atomic) -> Self {
        this.load(Ordering::Relaxed)
    }

    #[inline]
    fn atomic_store(this: &Self::Atomic, value: Self) {
        this.store(value, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn linear_f32_next_equivalence() {
        let style = SmoothingStyle::Linear(100.0);

        let mut current = 0.4;
        let target = 0.8;
        let steps = 15;
        let step_size = style.step_size(current, target, steps);

        let expected_result = style.next_step(current, target, step_size, steps);
        for _ in 0..steps {
            current = style.next(current, target, step_size);
        }

        approx::assert_relative_eq!(current, expected_result, epsilon = 1e-5);
    }

    #[test]
    fn logarithmic_f32_next_equivalence() {
        let style = SmoothingStyle::Logarithmic(100.0);

        let mut current = 0.4;
        let target = 0.8;
        let steps = 15;
        let step_size = style.step_size(current, target, steps);

        let expected_result = style.next_step(current, target, step_size, steps);
        for _ in 0..steps {
            current = style.next(current, target, step_size);
        }

        approx::assert_relative_eq!(current, expected_result, epsilon = 1e-5);
    }

    #[test]
    fn exponential_f32_next_equivalence() {
        let style = SmoothingStyle::Exponential(100.0);

        let mut current = 0.4;
        let target = 0.8;
        let steps = 15;
        let step_size = style.step_size(current, target, steps);

        let expected_result = style.next_step(current, target, step_size, steps);
        for _ in 0..steps {
            current = style.next(current, target, step_size);
        }

        approx::assert_relative_eq!(current, expected_result, epsilon = 1e-5);
    }

    #[test]
    fn linear_f32_smoothing() {
        let smoother: Smoother<f32> = Smoother::new(SmoothingStyle::Linear(100.0));
        smoother.reset(10.0);
        assert_eq!(smoother.next(), 10.0);

        
        
        smoother.set_target(100.0, 20.0);
        for _ in 0..(10 - 2) {
            smoother.next();
        }
        assert_ne!(smoother.next(), 20.0);
        assert_eq!(smoother.next(), 20.0);
    }

    #[test]
    fn linear_i32_smoothing() {
        let smoother: Smoother<i32> = Smoother::new(SmoothingStyle::Linear(100.0));
        smoother.reset(10);
        assert_eq!(smoother.next(), 10);

        
        smoother.set_target(100.0, 20);
        for _ in 0..(10 - 2) {
            smoother.next();
        }
        assert_ne!(smoother.next(), 20);
        assert_eq!(smoother.next(), 20);
    }

    #[test]
    fn logarithmic_f32_smoothing() {
        let smoother: Smoother<f32> = Smoother::new(SmoothingStyle::Logarithmic(100.0));
        smoother.reset(10.0);
        assert_eq!(smoother.next(), 10.0);

        
        
        smoother.set_target(100.0, 20.0);
        for _ in 0..(10 - 2) {
            smoother.next();
        }
        assert_ne!(smoother.next(), 20.0);
        assert_eq!(smoother.next(), 20.0);
    }

    #[test]
    fn logarithmic_i32_smoothing() {
        let smoother: Smoother<i32> = Smoother::new(SmoothingStyle::Logarithmic(100.0));
        smoother.reset(10);
        assert_eq!(smoother.next(), 10);

        
        smoother.set_target(100.0, 20);
        for _ in 0..(10 - 2) {
            smoother.next();
        }
        assert_ne!(smoother.next(), 20);
        assert_eq!(smoother.next(), 20);
    }

    
    #[test]
    fn skipping_linear_f32_smoothing() {
        let smoother: Smoother<f32> = Smoother::new(SmoothingStyle::Linear(100.0));
        smoother.reset(10.0);
        assert_eq!(smoother.next(), 10.0);

        smoother.set_target(100.0, 20.0);
        smoother.next_step(8);
        assert_ne!(smoother.next(), 20.0);
        assert_eq!(smoother.next(), 20.0);
    }

    
    #[test]
    fn skipping_linear_i32_smoothing() {
        let smoother: Smoother<i32> = Smoother::new(SmoothingStyle::Linear(100.0));
        smoother.reset(10);
        assert_eq!(smoother.next(), 10);

        smoother.set_target(100.0, 20);
        smoother.next_step(8);
        assert_ne!(smoother.next(), 20);
        assert_eq!(smoother.next(), 20);
    }

    
    #[test]
    fn skipping_logarithmic_f32_smoothing() {
        let smoother: Smoother<f32> = Smoother::new(SmoothingStyle::Logarithmic(100.0));
        smoother.reset(10.0);
        assert_eq!(smoother.next(), 10.0);

        smoother.set_target(100.0, 20.0);
        smoother.next_step(8);
        assert_ne!(smoother.next(), 20.0);
        assert_eq!(smoother.next(), 20.0);
    }

    
    #[test]
    fn skipping_logarithmic_i32_smoothing() {
        let smoother: Smoother<i32> = Smoother::new(SmoothingStyle::Logarithmic(100.0));
        smoother.reset(10);
        assert_eq!(smoother.next(), 10);

        smoother.set_target(100.0, 20);
        smoother.next_step(8);
        assert_ne!(smoother.next(), 20);
        assert_eq!(smoother.next(), 20);
    }

    
}