use super::Backoff;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{iter, time};

/// Iterator
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
      return Some(None);
    } else if self.retry_count == self.inner.retries + 1 {
      return None;
    }

    // Create exponential duration.
    self.retry_count += 1;
    let exponent = self.inner.factor.pow(self.retry_count);
    let mut duration = self.inner.min * exponent;

    // Apply jitter. Uses multiples of 100 to prevent relying on floats.
    let jitter_factor = self.inner.jitter as u32 * 100;
    let random: u32 = self.rng.gen_range(0, jitter_factor * 2);
    duration *= 100;
    if random < jitter_factor {
      let jitter = duration * random;
      duration -= jitter;
    } else {
      let jitter = duration * (random / 2);
      duration += jitter;
    };
    duration /= 100;

    // Make sure it doesn't exceed upper / lower bounds.
    duration = duration.min(self.inner.max);
    duration = duration.max(self.inner.min);

    Some(Some(duration))
  }
}
