use super::Backoff;
use fastrand::Rng;
use std::{iter, time::Duration};

/// An exponential backoff iterator.
#[derive(Debug, Clone)]
pub struct IntoIter {
    inner: Backoff,
    rng: Rng,
    attempts: u32,
}

impl IntoIter {
    pub(crate) fn new(inner: Backoff) -> Self {
        Self {
            attempts: 0,
            rng: Rng::new(),
            inner,
        }
    }
}

impl iter::Iterator for IntoIter {
    type Item = Option<Duration>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Check whether we've exceeded the number of attempts,
        // or whether we're on our last attempt. We don't want to sleep after
        // the last attempt.
        if self.attempts == self.inner.max_attempts {
            return None;
        } else if self.attempts == self.inner.max_attempts - 1 {
            self.attempts = self.attempts.saturating_add(1);
            return Some(None);
        }

        self.attempts = self.attempts.saturating_add(1);

        // Create exponential duration.
        let exponent = self.inner.factor.saturating_pow(self.attempts);
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
        duration = duration.clamp(self.inner.min, self.inner.max);

        Some(Some(duration))
    }
}
