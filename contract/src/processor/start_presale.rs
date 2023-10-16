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

pub fn start_presale(
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	)->ProgramResult{
	msg!("+ Processing StartPresale");
	let account_iter = &mut accounts.iter();
	let authority_account = next_account_info(account_iter)?;
	let presale_account = next_account_info(account_iter)?;

	assert_owned_by(presale_account, program_id)?;
	assert_signer(authority_account)?;

	let mut presale = PresaleData::from_account_info(presale_account)?;

	if presale.authority != *authority_account.key {
		return Err(PresaleError::InvalidAuthority.into());
	}

	if presale.is_active ==true {
		return Err(PresaleError::AlreadyStarted.into());
	}

	presale.is_active=true;
	presale.serialize(&mut *presale_account.data.borrow_mut())?;
	Ok(())
}