// Import necessary libraries and modules
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::convert::TryInto;

// Define the GameState struct
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameState {
    love: u64,
    prestige: u64,
    game_time: u64,
    boss: Boss,
    owner_id: AccountId,
}

// Implement the Default trait for GameState
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

// Define the Boss struct
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Boss {
    level: u64,
    hp: u64,
}

// Implement the GameState methods
#[near_bindgen]
impl GameState {
    // Create a new GameState instance
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

    // Update the game_time with the elapsed_time
    pub fn update_game_time(&mut self, elapsed_time: u64) {
        self.game_time += elapsed_time;
    }

    // Increase the prestige by 1 and emit an event
    pub fn increase_prestige(&mut self) {
        self.prestige += 1;
        // Emit event for prestige increase
        env::log_str(&format!("Prestige level increased to {}", self.prestige));
    }

    // Spread love by damaging the boss and reset the love to 0
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

// Implement the Boss methods
impl Boss {
    // Create a new Boss instance with default values
    pub fn new() -> Self {
        Self { level: 1, hp: 100 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_game_state() -> GameState {
        let account_id = "player.testnet".to_string();
        GameState::new(account_id)
    }

    // Test updating game time
    #[test]
    fn test_update_game_time() {
        let mut game_state = setup_game_state();
        game_state.update_game_time(10);

        assert_eq!(game_state.game_time, 10);
    }

    // Test increasing prestige and spreading love
    #[test]
    fn test_increase_prestige_and_spread_love() {
        let mut game_state = setup_game_state();

        // Increase prestige
        game_state.increase_prestige();
        assert_eq!(game_state.prestige, 1);

        // Spread love
        game_state.love = 100;
        game_state.spread_love();
        assert_eq!(game_state.love, 0);
        assert_eq!(game_state.boss.hp, 0);
        assert_eq!(game_state.boss.level, 2);
    }

    // Test boss leveling up and HP scaling
    #[test]
    fn test_boss_leveling_and_hp_scaling() {
        let mut game_state = setup_game_state();

        // Defeat the first boss
        game_state.love = 100;
        game_state.spread_love();

        // Check if the boss leveled up
        assert_eq!(game_state.boss.level, 2);
        assert_eq!(game_state.boss.hp, 200); // HP scaling based on level

        // Defeat the second boss
        game_state.love = 200;
        game_state.spread_love();

        // Check if the boss leveled up
        assert_eq!(game_state.boss.level, 3);
        assert_eq!(game_state.boss.hp, 300); // HP scaling based on level
    }
}
