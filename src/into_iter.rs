use super::Backoff;
use fastrand::Rng;
use std::{iter, time::Duration};

/// An exponential backoff iterator.
#[derive(Debug, Clone)]
pub struct IntoIter {
    inner: Backoff,
    rng: Rng,
    retry_count: u32,
}

impl IntoIter {
    pub(crate) fn new(inner: Backoff) -> Self {
        Self::with_count(inner, 0)
    }

    pub(crate) fn with_count(inner: Backoff, retry_count: u32) -> Self {
        Self {
            inner,
            retry_count,
            rng: Rng::new(),
        }
    }
}

impl iter::Iterator for IntoIter {
    type Item = Option<Duration>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let retries = self.inner.retries.saturating_add(1);
        // Check whether we've exceeded the number of retries.
        // We use `saturating_add` to prevent overflowing on `int::MAX + 1`.
        if self.retry_count == retries {
            return None;
        }

        self.retry_count = self.retry_count.saturating_add(1);

        // This is the last time we should retry, but we don't want to sleep
        // after this iteration.
        if self.retry_count == retries {
            return Some(None);
        }

        // Create exponential duration.
        let exponent = self.inner.factor.saturating_pow(self.retry_count);
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
