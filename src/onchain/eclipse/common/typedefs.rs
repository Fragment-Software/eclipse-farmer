use borsh::BorshSerialize;
use serde::Deserialize;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

pub trait InstructionArgs<T>
where
    T: BorshSerialize,
{
    fn program_id(&self) -> Pubkey;
    fn accounts(&self) -> Vec<AccountMeta>;
    fn data(&self) -> T;
}

#[derive(Debug)]
pub struct CreateAtaArgs {
    pub funding_address: Pubkey,
    pub associated_account_address: Pubkey,
    pub wallet_address: Pubkey,
    pub token_mint_address: Pubkey,
    pub token_program_id: Pubkey,
    pub instruction: u8,
}

#[derive(Deserialize, Debug)]
pub struct TokenAmount {
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct ParsedTokenAccountInfo {
    #[serde(rename = "tokenAmount")]
    pub token_amount: TokenAmount,
    pub mint: String,
}

#[derive(Deserialize, Debug)]
pub struct ParsedTokenAccount {
    pub info: ParsedTokenAccountInfo,
}
