//! Provide operations over IP addresses.
use std::fmt;
use std::io::IpAddr as StdIpAddr;
use std::ops::*;
use std::str::FromStr;
pub use self::IpAddr::*;
pub use self::IpAddrVersion::*;

pub mod ipv4;
pub mod ipv6;

/// Describe an IP address
#[derive(Copy, Clone, Show, PartialEq, Eq, PartialOrd,
            Ord, Hash, RustcEncodable, RustcDecodable)]
pub enum IpAddr {
    Ipv4Addr(ipv4::IpAddr),
    Ipv6Addr(ipv6::IpAddr),
}

/// Describe the version of an IP address.
#[derive(Show, Copy, PartialEq, Eq, Hash,
            RustcEncodable, RustcDecodable)]
pub enum IpAddrVersion {
    Ipv4,
    Ipv6,
}

macro_rules! mirror(
    ($addr:expr, $ip:ident => $value:expr) => ({
        match $addr {
            Ipv4Addr(ref $ip) => $value,
            Ipv6Addr(ref $ip) => $value,
        }
    });
);

impl IpAddr {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        mirror!(*self, ip => ip.version())
    }

    /// The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.
    ///
    /// The prefix defines the number of leading bits in an address that are compared to determine whether or not an address is part of a network.
    pub fn max_prefixlen(&self) -> uint {
        mirror!(*self, ip => ip.max_prefixlen())
    }

    /// Create an IP mask with the specified prefixlen.
    ///
    /// The provided prefixlen must be in the prefixlen-range corresponding to the IP version.
    pub fn with_prefixlen(version: IpAddrVersion, n: uint) -> Option<IpAddr> {
        let ip = match version {
            Ipv4 if n <= ipv4::MAX_PREFIXLEN => Ipv4Addr(ipv4::IpAddr::with_prefixlen(n)),
            Ipv6 if n <= ipv6::MAX_PREFIXLEN => Ipv6Addr(ipv6::IpAddr::with_prefixlen(n)),
            _ => {
                return None;
            }
        };

        Some(ip)
    }

    /// The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).
    /// This is 4 bytes for IPv4 and 16 bytes for IPv6.
    pub fn packed(&self) -> Vec<u8> {
        mirror!(*self, ip => ip.packed().to_vec())
    }
}

impl Add<uint> for IpAddr {
    type Output = Self;

    fn add(self, rhs: uint) -> IpAddr {
        match self {
            Ipv4Addr(ip) => Ipv4Addr(ip + rhs as u32),
            Ipv6Addr(ip) => Ipv6Addr(ip + rhs as u64),
        }
    }
}

impl Sub<uint> for IpAddr {
    type Output = Self;

    fn sub(self, rhs: uint) -> IpAddr {
        match self {
            Ipv4Addr(ip) => Ipv4Addr(ip - rhs as u32),
            Ipv6Addr(ip) => Ipv6Addr(ip - rhs as u64),
        }
    }
}

impl BitXor<IpAddr> for IpAddr {
    type Output = Option<Self>;

    fn bitxor(self, rhs: IpAddr) -> Option<IpAddr> {
        match (self, rhs) {
            (Ipv4Addr(left), Ipv4Addr(right)) =>
                Some(Ipv4Addr(left ^ right)),
            (Ipv6Addr(left), Ipv6Addr(right)) =>
                Some(Ipv6Addr(left ^ right)),
            _ => None,
        }
    }
}

impl BitOr<IpAddr> for IpAddr {
    type Output = Option<Self>;

    fn bitor(self, rhs: IpAddr) -> Option<IpAddr> {
        match (self, rhs) {
            (Ipv4Addr(left), Ipv4Addr(right)) =>
                Some(Ipv4Addr(left | right)),
            (Ipv6Addr(left), Ipv6Addr(right)) =>
                Some(Ipv6Addr(left | right)),
            _ => None,
        }
    }
}

impl BitAnd<IpAddr> for IpAddr {
    type Output = Option<Self>;

