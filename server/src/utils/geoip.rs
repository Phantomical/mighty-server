#[cfg(not(feature = "geoip"))]
mod default_geoip {
	use protocol::FlagCode;
	use std::net::IpAddr;

	/// An empty lookup that always returns None
	pub fn locate(_: &IpAddr) -> Option<FlagCode> {
		None
	}
}

#[cfg(feature = "geoip")]
mod full_geoip {
	extern crate geolocate_ip;

	use protocol::FlagCode;
	use std::net::IpAddr;

	/// Look up ISO-2 country code
	pub fn locate(addr: &IpAddr) -> Option<FlagCode> {
		match *addr {
			IpAddr::V4(a) => match geolocate_ip::lookup_ip(&a) {
				Some(s) => FlagCode::from_str(s),
				None => None,
			},
			// IP lookups not done for Ipv6 addresses yet
			IpAddr::V6(_) => None,
		}
	}
}

#[cfg(feature = "geoip")]
pub use self::full_geoip::*;

#[cfg(not(feature = "geoip"))]
pub use self::default_geoip::*;
