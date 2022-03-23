#[warn(unused_assignments)]
#[warn(non_snake_case)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::{U64, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise, BlockHeight };
use std::{convert::TryInto, mem::size_of};

use crate::internal::*;
pub use crate::metadata::*;

mod internal;
mod metadata;

const REVEAL_FEE: u128 = 1_000_000_000_000_000_000_000_000;

//A contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HeroRustContract {
    pub owner_id: AccountId,
    pub reward_pool: Balance,
    pub hero_reveal: Vector<Hero>,
    pub play_history: Vector<PlayLog>,
    pub hero_map: LookupMap<AccountId, String>,
}

impl Default for HeroRustContract {
    fn default() -> Self {
        env::panic(b"Hero Rust Contract ne init")
    }
}

#[near_bindgen]
impl HeroRustContract {

    #[init]
    pub fn new (
        owner_id: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already Init.");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),"You not owned account ID"
        );

        Self {
            owner_id,
            reward_pool: 0_u128,
            hero_reveal: Vector::new(b"v".to_vec()),
            play_history: Vector::new(b"r".to_vec()),
            hero_map: LookupMap::new(b"h".to_vec()),
        }
    }

    //Reveal Hero
    #[payable]
    pub fn reveal_hero(&mut self) {
        let owner = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        
        if deposit != REVEAL_FEE {
            panic!("Reveal Fee must be {}", REVEAL_FEE)
        }

        self.reward_pool = self.reward_pool + deposit;

        let rarity = from_seed(env::random_seed());
        let heroseed = env::random_seed()[..size_of::<HeroSeed>()].try_into().unwrap();
        
        self.hero_reveal.push(&Hero {
            id: encode(heroseed),
            rarity: from_seed(env::random_seed()),
            owner: owner.clone(),
            created: env::block_timestamp(),
        });

        self.hero_map.insert(&owner, &rarity);
    }

    //Ruble Hero 
    pub fn hero_rumble(&mut self) -> HeroRumbleResult {

        let owner = env::predecessor_account_id();
        assert_eq!(
            owner.clone(),
            env::signer_account_id(),
            "Invalid user for method"
        );

        let rarity = self.get_hero_info(owner.clone());
        assert!(
            rarity != "DEAD".to_string(),
            "Need Reveal New Hero"
        );

        let inhuman = "INHUMAN".to_string();
        let hero = "HERO".to_string();

        let mut player_power = 0_u8;
        if rarity == inhuman {
            player_power = 60_u8;
        } else if rarity == hero {
            player_power = 70_u8;
        } else {
            player_power = 50_u8;
        }

        let enermy_power = get_power();
        let mut rumble_result = "DRAW".to_string();
        let mut reward_share = self.reward_pool / 4_u128;

        if enermy_power >= player_power {
            let dead =  "DEAD".to_string();
            self.hero_map.insert(&owner.clone(), &dead);
            rumble_result = "LOSE".to_string();
            reward_share = 0_u128;
        } else {
            rumble_result = "WIN".to_string();
            Promise::new(owner.clone()).transfer(reward_share);
            self.reward_pool = self.reward_pool - reward_share;
        }

        let result = HeroRumbleResult {
            block: env::block_index().into(),
            player: owner.clone(),
            player_power: player_power.into(),
            enermy_power: enermy_power.into(),
            result: rumble_result,
            reward: reward_share.into(),
            playtime: env::block_timestamp().into(),
        };

        self.play_history.push(&PlayLog {
            block: env::block_index(),
            player: owner.clone(),
            result: result.result.clone(),
            reward: result.reward.into(),
        });

        result
    }

    //View mode 
    pub fn get_contract_pool(&self) -> u128 {
        self.reward_pool
    }

    pub fn get_hero_info(&self, account_id: AccountId) -> String {
        let herorarity = self.hero_map.get(&account_id.into()).expect("UNBORN");
        herorarity as String
    }

    fn get_revealhero(&self, index: u64) -> HeroRevealInfo {
        let enq = self.hero_reveal.get(index).expect("Error: No reveal item");
        HeroRevealInfo {
            id: enq.id.into(),
            rarity: enq.rarity.into(),
            owner: enq.owner.clone(),
            created: enq.created.into(),
        }
    }

    pub fn get_revealhero_history(&self, from_index: u64, limit: u64) -> Vec<HeroRevealInfo> {
        let c: u64 = self.hero_reveal.len() as u64;
        (from_index..std::cmp::min(from_index + limit, c))
            .map(|index| self.get_revealhero(c - index - 1))
            .collect()
    }

    fn get_rumble(&self, index: u64) -> PlayHistory {
        let enq = self.play_history.get(index).expect("Error: No play time");
        PlayHistory {
            block: enq.block.into(),
            player: enq.player.into(),
            result: enq.result.into(),
            reward: self.reward_pool.into(),
        }
    }

    pub fn get_rumble_history(&self, from_index: u64, limit: u64) -> Vec<PlayHistory> {
        let c: u64 = self.play_history.len() as u64;
        (from_index..std::cmp::min(from_index + limit, c))
            .map(|index| self.get_rumble(c - index - 1))
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, MockedBlockchain};

    mod test_utils;
    use test_utils::*;

    fn account_battlefields() -> String {
        "battlefields".to_string()
    }

    fn account_hero() -> String {
        "hero".to_string()
    }

    fn open_battlefields() -> HeroRustContract {
        let context = VMContextBuilder::new()
            .current_account_id(account_battlefields())
            .predecessor_account_id(account_battlefields())
            .signer_account_id(account_battlefields())
            .attached_deposit(REVEAL_FEE * 10)
            .finish();
        testing_env!(context.clone());
        let mut contract = HeroRustContract::new(account_battlefields());
        contract
    }

    #[test]
    #[should_panic(expected = "Reveal Fee must be {}", REVEAL_FEE)]
    fn test_reveal_hero_fail() {
        let mut contract = open_battlefields();
        
        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .attached_deposit(REVEAL_FEE * 10)
            .finish();
        testing_env!(context.clone());
        contract.reveal_hero();
    }

    #[test]
    fn test_reveal_hero() {
        let mut contract = open_battlefields();
        
        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .attached_deposit(REVEAL_FEE)
            .finish();
        testing_env!(context.clone());
        contract.reveal_hero();
        assert_eq!(contract.get_contract_pool(),REVEAL_FEE);
        assert_eq!(contract.get_revealhero_history(0,1).len(),1);
    }

    #[test]
    #[should_panic(expected = "Invalid user for method")]
    fn test_rumblehero_invalid_signer() {
        let mut contract = open_battlefields();

        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .attached_deposit(REVEAL_FEE)
            .finish();
        testing_env!(context.clone());
        contract.hero_rumble();
    }

    #[test]
    #[should_panic(expected = "UNBORN")]
    fn test_rumble_without_hero() {
        let mut contract = open_battlefields();

        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .signer_account_id(account_hero())
            .attached_deposit(REVEAL_FEE)
            .finish();
        testing_env!(context.clone());
        contract.hero_rumble();
    }

    #[test]
    #[should_panic(expected = "Error: No play time")]
    fn test_before_rumble() {
        let mut contract = open_battlefields();

        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .signer_account_id(account_hero())
            .attached_deposit(REVEAL_FEE)
            .finish();
        testing_env!(context.clone());
        contract.get_rumble(0);
    }
    
    #[test]
    fn test_rumble_completed() {
        let mut contract = open_battlefields();

        let context = VMContextBuilder::new()
            .predecessor_account_id(account_hero())
            .signer_account_id(account_hero())
            .attached_deposit(REVEAL_FEE)
            .finish();
        testing_env!(context.clone());
        contract.reveal_hero();
        contract.hero_rumble();
        contract.get_rumble(0);
        assert_eq!(contract.get_rumble_history(0,1).len(),1);
    }
}