#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::ensure;
use sp_std::vec::Vec;
use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Is mapping UniqueAssetId with UniqueAssetDetails
	#[pallet::storage]
	#[pallet::getter(fn unique_asset)]
	pub(super) type UniqueAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, UniqueAssetId, UniqueAssetDetails<T>>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	/// The holdings of a specific account for a specific asset.
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniqueAssetId,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	/// Nonce for id of the next created asset
	pub(super) type Nonce<T: Config> = StorageValue<_, UniqueAssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New unique asset created
		Created { creator: T::AccountId, asset_id: UniqueAssetId },
		/// Some assets have been burned
		Burned { asset_id: UniqueAssetId, owner: T::AccountId, total_supply: u128 },
		/// Some assets have been transferred
		Transferred { asset_id: UniqueAssetId, from: T::AccountId, to: T::AccountId, amount: u128 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The asset ID is unknown
		Unknown,
		/// The signing account does not own any amount of this asset
		NotOwned,
		/// Supply must be positive
		NoSupply,
		/// Type overflow
		TypeOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint(origin: OriginFor<T>, metadata: Vec<u8>, supply: u128) -> DispatchResult {
			// Ensure call is signed
			let who = ensure_signed(origin)?;

			// Ensure supply is positive
			if supply <= 0 {
				return Err(Error::<T>::NoSupply)?;
			}

			// Increments nonce for next ids
			let id = Self::nonce();
			<Nonce<T>>::set(Self::nonce().checked_add(1).ok_or(Error::<T>::TypeOverflow)?);

			// Generates asset details
			let asset_details = UniqueAssetDetails::new(who.clone(), metadata, supply);

			// Stores unique asset
			<UniqueAsset<T>>::insert(id, asset_details);

			// Stores Account
			<Account<T>>::insert(id, who.clone(), supply);

			// Emmit event
			Self::deposit_event(Event::Created { creator: who, asset_id: id });

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn burn(origin: OriginFor<T>, asset_id: UniqueAssetId, amount: u128) -> DispatchResult {
			// Ensure call is signed
			let who = ensure_signed(origin)?;

			// Ensure asset exists
			ensure!(Self::unique_asset(asset_id).is_some(), Error::<T>::Unknown);

			// Ensure own some
			ensure!(Self::account(0, who.clone()) > 0, Error::<T>::NotOwned);

			// Handle situation where origin is transfering more than his amount
			let origin_amount = Self::account(0, who.clone());
			let mut new_amount = amount;
			if amount > origin_amount {
				new_amount = origin_amount;
			}

			// Remove amount from origin Account
			Account::<T>::mutate(asset_id, who.clone(), |total_amount| -> DispatchResult {
				*total_amount -= new_amount;
				Ok(())
			})?;

			let mut new_supply = 0;

			// Remove amount from UniqueAsset
			UniqueAsset::<T>::try_mutate(asset_id, |details| -> DispatchResult {
				let asset_details = details.as_mut().ok_or(Error::<T>::Unknown)?;

				asset_details.supply -= new_amount;
				new_supply = asset_details.supply;

				Ok(())
			})?;

			// Emmit event
			Self::deposit_event(Event::Burned { asset_id, owner: who, total_supply: new_supply });

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: UniqueAssetId,
			amount: u128,
			to: T::AccountId,
		) -> DispatchResult {
			// Ensure call is signed
			let who = ensure_signed(origin)?;

			// Ensure asset exists
			ensure!(Self::unique_asset(asset_id).is_some(), Error::<T>::Unknown);

			// Ensure own some
			ensure!(Self::account(0, who.clone()) > 0, Error::<T>::NotOwned);

			// Handle situation where origin is transfering more than his amount
			let origin_amount = Self::account(0, who.clone());
			let mut new_amount = amount;
			if amount > origin_amount {
				new_amount = origin_amount;
			}

			// Remove amount from origin Account
			Account::<T>::mutate(asset_id, who.clone(), |total_amount| -> DispatchResult {
				*total_amount -= new_amount;
				Ok(())
			})?;

			// Add amount from origin Account
			Account::<T>::mutate(asset_id, to.clone(), |total_amount| -> DispatchResult {
				*total_amount += new_amount;
				Ok(())
			})?;

			// Emmit event
			Self::deposit_event(Event::Transferred { asset_id, from: who, to, amount: new_amount });

			Ok(())
		}
	}
}
