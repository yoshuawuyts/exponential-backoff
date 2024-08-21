extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time::Duration};

#[test]
fn doesnt_crash() -> std::io::Result<()> {
    let retries = 8;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let backoff = Backoff::new(retries, min, max);

    for duration in &backoff {
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
fn iter_completes() {
    let retries = 3;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let backoff = Backoff::new(retries, min, max);
    let mut counter = 0;
    let mut slept = 0;
    for duration in &backoff {
        counter += 1;
        if let Some(duration) = duration {
            thread::sleep(duration);
            slept += 1;
        }
    }
    assert_eq!(slept, retries);
    assert_eq!(counter, 1 + retries);
}

#[test]
fn into_iter_completes() {
    let retries = 3;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let backoff = Backoff::new(retries, min, max);
    let mut counter = 0;
    let mut slept = 0;
    for duration in backoff {
        counter += 1;
        if let Some(duration) = duration {
            thread::sleep(duration);
            slept += 1;
        }
    }
    assert_eq!(slept, retries);
    assert_eq!(counter, 1 + retries);
}

#[test]
fn max_backoff_without_crashing() {
    let retries = u32::MAX;
    let min = Duration::MAX;
    let backoff = Backoff::new(retries, min, None);

    let mut counter = 0u32;
    for _ in &backoff {
        counter += 1;
        if counter > 32 {
            break;
        }
    }
}

#[test]
fn no_retry() {
    let mut count = 0;
    for duration in &Backoff::new(0, Duration::from_millis(10), None) {
        assert!(duration.is_none());
        count += 1;
    }
    assert_eq!(count, 1);
}
