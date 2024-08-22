//! An exponential backoff generator with jitter. Serves as a building block to
//! implement custom retry functions.
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
//! let attempts = 3;
//! let min = Duration::from_millis(100);
//! let max = Duration::from_secs(10);
//!
//! for duration in Backoff::new(attempts, min, max) {
//!     match fs::read_to_string("README.md") {
//!         Ok(s) => {
//!             println!("{}", s);
//!             break;
//!         }
//!         Err(err) => {
//!             if let Some(duration) = duration {
//!                 thread::sleep(duration);
//!             } else {
//!                 return Err(err);        
//!             }
//!         }
//!     }
//! }
//! # Ok(()) }
//! ```

use std::time::Duration;

pub use into_iter::IntoIter;
pub use iter::Iter;

mod into_iter;
mod iter;

/// Exponential backoff type.
#[derive(Debug, Clone)]
pub struct Backoff {
    attempts: u32,
    min: Duration,
    max: Option<Duration>,
    jitter: f32,
    factor: u32,
}

impl Backoff {
    /// Create a new instance.
    #[inline]
    pub fn new(retries: u32, min: Duration, max: impl Into<Option<Duration>>) -> Self {
        Self {
            attempts: retries,
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

    /// Create an iterator.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl<'b> IntoIterator for &'b Backoff {
    type Item = Option<Duration>;
    type IntoIter = Iter<'b>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl IntoIterator for Backoff {
    type Item = Option<Duration>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
