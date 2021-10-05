//! Implementation of stakes.
use alloc::{collections::BTreeMap, collections::BTreeSet, string::String, vec::Vec};

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

#[inline]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();

    base64::encode(&preimage)
}

/// Writes an stake for owner for a specific amount.
pub(crate) fn write_stake_to(
    stakes_uref: URef,
    owner: Address,
    amount: U256,
) {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_put(stakes_uref, &dictionary_item_key, amount)
}

/// Reads an stake for a owner
pub(crate) fn read_stake_from(stakes_uref: URef, owner: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_get(stakes_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Reads stakes for all owners
pub(crate) fn read_stakes_from(stakes_uref: URef) -> BTreeMap<Address, U256> {
    storage::read_or_revert(stakes_uref)
}

/// Reads a reward for a owner
pub(crate) fn read_reward_from(rewards_uref: URef, owner: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(owner);
    storage::dictionary_get(rewards_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Reads rewards for all owners
pub(crate) fn read_rewards_from(rewards_uref: URef) -> BTreeMap<Address, U256> {
    storage::read_or_revert(rewards_uref)
}

/// Reads stakeholders
pub(crate) fn read_stakeholders_from(stakeholders_uref: URef) -> Vec<Address> {
    storage::read_or_revert(stakeholders_uref)
}