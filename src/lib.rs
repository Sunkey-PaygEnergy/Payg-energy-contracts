#![no_std]

pub mod errors;
pub mod events;
pub mod types;

use soroban_sdk::{contract, contractimpl, Env};
pub use errors::Error;
pub use types::*;

#[contract]
pub struct SunkeyPaygContract;

#[contractimpl]
impl SunkeyPaygContract {
    pub fn version(_e: Env) -> u32 {
        1
    }
}
