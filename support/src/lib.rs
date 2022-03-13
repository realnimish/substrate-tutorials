#![cfg_attr(not(feature = "std"), no_std)]

pub trait Sellable<AccountId, RessourceId> {
	fn amount_owned(id: RessourceId, account: AccountId) -> u128;
	fn transfer(id: RessourceId, from: AccountId, to: AccountId, amount: u128) -> u128;
}
