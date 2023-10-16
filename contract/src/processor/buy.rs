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
pub struct BuyArgs {
	pub amount : u64,
}

pub fn buy(
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	args : BuyArgs,
	)->ProgramResult{
	msg!("+ Processing Buy");
	let account_iter = &mut accounts.iter();
	let bidder_account = next_account_info(account_iter)?;
	let bidder_token_account = next_account_info(account_iter)?;
	let presale_pot_account = next_account_info(account_iter)?;
	let transfer_authority = next_account_info(account_iter)?;
	let presale_account = next_account_info(account_iter)?;
	let client_account = next_account_info(account_iter)?;
	let mint_account = next_account_info(account_iter)?;
	let token_program = next_account_info(account_iter)?;

	assert_owned_by(bidder_token_account,&spl_token::id())?;
	assert_owned_by(presale_pot_account,&spl_token::id())?;
	assert_owned_by(mint_account,&spl_token::id())?;
	assert_owned_by(presale_account,program_id)?;
	assert_owned_by(client_account,program_id)?;
	assert_signer(bidder_account)?;
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

//////////////////////////////////////////////////////////////////////////
	if presale.is_active == false {
		return Err(PresaleError::NotActiveYet.into());
	}

	if args.amount < presale.min_allocation || args.amount > presale.max_allocation {
		return Err(PresaleError::InvalidAmount.into());
	}

	let bidder_token : Account = Account::unpack_from_slice(&bidder_token_account.data.borrow())?;
	if bidder_token.amount.saturating_sub(args.amount) < 0 {
		return Err(PresaleError::BalanceTooLow.into());
	}

	if presale.total_raised > presale.hardcap {
		return Err(PresaleError::HardcapReached.into());
	}

	if (presale.total_raised + args.amount) > presale.hardcap {
		return Err(PresaleError::WillOverHardcap.into());
	}

	if (client.amount + args.amount) > presale.max_allocation {
		return Err(PresaleError::MoreThanMaxAllocation.into());
	}

	if presale.is_whitelist == true && client.is_whitelisted==false {
		return Err(PresaleError::NotWhitelisted.into());
	}

	//token_transfer
	spl_token_transfer_without_seed(TokenTransferParamsWithoutSeed{
 		source : bidder_token_account.clone(),
 		destination : presale_pot_account.clone(),
 		authority : transfer_authority.clone(),
 		token_program : token_program.clone(),
 		amount : args.amount,
	})?;

	presale.total_raised = presale.total_raised + args.amount;

	client.amount = client.amount + args.amount;

	client.serialize(&mut *client_account.data.borrow_mut())?;
	presale.serialize(&mut *presale_account.data.borrow_mut())?;

	Ok(())
}