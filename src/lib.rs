#![no_std]

pub mod errors;

use soroban_sdk::{contract, contractimpl, Env};
pub use errors::Error;

#[contract]
pub struct SunkeyPaygContract;

#[contractimpl]
impl SunkeyPaygContract {
    pub fn version(_e: Env) -> u32 {
        1
    }
}
