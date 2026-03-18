//! Liquidation Engine for LendSphere Protocol
//! 
//! This module handles the liquidation of undercollateralized positions
//! to ensure the solvency of the lending protocol.

use soroban_sdk::{contracttype, Address, Env, BigInt, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct LiquidationParams {
    pub liquidator_incentive: BigInt,
    pub close_factor: BigInt,
    pub liquidation_delay: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct LiquidationEvent {
    pub liquidator: Address,
    pub borrower: Address,
    pub collateral_token: Address,
    pub debt_token: Address,
    pub collateral_amount: BigInt,
    pub debt_amount: BigInt,
    pub timestamp: u64,
}

pub struct LiquidationEngine;

impl LiquidationEngine {
    /// Default liquidation parameters
    pub fn default_params(env: &Env) -> LiquidationParams {
        LiquidationParams {
            liquidator_incentive: BigInt::from_u32(env, 1050000), // 5% incentive
            close_factor: BigInt::from_u32(env, 500000), // 50% close factor
            liquidation_delay: 0, // No delay for immediate liquidation
        }
    }

    /// Check if a position can be liquidated
    pub fn can_liquidate(
        env: &Env,
        borrower: Address,
        collateral_token: Address,
        debt_token: Address,
        health_factor: BigInt,
        liquidation_threshold: BigInt,
    ) -> bool {
        // Position is liquidatable if health factor < liquidation threshold
        health_factor < liquidation_threshold
    }

    /// Calculate liquidation amounts
    pub fn calculate_liquidation_amounts(
        env: &Env,
        debt_to_cover: BigInt,
        collateral_price: BigInt,
        debt_price: BigInt,
        params: &LiquidationParams,
    ) -> (BigInt, BigInt) {
        // Calculate required collateral to cover the debt
        let debt_value = debt_to_cover * debt_price;
        let required_collateral = (debt_value * params.liquidator_incentive) / 
            (collateral_price * 1000000);

        (required_collateral, debt_to_cover)
    }

    /// Execute liquidation
    pub fn liquidate(
        env: Env,
        liquidator: Address,
        borrower: Address,
        collateral_token: Address,
        debt_token: Address,
        debt_to_cover: BigInt,
    ) -> LiquidationEvent {
        liquidator.require_auth();

        // Get prices from oracle
        let collateral_price = crate::oracle::PriceOracle::get_price(env.clone(), collateral_token.clone());
        let debt_price = crate::oracle::PriceOracle::get_price(env.clone(), debt_token.clone());

        // Get liquidation parameters
        let params = Self::default_params(&env);

        // Calculate liquidation amounts
        let (collateral_amount, actual_debt_covered) = Self::calculate_liquidation_amounts(
            &env,
            debt_to_cover,
            collateral_price,
            debt_price,
            &params,
        );

        // Update borrower's position
        let borrower_position_key = (borrower.clone(), debt_token.clone());
        let mut borrower_position: crate::lib::UserPosition = env.storage().persistent()
            .get(&borrower_position_key)
            .expect("Borrower position not found");

        // Reduce borrowed amount
        borrower_position.borrowed_amount -= actual_debt_covered;
        
        // Update collateral position
        let collateral_position_key = (borrower.clone(), collateral_token.clone());
        let mut collateral_position: crate::lib::UserPosition = env.storage().persistent()
            .get(&collateral_position_key)
            .expect("Collateral position not found");

        collateral_position.supplied_amount -= collateral_amount;

        // Update positions in storage
        if borrower_position.borrowed_amount == 0.into() && borrower_position.supplied_amount == 0.into() {
            env.storage().persistent().remove(&borrower_position_key);
        } else {
            borrower_position.health_factor = crate::lib::LendSphereProtocol::calculate_health_factor(
                env.clone(), 
                borrower.clone()
            );
            env.storage().persistent().set(&borrower_position_key, borrower_position);
        }

        if collateral_position.supplied_amount == 0.into() && collateral_position.borrowed_amount == 0.into() {
            env.storage().persistent().remove(&collateral_position_key);
        } else {
            env.storage().persistent().set(&collateral_position_key, collateral_position);
        }

        // Transfer debt tokens from liquidator to protocol
        let debt_token_contract = soroban_token_sdk::Token::new(&env, &debt_token);
        debt_token_contract.transfer(&liquidator, &env.current_contract_address(), &actual_debt_covered);

        // Transfer collateral tokens from protocol to liquidator
        let collateral_token_contract = soroban_token_sdk::Token::new(&env, &collateral_token);
        collateral_token_contract.transfer(&env.current_contract_address(), &liquidator, &collateral_amount);

        // Create liquidation event
        let event = LiquidationEvent {
            liquidator: liquidator.clone(),
            borrower: borrower.clone(),
            collateral_token: collateral_token.clone(),
            debt_token: debt_token.clone(),
            collateral_amount,
            debt_amount: actual_debt_covered,
            timestamp: env.ledger().timestamp(),
        };

        // Store liquidation event
        let events_key = soroban_sdk::symbol!("LIQ_EVENTS");
        let mut events: Vec<LiquidationEvent> = env.storage().instance()
            .get(&events_key)
            .unwrap_or(Vec::new(&env));
        events.push_back(event.clone());
        env.storage().instance().set(&events_key, events);

        event
    }

    /// Get liquidation history
    pub fn get_liquidation_history(env: Env, limit: u32) -> Vec<LiquidationEvent> {
        let events_key = soroban_sdk::symbol!("LIQ_EVENTS");
        let events: Vec<LiquidationEvent> = env.storage().instance()
            .get(&events_key)
            .unwrap_or(Vec::new(&env));

        if events.len() <= limit {
            events
        } else {
            let start = events.len() - limit;
            let mut result = Vec::new(&env);
            for i in start..events.len() {
                result.push_back(events.get(i).unwrap());
            }
            result
        }
    }

    /// Calculate health factor for a user
    pub fn calculate_health_factor_detailed(
        env: &Env,
        user: Address,
    ) -> BigInt {
        let mut total_collateral_value = 0.into();
        let mut total_debt_value = 0.into();

        // This would need to iterate through all user positions
        // For now, using simplified calculation
        
        if total_debt_value == 0.into() {
            return BigInt::from_u32(env, 1000000); // Max health factor
        }

        (total_collateral_value * 1000000) / total_debt_value
    }
}
