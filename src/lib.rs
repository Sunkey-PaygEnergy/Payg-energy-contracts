#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};
pub use errors::Error;
pub use types::*;

#[contract]
pub struct SunkeyPaygContract;

#[contractimpl]
impl SunkeyPaygContract {
    pub fn initialize(e: Env, admin: Address) -> Result<(), Error> {
        if storage::get_admin(&e).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        storage::set_admin(&e, &admin);
        Ok(())
    }

    pub fn set_admin(e: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin = storage::get_admin(&e).ok_or(Error::NotInitialized)?;
        current_admin.require_auth();
        storage::set_admin(&e, &new_admin);
        Ok(())
    }

    pub fn get_admin(e: Env) -> Result<Address, Error> {
        storage::get_admin(&e).ok_or(Error::NotInitialized)
    }

    pub fn version(_e: Env) -> u32 {
        1
    }
}
