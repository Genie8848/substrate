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

//! Autogenerated weights for pallet_bounties
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_bounties
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/bounties/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bounties.
pub trait WeightInfo {
	fn propose_bounty(d: u32, ) -> Weight;
	fn approve_bounty() -> Weight;
	fn propose_curator() -> Weight;
	fn unassign_curator() -> Weight;
	fn accept_curator() -> Weight;
	fn award_bounty() -> Weight;
	fn claim_bounty() -> Weight;
	fn close_bounty_proposed() -> Weight;
	fn close_bounty_active() -> Weight;
	fn extend_bounty_expiry() -> Weight;
	fn spend_funds(b: u32, ) -> Weight;
}

/// Weights for pallet_bounties using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	/// The range of component `d` is `[0, 300]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Minimum execution time: 33_366 nanoseconds.
		Weight::from_ref_time(34_444_773)
			// Standard Error: 1_161
			.saturating_add(Weight::from_ref_time(4_723).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		// Minimum execution time: 14_478 nanoseconds.
		Weight::from_ref_time(14_763_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		// Minimum execution time: 13_376 nanoseconds.
		Weight::from_ref_time(13_705_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		// Minimum execution time: 38_072 nanoseconds.
		Weight::from_ref_time(38_676_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		// Minimum execution time: 33_207 nanoseconds.
		Weight::from_ref_time(34_415_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	fn award_bounty() -> Weight {
		// Minimum execution time: 28_033 nanoseconds.
		Weight::from_ref_time(28_343_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		// Minimum execution time: 75_855 nanoseconds.
		Weight::from_ref_time(76_318_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		// Minimum execution time: 41_955 nanoseconds.
		Weight::from_ref_time(42_733_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		// Minimum execution time: 58_267 nanoseconds.
		Weight::from_ref_time(59_604_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		// Minimum execution time: 24_893 nanoseconds.
		Weight::from_ref_time(25_299_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:2 w:2)
	// Storage: System Account (r:4 w:4)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Minimum execution time: 8_846 nanoseconds.
		Weight::from_ref_time(20_166_004)
			// Standard Error: 28_485
			.saturating_add(Weight::from_ref_time(26_712_253).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	/// The range of component `d` is `[0, 300]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Minimum execution time: 33_366 nanoseconds.
		Weight::from_ref_time(34_444_773)
			// Standard Error: 1_161
			.saturating_add(Weight::from_ref_time(4_723).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		// Minimum execution time: 14_478 nanoseconds.
		Weight::from_ref_time(14_763_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		// Minimum execution time: 13_376 nanoseconds.
		Weight::from_ref_time(13_705_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		// Minimum execution time: 38_072 nanoseconds.
		Weight::from_ref_time(38_676_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		// Minimum execution time: 33_207 nanoseconds.
		Weight::from_ref_time(34_415_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	fn award_bounty() -> Weight {
		// Minimum execution time: 28_033 nanoseconds.
		Weight::from_ref_time(28_343_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		// Minimum execution time: 75_855 nanoseconds.
		Weight::from_ref_time(76_318_000)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		// Minimum execution time: 41_955 nanoseconds.
		Weight::from_ref_time(42_733_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		// Minimum execution time: 58_267 nanoseconds.
		Weight::from_ref_time(59_604_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		// Minimum execution time: 24_893 nanoseconds.
		Weight::from_ref_time(25_299_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:2 w:2)
	// Storage: System Account (r:4 w:4)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Minimum execution time: 8_846 nanoseconds.
		Weight::from_ref_time(20_166_004)
			// Standard Error: 28_485
			.saturating_add(Weight::from_ref_time(26_712_253).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(b.into())))
	}
}
