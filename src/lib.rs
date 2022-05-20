use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault,
    Promise,
    PromiseOrValue
};

use crate::config::*;
mod config;
use crate::account::*;
mod account;
use crate::util::*;
mod util;
use crate::internal::*;
mod internal;
use crate::enumuration::*;
mod enumuration;
use crate::core_impl::*;
mod core_impl;
#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// pub struct StakingContractV1 {
//     pub owner_id: AccountId,
//     pub ft_contract_id: AccountId,
//     pub config: Config,
//     pub total_stake_balance: Balance,
//     pub total_paid_reward_balance: Balance,
//     pub total_stakers: Balance,
//     pub pre_reward: Balance,
//     pub last_block_balance_change: BlockHeight,
//     pub accounts: LookupMap<AccountId, Account>,
//     pub paused: bool,
//     pub pause_in_block: BlockHeight,
// }

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_stakers: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>,
    pub paused: bool,
    pub pause_in_block: BlockHeight,
    // pub new_data: U128(0)
}
#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_stakers: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            paused: false,
            pause_in_block: 0,
        }
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());

        let account_stake = self.accounts.get(&account);

        if account_stake.is_some() {
            // refund
            refund_deposit(0);
        } else {
            // crate account
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account.clone());
            let after_storage_usage = env::storage_usage();

            refund_deposit(after_storage_usage - before_storage_usage);
            // refund
        }
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let account = self.accounts.get(&account_id);

        if account.is_some() {
            U128(1)
        } else {
            U128(0)
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
    // #[private]
    // #[init(ignore_state)]
    // pub fn migrate(&)
}
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_content(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_content(false);

        testing_env!(context.build());

        let config = Config {
            reward_numerator: 500,
            reward_denumerator: 100000,
        };

        let contract = StakingContract::new(accounts(0), "ft_contract".to_string(), config);

        assert_eq!(contract.owner_id, accounts(0).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string());
        assert_eq!(config.reward_numerator, contract.config.reward_numerator);
        assert_eq!(
            config.reward_denumerator,
            contract.config.reward_denumerator
        );
        assert_eq!(config.paussed, false);
    }
}
