//! LendSphere - Decentralized Lending & Borrowing Protocol on Stellar
//! 
//! This smart contract implements the core lending and borrowing functionality
//! for the LendSphere protocol on the Stellar network using Soroban.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map, BigInt, IntoVal};
use soroban_token_sdk::Token;

// Contract storage keys
const DATA_KEY: Symbol = soroban_sdk::symbol!("DATA");
const POOLS_KEY: Symbol = soroban_sdk::symbol!("POOLS");
const ORACLE_KEY: Symbol = soroban_sdk::symbol!("ORACLE");
const GOVERNANCE_KEY: Symbol = soroban_sdk::symbol!("GOV");

// Data structures
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pool {
    pub token_address: Address,
    pub total_supply: BigInt,
    pub total_borrows: BigInt,
    pub supply_rate: BigInt,
    pub borrow_rate: BigInt,
    pub collateral_factor: BigInt,
    pub liquidation_threshold: BigInt,
    pub last_update_time: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPosition {
    pub supplied_amount: BigInt,
    pub borrowed_amount: BigInt,
    pub collateral_enabled: bool,
    pub health_factor: BigInt,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolData {
    pub admin: Address,
    pub paused: bool,
    pub emergency_pause: bool,
    pub total_market_cap: BigInt,
}

#[contract]
pub struct LendSphereProtocol;

#[contractimpl]
impl LendSphereProtocol {
    /// Initialize the protocol with admin address
    pub fn initialize(env: Env, admin: Address) {
        // Ensure contract is not already initialized
        if env.storage().instance().has(&DATA_KEY) {
            panic!("Contract already initialized");
        }

        let protocol_data = ProtocolData {
            admin: admin.clone(),
            paused: false,
            emergency_pause: false,
            total_market_cap: 0.into(),
        };

        env.storage().instance().set(&DATA_KEY, protocol_data);
        env.storage().instance().set(&POOLS_KEY, Vec::new(env));
    }

    /// Create a new lending pool for a token
    pub fn create_pool(
        env: Env,
        token_address: Address,
        collateral_factor: BigInt,
        liquidation_threshold: BigInt,
    ) {
        let data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can create pools
        data.admin.require_auth();

        let pool = Pool {
            token_address: token_address.clone(),
            total_supply: 0.into(),
            total_borrows: 0.into(),
            supply_rate: 0.into(),
            borrow_rate: 0.into(),
            collateral_factor,
            liquidation_threshold,
            last_update_time: env.ledger().timestamp(),
        };

        let mut pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        pools.push_back(pool);
        env.storage().instance().set(&POOLS_KEY, pools);
    }

    /// Supply tokens to a lending pool
    pub fn supply(env: Env, user: Address, token_address: Address, amount: BigInt) {
        let data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Check if protocol is paused
        if data.paused || data.emergency_pause {
            panic!("Protocol is paused");
        }

        user.require_auth();

        // Find the pool
        let pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let pool_index = pools.iter().position(|p| p.token_address == token_address)
            .expect("Pool not found");

        // Transfer tokens from user to contract
        let token = Token::new(env, &token_address);
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Update user position
        let position_key = (user.clone(), token_address.clone());
        let mut position: UserPosition = env.storage().persistent()
            .get(&position_key)
            .unwrap_or(UserPosition {
                supplied_amount: 0.into(),
                borrowed_amount: 0.into(),
                collateral_enabled: true,
                health_factor: BigInt::from_u32(&env, 1000000), // 1x health factor
            });

        position.supplied_amount += amount;
        env.storage().persistent().set(&position_key, position);

        // Update pool
        let mut pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let mut pool = pools.get(pool_index).unwrap();
        pool.total_supply += amount;
        pools.set(pool_index, pool);
        env.storage().instance().set(&POOLS_KEY, pools);
    }

    /// Borrow tokens from a lending pool
    pub fn borrow(env: Env, user: Address, token_address: Address, amount: BigInt) {
        let data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Check if protocol is paused
        if data.paused || data.emergency_pause {
            panic!("Protocol is paused");
        }

        user.require_auth();

        // Check if user has sufficient collateral
        let health_factor = Self::calculate_health_factor(env.clone(), user.clone());
        if health_factor < BigInt::from_u32(&env, 150000) { // 1.5x minimum health factor
            panic!("Insufficient collateral");
        }

        // Find the pool
        let pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let pool_index = pools.iter().position(|p| p.token_address == token_address)
            .expect("Pool not found");

        // Check if pool has sufficient liquidity
        let pool = pools.get(pool_index).unwrap();
        if pool.total_supply - pool.total_borrows < amount {
            panic!("Insufficient liquidity in pool");
        }

        // Update user position
        let position_key = (user.clone(), token_address.clone());
        let mut position: UserPosition = env.storage().persistent()
            .get(&position_key)
            .unwrap_or(UserPosition {
                supplied_amount: 0.into(),
                borrowed_amount: 0.into(),
                collateral_enabled: true,
                health_factor: BigInt::from_u32(&env, 1000000),
            });

        position.borrowed_amount += amount;
        position.health_factor = Self::calculate_health_factor(env.clone(), user.clone());
        env.storage().persistent().set(&position_key, position);

        // Update pool
        let mut pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let mut pool = pools.get(pool_index).unwrap();
        pool.total_borrows += amount;
        pools.set(pool_index, pool);
        env.storage().instance().set(&POOLS_KEY, pools);

        // Transfer tokens to user
        let token = Token::new(env, &token_address);
        token.transfer(&env.current_contract_address(), &user, &amount);
    }

    /// Withdraw tokens from a lending pool
    pub fn withdraw(env: Env, user: Address, token_address: Address, amount: BigInt) {
        let data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Check if protocol is paused
        if data.paused {
            panic!("Protocol is paused");
        }

        user.require_auth();

        // Update user position
        let position_key = (user.clone(), token_address.clone());
        let mut position: UserPosition = env.storage().persistent()
            .get(&position_key)
            .expect("User position not found");

        if position.supplied_amount < amount {
            panic!("Insufficient supplied amount");
        }

        // Check health factor after withdrawal
        let temp_supplied = position.supplied_amount - amount;
        if position.borrowed_amount > 0 {
            let temp_health = Self::calculate_health_factor_with_values(
                env.clone(), 
                user.clone(), 
                temp_supplied, 
                position.borrowed_amount
            );
            if temp_health < BigInt::from_u32(&env, 150000) {
                panic!("Withdrawal would cause undercollateralization");
            }
        }

        position.supplied_amount -= amount;
        if position.supplied_amount == 0.into() && position.borrowed_amount == 0.into() {
            env.storage().persistent().remove(&position_key);
        } else {
            position.health_factor = Self::calculate_health_factor(env.clone(), user.clone());
            env.storage().persistent().set(&position_key, position);
        }

        // Update pool
        let pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let pool_index = pools.iter().position(|p| p.token_address == token_address)
            .expect("Pool not found");

        let mut pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let mut pool = pools.get(pool_index).unwrap();
        pool.total_supply -= amount;
        pools.set(pool_index, pool);
        env.storage().instance().set(&POOLS_KEY, pools);

        // Transfer tokens to user
        let token = Token::new(env, &token_address);
        token.transfer(&env.current_contract_address(), &user, &amount);
    }

    /// Repay borrowed tokens
    pub fn repay(env: Env, user: Address, token_address: Address, amount: BigInt) {
        user.require_auth();

        // Update user position
        let position_key = (user.clone(), token_address.clone());
        let mut position: UserPosition = env.storage().persistent()
            .get(&position_key)
            .expect("User position not found");

        if position.borrowed_amount < amount {
            panic!("Repayment amount exceeds borrowed amount");
        }

        position.borrowed_amount -= amount;
        if position.supplied_amount == 0.into() && position.borrowed_amount == 0.into() {
            env.storage().persistent().remove(&position_key);
        } else {
            position.health_factor = Self::calculate_health_factor(env.clone(), user.clone());
            env.storage().persistent().set(&position_key, position);
        }

        // Update pool
        let pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let pool_index = pools.iter().position(|p| p.token_address == token_address)
            .expect("Pool not found");

        let mut pools: Vec<Pool> = env.storage().instance().get(&POOLS_KEY).unwrap();
        let mut pool = pools.get(pool_index).unwrap();
        pool.total_borrows -= amount;
        pools.set(pool_index, pool);
        env.storage().instance().set(&POOLS_KEY, pools);

        // Transfer tokens from user to contract
        let token = Token::new(env, &token_address);
        token.transfer(&user, &env.current_contract_address(), &amount);
    }

    /// Get user's position in a specific pool
    pub fn get_user_position(env: Env, user: Address, token_address: Address) -> UserPosition {
        let position_key = (user, token_address);
        env.storage().persistent()
            .get(&position_key)
            .unwrap_or(UserPosition {
                supplied_amount: 0.into(),
                borrowed_amount: 0.into(),
                collateral_enabled: true,
                health_factor: BigInt::from_u32(&env, 1000000),
            })
    }

    /// Get all pools
    pub fn get_pools(env: Env) -> Vec<Pool> {
        env.storage().instance().get(&POOLS_KEY).unwrap()
    }

    /// Calculate user's health factor
    fn calculate_health_factor(env: Env, user: Address) -> BigInt {
        // This is a simplified calculation
        // In production, this would use real oracle prices and multiple positions
        let total_collateral = 1000000.into(); // Placeholder
        let total_debt = 500000.into(); // Placeholder
        
        if total_debt == 0.into() {
            return BigInt::from_u32(&env, 1000000); // Max health factor for no debt
        }

        (total_collateral * 1000000) / total_debt
    }

    /// Calculate health factor with custom values
    fn calculate_health_factor_with_values(env: Env, _user: Address, collateral: BigInt, debt: BigInt) -> BigInt {
        if debt == 0.into() {
            return BigInt::from_u32(&env, 1000000);
        }

        (collateral * 1000000) / debt
    }

    /// Emergency pause function (admin only)
    pub fn emergency_pause(env: Env, admin: Address) {
        let mut data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        admin.require_auth();
        data.emergency_pause = true;
        env.storage().instance().set(&DATA_KEY, data);
    }

    /// Unpause function (admin only)
    pub fn unpause(env: Env, admin: Address) {
        let mut data: ProtocolData = env.storage().instance().get(&DATA_KEY).unwrap();
        admin.require_auth();
        data.paused = false;
        data.emergency_pause = false;
        env.storage().instance().set(&DATA_KEY, data);
    }
}
