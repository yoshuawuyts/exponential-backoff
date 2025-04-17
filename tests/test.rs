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
fn it_completes_iter() {
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
