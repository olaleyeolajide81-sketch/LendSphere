//! Interest Rate Model for LendSphere Protocol
//! 
//! This module implements the interest rate calculation logic
//! based on market utilization and other factors.

use soroban_sdk::{Env, BigInt};

/// Interest rate model parameters
#[derive(Clone, Debug)]
pub struct InterestRateModel {
    pub base_rate: BigInt,
    pub multiplier: BigInt,
    pub jump_multiplier: BigInt,
    pub kink: BigInt,
}

impl InterestRateModel {
    /// Create a new interest rate model
    pub fn new(base_rate: BigInt, multiplier: BigInt, jump_multiplier: BigInt, kink: BigInt) -> Self {
        Self {
            base_rate,
            multiplier,
            jump_multiplier,
            kink,
        }
    }

    /// Calculate borrow rate based on utilization
    pub fn get_borrow_rate(&self, env: &Env, cash: BigInt, borrows: BigInt) -> BigInt {
        let utilization = self.get_utilization_rate(env, cash, borrows);
        
        if utilization < self.kink {
            // Below kink: base + multiplier * utilization
            self.base_rate + (self.multiplier * utilization) / 1000000
        } else {
            // Above kink: base + multiplier * kink + jump_multiplier * (utilization - kink)
            let normal_rate = self.base_rate + (self.multiplier * self.kink) / 1000000;
            let excess_util = utilization - self.kink;
            normal_rate + (self.jump_multiplier * excess_util) / 1000000
        }
    }

    /// Calculate supply rate based on utilization and borrow rate
    pub fn get_supply_rate(&self, env: &Env, cash: BigInt, borrows: BigInt, borrow_rate: BigInt) -> BigInt {
        let utilization = self.get_utilization_rate(env, cash, borrows);
        
        // Supply rate = borrow rate * utilization * (1 - reserve factor)
        // Assuming 10% reserve factor for now
        let reserve_factor = BigInt::from_u32(env, 100000); // 10%
        (borrow_rate * utilization * (1000000 - reserve_factor)) / (1000000 * 1000000)
    }

    /// Calculate utilization rate
    pub fn get_utilization_rate(&self, env: &Env, cash: BigInt, borrows: BigInt) -> BigInt {
        let total = cash + borrows;
        if total == 0.into() {
            return 0.into();
        }
        
        (borrows * 1000000) / total
    }
}

/// Default interest rate model parameters
pub fn default_interest_rate_model(env: &Env) -> InterestRateModel {
    InterestRateModel::new(
        BigInt::from_u32(env, 20000),    // 2% base rate
        BigInt::from_u32(env, 400000),   // 40% multiplier
        BigInt::from_u32(env, 800000),   // 80% jump multiplier
        BigInt::from_u32(env, 800000),   // 80% kink
    )
}
