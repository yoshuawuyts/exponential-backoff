use super::Backoff;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{iter, time};

/// Immutable iterator.
#[derive(Debug, Clone)]
pub struct Iter<'b> {
  inner: &'b Backoff,
  rng: ThreadRng,
  retry_count: u32,
}

impl<'b> Iter<'b> {
  #[inline]
  pub fn new(inner: &'b Backoff) -> Self {
    Self {
      inner,
      retry_count: 0,
      rng: thread_rng(),
    }
  }
}

impl<'b> iter::Iterator for Iter<'b> {
  type Item = Option<time::Duration>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.retry_count == self.inner.retries {
      self.retry_count += 1;
      return Some(None);
    } else if self.retry_count == self.inner.retries + 1 {
      return None;
    }

    // Create exponential duration.
    let exponent = self.inner.factor.pow(self.retry_count);
    let mut duration = self.inner.min * exponent;

    self.retry_count += 1;

    // Apply jitter. Uses multiples of 100 to prevent relying on floats.
    let jitter_factor = (self.inner.jitter * 100f32) as u32;
    let random: u32 = self.rng.gen_range(0, jitter_factor * 2);
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
    duration = duration.min(self.inner.max);
    duration = duration.max(self.inner.min);

    Some(Some(duration))
  }
}
