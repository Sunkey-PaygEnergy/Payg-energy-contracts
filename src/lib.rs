#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct SunkeyPaygContract;

#[contractimpl]
impl SunkeyPaygContract {
    pub fn version(_e: Env) -> u32 {
        1
    }
}
