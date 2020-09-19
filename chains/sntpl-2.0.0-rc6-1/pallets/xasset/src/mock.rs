use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
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
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type GenericAsset = pallet_generic_asset::Module<Test>;
pub type TransactionPayment = pallet_transaction_payment::Module<Test>;

parameter_types! {
	pub const ExistentialDeposit: u128 = 0;
}

// https://substrate.dev/rustdocs/v2.0.0-rc6/pallet_balances/struct.Module.html
impl pallet_balances::Trait for Test {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// The ubiquitous event type.
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

// https://substrate.dev/rustdocs/v2.0.0-rc6/pallet_generic_asset/struct.Module.html
impl pallet_generic_asset::Trait for Test {
    type Event = ();
    type Balance = u128;
    type AssetId = u32;
}

impl Trait for Test {
	type Event = ();
	type Currency = Balances;
	// type AssetCurrency = pallet_generic_asset::SpendingAssetCurrency<Self>;
}

pub type TemplateModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 1000), (2, 1000), (3, 100), (4, 100), (5, 20)],
	}.assimilate_storage(&mut t).unwrap();

	pallet_generic_asset::GenesisConfig::<Test> {
		assets: vec![99,100,101],
		endowed_accounts:vec![1,2,3,4,5],
		initial_balance: 8899,
		next_asset_id: 1001,
		staking_asset_id: 16000,
		spending_asset_id: 16001,
	}.assimilate_storage(&mut t).unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
