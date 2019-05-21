#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate libercoin_core;
extern crate libercoin_p2p;

use libercoin_core::ser;
use libercoin_p2p::msg::SockAddr;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<SockAddr, ser::Error> = ser::deserialize(&mut d);
});
