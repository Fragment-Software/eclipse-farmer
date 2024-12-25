use solana_sdk::pubkey::Pubkey;

use super::constants::{ASSOCIATED_TOKEN_PROGRAM_ID, LIFINITY_PROGRAM_ID};

pub fn derive_ata(user: &Pubkey, token_mint: &Pubkey, token_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&user.to_bytes(), &token_program_id.to_bytes(), &token_mint.to_bytes()],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
}

pub fn derive_program_authority(amm_pubkey: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[&amm_pubkey.to_bytes()], &LIFINITY_PROGRAM_ID)
}
