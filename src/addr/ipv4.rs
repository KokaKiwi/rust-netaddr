//! Provide operations over IPv4 addresses.
use std::cmp::Ordering;
use std::fmt;
use std::io::IpAddr as StdIpAddr;
use std::ops::*;
use std::str::FromStr;
use super::IpAddrVersion::{self, Ipv4};

pub const MAX_PREFIXLEN: uint = 32;

#[derive(Copy, Clone, Show, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct IpAddr(pub u8, pub u8, pub u8, pub u8);

impl IpAddr {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        Ipv4
    }

    /// The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.
    ///
    /// The prefix defines the number of leading bits in an address that are compared to determine whether or not an address is part of a network.
    pub fn max_prefixlen(&self) -> uint {
        MAX_PREFIXLEN
    }

    /// Create an IP mask with the specified prefixlen.
    ///
    /// The provided prefixlen must be in the prefixlen-range (`0` <= `n` <= `32`).
    pub fn with_prefixlen(n: uint) -> IpAddr {
        assert!(n <= MAX_PREFIXLEN);
        let mask: u32 = (1 << n) - 1;
        let mask = mask << (MAX_PREFIXLEN - n);
        IpAddr::from_u32(mask)
    }

    /// The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).
    /// This is 4 bytes for IPv4 and 16 bytes for IPv6.
    pub fn packed(&self) -> [u8; 4] {
        let &IpAddr(a, b, c, d) = self;
        [a, b, c, d]
    }

    /// Create an `IpAddr` instance from a 32-bits integer.
    pub fn from_u32(n: u32) -> IpAddr {
        let a = (n >> 24) & 0xff;
        let b = (n >> 16) & 0xff;
        let c = (n >> 8) & 0xff;
        let d = n & 0xff;

        IpAddr(a as u8, b as u8, c as u8, d as u8)
    }

    /// Convert an `IpAddr` instance to a 32-bits integer.
    pub fn to_u32(&self) -> u32 {
        let &IpAddr(a, b, c, d) = self;
        let (a, b, c, d) = (a as u32, b as u32, c as u32, d as u32);
        (a << 24) | (b << 16) | (c << 8) | d
    }
}

impl Add<u32> for IpAddr {
    type Output = Self;

    fn add(self, rhs: u32) -> IpAddr {
        IpAddr::from_u32(self.to_u32() + rhs)
    }
}

impl Sub<u32> for IpAddr {
    type Output = Self;

    fn sub(self, rhs: u32) -> IpAddr {
        IpAddr::from_u32(self.to_u32() - rhs)
    }
}

impl BitXor<IpAddr> for IpAddr {
    type Output = Self;

    fn bitxor(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_u32(self.to_u32() ^ rhs.to_u32())
    }
}

impl BitOr<IpAddr> for IpAddr {
    type Output = Self;

    fn bitor(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_u32(self.to_u32() | rhs.to_u32())
    }
}

impl BitAnd<IpAddr> for IpAddr {
    type Output = Self;

    fn bitand(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_u32(self.to_u32() & rhs.to_u32())
    }
}

impl Not for IpAddr {
    type Output = Self;

    fn not(self) -> IpAddr {
        IpAddr::from_u32(!self.to_u32())
    }
}

impl PartialOrd for IpAddr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        self.to_u32().partial_cmp(&other.to_u32())
    }
}

impl Ord for IpAddr {
    fn cmp(&self, other: &IpAddr) -> Ordering {
        self.to_u32().cmp(&other.to_u32())
    }
}

///! Convert operations from/to Rust's standard library IP addresses.
impl IpAddr {
    /// Create an `ipv4::IpAddr` instance from a Rust's standard library `IpAddr` instance.
    pub fn from_std(ip: StdIpAddr) -> Option<IpAddr> {
        match ip {
            StdIpAddr::Ipv4Addr(a, b, c, d) =>
                Some(IpAddr(a, b, c, d)),
            _ =>
                None,
        }
    }

    /// Create a Rust's standard library `IpAddr` instance from an `ipv4::IpAddr` instance.
    pub fn to_std(&self) -> StdIpAddr {
        let &IpAddr(a, b, c, d) = self;
        StdIpAddr::Ipv4Addr(a, b, c, d)
    }

    /// Convert an `ipv4::IpAddr` instance into a Rust's standard library `IpAddr` instance.
    pub fn into_std(self) -> StdIpAddr {
        self.to_std()
    }
}

impl fmt::String for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(f)
    }
}

impl FromStr for IpAddr {
    fn from_str(s: &str) -> Option<IpAddr> {
        let ip: Option<::std::io::IpAddr> = FromStr::from_str(s);
        ip.and_then(|ip| IpAddr::from_std(ip))
    }
}

#[cfg(test)]
mod test {
    use super::IpAddr;

    #[test]
    fn test_num() {
        assert_eq!(IpAddr(127, 0, 0, 1) + 1, IpAddr(127, 0, 0, 2));
        assert_eq!(IpAddr(127, 0, 0, 255) + 1, IpAddr(127, 0, 1, 0));

        assert_eq!(IpAddr(127, 0, 0, 2) - 1, IpAddr(127, 0, 0 ,1));
        assert_eq!(IpAddr(127, 0, 1, 0) - 1, IpAddr(127, 0, 0, 255));
    }

    #[test]
    fn test_bitops() {
        let ip = IpAddr(127, 0, 0, 1);
        let mask = IpAddr(255, 255, 255, 0);

        assert_eq!(ip | mask, IpAddr(255, 255, 255, 1));
        assert_eq!(ip & mask, IpAddr(127, 0, 0, 0));
        assert_eq!(!mask, IpAddr(0, 0, 0, 255));
    }

    #[test]
    fn test_prefixlen() {
        assert_eq!(IpAddr::with_prefixlen(16), IpAddr(255, 255, 0, 0));
        assert_eq!(IpAddr::with_prefixlen(24), IpAddr(255, 255, 255, 0));
    }
}
