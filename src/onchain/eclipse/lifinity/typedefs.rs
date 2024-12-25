use borsh::BorshSerialize;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use crate::onchain::eclipse::common::{constants::LIFINITY_PROGRAM_ID, typedefs::InstructionArgs};

#[derive(Debug, PartialEq, Eq)]
pub enum TradeDirection {
    AtoB,
    BtoA,
}

pub struct SwapInfo {
    pub wallet_pubkey: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amm_pool_pubkey: Pubkey,
    pub amount_in: u64,
    pub amount_out: Option<u64>,
    pub should_transfer_source: bool,
}

impl SwapInfo {
    pub fn new(
        wallet_pubkey: &Pubkey,
        token_a: &Pubkey,
        token_b: &Pubkey,
        amm_pool_key: &Pubkey,
        amount_in: u64,
        amount_out: Option<u64>,
        should_transfer_source: bool,
    ) -> Self {
        Self {
            wallet_pubkey: *wallet_pubkey,
            token_a: *token_a,
            token_b: *token_b,
            amm_pool_pubkey: *amm_pool_key,
            amount_in,
            amount_out,
            should_transfer_source,
        }
    }
}

#[derive(Debug, BorshSerialize)]
pub struct LifinitySwapInput {
    amount_in: u64,
    minimum_amount_out: u64,
}

#[derive(Debug)]
pub struct LifinitySwapArgs {
    pub authority: Pubkey,
    pub amm: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_info: Pubkey,
    pub destination_info: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub pool_mint: Pubkey,
    pub fee_account: Pubkey,
    pub token_program: Pubkey,
    pub oracle_main_account: Pubkey,
    pub oracle_sub_account: Pubkey,
    pub oracle_pc_account: Pubkey,
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

impl InstructionArgs<LifinitySwapInput> for LifinitySwapArgs {
    fn program_id(&self) -> Pubkey {
        LIFINITY_PROGRAM_ID
    }

    fn accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.authority, false),
            AccountMeta::new(self.amm, false),
            AccountMeta::new_readonly(self.user_transfer_authority, true),
            AccountMeta::new(self.source_info, false),
            AccountMeta::new(self.destination_info, false),
            AccountMeta::new(self.swap_source, false),
            AccountMeta::new(self.swap_destination, false),
            AccountMeta::new(self.source_mint, false),
            AccountMeta::new(self.destination_mint, false),
            AccountMeta::new(self.pool_mint, false),
            AccountMeta::new(self.fee_account, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.oracle_main_account, false),
            AccountMeta::new_readonly(self.oracle_sub_account, false),
            AccountMeta::new_readonly(self.oracle_pc_account, false),
        ]
    }

    fn data(&self) -> LifinitySwapInput {
        LifinitySwapInput { amount_in: self.amount_in, minimum_amount_out: self.minimum_amount_out }
    }
}
