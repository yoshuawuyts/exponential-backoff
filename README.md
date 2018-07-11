# exponential-backoff
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Exponential backoff generator. Serves as a building block to implement custom
retry functions.

- [Documentation][8]
- [Crates.io][2]

## Usage

```rust
extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread};

let retries = 8;
let backoff = Backoff::new(retries)
  .range(Duration::from_millis(100), Duration::from_secs(10))
  .jitter(0.3)
  .factor(2);

for duration in &backoff {
  match fs::read_to_string("README.md") {
    Ok(string) => return Ok(string),
    Err(err) => match duration {
      Some(duration) => thread::sleep(duration),
      None => return err,
    }
  }
}
```

## Installation
```sh
$ cargo add exponential-backoff
```

## Further Reading
- [segment/backo](https://github.com/segmentio/backo)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/back.svg?style=flat-square
[2]: https://crates.io/crates/back
[3]: https://img.shields.io/travis/yoshuawuyts/back.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/back
[5]: https://img.shields.io/crates/d/back.svg?style=flat-square
[6]: https://crates.io/crates/back
[7]: https://docs.rs/back/badge.svg
[8]: https://docs.rs/back
