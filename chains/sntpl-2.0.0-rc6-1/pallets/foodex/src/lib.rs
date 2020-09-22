// https://github.com/paritytech/substrate/blob/ddf83eb5ab514588281fb20aac9df4e048a5508b/frame/assets/src/lib.rs
// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

// use sp_std::{fmt::Debug};
use sp_runtime::{ traits::{
	Member, AtLeast32Bit, AtLeast32BitUnsigned}};
// use codec::{Encode, Decode};
use frame_support::{Parameter, decl_module, decl_event, decl_storage, decl_error,
	traits::{ ReservableCurrency }
};
use frame_system::ensure_signed;

// type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// The module configuration trait.
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	/// The arithmetic type of asset identifier.
	type AssetId: Parameter + AtLeast32Bit + Default + Copy;

	/// The currency mechanism.
	type Currency: ReservableCurrency<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Trait> as FoodexModule {
		//
	}
}

// TODO: force_create, allowing for an assets `virtuals` to be hot-wired.

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::Balance,
		<T as Trait>::AssetId,
	{
		/// Some asset class was created. \[asset_id, creator, owner\]
		Created(AssetId, AccountId, AccountId),
		/// Some assets were issued. \[asset_id, owner, total_supply\]
		Issued(AssetId, AccountId, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Transfer amount should be non-zero
		AmountZero,
		/// Account balance must be greater than or equal to the transfer amount
		BalanceLow,
		/// Balance should be non-zero
		BalanceZero,
		/// The signing account has no permission to do the operation.
		NoPermission,
		/// The given asset ID is unknown.
		Unknown,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		fn foo(origin) {
			let _origin = ensure_signed(origin)?;
		}
	}
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
	// Public immutables

	// Get the asset `id` balance of `who`.
	//pub fn balance(id: T::AssetId, who: T::AccountId) -> T::Balance {
	//	Account::<T>::get(id, who).balance
	//}

	// Get the total supply of an asset `id`.
	//pub fn total_supply(id: T::AssetId) -> T::Balance {
	//	Details::<T>::get(id).map(|x| x.supply).unwrap_or_else(Zero::zero)
	//}
}

#[cfg(test)]
mod tests {
	use super::*;

	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use sp_core::H256;
	use sp_runtime::{Perbill, traits::{BlakeTwo256, IdentityLookup}, testing::Header};

	impl_outer_origin! {
		pub enum Origin for Test where system = frame_system {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl frame_system::Trait for Test {
		type BaseCallFilter = ();
		type Origin = Origin;
		type Index = u64;
		type Call = ();
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type MaximumExtrinsicWeight = MaximumBlockWeight;
		type AvailableBlockRatio = AvailableBlockRatio;
		type MaximumBlockLength = MaximumBlockLength;
		type Version = ();
		type ModuleToIndex = ();
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}

	impl pallet_balances::Trait for Test {
		// type MaxLocks = ();
		type Balance = u64;
		type DustRemoval = ();
		type Event = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type WeightInfo = ();
	}

	impl Trait for Test {
		type Currency = Balances;
		type Event = ();
		type Balance = u64;
		type AssetId = u32;
	}
	type System = frame_system::Module<Test>;
	type Balances = pallet_balances::Module<Test>;
	type FoodexModule = Module<Test>;

	fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn issuing_asset_units_to_issuer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(FoodexModule::foo(Origin::signed(1)));
		});
	}
	
}
