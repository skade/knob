extern mod knob;
extern mod extra;

#[cfg(test)]
mod tests {
  use knob::Settings;
  use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
  use extra::getopts::groups::{optopt,reqopt};

  #[deriving(ToStr)]
  enum Keys {
    Ip,
    Port,
    Addr
  }

  pub trait SocketSettings {
    fn socket(&self) -> SocketAddr;
    fn port(&self) -> u16;
    fn ip(&self) -> IpAddr;
  }

  impl SocketSettings for Settings {
    fn socket(&self) -> SocketAddr {
      self.fetch_with(Addr, |addr| {
        match addr {
          Some(socket_addr) => { socket_addr },
          None => {
            let port: u16 = self.port();
            let ip: IpAddr = self.ip();
            SocketAddr { ip: ip, port: port }
          }
        }
      })
    }

    fn port(&self) -> u16 {
      self.fetch(Port).unwrap_or(8080)
    }

    fn ip(&self) -> IpAddr {
      self.fetch(Ip).unwrap_or(Ipv4Addr(127,0,0,1))
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
  #[should_fail]
  fn test_should_fail_on_garbage() {
    let mut settings = Settings::new();
    settings.set("port", "foobar");
    let settings: Option<int> = settings.fetch("port");
    assert_eq!(Some(12345), settings)
  }

  #[test]
  fn test_enum() {
    let mut settings = Settings::new();
    settings.set(Port, 12345);
    let settings = settings.fetch(Port);
    assert_eq!(Some(12345), settings)
  }

  #[test]
  fn test_compound_socket_settings() {
    let mut settings = Settings::new();
    settings.set(Port, "12345");
    settings.set(Ip, "127.0.0.1");
    let socket = settings.socket();
    assert_eq!(socket.to_str(), ~"127.0.0.1:12345")
  }

  #[test]
  fn test_ipv6_addr() {
    let mut settings = Settings::new();
    settings.set(Ip, "::0.0.0.1");
    let ip = settings.ip();
    assert_eq!(ip.to_str(), ~"::0.0.0.1")
  }

  #[test]
  fn test_socket_overrides_port() {
    let mut settings = Settings::new();
    settings.set(Port, "12345");
    settings.set(Ip, "127.0.0.1");
    settings.set(Addr, "0.0.0.0:4567");
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

  #[test]
  fn test_goes_up_to_eleven() {
    let mut settings = Settings::new();
    settings.set("knob", 11);
    assert_eq!(settings.fetch("knob"), Some(11));
  }
}
