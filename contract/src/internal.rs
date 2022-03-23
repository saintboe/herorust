use crate::*;
use near_sdk::{bs58};

pub(crate) fn from_seed(seed: Vec<u8>) -> Rarity {
    let rarity = u128::from_le_bytes(seed[..16].try_into().unwrap()) % 100;
    if rarity > 50 {
        "HUMAN".to_string()
    } else if rarity > 20 {
        "INHUMAN".to_string()
    } else {
        "HERO".to_string()
    }
}

pub(crate) fn encode(key: HeroSeed) -> HeroId {
    bs58::encode(key).into_string()
}

pub(crate) fn get_power() -> u8 {
    let ramdompower: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
    let power = 100_u16 * ramdompower as u16 / 0x100_u16 + 1;

    power as u8
}