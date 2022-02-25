use super::Backoff;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{iter, time};

/// An exponential backoff iterator.
#[derive(Debug, Clone)]
pub struct Iter<'b> {
    inner: &'b Backoff,
    rng: StdRng,
    retry_count: u32,
}

impl<'b> Iter<'b> {
    pub(crate) fn new(inner: &'b Backoff) -> Self {
        Self::with_count(inner, 0)
    }

    pub(crate) fn with_count(inner: &'b Backoff, retry_count: u32) -> Self {
        Self {
            inner,
            retry_count,
            rng: StdRng::from_entropy(),
        }
    }
}

impl<'b> iter::Iterator for Iter<'b> {
    type Item = time::Duration;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Check whether we've exceeded the number of retries.
        // We use `saturating_add` to prevent overflowing on `int::MAX + 1`.
        if self.retry_count == self.inner.retries.saturating_add(1) {
            return None;
        }

        // Create exponential duration.
        let exponent = self.inner.factor.pow(self.retry_count);
        let mut duration = self.inner.min * exponent;

        self.retry_count += 1;

        // Apply jitter. Uses multiples of 100 to prevent relying on floats.
        let jitter_factor = (self.inner.jitter * 100f32) as u32;
        let random: u32 = self.rng.gen_range(0..jitter_factor * 2);
        duration *= 100;
        if random < jitter_factor {
            let jitter = (duration * random) / 100;
            duration -= jitter;
        } else {
            let jitter = (duration * (random / 2)) / 100;
            duration += jitter;
        };
        duration /= 100;

        // Make sure it doesn't exceed upper / lower bounds.
        if let Some(max) = self.inner.max {
            duration = duration.min(max);
        }
        duration = duration.max(self.inner.min);

        Some(duration)
    }
}
