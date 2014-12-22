//! Provide operations over IPv6 addresses.
use std::fmt;
use std::str::FromStr;
use std::io::IpAddr as StdIpAddr;
use std::simd::u64x2;
use super::IpAddrVersion::{mod, Ipv6};

pub const MAX_PREFIXLEN: uint = 128;

#[deriving(Copy, Clone, PartialEq, Eq, Hash, Encodable, Decodable)]
pub struct IpAddr(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);

impl IpAddr {
    /// Get the corresponding IP address version.
    pub fn version(&self) -> IpAddrVersion {
        Ipv6
    }

    /// The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.
    ///
    /// The prefix defines the number of leading bits in an address that are compared to determine whether or not an address is part of a network.
    pub fn max_prefixlen(&self) -> uint {
        MAX_PREFIXLEN
    }

    /// Create an IP mask with the specified prefixlen.
    ///
    /// The provided prefixlen must be in the prefixlen-range (`0` <= `n` <= `128`).
    pub fn with_prefixlen(n: uint) -> IpAddr {
        assert!(n <= MAX_PREFIXLEN);
        let mid = MAX_PREFIXLEN / 2;
        if n >= mid {
            let n = n - mid;

            let hi = ::std::u64::MAX;
            let lo = (1 << n) - 1;
            let lo = lo << (mid - n);

            IpAddr::from_simd(u64x2(hi, lo))
        } else {
            let hi = (1 << n) - 1;
            let hi = hi << (MAX_PREFIXLEN - n);

            IpAddr::from_simd(u64x2(hi, 0))
        }
    }

    /// The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).
    /// This is 4 bytes for IPv4 and 16 bytes for IPv6.
    pub fn packed(&self) -> [u8, ..16] {
        let &IpAddr(a, b, c, d, e, f, g, h) = self;
        [
            ((a >> 8) & 0xff) as u8,
            (a & 0xff) as u8,
            ((b >> 8) & 0xff) as u8,
            (b & 0xff) as u8,
            ((c >> 8) & 0xff) as u8,
            (c & 0xff) as u8,
            ((d >> 8) & 0xff) as u8,
            (d & 0xff) as u8,
            ((e >> 8) & 0xff) as u8,
            (e & 0xff) as u8,
            ((f >> 8) & 0xff) as u8,
            (f & 0xff) as u8,
            ((g >> 8) & 0xff) as u8,
            (g & 0xff) as u8,
            ((h >> 8) & 0xff) as u8,
            (h & 0xff) as u8,
        ]
    }

    /// Create an `IpAddr` instance from a 128-bits integer.
    ///
    /// As Rust doesn't support u128-bits integer natively, this
    /// method take an array of two u64-bits integers
    pub fn from_u128(n: [u64, ..2]) -> IpAddr {
        let a = (n[0] >> 48) & 0xffff;
        let b = (n[0] >> 32) & 0xffff;
        let c = (n[0] >> 16) & 0xffff;
        let d = n[0] & 0xffff;
        let e = (n[1] >> 48) & 0xffff;
        let f = (n[1] >> 32) & 0xffff;
        let g = (n[1] >> 16) & 0xffff;
        let h = n[1] & 0xffff;

        IpAddr(a as u16, b as u16, c as u16, d as u16,
                e as u16, f as u16, g as u16, h as u16)
    }

    /// Convert an `IpAddr` instance to a 128-bits integer.
    ///
    /// As Rust doesn't support u128-bits integer natively, this
    /// method return an array of two u64-bits integers
    pub fn to_u128(&self) -> [u64, ..2] {
        let &IpAddr(a, b, c, d, e, f, g, h) = self;
        let (a, b, c, d) = (a as u64, b as u64, c as u64, d as u64);
        let (e, f, g, h) = (e as u64, f as u64, g as u64, h as u64);
        [(a << 48) | (b << 32) | (c << 16) | d, (e << 48) | (f << 32) | (g << 16) | h]
    }

    /// Create an `IpAddr` instance from a 128-bits SIMD integer.
    pub fn from_simd(n: u64x2) -> IpAddr {
        let u64x2(a, b) = n;
        IpAddr::from_u128([a, b])
    }

    /// Convert an `IpAddr` instance to a 128-bits SIMD integer.
    pub fn to_simd(&self) -> u64x2 {
        let [a, b] = self.to_u128();
        u64x2(a, b)
    }
}

