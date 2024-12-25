use borsh::BorshSerialize;
use solana_program::hash::hash;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::{
    constants::{
        ASSOCIATED_TOKEN_PROGRAM_ID, INSTRUCTION_NAMESPACE, SYSTEM_PROGRAM_ID,
        TOKEN_2022_PROGRAM_ID,
    },
    typedefs::CreateAtaArgs,
};

pub trait InstructionData: BorshSerialize {
    const INSTRUCTION_NAME: &'static str;

    fn get_function_hash() -> [u8; 8] {
        let preimage: String = format!("{}:{}", INSTRUCTION_NAMESPACE, Self::INSTRUCTION_NAME);
        let sighash: [u8; 32] = hash(preimage.as_bytes()).to_bytes();
        let mut output: [u8; 8] = [0u8; 8];
        output.copy_from_slice(&sighash[..8]);
        output
    }

    fn get_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&Self::get_function_hash());
        self.serialize(&mut buf).expect("Failed to serialize data");
        buf
    }
}

pub fn create_ata(args: CreateAtaArgs) -> Instruction {
    Instruction {
        program_id: ASSOCIATED_TOKEN_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(args.funding_address, true),
            AccountMeta::new(args.associated_account_address, false),
            AccountMeta::new_readonly(args.wallet_address, false),
            AccountMeta::new_readonly(args.token_mint_address, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(args.token_program_id, false),
        ],
        data: vec![args.instruction],
    }
}

pub fn sync_native(token_program_id: &Pubkey, account_pubkey: &Pubkey) -> Instruction {
    Instruction {
        program_id: *token_program_id,
        accounts: vec![AccountMeta::new(*account_pubkey, false)],
        data: vec![17],
    }
}

pub fn close_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Instruction {
    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*owner_pubkey, signer_pubkeys.is_empty()));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Instruction { program_id: *token_program_id, accounts, data: vec![9] }
}

pub fn unwrap_eth(user: &Pubkey, user_ata: &Pubkey) -> [Instruction; 2] {
    [
        sync_native(&TOKEN_2022_PROGRAM_ID, user_ata),
        close_account(&TOKEN_2022_PROGRAM_ID, user_ata, user, user, &[user]),
    ]
}
