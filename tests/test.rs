extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time};

#[test]
fn doesnt_crash() -> std::io::Result<()> {
  let retries = 8;
  let backoff = Backoff::new(retries)
    .timeout_range(
      time::Duration::from_millis(100),
      time::Duration::from_secs(10),
    )
    .jitter(0.3)
    .factor(2);

  for duration in backoff.iter() {
    println!("duration {:?}", duration);
    match fs::read_to_string("README.md") {
      Ok(string) => return Ok(()),
      Err(err) => match duration {
        Some(duration) => thread::sleep(duration),
        None => return Err(err),
      },
    }
  }

  unreachable!();
}