impl Add<u64, IpAddr> for IpAddr {
    fn add(self, rhs: u64) -> IpAddr {
        let u64x2(mut hi, mut lo) = self.to_simd();

        lo += rhs;
        if lo < rhs {
            hi += 1;
        }

        IpAddr::from_simd(u64x2(hi, lo))
    }
}

impl Sub<u64, IpAddr> for IpAddr {
    fn sub(self, rhs: u64) -> IpAddr {
        let u64x2(mut hi, mut lo) = self.to_simd();

        lo -= rhs;
        if lo > rhs {
            hi -= 1;
        }

        IpAddr::from_simd(u64x2(hi, lo))
    }
}

impl BitXor<IpAddr, IpAddr> for IpAddr {
    /// > Use SIMD to do operations on 128-bits integer.
    fn bitxor(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_simd(self.to_simd() ^ rhs.to_simd())
    }
}

impl BitOr<IpAddr, IpAddr> for IpAddr {
    /// > Use SIMD to do operations on 128-bits integer.
    fn bitor(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_simd(self.to_simd() | rhs.to_simd())
    }
}

impl BitAnd<IpAddr, IpAddr> for IpAddr {
    /// > Use SIMD to do operations on 128-bits integer.
    fn bitand(self, rhs: IpAddr) -> IpAddr {
        IpAddr::from_simd(self.to_simd() & rhs.to_simd())
    }
}

impl Not<IpAddr> for IpAddr {
    fn not(self) -> IpAddr {
        let u64x2(a, b) = self.to_simd();
        IpAddr::from_simd(u64x2(!a, !b))
    }
}

impl PartialOrd for IpAddr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        self.to_u128().partial_cmp(&other.to_u128())
    }
}

impl Ord for IpAddr {
    fn cmp(&self, other: &IpAddr) -> Ordering {
        self.to_u128().cmp(&other.to_u128())
    }
}

///! Convert operations from/to Rust's standard library IP addresses.
impl IpAddr {
    /// Create an `ipv6::IpAddr` instance from a Rust's standard library `IpAddr` instance.
    pub fn from_std(ip: StdIpAddr) -> Option<IpAddr> {
        match ip {
            StdIpAddr::Ipv6Addr(a, b, c, d, e, f, g, h) =>
                Some(IpAddr(a, b, c, d, e, f, g, h)),
            _ =>
                None,
        }
    }

    /// Create a Rust's standard library `IpAddr` instance from an `ipv6::IpAddr` instance.
    pub fn to_std(&self) -> StdIpAddr {
        let &IpAddr(a, b, c, d, e, f, g, h) = self;
        StdIpAddr::Ipv6Addr(a, b, c, d, e, f, g, h)
    }

    /// Convert an `ipv6::IpAddr` instance into a Rust's standard library `IpAddr` instance.
    pub fn into_std(self) -> StdIpAddr {
        self.to_std()
    }
}

impl fmt::Show for IpAddr {
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
        let a = IpAddr(0, 0, 0, 0, 0, 0, 0, 1);
        let b = IpAddr(0, 0, 0, 0, 0, 0, 0, 2);
        let c = IpAddr(0, 0, 0, 0, 0xffff, 0xffff, 0xffff, 0xffff);
        let d = IpAddr(0, 0, 0, 1, 0, 0, 0, 0);

        assert_eq!(a + 1, b);
        assert_eq!(c + 1, d);

        assert_eq!(b - 1, a);
        assert_eq!(d - 1, c);

        assert!(a < b);
    }

    #[test]
    fn test_bitops() {
        let ip = IpAddr(12, 15, 18, 22, 32, 33, 34, 35);
        let mask = IpAddr(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0);

        assert_eq!(ip | mask, IpAddr(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 33, 34, 35));
        assert_eq!(ip & mask, IpAddr(12, 15, 18, 22, 32, 0, 0, 0));
        assert_eq!(!mask, IpAddr(0, 0, 0, 0, 0, 0xffff, 0xffff, 0xffff));
    }

    #[test]
    fn test_prefixlen() {
        assert_eq!(IpAddr::with_prefixlen(32), IpAddr(0xffff, 0xffff, 0, 0, 0, 0, 0, 0));
        assert_eq!(IpAddr::with_prefixlen(64), IpAddr(0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0));
        assert_eq!(IpAddr::with_prefixlen(96), IpAddr(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0));
    }
}
