//! Price Oracle for LendSphere Protocol
//! 
//! This module implements price feed functionality for asset valuation
//! used in collateral calculations and liquidation logic.

use soroban_sdk::{contracttype, Address, Env, BigInt, Map};

#[contracttype]
#[derive(Clone, Debug)]
pub struct PriceFeed {
    pub asset_address: Address,
    pub price: BigInt,
    pub decimals: u32,
    pub last_updated: u64,
    pub confidence: BigInt,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct OracleData {
    pub admin: Address,
    pub paused: bool,
    pub price_feeds: Map<Address, PriceFeed>,
    pub stale_period: u64,
}

pub struct PriceOracle;

impl PriceOracle {
    /// Initialize the oracle with admin address
    pub fn initialize(env: Env, admin: Address) {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        
        if env.storage().instance().has(&oracle_key) {
            panic!("Oracle already initialized");
        }

        let oracle_data = OracleData {
            admin: admin.clone(),
            paused: false,
            price_feeds: Map::new(&env),
            stale_period: 3600, // 1 hour
        };

        env.storage().instance().set(&oracle_key, oracle_data);
    }

    /// Update price for an asset (admin only)
    pub fn update_price(env: Env, admin: Address, asset_address: Address, price: BigInt, decimals: u32) {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let mut oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        admin.require_auth();

        let price_feed = PriceFeed {
            asset_address: asset_address.clone(),
            price,
            decimals,
            last_updated: env.ledger().timestamp(),
            confidence: BigInt::from_u32(&env, 950000), // 95% confidence
        };

        oracle_data.price_feeds.set(asset_address, price_feed);
        env.storage().instance().set(&oracle_key, oracle_data);
    }

    /// Get price for an asset
    pub fn get_price(env: Env, asset_address: Address) -> BigInt {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        if oracle_data.paused {
            panic!("Oracle is paused");
        }

        let price_feed = oracle_data.price_feeds.get(asset_address)
            .expect("Price feed not found for asset");

        // Check if price is stale
        let current_time = env.ledger().timestamp();
        if current_time - price_feed.last_updated > oracle_data.stale_period {
            panic!("Price feed is stale");
        }

        price_feed.price
    }

    /// Get price with decimals adjustment
    pub fn get_price_normalized(env: Env, asset_address: Address, target_decimals: u32) -> BigInt {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        let price_feed = oracle_data.price_feeds.get(asset_address)
            .expect("Price feed not found for asset");

        let price = price_feed.price;
        let decimals_diff = price_feed.decimals as i32 - target_decimals as i32;

        if decimals_diff > 0 {
            price * BigInt::from_u32(&env, 10_u32.pow(decimals_diff as u32))
        } else if decimals_diff < 0 {
            price / BigInt::from_u32(&env, 10_u32.pow((-decimals_diff) as u32))
        } else {
            price
        }
    }

    /// Pause oracle (admin only)
    pub fn pause(env: Env, admin: Address) {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let mut oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        admin.require_auth();
        oracle_data.paused = true;
        env.storage().instance().set(&oracle_key, oracle_data);
    }

    /// Unpause oracle (admin only)
    pub fn unpause(env: Env, admin: Address) {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let mut oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        admin.require_auth();
        oracle_data.paused = false;
        env.storage().instance().set(&oracle_key, oracle_data);
    }

    /// Set stale period (admin only)
    pub fn set_stale_period(env: Env, admin: Address, stale_period: u64) {
        let oracle_key = soroban_sdk::symbol!("ORACLE");
        let mut oracle_data: OracleData = env.storage().instance().get(&oracle_key).unwrap();
        
        admin.require_auth();
        oracle_data.stale_period = stale_period;
        env.storage().instance().set(&oracle_key, oracle_data);
    }
}
