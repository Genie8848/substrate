// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_whitelist
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_whitelist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/whitelist/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_whitelist.
pub trait WeightInfo {
	fn whitelist_call() -> Weight;
	fn remove_whitelisted_call() -> Weight;
	fn dispatch_whitelisted_call() -> Weight;
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight;
}

/// Weights for pallet_whitelist using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn whitelist_call() -> Weight {
		// Minimum execution time: 26_352 nanoseconds.
		Weight::from_ref_time(26_727_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn remove_whitelisted_call() -> Weight {
		// Minimum execution time: 25_536 nanoseconds.
		Weight::from_ref_time(25_969_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	fn dispatch_whitelisted_call() -> Weight {
		// Minimum execution time: 4_802_466 nanoseconds.
		Weight::from_ref_time(4_820_197_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Minimum execution time: 29_184 nanoseconds.
		Weight::from_ref_time(30_530_970 as u64)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(1_496 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn whitelist_call() -> Weight {
		// Minimum execution time: 26_352 nanoseconds.
		Weight::from_ref_time(26_727_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn remove_whitelisted_call() -> Weight {
		// Minimum execution time: 25_536 nanoseconds.
		Weight::from_ref_time(25_969_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	fn dispatch_whitelisted_call() -> Weight {
		// Minimum execution time: 4_802_466 nanoseconds.
		Weight::from_ref_time(4_820_197_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Whitelist WhitelistedCall (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Minimum execution time: 29_184 nanoseconds.
		Weight::from_ref_time(30_530_970 as u64)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(1_496 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}