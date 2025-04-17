extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time::Duration};

#[test]
fn it_doesnt_crash() -> std::io::Result<()> {
    let attempts = 8;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);

    for duration in &Backoff::new(attempts, min, max) {
        println!("duration {:?}", duration);
        match fs::read_to_string("README.md") {
            Ok(_string) => return Ok(()),
            Err(_) => {
                if let Some(duration) = duration {
                    thread::sleep(duration);
                }
            }
        }
    }

    unreachable!();
}

#[test]
fn it_correctly_handles_max_attempts() {
    let attempts = 3;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);

    let mut counter = 0;
    let mut slept = 0;

    for duration in &Backoff::new(attempts, min, max) {
        counter += 1;
        if let Some(duration) = duration {
            thread::sleep(duration);
            slept += 1;
        }
    }
    assert_eq!(slept, attempts - 1);
    assert_eq!(counter, attempts);
}

#[test]
fn it_completes_into_iter() {
    let attempts = 3;

    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let mut counter = 0;
    let mut slept = 0;
    for duration in Backoff::new(attempts, min, max) {
        counter += 1;
        if let Some(duration) = duration {
            thread::sleep(duration);
            slept += 1;
        }
    }
    assert_eq!(slept, attempts - 1);
    assert_eq!(counter, attempts);
}

#[test]
fn it_handles_max_backoff() {
    let attempts = u32::MAX;
    let min = Duration::MAX;
    let mut counter = 0u32;
    for _ in &Backoff::new(attempts, min, None) {
        counter += 1;
        if counter > 32 {
            break;
        }
    }
}

#[test]
fn it_handles_zero_attempts() {
    let mut count = 0;
    let attempts = 0;
    for duration in &Backoff::new(attempts, Duration::from_millis(10), None) {
        assert!(duration.is_none());
        count += 1;
    }
    assert_eq!(count, 0);
}

#[test]
fn it_handles_no_jitter() {
    let mut backoff = Backoff::default();
    backoff.set_jitter(0.0);

    // Exercise the iterator a number of times
    let mut durations = backoff.into_iter();
    durations.next();
    durations.next();
    durations.next();
}

#[test]
fn it_has_the_right_min_value() {
    // Set up a backoff with predictable values
    let mut backoff = Backoff::new(4, Duration::from_secs(1), None);
    backoff.set_factor(2);
    backoff.set_jitter(0.0); // No jitter to make test deterministic

    let mut durations = backoff.into_iter();
    assert_eq!(
        durations.next(),
        Some(Some(Duration::from_secs(1))),
        "First interval should equal the min value, not double it"
    );
    assert_eq!(
        durations.next(),
        Some(Some(Duration::from_secs(2))),
        "Second interval should be min value * factor"
    );
    assert_eq!(
        durations.next(),
        Some(Some(Duration::from_secs(4))),
        "Third interval should be min value * factor^2"
    );
}

/// Tests that we uphold the invariant of ever-increasing sleep values.
#[test]
fn it_generates_ascending_sleep_values() {
    let mut backoff = Backoff::new(20, Duration::from_secs(1), None);
    backoff.set_factor(2);
    backoff.set_jitter(0.0); // No jitter to make test deterministic

    let mut max = Duration::from_millis(0);
    for duration in backoff {
        if let Some(duration) = duration {
            assert!(duration >= max);
            max = duration;
        }
    }
}
