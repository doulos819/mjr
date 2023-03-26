use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::convert::TryInto;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameState {
    love: u64,
    prestige: u64,
    game_time: u64,
    boss: Boss,
    owner_id: AccountId,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            love: 0,
            prestige: 0,
            game_time: 0,
            boss: Boss::new(),
            owner_id: AccountId::new_unchecked("".to_string()),
        }
    }
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Boss {
    level: u64,
    hp: u64,
}

#[near_bindgen]
impl GameState {
    pub fn new(account_id_str: String) -> Self {
        // Create an AccountId using `TryInto`.
        let account_id: AccountId = account_id_str.try_into().unwrap_or_else(|_| {
            panic!("Invalid account ID provided");
        });

        Self {
            love: 0,
            prestige: 0,
            game_time: 0,
            boss: Boss::new(),
            owner_id: account_id,
        }
    }

    pub fn update_game_time(&mut self, elapsed_time: u64) {
        self.game_time += elapsed_time;
    }

    pub fn increase_prestige(&mut self) {
        self.prestige += 1;
        // Emit event for prestige increase
        env::log_str(&format!("Prestige level increased to {}", self.prestige));
    }

    pub fn spread_love(&mut self) {
        let damage = self.love;
        self.boss.hp = self.boss.hp.saturating_sub(damage);
        self.love = 0;

        if self.boss.hp == 0 {
            // Emit event for boss defeat
            env::log_str(&format!("Boss at level {} defeated", self.boss.level));
            self.boss.level += 1;
            self.boss.hp = self.boss.level * 100; // Example HP scaling based on level
        }
    }
}

impl Boss {
    pub fn new() -> Self {
        Self { level: 1, hp: 100 }
    }
}

