// Copyright 2015-2017 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Primitive types shared by Substrate and Parity Ethereum.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "impl-serde", not(feature = "std")))]
compile_error!("Feature \"impl-serde\" requires feature \"std\" to build.");

#[cfg(all(feature = "impl-rlp", not(feature = "std")))]
compile_error!("Feature \"impl-rlp\" requires feature \"std\" to build.");

#[macro_use]
extern crate uint;

#[macro_use]
extern crate fixed_hash;

#[macro_use]
extern crate crunchy;

#[cfg(feature = "impl-serde")]
#[macro_use]
extern crate impl_serde;

#[cfg(feature = "impl-codec")]
#[macro_use]
extern crate impl_codec;

#[cfg(feature = "impl-rlp")]
#[macro_use]
extern crate impl_rlp;

construct_uint!(U256, 4);
#[cfg(feature = "impl-serde")] impl_uint_serde!(U256, 4);
#[cfg(feature = "impl-codec")] impl_uint_codec!(U256, 4);
#[cfg(feature = "impl-rlp")] impl_uint_rlp!(U256, 4);

construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
	pub struct H160(20);
}
#[cfg(feature = "impl-serde")] impl_fixed_hash_serde!(H160, 20);
#[cfg(feature = "impl-codec")] impl_fixed_hash_codec!(H160, 20);
#[cfg(feature = "impl-rlp")] impl_fixed_hash_rlp!(H160, 20);

impl From<u64> for H160 {
	fn from(val: u64) -> Self {
		Self::from_low_u64_be(val)
	}
}

construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
	pub struct H256(32);
}
#[cfg(feature = "impl-serde")] impl_fixed_hash_serde!(H256, 32);
#[cfg(feature = "impl-codec")] impl_fixed_hash_codec!(H256, 32);
#[cfg(feature = "impl-rlp")] impl_fixed_hash_rlp!(H256, 32);

impl From<u64> for H256 {
	fn from(val: u64) -> Self {
		Self::from_low_u64_be(val)
	}
}

construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
	pub struct H512(64);
}
#[cfg(feature = "impl-serde")] impl_fixed_hash_serde!(H512, 64);
#[cfg(feature = "impl-codec")] impl_fixed_hash_codec!(H512, 64);
#[cfg(feature = "impl-rlp")] impl_fixed_hash_rlp!(H512, 64);

impl From<u64> for H512 {
	fn from(val: u64) -> Self {
		Self::from_low_u64_be(val)
	}
}
