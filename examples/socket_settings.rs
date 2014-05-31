extern crate knob;

use std::io::net::ip::IpAddr;
use knob::Settings;

fn main() {
    let mut settings = Settings::new();
    settings.set("ip", "::0.0.0.1");
    let socket: IpAddr = settings.fetch("ip").unwrap();
    assert_eq!(socket.to_str(), "::0.0.0.1".to_string());
}
