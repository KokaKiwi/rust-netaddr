//! Provide operations over IP networks.
use std::fmt;
use addr::{IpAddrVersion, IpAddr};
pub use self::IpNetwork::*;

pub mod ipv4;
pub mod ipv6;

/// Describe an IP network.
#[deriving(Copy, Clone, PartialEq, Eq, PartialOrd,
            Ord, Hash, RustcEncodable, RustcDecodable)]
pub enum IpNetwork {
    Ipv4Network(ipv4::IpNetwork),
    Ipv6Network(ipv6::IpNetwork),
}

macro_rules! mirror(
    ($addr:expr, $net:ident => $value:expr) => ({
        match $addr {
            Ipv4Network(ref $net) => $value,
            Ipv6Network(ref $net) => $value,
        }
    });
    (ip: $addr:expr, $net:ident => $value:expr) => ({
        use addr::IpAddr::*;

        match $addr {
            Ipv4Network(ref $net) => Ipv4Addr($value),
            Ipv6Network(ref $net) => Ipv6Addr($value),
        }
    });
    (net: $addr:expr, $net:ident => $value:expr) => ({
        match $addr {
            Ipv4Network(ref $net) => Ipv4Network($value),
            Ipv6Network(ref $net) => Ipv6Network($value),
        }
    });
);

impl IpNetwork {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        mirror!(*self, net => net.version())
    }

    /// Get the network address for the network.
    pub fn address(&self) -> IpAddr {
        mirror!(ip: *self, net => net.address())
    }

    /// Get the broadcast address for the network.
    pub fn broadcast_address(&self) -> IpAddr {
        mirror!(ip: *self, net => net.broadcast_address())
    }

    /// Get the length of the network prefix, in bits.
    pub fn prefix(&self) -> uint {
        mirror!(*self, net => net.prefix())
    }

    /// Get the length of the host prefix, in bits.
    pub fn host_prefix(&self) -> uint {
        mirror!(*self, net => net.host_prefix())
    }

    /// The total number of addresses in the network.
    pub fn num_addresses(&self) -> uint {
        mirror!(*self, net => net.num_addresses())
    }

    /// Get the mask of the network.
    pub fn mask(&self) -> IpAddr {
        mirror!(ip: *self, net => net.mask())
    }

    /// Get the hosts range this network have.
    pub fn range(&self) -> (IpAddr, IpAddr) {
        use addr::IpAddr::*;

        match *self {
            Ipv4Network(ref net) => {
                let (start, stop) = net.range();
                (Ipv4Addr(start), Ipv4Addr(stop))
            }
            Ipv6Network(ref net) => {
                let (start, stop) = net.range();
                (Ipv6Addr(start), Ipv6Addr(stop))
            }
        }
    }

    /// `true` if this ip is contained in the network.
    pub fn contains(&self, ip: IpAddr) -> bool {
        use addr::IpAddr::*;

        match (*self, ip) {
            (Ipv4Network(ref net), Ipv4Addr(ip)) => net.contains(ip),
            (Ipv6Network(ref net), Ipv6Addr(ip)) => net.contains(ip),
            _ => false,
        }
    }
    /// `true` if this network is partly or wholly contained in other or other is wholly contained in this network.
    pub fn overlaps(&self, other: IpNetwork) -> bool {
        match (*self, other) {
            (Ipv4Network(ref neta), Ipv4Network(netb)) => neta.overlaps(netb),
            (Ipv6Network(ref neta), Ipv6Network(netb)) => neta.overlaps(netb),
            _ => false,
        }
    }

    /// Iterate over all addresses of this network.
    pub fn iter(&self) -> Hosts {
        let (start, stop) = self.range();
        Hosts {
            state: start,
            stop: stop,
        }
    }

    /// Iterate over all usable hosts of this network.
    pub fn hosts_iter(&self) -> Hosts {
        let (start, stop) = self.range();
        Hosts {
            state: start + 1,
            stop: stop - 1,
        }
    }
}

impl fmt::Show for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        mirror!(*self, net => net.fmt(f))
    }
}

#[deriving(Copy, Clone)]
pub struct Hosts {
    state: IpAddr,
    stop: IpAddr,
}

impl Iterator<IpAddr> for Hosts {
    fn next(&mut self) -> Option<IpAddr> {
        if self.state <= self.stop {
            let result = self.state;
            self.state = self.state + 1;
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator<IpAddr> for Hosts {
    fn next_back(&mut self) -> Option<IpAddr> {
        if self.stop >= self.state {
            self.stop = self.stop - 1;
            Some(self.stop)
        } else {
            None
        }
    }
}
