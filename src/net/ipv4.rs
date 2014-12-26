//! Provide operations over IPv4 networks.
use std::fmt;
use addr::ipv4::{IpAddr, MAX_PREFIXLEN};
use addr::{IpAddrVersion, Ipv4};

#[deriving(Copy, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct IpNetwork(pub IpAddr, pub uint);

impl IpNetwork {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        Ipv4
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

#[cfg(test)]
mod test {
    use super::IpNetwork;
    use addr::ipv4::IpAddr;

    #[test]
    fn test_mask() {
        let net = IpNetwork(IpAddr(127, 0, 0, 1), 24);

        assert_eq!(net.mask(), IpAddr(255, 255, 255, 0));
    }

    #[test]
    fn test_broadcast() {
        let net = IpNetwork(IpAddr(127, 0, 0, 1), 24);

        assert_eq!(net.broadcast_address(), IpAddr(127, 0, 0, 255));
    }

    #[test]
    fn test_num_addresses() {
        assert_eq!(IpNetwork(IpAddr(127, 0, 0, 1), 24).num_addresses(), 256);
        assert_eq!(IpNetwork(IpAddr(127, 0, 0, 1), 0).num_addresses(), ::std::u32::MAX as uint + 1);
    }

    #[test]
    fn test_iter() {
        let net = IpNetwork(IpAddr(127, 0, 0, 1), 24);

        assert_eq!(net.iter().count(), net.num_addresses());
        assert_eq!(net.iter().rev().count(), net.num_addresses());

        assert_eq!(net.hosts_iter().count(), net.num_addresses() - 2);
        assert_eq!(net.hosts_iter().rev().count(), net.num_addresses() - 2);
    }

    #[test]
    fn test_contains() {
        let net = IpNetwork(IpAddr(127, 0, 0, 1), 24);

        assert!(net.contains(IpAddr(127, 0, 0, 25)));
        assert!(!net.contains(IpAddr(128, 0, 0, 25)));
    }

    #[test]
    fn test_overlaps() {
        let net1 = IpNetwork(IpAddr(127, 0, 0, 1), 24);
        let net2 = IpNetwork(IpAddr(127, 0, 0, 1), 16);
        let net3 = IpNetwork(IpAddr(128, 0, 0, 1), 16);

        assert!(net1.overlaps(net1));
        assert!(net1.overlaps(net2));
        assert!(!net1.overlaps(net3));
    }
}
