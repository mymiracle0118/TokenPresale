use crate::{
	errors::PresaleError,
	processor::{PresaleData,ClientData},
	utils::{
		assert_owned_by,assert_signer,assert_derivation,
		spl_token_transfer,TokenTransferParams,
		spl_token_transfer_without_seed,TokenTransferParamsWithoutSeed,
	},
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
	spl_token::state::Account,
	std::mem,
};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DistributeTokenArgs {
	pub percentageOfAmountOwed : u64,
}

pub fn distribute_token(
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	args : DistributeTokenArgs,
	)->ProgramResult{
	msg!("+ Processing Distribute Token");
	let account_iter = &mut accounts.iter();
	let authority_account = next_account_info(account_iter)?;
	let authority_token_account = next_account_info(account_iter)?;
	let bidder_account = next_account_info(account_iter)?;
	let bidder_token_account = next_account_info(account_iter)?;
	let presale_account = next_account_info(account_iter)?;
	let transfer_authority = next_account_info(account_iter)?;
	let client_account = next_account_info(account_iter)?;
	let mint_account = next_account_info(account_iter)?;
	let token_program = next_account_info(account_iter)?;

	assert_owned_by(authority_token_account,&spl_token::id())?;
	assert_owned_by(bidder_token_account,&spl_token::id())?;
	assert_owned_by(mint_account,&spl_token::id())?;
	assert_owned_by(presale_account,program_id)?;
	assert_owned_by(client_account,program_id)?;
	assert_signer(authority_account)?;
	assert_signer(transfer_authority)?;

	assert_derivation(
		program_id,
		client_account,
		&[
			PRESALE.as_bytes(),
			program_id.as_ref(),
			(*presale_account.key).as_ref(),
			(*bidder_account.key).as_ref(),
		],
	)?;

	if *token_program.key != spl_token::id() {
		return Err(PresaleError::InvalidTokenProgram.into());
	}

	let mut presale=PresaleData::from_account_info(presale_account)?;
	let mut client=ClientData::from_account_info(client_account)?;	

	if client.owner != *bidder_account.key {
		return Err(PresaleError::InvalidPresaleAccount.into());
	}

	if client.presale != *presale_account.key {
		return Err(PresaleError::NotMatchPresale.into());
	}

	if *mint_account.key != presale.token_being_raised {
		return Err(PresaleError::NotMatchTokenAddress.into());
	}

	if presale.authority != *authority_account.key {
		return Err(PresaleError::InvalidAuthority.into());
	}

//////////////////////////////////////////////////////////
	if (presale.total_percentage_distributed + args.percentageOfAmountOwed) >= 100 {
		return Err(PresaleError::AlreadyDistributedOverflow.into());
	}

	let real_amount = ((client.amount as f64) * presale.token_per_usd / (100.0 as f64) * (args.percentageOfAmountOwed as f64)) as u64 ;

	spl_token_transfer_without_seed(TokenTransferParamsWithoutSeed{
		source : authority_token_account.clone(),
		destination : bidder_token_account.clone(),
		authority : transfer_authority.clone(),
		token_program : token_program.clone(),
		amount : real_amount,
	})?;

	client.already_paid=true;
	client.serialize(&mut *client_account.data.borrow_mut())?;

	Ok(())
}