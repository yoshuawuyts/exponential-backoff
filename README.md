# exponential-backoff
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Exponential backoff generator. Serves as a building block to implement custom
retry functions.

- [Documentation][8]
- [Crates.io][2]

## Why?
When an network requests times out, often the best way to solve it is to try
again. But trying again straight away might at best cause some network overhead,
and at worst a full fledged DDOS. So we have to be responsible about it.

A good explanation of retry strategies can be found on the [Stripe
blog](https://stripe.com/blog/idempotency).

## Usage
Here we try and read a file from disk, and try again if it fails. A more
realistic scenario would probably to perform an HTTP request, but the approach
should be similar.

```rust
extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time::Duration};

let retries = 8;
let backoff = Backoff::new(retries)
  .timeout_range(Duration::from_millis(100), Duration::from_secs(10))
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

## See Also
- [segment/backo](https://github.com/segmentio/backo)
- [stripe/stripe-ruby](https://github.com/stripe/stripe-ruby/blob/1bb9ac48b916b1c60591795cdb7ba6d18495e82d/lib/stripe/stripe_client.rb#L78-L92)

## Further Reading
- https://stripe.com/blog/idempotency
- https://en.wikipedia.org/wiki/Exponential_backoff

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/exponential-backoff.svg?style=flat-square
[2]: https://crates.io/crates/exponential-backoff
[3]: https://img.shields.io/travis/yoshuawuyts/exponential-backoff.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/exponential-backoff
[5]: https://img.shields.io/crates/d/exponential-backoff.svg?style=flat-square
[6]: https://crates.io/crates/exponential-backoff
[7]: https://docs.rs/exponential-backoff/badge.svg
[8]: https://docs.rs/exponential-backoff
