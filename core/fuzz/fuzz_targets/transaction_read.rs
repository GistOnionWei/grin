#![no_main]
extern crate libercoin_core;
#[macro_use]
extern crate libfuzzer_sys;

use libercoin_core::core::transaction;
use libercoin_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<transaction::Transaction, ser::Error> = ser::deserialize(&mut d);
});
