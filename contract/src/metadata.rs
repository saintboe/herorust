use crate::*;

pub type HeroId = String;
pub type Rarity = String;

pub type HeroSeed = [u8; size_of::<u128>()];

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hero {
    pub id: HeroId,
    pub rarity: Rarity,
    pub owner: AccountId,
    pub created: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayLog {
    pub block: BlockHeight,
    pub player: AccountId,
    pub result: String,
    pub reward: Balance,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayHistory {
    pub block: U64,
    pub player: AccountId,
    pub result: String,
    pub reward: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HeroRevealInfo {
    pub id: String,
    pub rarity: String,
    pub owner: AccountId,
    pub created: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HeroRumbleResult {
    pub block: U64,
    pub player: AccountId,
    pub player_power: u8,
    pub enermy_power: u8,
    pub result: String,
    pub reward: U128,
    pub playtime: U64,
}
