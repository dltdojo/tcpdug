#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_support::{
	weights::{ Pays , DispatchClass },
	traits::{ Currency, ExistenceRequirement }
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// balance type using currency type
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: pallet_generic_asset::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
	// type AssetCurrency: Currency<Self::AccountId>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as XassetModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId,
							Balance = BalanceOf<T>,
							BlockNumber = <T as system::Trait>::BlockNumber,
							AssetId = <T as pallet_generic_asset::Trait>::AssetId,
							AssetBalance = <T as pallet_generic_asset::Trait>::Balance,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		TransferFunds(AccountId, AccountId,  Balance, BlockNumber),
		TransferToken(AccountId, AccountId, AssetId, AssetBalance, BlockNumber),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 ]
		pub fn transfer_funds ( origin, 
			  dest: T::AccountId, 
			  amount: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			T::Currency::transfer(&sender, &dest , amount, ExistenceRequirement::AllowDeath)?;
			Self::deposit_event(RawEvent::TransferFunds(sender, dest, amount, Self::now()));
			Ok(())
		}

		#[weight = 10_000 ]
		pub fn transfer_token ( origin, 
			  dest: T:: AccountId,
			  asset_id: T:: AssetId,
			  amount: <T as pallet_generic_asset::Trait>::Balance) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			pallet_generic_asset::Module::<T>::make_transfer(&asset_id, &sender, &dest, amount)?;
			Self::deposit_event(RawEvent::TransferToken(sender, dest, asset_id , amount, Self::now()));
			Ok(())
		}

		#[weight = (10_000, DispatchClass::Normal, Pays::No) ]
		pub fn transfer_token_feeless ( origin, 
			  dest: T:: AccountId,
			  asset_id: T:: AssetId,
			  amount: <T as pallet_generic_asset::Trait>::Balance) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			pallet_generic_asset::Module::<T>::make_transfer(&asset_id, &sender, &dest, amount)?;
			Self::deposit_event(RawEvent::TransferToken(sender, dest, asset_id , amount, Self::now()));
			Ok(())
		}
	}
}


impl <T: Trait> Module<T> {

	fn now() -> T::BlockNumber{
		<system::Module<T>>::block_number()
	}

}