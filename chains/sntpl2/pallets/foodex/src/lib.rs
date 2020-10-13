#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::EnsureOrigin,
	weights::{DispatchClass, Pays},
};
use frame_system::ensure_signed;
use primitives::{Amount, AssetId, Balance, Price};
use sp_runtime::{
	traits::{StaticLookup, Zero},
	FixedPointNumber,
};

use orml_traits::{MultiCurrency, MultiCurrencyExtended};

/// The module configuration trait.
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type OrmlCurrency: MultiCurrencyExtended<
		Self::AccountId,
		CurrencyId = AssetId,
		Balance = Balance,
		Amount = Amount,
	>;

	// Origins are used to identify network participants and control access.
	// This is used to identify the pallet's admin.
	// https://substrate.dev/docs/en/knowledgebase/runtime/origin
	type EnsureRoot: EnsureOrigin<Self::Origin>;
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
	{
		Mint(AccountId, AssetId, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		Unknown,
		CannotCreatePoolWithZeroLiquidity,
		CannotCreatePoolWithZeroInitialPrice,
		CannotCreatePoolWithSameAssets,
		TokenPoolAlreadyExists,
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

		#[weight = (10_000, DispatchClass::Normal, Pays::No)]
		fn sudo_mint( origin, target: <T::Lookup as StaticLookup>::Source , asset: AssetId, amount: Balance) -> dispatch::DispatchResult {
			// https://github.com/paritytech/substrate/blob/58d6df36f32452027010d7f532c9aaa5ded54570/frame/nicks/src/lib.rs#L202
			T::EnsureRoot::ensure_origin(origin)?;
			let who = T::Lookup::lookup(target)?;
			T::OrmlCurrency::deposit(asset, &who, amount)?;
			Self::deposit_event(RawEvent::Mint(who, asset, amount));
			Ok(())
		}

		#[weight = (10_000, DispatchClass::Normal, Pays::No)]
		pub fn mint(origin, asset: AssetId, amount: Balance) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			T::OrmlCurrency::deposit(asset, &who, amount)?;
			Self::deposit_event(RawEvent::Mint(who, asset, amount));
			Ok(())
		}

		#[weight = 10_000]
		pub fn create_swap_pool(origin, asset_a: AssetId, asset_b: AssetId, amount: Balance, initial_price: Price) -> dispatch::DispatchResult {
			// https://github.com/galacticcouncil/hack.HydraDX-node/blob/7ce0806150c41d61eca18d6d26ed1e710572aa10/pallets/amm/src/lib.rs#L173
			let who = ensure_signed(origin)?;
			ensure!(
				!amount.is_zero(),
				Error::<T>::CannotCreatePoolWithZeroLiquidity
			);
			ensure!(
				!initial_price.is_zero(),
				Error::<T>::CannotCreatePoolWithZeroInitialPrice
			);
			ensure!(
				asset_a != asset_b,
				Error::<T>::CannotCreatePoolWithSameAssets
			);

			ensure!(
				!Self::exists(asset_a, asset_b),
				Error::<T>::TokenPoolAlreadyExists
			);

			// TODO
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	// Public immutables
	// https://github.com/galacticcouncil/hack.HydraDX-node/blob/0a10dfdc6648d40d19853938d811e0ac8e3627a7/pallets/amm/src/lib.rs#L574
	fn exists(asset_a: AssetId, asset_b: AssetId) -> bool {
		// TODO
		false
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	use frame_support::{
		assert_noop, assert_ok, impl_outer_origin, parameter_types, weights::Weight,
	};
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
		Perbill,
	};

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
		frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap()
			.into()
	}

	#[test]
	fn issuing_asset_units_to_issuer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(FoodexModule::foo(Origin::signed(1)));
		});
	}
}
