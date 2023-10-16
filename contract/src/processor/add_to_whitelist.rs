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

pub fn add_to_whitelist(
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	)->ProgramResult{
	msg!("Processing AddToWhitelist");
	let account_iter = &mut accounts.iter();
	let authority_account = next_account_info(account_iter)?;
	let member_account = next_account_info(account_iter)?;
	let presale_account = next_account_info(account_iter)?;
	let client_account = next_account_info(account_iter)?;

	assert_owned_by(presale_account,program_id)?;
	assert_owned_by(client_account,program_id)?;
	assert_signer(authority_account)?;

	assert_derivation(
		program_id,
		client_account,
		&[
			PRESALE.as_bytes(),
			program_id.as_ref(),
			(*presale_account.key).as_ref(),
			(*member_account.key).as_ref(),
		],
	)?;

	let mut presale=PresaleData::from_account_info(presale_account)?;
	let mut client=ClientData::from_account_info(client_account)?;

	if presale.authority != *authority_account.key {
		return Err(PresaleError::InvalidAuthority.into());
	}

	if client.owner != *member_account.key {
		return Err(PresaleError::InvalidClientOwner.into());
	}

	if client.presale != *presale_account.key{
		return Err(PresaleError::InvalidPresaleAccount.into())
	}

	if presale.is_active == true {
		return Err(PresaleError::AlreadyStarted.into());
	}

	client.is_whitelisted = true;
	client.serialize(&mut *client_account.data.borrow_mut())?;

	Ok(())
}