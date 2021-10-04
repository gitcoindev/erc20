//! Implementation of stakes.
use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{constants::STAKEHOLDERS_KEY_NAME, constants::STAKES_KEY_NAME, 
    constants::REWARDS_KEY_NAME, detail, Address};

#[inline]
pub(crate) fn stakeholders_uref() -> URef {
    detail::get_uref(STAKEHOLDERS_KEY_NAME)
}

#[inline]
pub(crate) fn stakes_uref() -> URef {
    detail::get_uref(STAKES_KEY_NAME)
}

#[inline]
pub(crate) fn rewards_uref() -> URef {
    detail::get_uref(REWARDS_KEY_NAME)
}

/// Creates a dictionary item key for an (owner, spender) pair.
fn make_dictionary_item_key(owner: Address, spender: Address) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut owner.to_bytes().unwrap_or_revert());
    preimage.append(&mut spender.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    hex::encode(&key_bytes)
}

/// Writes an stake for owner for a specific amount.
pub(crate) fn write_stake_to(
    stakes_uref: URef,
    owner: Address,
    spender: Address,
    amount: U256,
) {
    let dictionary_item_key = make_dictionary_item_key(owner, spender);
    storage::dictionary_put(stakes_uref, &dictionary_item_key, amount)
}

/// Reads an stake for a owner
pub(crate) fn read_stake_from(stakes_uref: URef, owner: Address, spender: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(owner, spender);
    storage::dictionary_get(stakes_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
