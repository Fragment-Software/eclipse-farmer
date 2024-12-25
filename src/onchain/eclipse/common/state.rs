use arrayref::{array_ref, array_refs};
use borsh::BorshDeserialize;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use solana_sdk::{program_error::ProgramError, program_option::COption, pubkey::Pubkey};

#[allow(unused)]
#[derive(BorshDeserialize, Debug)]
pub struct AmmFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}

#[allow(unused)]
#[derive(BorshDeserialize, Debug)]
pub struct AmmCurve {
    pub curve_type: u8,
    pub curve_parameters: u64,
}

#[allow(unused)]
#[derive(BorshDeserialize, Debug)]
pub struct AmmConfig {
    pub last_price: u64,
    pub last_balanced_price: u64,
    pub config_denominator: u64,
    pub volume_x: u64,
    pub volume_y: u64,
    pub volume_x_in_y: u64,
    pub deposit_cap: u64,
    pub regression_target: u64,
    pub oracle_type: u64,
    pub oracle_status: u64,
    pub oracle_main_slot_limit: u64,
    pub oracle_sub_confidence_limit: u64,
    pub oracle_sub_slot_limit: u64,
    pub oracle_pc_confidence_limit: u64,
    pub oracle_pc_slot_limit: u64,
    pub std_spread: u64,
    pub std_spread_buffer: u64,
    pub spread_coefficient: u64,
    pub price_buffer_coin: i64,
    pub price_buffer_pc: i64,
    pub rebalance_ratio: u64,
    pub fee_trade: u64,
    pub fee_platform: u64,
    pub oracle_main_slot_buffer: u64,
    pub config_temp4: u64,
    pub config_temp5: u64,
    pub config_temp6: u64,
    pub config_temp7: u64,
    pub config_temp8: u64,
}

#[allow(unused)]
#[derive(BorshDeserialize, Debug)]
pub struct Amm {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub freeze_trade: u8,
    pub freeze_deposit: u8,
    pub freeze_withdraw: u8,
    pub base_decimals: u8,
    pub token_program_id: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub fee_account: Pubkey,
    pub oracle_main_account: Pubkey,
    pub oracle_sub_account: Pubkey,
    pub oracle_pc_account: Pubkey,
    pub fees: AmmFees,
    pub curve: AmmCurve,
    pub config: AmmConfig,
    pub amm_p_temp1: Pubkey,
    pub amm_p_temp2: Pubkey,
    pub amm_p_temp3: Pubkey,
    pub amm_p_temp4: Pubkey,
    pub amm_p_temp5: Pubkey,
}

fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 32];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}

fn unpack_coption_u64(src: &[u8; 12]) -> Result<COption<u64>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 8];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountState {
    #[default]
    Uninitialized,
    Initialized,
    Frozen,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate: COption<Pubkey>,
    pub state: AccountState,
    pub is_native: COption<u64>,
    pub delegated_amount: u64,
    pub close_authority: COption<Pubkey>,
}

impl Account {
    pub fn deserialize(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 165];
        let (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority) =
            array_refs![src, 32, 32, 8, 36, 1, 12, 8, 36];
        Ok(Self {
            mint: Pubkey::new_from_array(*mint),
            owner: Pubkey::new_from_array(*owner),
            amount: u64::from_le_bytes(*amount),
            delegate: unpack_coption_key(delegate)?,
            state: AccountState::try_from_primitive(state[0])
                .or(Err(ProgramError::InvalidAccountData))?,
            is_native: unpack_coption_u64(is_native)?,
            delegated_amount: u64::from_le_bytes(*delegated_amount),
            close_authority: unpack_coption_key(close_authority)?,
        })
    }
}
