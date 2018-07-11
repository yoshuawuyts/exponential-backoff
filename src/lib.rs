#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

use std::time;

/// Exponential backoff struct.
pub struct Backoff {
  // last_attempt: time::SystemTime,
  // attempt_count: u16,
  retries: u16,
  min: time::Duration,
  max: time::Duration,
  jitter: f32,
  factor: u16,
}

impl Backoff {
  /// Create a new instance.
  pub fn new(retries: u16) -> Self {
    Self {
      retries,
      last: time::SystemTime::now(),
      min: time::Duration::from_millis(100),
      max: time::Duration::from_secs(10),
      jitter: 0.3,
      factor: 2,
    }
  }

  /// Set the min and max retry times.
  pub fn range(
    &mut self,
    min: time::Duration,
    max: time::Duration,
  ) -> &mut Self {
    self.min = min;
    self.max = max;
    self
  }

  /// Set the amount of jitter per backoff.
  pub fn jitter(&mut self, jitter: f32) -> &mut Self {
    self.jitter = jitter;
    self
  }

  /// Set the growth factor for each iteration of the backoff.
  pub fn factor(&mut self, factor: u16) -> &mut Self {
    self.factor = factor;
    self
  }
}
