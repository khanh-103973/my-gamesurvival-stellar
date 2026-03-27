#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Map, String};

const PLAYER_KEY: Symbol = symbol_short!("PLAYER");

#[contract]
pub struct GameContract;

#[contractimpl]
impl GameContract {

    // Tạo player mới
    pub fn init_player(env: Env) {
        let mut player = Map::new(&env);
        player.set(symbol_short!("skill"), 0);
        player.set(symbol_short!("stress"), 0);
        player.set(symbol_short!("exp"), 0);

        env.storage().instance().set(&PLAYER_KEY, &player);
    }

    // Update stats khi chọn lựa
    pub fn choose(env: Env, skill: i32, stress: i32, exp: i32) {
        let mut player: Map<Symbol, i32> =
            env.storage().instance().get(&PLAYER_KEY).unwrap();

        player.set(symbol_short!("skill"), player.get(symbol_short!("skill")).unwrap() + skill);
        player.set(symbol_short!("stress"), player.get(symbol_short!("stress")).unwrap() + stress);
        player.set(symbol_short!("exp"), player.get(symbol_short!("exp")).unwrap() + exp);

        env.storage().instance().set(&PLAYER_KEY, &player);
    }

    // Lấy stats
    pub fn get_player(env: Env) -> Map<Symbol, i32> {
        env.storage().instance().get(&PLAYER_KEY).unwrap()
    }
}