use crate::{
	errors::PresaleError,
	processor::{PresaleData,ClientData},
	utils::{assert_owned_by,assert_signer,assert_derivation},
	PRESALE,
};

use {
	borsh::{BorshDeserialize,BorshSerialize},
	solana_program::{
		account_info::{next_account_info,AccountInfo},
		entrypoint::ProgramResult,
		msg,
		program::invoke_signed,
		program_error::ProgramError,
		program_pack::Pack,
		pubkey::Pubkey,
		system_instruction,
		sysvar::{clock::Clock,Sysvar},
	},
};

pub fn set_authority(
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	)->ProgramResult{
	msg!("+ Processing SetAuthority");
	let account_iter = &mut accounts.iter();
	let old_authority_account = next_account_info(account_iter)?;
	let new_authority_account = next_account_info(account_iter)?;
	let presale_account = next_account_info(account_iter)?;

	assert_owned_by(presale_account, program_id)?;
	assert_signer(old_authority_account)?;

	let mut presale = PresaleData::from_account_info(presale_account)?;

	if presale.authority != *old_authority_account.key {
		return Err(PresaleError::InvalidAuthority.into());
	}

	if new_authority_account.data_is_empty() || new_authority_account.lamports() == 0 {
		return Err(PresaleError::InvalidAuthority.into());
	}

	presale.authority = *new_authority_account.key;
	presale.serialize(&mut *presale_account.data.borrow_mut())?;
	Ok(())
}