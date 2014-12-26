//! Provide operations over IPv6 networks.
use std::fmt;
use addr::ipv6::{IpAddr, MAX_PREFIXLEN};
use addr::{IpAddrVersion, Ipv6};

#[deriving(Copy, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct IpNetwork(pub IpAddr, pub uint);

impl IpNetwork {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        Ipv6
    }

    /// Get the network address for the network.
    pub fn address(&self) -> IpAddr {
        let &IpNetwork(addr, _) = self;
        addr & self.mask()
    }

    /// Get the broadcast address for the network.
    pub fn broadcast_address(&self) -> IpAddr {
        self.address() | !self.mask()
    }

    /// Get the length of the network prefix, in bits.
    pub fn prefix(&self) -> uint {
        let &IpNetwork(_, prefix) = self;
        prefix
    }

    /// Get the length of the host prefix, in bits.
    pub fn host_prefix(&self) -> uint {
        MAX_PREFIXLEN - self.prefix()
    }

    /// The total number of addresses in the network.
    pub fn num_addresses(&self) -> uint {
        use std::num::Int;

        2u.pow(self.host_prefix())
    }

    /// Get the mask of the network.
    pub fn mask(&self) -> IpAddr {
        IpAddr::with_prefixlen(self.prefix())
    }

    /// Get the hosts range this network have.
    pub fn range(&self) -> (IpAddr, IpAddr) {
        (self.address(), self.broadcast_address())
    }

    /// `true` if this ip is contained in the network.
    pub fn contains(&self, ip: IpAddr) -> bool {
        let (start, stop) = self.range();
        start <= ip && ip <= stop
    }

    /// `true` if this network is partly or wholly contained in other or other is wholly contained in this network.
    pub fn overlaps(&self, other: IpNetwork) -> bool {
        other.contains(self.address()) || other.contains(self.broadcast_address())
        || self.contains(other.address()) || self.contains(other.broadcast_address())
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

impl PartialOrd for IpNetwork {
    fn partial_cmp(&self, other: &IpNetwork) -> Option<Ordering> {
        self.address().partial_cmp(&other.address())
    }
}

impl Ord for IpNetwork {
    fn cmp(&self, other: &IpNetwork) -> Ordering {
        self.address().cmp(&other.address())
    }
}

impl fmt::Show for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.address(), self.prefix())
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
