//! Exponential backoff generator. Serves as a building block to implement custom
//! retry functions.
//!
//! # Why?
//! When an network requests times out, often the best way to solve it is to try
//! again. But trying again straight away might at best cause some network overhead,
//! and at worst a full fledged DDOS. So we have to be responsible about it.
//!
//! A good explanation of retry strategies can be found on the [Stripe
//! blog](https://stripe.com/blog/idempotency).
//!
//! # Usage
//! Here we try and read a file from disk, and try again if it fails. A more
//! realistic scenario would probably to perform an HTTP request, but the approach
//! should be similar.
//!
//! ```rust
//! # fn retry() -> std::io::Result<()> {
//! use exponential_backoff::Backoff;
//! use std::{fs, thread, time::Duration};
//!
//! let retries = 8;
//! let min = Duration::from_millis(100);
//! let max = Duration::from_secs(10);
//! let backoff = Backoff::new(retries, min, max);
//!
//! for duration in &backoff {
//!     match fs::read_to_string("README.md") {
//!         Ok(s) => {
//!             println!("{}", s);
//!             break;
//!         }
//!         Err(err) => thread::sleep(duration),
//!     }
//! }
//! # Ok(()) }
//! ```

use std::iter;
use std::time::Duration;

pub use iterator::Iter;

mod iterator;

/// Exponential backoff type.
#[derive(Debug, Clone)]
pub struct Backoff {
    retries: u32,
    min: Duration,
    max: Option<Duration>,
    jitter: f32,
    factor: u32,
}

impl Backoff {
    /// Create a new instance.
    ///
    /// # Panics
    ///
    /// This method panics if the retry count is set to 0.
    #[inline]
    pub fn new(retries: u32, min: Duration, max: impl Into<Option<Duration>>) -> Self {
        assert!(
            retries >= 1,
            "<exponential-backoff>: retries should be 1 or higher."
        );

        Self {
            retries,
            min,
            max: max.into(),
            jitter: 0.3,
            factor: 2,
        }
    }

    /// Set the min duration.
    #[inline]
    pub fn set_min(&mut self, min: Duration) {
        self.min = min;
    }

    /// Set the max duration.
    #[inline]
    pub fn set_max(&mut self, max: Option<Duration>) {
        self.max = max;
    }

    /// Set the amount of jitter per backoff.
    ///
    /// ## Panics
    /// This method panics if a number smaller than `0` or larger than `1` is
    /// provided.
    #[inline]
    pub fn set_jitter(&mut self, jitter: f32) {
        assert!(
            jitter > 0f32 && jitter < 1f32,
            "<exponential-backoff>: jitter must be between 0 and 1."
        );
        self.jitter = jitter;
    }

    /// Set the growth factor for each iteration of the backoff.
    #[inline]
    pub fn set_factor(&mut self, factor: u32) {
        self.factor = factor;
    }

    /// Get the next value for the retry count.
    pub fn next(&self, retry_attempt: u32) -> Option<Duration> {
        Iter::with_count(self, retry_attempt).next()
    }

    /// Create an iterator.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl<'b> iter::IntoIterator for &'b Backoff {
    type Item = Duration;
    type IntoIter = Iter<'b>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
