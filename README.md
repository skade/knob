% README

# Knob

[![Build Status](https://travis-ci.org/skade/knob.png)](https://travis-ci.org/skade/knob)

Knob is a simple settings structure that is convenient to work with. It allows you to read and write to in a structured manner. It is intended for simple settings and data from the outside world.

It also handles argument lists.

## Usage

```rust
extern crate knob;

use std::io::net::ip::IpAddr;
use knob::Settings;

fn main() {
  let mut settings = Settings::new();
  settings.set("ip", "::0.0.0.1");
  let socket: IpAddr = settings.fetch("ip").unwrap();
  assert_eq!(socket.to_str(), "::0.0.0.1".to_owned());
}
```

```rust
extern crate getopts;
extern crate knob;

use getopts::optopt;
use knob::Settings;

fn main() {
  let mut settings = Settings::new();
  settings.opt(optopt("p", "port", "the port to bind to", "4000"));
  settings.opt(optopt("e", "environment", "the environment to run in", ""));
  let errors = settings.load_os_args();
  if errors.is_some() {
    println!("{}", settings.usage("Try one of these:"));
  }
}
```

For more elaborate examples, see [the API documentation](http://skade.github.io/knob/doc/knob/index.html).

## Version policy

This library follows [semver](http://semver.org), with a notable, but temporary
exception, as Rust is currently still in flux:

Changes to maintain compatibility with the current development state of Rust are
considered patches, as long as the resulting API stays faithful to the previous
API and doesn't add or loose any features.

## Contributors

Original Author: [Florian Gilcher <florian.gilcher@asquera.de>](http://github.com/skade)
With patches by:
* [eliovir](http://github.com/eliovir)
* [m-r-r](http://github.com/m-r-r)

## License

MIT, see `LICENSE.md`
