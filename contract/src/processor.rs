use crate::errors::PresaleError;
use arrayref::array_ref;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, clock::UnixTimestamp,
    entrypoint::ProgramResult, hash::Hash, msg, program_error::ProgramError, pubkey::Pubkey,
};
use std::{cell::Ref, cmp, mem};

pub mod add_to_whitelist;
pub mod start_presale;
pub mod stop_presale;
pub mod stop_whitelist;
pub mod set_authority;
pub mod buy;
pub mod distribute_token;
pub mod init_presale;

pub use add_to_whitelist::*;
pub use start_presale::*;
pub use stop_presale::*;
pub use stop_whitelist::*;
pub use set_authority::*;
pub use buy::*;
pub use distribute_token::*;
pub use init_presale::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    use crate::instruction::PresaleInstruction;
    match PresaleInstruction::try_from_slice(input)? {
        PresaleInstruction::AddToWhitelist => add_to_whitelist(program_id, accounts),
        PresaleInstruction::StartPresale => start_presale(program_id, accounts),
        PresaleInstruction::StopPresale => stop_presale(program_id, accounts),
        PresaleInstruction::StopWhiteList => stop_whitelist(program_id,accounts),
        PresaleInstruction::SetAuthority => set_authority(program_id,accounts),
        PresaleInstruction::Buy(args) => buy(program_id,accounts,args),
        PresaleInstruction::DistributeToken(args) => distribute_token(program_id,accounts,args),
        PresaleInstruction::InitPresale(args) => init_presale(program_id,accounts,args),
    }
}

///Structure with client data
pub const CLIENT_DATA_SIZE : usize = 32 + 32 + 8 + 1 + 1;
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct ClientData{
    pub owner : Pubkey,
    pub presale : Pubkey,
    pub amount : u64,
    pub is_whitelisted : bool,
    pub already_paid : bool,
}

impl ClientData{
    pub fn from_account_info(a : &AccountInfo) -> Result<ClientData,ProgramError>{
        if a.data_len() != CLIENT_DATA_SIZE {
            return Err(PresaleError::DataTypeMismatch.into());
        }
        let client : ClientData = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(client)
    }
}

///Structure for Presale Data
pub const PRESALE_DATA_SIZE : usize = 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1;
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct PresaleData{
    pub authority : Pubkey,
    pub token_for_sale : Pubkey,
    pub token_being_raised : Pubkey,
    pub min_allocation : u64,
    pub max_allocation : u64,
    pub hardcap : u64,
    pub token_per_usd : f64,
    pub total_raised : u64,
    pub total_percentage_distributed : u64,
    pub is_active : bool,
    pub is_whitelist : bool,
}

impl PresaleData{
    pub fn from_account_info(a : &AccountInfo) -> Result<PresaleData,ProgramError>{
        if a.data_len() != PRESALE_DATA_SIZE {
            return Err(PresaleError::DataTypeMismatch.into());
        }
        let presale : PresaleData = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(presale)
    }
}