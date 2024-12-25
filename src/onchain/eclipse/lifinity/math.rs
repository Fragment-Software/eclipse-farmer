use super::{
    constants::{LIQUIDITY_FEES_DENOMINATOR, LIQUIDITY_FEES_NUMERATOR},
    typedefs::TradeDirection,
};

pub fn calculate_min_amount_out(
    amount_in: u64,
    token_a_reserves: u64,
    token_b_reserves: u64,
    direction: TradeDirection,
) -> u64 {
    let amount_out = get_amount_out(amount_in, token_a_reserves, token_b_reserves, direction);
    let slippage_adjustment = 1.0 - (1f64 / 100.0);
    (amount_out as f64 / slippage_adjustment).round() as u64
}

fn get_amount_out(
    amount_in: u64,
    token_a_reserves: u64,
    token_b_reserves: u64,
    direction: TradeDirection,
) -> u64 {
    let (pool_src, pool_dst) = match direction {
        TradeDirection::AtoB => (token_a_reserves, token_b_reserves),
        TradeDirection::BtoA => (token_b_reserves, token_a_reserves),
    };

    let fees = (amount_in * LIQUIDITY_FEES_NUMERATOR) / LIQUIDITY_FEES_DENOMINATOR;

    let amount_in_with_fee = amount_in - fees;
    ((pool_dst as u128 * amount_in_with_fee as u128) / (pool_src + amount_in_with_fee) as u128)
        as u64
}
