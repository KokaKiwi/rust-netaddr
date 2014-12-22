#![feature(phase, globs, macro_rules)]
#![experimental]

#[phase(plugin, link)]
extern crate log;
extern crate serialize;

pub use addr::IpAddr;
pub use net::IpNetwork;

pub mod addr;
pub mod net;