    fn bitand(self, rhs: IpAddr) -> Option<IpAddr> {
        match (self, rhs) {
            (Ipv4Addr(left), Ipv4Addr(right)) =>
                Some(Ipv4Addr(left & right)),
            (Ipv6Addr(left), Ipv6Addr(right)) =>
                Some(Ipv6Addr(left & right)),
            _ => None,
        }
    }
}

impl Not for IpAddr {
    type Output = Self;

    fn not(self) -> IpAddr {
        match self {
            Ipv4Addr(ref ip) => Ipv4Addr(!*ip),
            Ipv6Addr(ref ip) => Ipv6Addr(!*ip),
        }
    }
}

///! Convert operations from/to Rust's standard library IP addresses.
impl IpAddr {
    /// Create an `IpAddr` instance from a Rust's standard library `IpAddr` instance.
    pub fn from_std(ip: StdIpAddr) -> IpAddr {
        match ip {
            StdIpAddr::Ipv4Addr(a, b, c, d) =>
                Ipv4Addr(ipv4::IpAddr(a, b, c, d)),
            StdIpAddr::Ipv6Addr(a, b, c, d, e, f, g, h) =>
                Ipv6Addr(ipv6::IpAddr(a, b, c, d, e, f, g, h)),
        }
    }

    /// Create a Rust's standard library `IpAddr` instance from an `IpAddr` instance.
    pub fn to_std(&self) -> StdIpAddr {
        mirror!(*self, ip => ip.to_std())
    }

    /// Convert an `IpAddr` instance into a Rust's standard library `IpAddr` instance.
    pub fn into_std(self) -> StdIpAddr {
        mirror!(self, ip => ip.into_std())
    }
}

impl fmt::String for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        mirror!(*self, ip => ip.fmt(f))
    }
}

impl FromStr for IpAddr {
    fn from_str(s: &str) -> Option<IpAddr> {
        FromStr::from_str(s).map(|ip| IpAddr::from_std(ip))
    }
}

#[cfg(test)]
mod test {
    use super::IpAddr;
    use super::{Ipv4, Ipv6, Ipv4Addr, Ipv6Addr};
    use super::{ipv4, ipv6};

    #[test]
    fn test_version() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(ip.version(), Ipv4);

        let ip: IpAddr = "2001:db8:0:0:0:ff00:42:8329".parse().unwrap();
        assert_eq!(ip.version(), Ipv6);
    }

    #[test]
    fn test_packed() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(ip.packed(), vec![127, 0, 0, 1]);

        let ip: IpAddr = "2001:db8:0:0:0:ff00:42:8329".parse().unwrap();
        assert_eq!(ip.packed(), vec![0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
                                     0x00, 0x00, 0xff, 0x00, 0x00, 0x42, 0x83, 0x29]);
    }

    #[test]
    fn test_convert() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(ip, Ipv4Addr(ipv4::IpAddr(127, 0, 0, 1)));

        let ip: IpAddr = "2001:db8:0:0:0:ff00:42:8329".parse().unwrap();
        assert_eq!(ip, Ipv6Addr(ipv6::IpAddr(0x2001, 0x0db8, 0x0, 0x0, 0x0, 0xff00, 0x42, 0x8329)));
    }

    #[test]
    fn test_ord() {
        let a: IpAddr = "127.0.0.2".parse().unwrap();
        let b: IpAddr = "127.0.0.1".parse().unwrap();

        assert!(a > b);
        assert!(a != b);
    }

    #[test]
    fn test_bitops() {
        let ip: IpAddr = Ipv4Addr(ipv4::IpAddr(127, 0, 0, 1));
        let mask: IpAddr = Ipv4Addr(ipv4::IpAddr(255, 255, 0, 0));

        assert_eq!(ip | mask, Some(Ipv4Addr(ipv4::IpAddr(255, 255, 0, 1))));
        assert_eq!(ip & mask, Some(Ipv4Addr(ipv4::IpAddr(127, 0, 0, 0))));
        assert_eq!(!mask, Ipv4Addr(ipv4::IpAddr(0, 0, 255, 255)));
    }

    #[test]
    fn test_prefixlen() {
        assert_eq!(IpAddr::with_prefixlen(Ipv4, 16), Some(Ipv4Addr(ipv4::IpAddr(255, 255, 0, 0))));
    }
}
