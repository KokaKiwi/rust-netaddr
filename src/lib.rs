#![feature(int_uint)]
#![experimental]
#![allow(unstable)]

#[macro_use]
extern crate log;
extern crate "rustc-serialize" as rustc_serialize;

pub use addr::IpAddr;
pub use net::IpNetwork;

pub mod addr;
pub mod net;
