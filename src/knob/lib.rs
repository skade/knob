//!
//! A convenient structure to store and load settings.
//!
//! It is meant for items that are rarely read and stored, like command line flags or
//! application configuration.
//!
//! knob::Settings expects all values to implement ToStr + FromStr and stores them as
//! strings internally. This allows sideloading of settings through multiple means, e.g.
//! the command line or a simple config file. knob is not meant for structured data. If
//! you want to load such data, store the location of the data as a Knob setting and do
//! the loading parsing yourself.
//!
//! knob is typesafe in a sense that it will return you the type you wanted back if possible and
//! fails otherwise.
//!
//! knob allows you to decorate the Settings structure yourself for convenience (see examples).
//!
//! # Storing and fetching values
//!
//! The following example shows you how to store an ip in knob:
//!
//! ~~~{.rust}
//! extern crate knob;
//! use knob::*;
//! use std::rt::io::net::ip::IpAddr;
//!
//! fn main() {
//!   let settings = Settings::new();
//!   settings.set("ip", "0.0.0.0:4567");
//!   let socket: IpAddr = settings.socket();
//!   assert_eq!(socket.to_str(), ~"0.0.0.0:4567")
//! }
//! ~~~
//!
//! This works the same for IPv6 addresses:
//!
//! ~~~{.rust}
//! fn main() {
//!   let settings = Settings::new();
//!   settings.set("ip", "::0.0.0.1");
//!   let socket: IpAddr = settings.fetch("ip");
//!   assert_eq!(socket.to_str(), ~"::0.0.0.1")
//! }
//! ~~~
//!
//! # Providing your own keys
//!
//! You can use enums as keys, as long as they implement ToStr:
//!
//! ~~~{.rust}
//! enum Keys {
//!   Port,
//!   Ip,
//!   Addr,
//! }
//!
//! fn main() {
//!   let settings = Settings::new();
//!   settings.set(Ip, "::0.0.0.1");
//!   let socket: IpAddr = settings.fetch(Ip);
//!   assert_eq!(socket.to_str(), ~"::0.0.0.1")
//! }
//! ~~~
//!
//! # Registering command line options
//!
//! Knob allows you to register command line options to read from the command line later.
//!
//! The options are getopts option groups.
//!
//! If the loading of the command line args fails, an error will be returned.
//!
//! ~~~{.rust}
//! use extra::getopts::groups::*;
//!
//! fn main() {
//!   let settings = Settings::new();
//!   settings.opt(optopt("p", "port", "the port to bind to", "4000"))
//!   settings.opt(reqopt("e", "environment", "the environment to run in", ""));;
//!   let errors = settings.load_os_args();
//!   if errors.is_some() {
//!     println(settings.usage("Try one of these:"))
//!   }
//! }
//! ~~~
//!
//! # Decorating the settings struct
//!
//! To make matters more convenient, you can implement a decorator
//! to implement your own loading behaviour.
//!
//! ~~~{.rust}
//! enum Keys {
//!   Port,
//!   Ip,
//!   Addr,
//! }
//!
//! pub trait SocketSettings {
//!   fn socket(&self) -> SocketAddr;
//!   fn port(&self) -> u16;
//!   fn ip(&self) -> IpAddr;
//! }
//!
//! impl SocketSettings for Settings {
//!   fn socket(&self) -> SocketAddr {
//!     do self.fetch_with(Addr) |addr| {
//!       match addr {
//!         Some(socket_addr) => { socket_addr },
//!         None => {
//!           let port: u16 = self.port();
//!           let ip: IpAddr = self.ip();
//!           SocketAddr { ip: ip, port: port }
//!         }
//!       }
//!     }
//!   }
//!
//!   fn port(&self) -> u16 {
//!     self.fetch(Port).unwrap_or(8080)
//!   }
//!
//!   fn ip(&self) -> IpAddr {
//!     self.fetch(Ip).unwrap_or(Ipv4Addr(127,0,0,1))
//!   }
//! }
//! enum Keys {
//!   Port,
//!   Ip,
//! }
//!
//! fn main() {
//!   let settings = Settings::new();
//!   settings.set(Ip, "::0.0.0.1");
//!   let socket: IpAddr = settings.ip();
//!   assert_eq!(socket.to_str(), ~"::0.0.0.1")
//! }
//! ~~~
//!
//! knob goes up to 11.

#[crate_id = "knob#1.0.1"];

#[crate_type = "lib"];
#[comment = "A convenient Rust settings system"];
#[license = "MIT"];

extern crate extra;
extern crate getopts;

use std::hashmap::HashMap;
use std::os;

use getopts::{usage,getopts,OptGroup};
use getopts::Fail_;

/// The settings structure we save the options and settings in.
#[deriving(Clone)]
pub struct Settings {
  priv store: HashMap<~str,~str>,
  priv options: ~[OptGroup],
}

impl Settings {
  /// Create a new Settings struct.
  pub fn new() -> Settings {
    Settings { store: HashMap::new(), options: ~[] }
  }

  /// Set a settings key to a value. The value will be serialized.
  pub fn set<A: ToStr, T: ToStr>(&mut self, setting: A, value: T) {
    self.store.swap(setting.to_str(), value.to_str());
  }

  /// Set a value using an Option struct. The value will only be set if the
  /// Some value is given. This way, you can avoid unwrapping the result of a
  /// previous operation by yourself.
  pub fn set_opt<A: ToStr, T: ToStr>(&mut self, setting: A, value: Option<T>) {
    if !value.is_none() {
      self.store.swap(setting.to_str(), value.unwrap().to_str());
    }
  }

  /// Fetch a setting for a key. Fails if the setting is present but could not be
  /// parsed.
  pub fn fetch<A: ToStr, T: FromStr>(&self, setting: A) -> Option<T> {
    match self.store.find(&setting.to_str()) {
      Some(string) => {
        let value = from_str(string.to_owned());
        if value.is_none() {
          fail!("setting could not be parsed:" + setting.to_str())
        }
        value
      },
      None => { None }
    }
  }

  /// Fetch a setting for a key and pass it to given function. The result of the function
  /// will be returned.
  pub fn fetch_with<A: ToStr, T: FromStr>(&self, setting: A, f: |Option<T>| -> T) -> T {
    let value = self.fetch(setting.to_str());
    f(value)
  }

  /// Register a commandline for laster use with load_args.
  ///
  /// Currently, only optopt and reqopt are properly supported.
  pub fn opt(&mut self, opt: OptGroup) {
    self.options.push(opt);
  }

  /// Load the command line argument given by the OS.
  ///
  /// Optionally returns failures.
  pub fn load_os_args(&mut self) -> Option<Fail_> {
    let args = os::args();
    self.load_args(args)
  }

  /// Load a list of command line arguments.
  ///
  /// Automatically sets "knob.progname" to the name of the program.
  ///
  /// Optionally returns failures.
  pub fn load_args(&mut self, args: ~[~str]) -> Option<Fail_> {
    let ref prog_name = args[0];

    self.set("knob.progname", prog_name.clone());

    let matches = match getopts(args.tail(), self.options) {
      Ok(m) => { m }
      Err(fail) => { return Some(fail) }
    };

    let given_options = self.options.clone();
    for opt in given_options.iter() {
      let opt_strings = &[opt.short_name.clone(), opt.long_name.clone()];
      self.set_opt(opt.long_name.clone(), matches.opts_str(opt_strings))
    };
    None
  }

  /// Returns the usage string for the stored OptGroups. Pass `brief`
  /// to have it included.
  pub fn usage(&self, brief: &str) -> ~str {
    usage(brief, self.options)
  }
}

