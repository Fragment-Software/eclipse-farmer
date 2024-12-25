use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use crate::onchain::eclipse::common::{
    constants::{ETH_PUBKEY, ETH_USDC_AMM_PUBKEY, SOL_PUBKEY, SOL_USDC_AMM_PUBKEY, USDC_PUBKEY},
    ixs::InstructionData,
    typedefs::InstructionArgs,
};

use super::typedefs::{LifinitySwapArgs, LifinitySwapInput, TradeDirection};

pub fn determine_trade_direction(
    pool_pubkey: &Pubkey,
    token_a: &Pubkey,
    token_b: &Pubkey,
) -> eyre::Result<TradeDirection> {
    match *pool_pubkey {
        ETH_USDC_AMM_PUBKEY => {
            if token_a == &ETH_PUBKEY && token_b == &USDC_PUBKEY {
                Ok(TradeDirection::AtoB)
            } else if token_a == &USDC_PUBKEY && token_b == &ETH_PUBKEY {
                Ok(TradeDirection::BtoA)
            } else {
                eyre::bail!("Failed to determine trade direction")
            }
        }
        SOL_USDC_AMM_PUBKEY => {
            if token_a == &SOL_PUBKEY && token_b == &USDC_PUBKEY {
                Ok(TradeDirection::AtoB)
            } else if token_a == &USDC_PUBKEY && token_b == &SOL_PUBKEY {
                Ok(TradeDirection::BtoA)
            } else {
                eyre::bail!("Failed to determine trade direction")
            }
        }
        _ => eyre::bail!("Failed to determine trade direction"),
    }
}

pub fn determine_pool_pubkey(token_a: &Pubkey, token_b: &Pubkey) -> Option<Pubkey> {
    if (token_a == &ETH_PUBKEY && token_b == &USDC_PUBKEY) ||
        (token_a == &USDC_PUBKEY && token_b == &ETH_PUBKEY)
    {
        Some(ETH_USDC_AMM_PUBKEY)
    } else if (token_a == &SOL_PUBKEY && token_b == &USDC_PUBKEY) ||
        (token_a == &USDC_PUBKEY && token_b == &SOL_PUBKEY)
    {
        Some(SOL_USDC_AMM_PUBKEY)
    } else {
        None
    }
}

impl InstructionData for LifinitySwapInput {
    const INSTRUCTION_NAME: &'static str = "swap";
}

pub fn assemble_swap_ix(args: LifinitySwapArgs) -> Instruction {
    Instruction {
        program_id: args.program_id(),
        accounts: args.accounts(),
        data: args.data().get_data(),
    }
}

pub fn extract_out_value(log: &str) -> Option<u64> {
    if let Some(start) = log.find(r#""out""#) {
        let start = start + 6;
        if let Some(end) = log[start..].find(',') {
            return log[start..start + end].trim().parse::<u64>().ok();
        }
    }
    None
}
