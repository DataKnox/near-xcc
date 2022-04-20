use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, Promise, PromiseResult};

near_sdk::setup_alloc!();

#[ext_contract(ext_ft)]
pub trait Game {
    fn createGame(&mut self) -> u32;
    fn makeAGuess(&mut self, guess: u32, gameId: u32) -> String;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
    fn my_other_callback(&self) -> String;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn start(&self) -> Promise {
        ext_ft::createGame(
            &"dev-1650465405897-76865791768218",
            0,
            5_000_000_000_000, //comment for formatting
        )
        .then(ext_self::my_callback(
            &env::current_account_id(),
            0,
            5_000_000_000_000,
        ))
    }

    pub fn my_callback(&self) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let game_id = near_sdk::serde_json::from_slice::<u32>(&result).unwrap();
                game_id.to_string()
            }
        }
    }

    pub fn play_game(&self, guess: u32, game_id: u32) -> Promise {
        ext_ft::makeAGuess(
            guess,
            game_id,
            &"dev-1650465405897-76865791768218",
            0, // comment for formatting
            10_000_000_000_000,
        )
        .then(ext_self::my_other_callback(
            &env::current_account_id(),
            0,
            10_000_000_000_000,
        ))
    }

    pub fn my_other_callback(&self) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let result = near_sdk::serde_json::from_slice::<String>(&result).unwrap();
                result.to_string()
            }
        }
    }
}
