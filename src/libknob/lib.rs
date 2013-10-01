#[link(name = "knob",
       vers = "0.5",
       uuid = "3299B6C2-99DE-44FD-8867-8EE7304959D7",
       url = "http://github.com/skade/knob")];

#[crate_type = "lib"];

extern mod extra;

use std::hashmap::*;
use std::os;

use extra::getopts::groups::{OptGroup,getopts,usage};
use extra::getopts::Fail_;

// The settings structure we save the options and settings in.
#[deriving(Clone)]
pub struct Settings {
  priv store: HashMap<~str,~str>,
  priv options: ~[OptGroup],
}

impl Settings {
  pub fn new() -> Settings {
    Settings { store: HashMap::new(), options: ~[] }
  }

  pub fn opt(&mut self, opt: OptGroup) {
    self.options.push(opt);
  }

  pub fn set<A: ToStr, T: ToStr>(&mut self, setting: A, value: T) {
    self.store.swap(setting.to_str(), value.to_str());
  }

  pub fn set_opt<A: ToStr, T: ToStr>(&mut self, setting: A, value: Option<T>) {
    if !value.is_none() {
      self.store.swap(setting.to_str(), value.unwrap().to_str());
    }
  }

  pub fn fetch_with<A: ToStr, T: FromStr>(&self, setting: A, f: &fn(Option<T>) -> T) -> T {
    let value = self.fetch(setting.to_str());
    f(value)
  }

  pub fn fetch<A: ToStr, T: FromStr>(&self, setting: A) -> Option<T> {
    match self.store.find(&setting.to_str()) {
      Some(string) => { from_str(string.to_owned()) },
      None => { None }
    }
  }

  pub fn load_os_args(&mut self) {
    let args = os::args();
    self.load_args(args);
  }

  pub fn load_args(&mut self, args: ~[~str]) -> Option<Fail_> {
    let ref prog_name = args[0];

    self.set("knob.progname", prog_name.clone());

    let matches = match getopts(args.tail(), self.options) {
      Ok(m) => { m }
      Err(fail) => { return Some(fail) }
    };

    debug!(matches);

    let given_options = self.options.clone();
    for opt in given_options.iter() {
      let opt_strings = &[opt.short_name.clone(), opt.long_name.clone()];
      self.set_opt(opt.long_name.clone(), matches.opts_str(opt_strings))
    };
    None
  }

  pub fn usage(&self, brief: &str) -> ~str {
    usage(brief, self.options)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::rt::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
  use extra::getopts::groups::*;

  pub trait SocketSettings {
    fn socket(&self) -> SocketAddr;
    fn port(&self) -> u16;
    fn ip(&self) -> IpAddr;
  }

  impl SocketSettings for Settings {
    fn socket(&self) -> SocketAddr {
      do self.fetch_with("bind") |addr| {
        match addr {
          Some(socket_addr) => { socket_addr },
          None => {
            let port: u16 = self.port();
            let ip: IpAddr = self.ip();
            SocketAddr { ip: ip, port: port }
          }
        }
      }
    }

    fn port(&self) -> u16 {
      self.fetch("port").unwrap_or(8080)
    }

    fn ip(&self) -> IpAddr {
      self.fetch("ip").unwrap_or(Ipv4Addr(127,0,0,1))
    }
  }

  #[test]
  fn test_simple_conversion() {
    let mut settings = Settings::new();
    settings.set("port", 12345);
    let settings = settings.fetch("port");
    assert_eq!(Some(12345), settings)
  }

  #[test]
  fn test_compound_socket_settings() {
    let mut settings = Settings::new();
    settings.set("port", "12345");
    settings.set("ip", "127.0.0.1");
    let socket = settings.socket();
    assert_eq!(socket.to_str(), ~"127.0.0.1:12345")
  }

  #[test]
  fn test_socket_overrides_port() {
    let mut settings = Settings::new();
    settings.set("port", "12345");
    settings.set("ip", "127.0.0.1");
    settings.set("bind", "0.0.0.0:4567");
    let socket = settings.socket();
    assert_eq!(socket.to_str(), ~"0.0.0.0:4567")
  }

  #[test]
  fn test_opt_parse_optional() {
    let mut settings = Settings::new();
    settings.opt(optopt("p", "port", "The port to bind to", "eg: 4000"));

    let args = ~[~"myprog", ~"-p", ~"3000"];
    let error = settings.load_args(args);

    assert!(error.is_none());
    assert_eq!(settings.fetch("port"), Some(3000))
  }

  #[test]
  fn test_opt_parse_req_with_given() {
    let mut settings = Settings::new();
    settings.opt(reqopt("p", "port", "The port to bind to", "eg: 4000"));

    let args = ~[~"myprog", ~"-p", ~"3000"];
    let error = settings.load_args(args);

    assert!(error.is_none());
    assert_eq!(settings.fetch("port"), Some(3000))
  }

  #[test]
  fn test_opt_parse_req_without_given() {
    let mut settings = Settings::new();
    settings.opt(reqopt("p", "port", "The port to bind to", "eg: 4000"));

    let args = ~[~"myprog"];
    let error = settings.load_args(args);

    assert!(error.is_some());
    let port: Option<int> = settings.fetch("port");
    assert_eq!(port, None)
  }

  #[test]
  fn test_usage() {
    let mut settings = Settings::new();
    settings.opt(reqopt("p", "port", "The port to bind to", "eg: 4000"));
    let usage = settings.usage(&"this is how it works");

    assert!(usage.contains("this is how it works"))
    assert!(usage.contains("--port"))
  }
}