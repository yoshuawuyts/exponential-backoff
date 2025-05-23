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
//!         Err(err) => match duration {
//!             Some(duration) => thread::sleep(duration),
//!             None => return Err(err),
//!         }
//!     }
//! }
//! # Ok(()) }
//! ```

mod into_iter;

use std::time::Duration;

pub use crate::into_iter::IntoIter;

/// Exponential backoff type.
#[derive(Debug, Clone)]
pub struct Backoff {
    max_attempts: u32,
    min: Duration,
    max: Duration,
    jitter: f32,
    factor: u32,
}
impl Backoff {
    /// Create a new instance of `Backoff`.
    ///
    /// # Examples
    ///
    /// With an explicit max duration:
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let backoff = Backoff::new(3, Duration::from_millis(100), Duration::from_secs(10));
    /// assert_eq!(backoff.max_attempts(), 3);
    /// assert_eq!(backoff.min(), &Duration::from_millis(100));
    /// assert_eq!(backoff.max(), &Duration::from_secs(10));
    /// ```
    ///
    /// With no max duration (sets it to 584,942,417,355 years):
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let backoff = Backoff::new(5, Duration::from_millis(50), None);
    /// # assert_eq!(backoff.max_attempts(), 5);
    /// # assert_eq!(backoff.min(), &Duration::from_millis(50));
    /// assert_eq!(backoff.max(), &Duration::MAX);
    /// ```
    #[inline]
    pub fn new(max_attempts: u32, min: Duration, max: impl Into<Option<Duration>>) -> Self {
        Self {
            max_attempts,
            min,
            max: max.into().unwrap_or(Duration::MAX),
            jitter: 0.3,
            factor: 2,
        }
    }

    /// Get the min duration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let mut backoff = Backoff::default();
    /// assert_eq!(backoff.min(), &Duration::from_millis(100));
    /// ```
    pub fn min(&self) -> &Duration {
        &self.min
    }

    /// Set the min duration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let mut backoff = Backoff::default();
    /// backoff.set_min(Duration::from_millis(50));
    /// assert_eq!(backoff.min(), &Duration::from_millis(50));
    /// ```
    #[inline]
    pub fn set_min(&mut self, min: Duration) {
        self.min = min;
    }

    /// Get the max duration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let mut backoff = Backoff::default();
    /// assert_eq!(backoff.max(), &Duration::from_secs(10));
    /// ```
    pub fn max(&self) -> &Duration {
        &self.max
    }

    /// Set the max duration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    /// use std::time::Duration;
    ///
    /// let mut backoff = Backoff::default();
    /// backoff.set_max(Duration::from_secs(30));
    /// assert_eq!(backoff.max(), &Duration::from_secs(30));
    /// ```
    #[inline]
    pub fn set_max(&mut self, max: Duration) {
        self.max = max;
    }

    /// Get the maximum number of attempts
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// assert_eq!(backoff.max_attempts(), 3);
    /// ```
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Set the maximum number of attempts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// backoff.set_max_attempts(5);
    /// assert_eq!(backoff.max_attempts(), 5);
    /// ```
    pub fn set_max_attempts(&mut self, max_attempts: u32) {
        self.max_attempts = max_attempts;
    }

    /// Get the jitter factor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// assert_eq!(backoff.jitter(), 0.3);
    /// ```
    pub fn jitter(&self) -> f32 {
        self.jitter
    }

    /// Set the amount of jitter per backoff.
    ///
    /// # Panics
    ///
    /// This method panics if a number smaller than `0` or larger than `1` is
    /// provided.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// backoff.set_jitter(0.3);  // default value
    /// backoff.set_jitter(0.0);  // min value
    /// backoff.set_jitter(1.0);  // max value
    /// ```
    #[inline]
    pub fn set_jitter(&mut self, jitter: f32) {
        assert!(
            jitter >= 0f32 && jitter <= 1f32,
            "<exponential-backoff>: jitter must be between 0 and 1."
        );
        self.jitter = jitter;
    }

    /// Get the growth factor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// assert_eq!(backoff.factor(), 2);
    /// ```
    pub fn factor(&self) -> u32 {
        self.factor
    }

    /// Set the growth factor for each iteration of the backoff.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use exponential_backoff::Backoff;
    ///
    /// let mut backoff = Backoff::default();
    /// backoff.set_factor(3);
    /// assert_eq!(backoff.factor(), 3);
    /// ```
    #[inline]
    pub fn set_factor(&mut self, factor: u32) {
        self.factor = factor;
    }

    /// Create an iterator.
    #[inline]
    pub fn iter(&self) -> IntoIter {
        IntoIter::new(self.clone())
    }
}

/// Implements the `IntoIterator` trait for borrowed `Backoff` instances.
///
/// # Examples
///
/// ```rust
/// use exponential_backoff::Backoff;
/// use std::time::Duration;
///
/// let backoff = Backoff::default();
/// let mut count = 0;
///
/// for duration in &backoff {
///     count += 1;
///     if count > 1 {
///         break;
///     }
/// }
/// ```
impl<'b> IntoIterator for &'b Backoff {
    type Item = Option<Duration>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.clone())
    }
}

/// Implements the `IntoIterator` trait for owned `Backoff` instances.
///
/// # Examples
///
/// ```rust
/// use exponential_backoff::Backoff;
/// use std::time::Duration;
///
/// let backoff = Backoff::default();
/// let mut count = 0;
///
/// for duration in backoff {
///     count += 1;
///     if count > 1 {
///         break;
///     }
/// }
/// ```
impl IntoIterator for Backoff {
    type Item = Option<Duration>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

/// Implements the `Default` trait for `Backoff`.
///
/// # Examples
///
/// ```rust
/// use exponential_backoff::Backoff;
/// use std::time::Duration;
///
/// let backoff = Backoff::default();
/// assert_eq!(backoff.max_attempts(), 3);
/// assert_eq!(backoff.min(), &Duration::from_millis(100));
/// assert_eq!(backoff.max(), &Duration::from_secs(10));
/// assert_eq!(backoff.jitter(), 0.3);
/// assert_eq!(backoff.factor(), 2);
/// ```
impl Default for Backoff {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            min: Duration::from_millis(100),
            max: Duration::from_secs(10),
            jitter: 0.3,
            factor: 2,
        }
    }
}
