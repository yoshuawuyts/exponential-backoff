use super::Backoff;
use fastrand::Rng;
use std::{iter, time};

/// An exponential backoff iterator.
#[derive(Debug, Clone)]
pub struct IntoIter {
    inner: Backoff,
    rng: Rng,
    attempt_count: u32,
}

impl IntoIter {
    pub(crate) fn new(inner: Backoff) -> Self {
        Self {
            attempt_count: inner.retries,
            rng: Rng::new(),
            inner,
        }
    }
}

impl iter::Iterator for IntoIter {
    type Item = Option<time::Duration>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Check whether we're done iterating.
        if self.attempt_count == 0 {
            return None;
        }

        // This is the last time we should retry, but we don't want to sleep
        // after this iteration.
        if self.attempt_count == 1 {
            self.attempt_count -= 1;
            return Some(None);
        } else {
            self.attempt_count -= 1;
        }

        // Create exponential duration.
        let exponent = self.inner.factor.saturating_pow(self.attempt_count);
        let duration = self.inner.min.saturating_mul(exponent);

        // Apply jitter. Uses multiples of 100 to prevent relying on floats.
        let jitter_factor = (self.inner.jitter * 100f32) as u32;
        let random = self.rng.u32(0..jitter_factor * 2);
        let mut duration = duration.saturating_mul(100);
        if random < jitter_factor {
            let jitter = duration.saturating_mul(random) / 100;
            duration = duration.saturating_sub(jitter);
        } else {
            let jitter = duration.saturating_mul(random / 2) / 100;
            duration = duration.saturating_add(jitter);
        };
        duration /= 100;

        // Make sure it doesn't exceed upper / lower bounds.
        if let Some(max) = self.inner.max {
            duration = duration.min(max);
        }

        duration = duration.max(self.inner.min);

        Some(Some(duration))
    }
}
