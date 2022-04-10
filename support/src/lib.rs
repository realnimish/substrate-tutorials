#![cfg_attr(not(feature = "std"), no_std)]

// The trait used by pallet_loose_marketplace
// It must be place on a separate crate to avoid circular dependecies
// Both pallet_loose_marketplace and pallet_nft now have a dependency on support
// It will be written by the people who wrote pallet_loose_market in order for the people that wrote pallet_nft toi infect their pallet into the first one
pub trait Sellable<AccountId, RessourceId> {
	fn amount_owned(id: RessourceId, account: AccountId) -> u128;
	fn transfer(id: RessourceId, from: AccountId, to: AccountId, amount: u128) -> u128;
}
