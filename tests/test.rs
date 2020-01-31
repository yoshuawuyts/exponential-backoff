extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time::Duration};

#[test]
fn doesnt_crash() -> std::io::Result<()> {
    let retries = 8;
    let min = Duration::from_millis(100);
    let max = Duration::from_secs(10);
    let backoff = Backoff::new(retries, min, max);

    for duration in &backoff {
        println!("duration {:?}", duration);
        match fs::read_to_string("README.md") {
            Ok(_string) => return Ok(()),
            Err(_) => thread::sleep(duration),
        }
    }

    unreachable!();
}

#[test]
fn iterator_completes() {
  let retries = 3;
  let backoff = Backoff::new(retries);
  let mut counter = 0;
  for duration in &backoff {
    counter += 1;
    if let Some(duration) = duration {
      thread::sleep(duration);
    }
  }
  assert_eq!(counter, 1 + retries);
}
