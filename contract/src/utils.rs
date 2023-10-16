use solana_program::program_pack::IsInitialized;

use {
    crate::errors::PresaleError,
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
        sysvar::{rent::Rent, Sysvar},
    },
    std::convert::TryInto,
};

pub fn assert_owned_by(account : &AccountInfo, owner : &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(PresaleError::IncorrectOwner.into())
    }else{
        Ok(())
    }
}

pub fn assert_signer(account : &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_derivation(
    program_id : &Pubkey,
    account : &AccountInfo,
    path : &[&[u8]],
    ) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(&path,program_id);
    if key != *account.key {
        return Err(PresaleError::DerivedKeyInvalid.into());
    }
    Ok(bump)
}

///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[authority_signer_seeds],
    );

    result.map_err(|_| PresaleError::TokenTransferFailed.into())
}

///TokenTransferParams
pub struct TokenTransferParamsWithoutSeed<'a> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer_without_seed(params: TokenTransferParamsWithoutSeed<'_>) -> ProgramResult {
    let TokenTransferParamsWithoutSeed {
        source,
        destination,
        authority,
        token_program,
        amount,
    } = params;

    let result = invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination ,authority, token_program],
    );

    result.map_err(|_| PresaleError::TokenTransferFailed.into())
}