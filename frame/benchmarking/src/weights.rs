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

//! Autogenerated weights for frame_benchmarking
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
// --pallet=frame_benchmarking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/benchmarking/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for frame_benchmarking.
pub trait WeightInfo {
	fn addition(i: u32, ) -> Weight;
	fn subtraction(i: u32, ) -> Weight;
	fn multiplication(i: u32, ) -> Weight;
	fn division(i: u32, ) -> Weight;
	fn hashing(i: u32, ) -> Weight;
	fn sr25519_verification(i: u32, ) -> Weight;
	fn storage_read(i: u32, ) -> Weight;
	fn storage_write(i: u32, ) -> Weight;
}

/// Weights for frame_benchmarking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Minimum execution time: 108 nanoseconds.
		Weight::from_ref_time(137_610 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Minimum execution time: 104 nanoseconds.
		Weight::from_ref_time(133_508 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Minimum execution time: 110 nanoseconds.
		Weight::from_ref_time(140_230 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Minimum execution time: 96 nanoseconds.
		Weight::from_ref_time(136_059 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(_i: u32, ) -> Weight {
		// Minimum execution time: 21_804_747 nanoseconds.
		Weight::from_ref_time(22_013_681_386 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Minimum execution time: 136 nanoseconds.
		Weight::from_ref_time(156_000 as u64)
			// Standard Error: 4_531
			.saturating_add(Weight::from_ref_time(46_817_640 as u64).saturating_mul(i as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		// Minimum execution time: 125 nanoseconds.
		Weight::from_ref_time(135_000 as u64)
			// Standard Error: 3_651
			.saturating_add(Weight::from_ref_time(2_021_172 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		// Minimum execution time: 120 nanoseconds.
		Weight::from_ref_time(131_000 as u64)
			// Standard Error: 348
			.saturating_add(Weight::from_ref_time(377_243 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Minimum execution time: 108 nanoseconds.
		Weight::from_ref_time(137_610 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Minimum execution time: 104 nanoseconds.
		Weight::from_ref_time(133_508 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Minimum execution time: 110 nanoseconds.
		Weight::from_ref_time(140_230 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Minimum execution time: 96 nanoseconds.
		Weight::from_ref_time(136_059 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(_i: u32, ) -> Weight {
		// Minimum execution time: 21_804_747 nanoseconds.
		Weight::from_ref_time(22_013_681_386 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Minimum execution time: 136 nanoseconds.
		Weight::from_ref_time(156_000 as u64)
			// Standard Error: 4_531
			.saturating_add(Weight::from_ref_time(46_817_640 as u64).saturating_mul(i as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		// Minimum execution time: 125 nanoseconds.
		Weight::from_ref_time(135_000 as u64)
			// Standard Error: 3_651
			.saturating_add(Weight::from_ref_time(2_021_172 as u64).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		// Minimum execution time: 120 nanoseconds.
		Weight::from_ref_time(131_000 as u64)
			// Standard Error: 348
			.saturating_add(Weight::from_ref_time(377_243 as u64).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
}
