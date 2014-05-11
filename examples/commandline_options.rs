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
