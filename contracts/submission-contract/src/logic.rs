#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, BytesN, Env, Symbol, Panic,
};

#[contracttype]
#[derive(Clone)]
pub struct LockData {
    pub unlock_time: u64,
    pub owner: Address,
}

#[contract]
pub struct Lock;

#[contractimpl]
impl Lock {
    pub fn init(env: Env, unlock_time: u64, owner: Address) {
        let current_time = env.ledger().timestamp();

        if unlock_time <= current_time {
            panic!(
                "LOCK__UnlockTimeNotInFuture: unlock_time={}, current_time={}",
                unlock_time,
                current_time
            );
        }

        let data = LockData {
            unlock_time,
            owner: owner.clone(),
        };

        env.storage().set(&Symbol::short("lock_data"), &data);

        env.events().publish(
            (Symbol::short("Init"), owner),
            unlock_time,
        );
    }

    pub fn withdraw(env: Env, caller: Address) {
        let current_time = env.ledger().timestamp();

        let lock_data: LockData = env
            .storage()
            .get_unchecked(&Symbol::short("lock_data"))
            .expect("LOCK__NoLockData");

        if caller != lock_data.owner {
            panic!(
                "LOCK__NotOwner: caller={}, owner={}",
                caller.to_string(&env),
                lock_data.owner.to_string(&env)
            );
        }

        if current_time < lock_data.unlock_time {
            panic!(
                "LOCK__WithdrawalTooEarly: unlock_time={}, current_time={}",
                lock_data.unlock_time,
                current_time
            );
        }

        let balance = env.balance();

        // Emit event like Solidity's Withdrawal
        env.events().publish(
            (Symbol::short("Withdrawal"), caller),
            (balance, current_time),
        );

        // Transfer funds to owner
        env.payments().pay(
            &env.current_contract_address(),
            &lock_data.owner,
            &balance,
        );
    }

    pub fn get_unlock_time(env: Env) -> u64 {
        let lock_data: LockData = env
            .storage()
            .get_unchecked(&Symbol::short("lock_data"))
            .unwrap();
        lock_data.unlock_time
    }

    pub fn get_owner(env: Env) -> Address {
        let lock_data: LockData = env
            .storage()
            .get_unchecked(&Symbol::short("lock_data"))
            .unwrap();
        lock_data.owner
    }
}
