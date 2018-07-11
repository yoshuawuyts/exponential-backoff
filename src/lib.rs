#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate rand;

mod iter;

use std::time;

/// Exponential backoff struct.
#[derive(Debug, Clone)]
pub struct Backoff {
  retries: u32,
  min: time::Duration,
  max: time::Duration,
  jitter: f32,
  factor: u32,
}

impl Backoff {
  /// Create a new instance.
  #[inline]
  pub fn new(retries: u32) -> Self {
    Self {
      retries,
      min: time::Duration::from_millis(100),
      max: time::Duration::from_secs(10),
      jitter: 0.3,
      factor: 2,
    }
  }

  /// Set the min and max durations.
  #[inline]
  pub fn timeout_range(
    mut self,
    min: time::Duration,
    max: time::Duration,
  ) -> Self {
    self.min = min;
    self.max = max;
    self
  }

  /// Set the amount of jitter per backoff.
  ///
  /// ## Panics
  /// This method panics if a number smaller than `0` or larger than `1` is
  /// provided.
  #[inline]
  pub fn jitter(mut self, jitter: f32) -> Self {
    assert!(
      jitter > 0f32 && jitter < 1f32,
      "<exponential-backoff>: jitter must be between 0 and 1."
    );
    self.jitter = jitter;
    self
  }

  /// Set the growth factor for each iteration of the backoff.
  #[inline]
  pub fn factor(mut self, factor: u32) -> Self {
    self.factor = factor;
    self
  }

  /// Create an iterator.
  #[inline]
  pub fn iter(&self) -> iter::Iter {
    iter::Iter::new(self)
  }
}
