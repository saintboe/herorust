/*
 * HeroRust Contract -> 
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, Balance};
use std::{mem::size_of, convert::TryInto};
use near_sdk::json_types::{U128};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const REVEAL_FEE: u128 = 0_000_100_000_000_000_000_000_000;

pub type HeroSeed = [u8; size_of::<u128>()];
pub type HeroId = String;

//hero 
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Hero {
    pub id: HeroId,
    pub rarity: Rarity,
    pub owner: AccountId,
    pub created: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_camel_case_types)]
pub enum Rarity {
    HUMAN,
    INHUMAN,
    HERO,
}


impl Default for Rarity {
    pub fn from_seed(seed: Vec<u8>) -> Rarity {
        let rarity = u128::from_le_bytes(seed[..16].try_into().unwrap()) % 100;
        if rarity > 8 {
            Rarity::HUMAN
        } else if rarity > 2 {
            Rarity::INHUMAN
        } else {
            Rarity::HERO
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HeroRust {
    pub owner_id: AccountId,
    pub reveal_fee: Balance,
    pub hero_map: LookupMap<AccountId, Hero>,
}

impl Default for HeroRust {
    fn default() -> Self {
        env::panic(b"HeroRust contract must be initial before game start")
    }
}

#[near_bindgen]
impl HeroRust {

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
            reveal_fee: 0_u128,
            hero_map: LookupMap::new(b"h".to_vec()),
        }
    }

    #[payable]
    pub fn reveal_hero(
        &mut self,
    ) -> Hero {
        let owner = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if deposit != U128(REVEAL_FEE) {
            panic!("Reveal Fee must be REVEAL_FEE")
        }

        let now = env::block_timestamp();
        let heroseed = env::random_seed()[..size_of::<HeroSeed>()]
            .try_into()
            .unwrap();
        let hero = Hero {
            id: heroseed,
            rarity: Rarity::from_seed(env::random_seed()),
            owner,
            created: now,
        };

        self.hero_map.insert(&owner, &hero);

    }
}

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct Welcome {
//     records: LookupMap<String, String>,
// }

// impl Default for Welcome {
//   fn default() -> Self {
//     Self {
//       records: LookupMap::new(b"a".to_vec()),
//     }
//   }
// }

// #[near_bindgen]
// impl Welcome {
//     pub fn set_greeting(&mut self, message: String) {
//         let account_id = env::signer_account_id();

//         // Use env::log to record logs permanently to the blockchain!
//         env::log(format!("Saving greeting '{}' for account '{}'", message, account_id,).as_bytes());

//         self.records.insert(&account_id, &message);
//     }

//     // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
//     // self.records.get(&account_id) is not yet defined.
//     // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
//     pub fn get_greeting(&self, account_id: String) -> String {
//         match self.records.get(&account_id) {
//             Some(greeting) => greeting,
//             None => "Hello".to_string(),
//         }
//     }
// }


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     // mock the context for testing, notice "signer_account_id" that was accessed above from env::
//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "alice_near".to_string(),
//             signer_account_id: "bob_near".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "carol_near".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 0,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Welcome::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             "howdy".to_string(),
//             contract.get_greeting("bob_near".to_string())
//         );
//     }

//     #[test]
//     fn get_default_greeting() {
//         let context = get_context(vec![], true);
//         testing_env!(context);
//         let contract = Welcome::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             "Hello".to_string(),
//             contract.get_greeting("francis.near".to_string())
//         );
//     }
// }
