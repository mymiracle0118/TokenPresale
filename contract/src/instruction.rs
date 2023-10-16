use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    sysvar,
};

pub use crate::processor::{
    buy::BuyArgs,
    distribute_token::DistributeTokenArgs,
};

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PresaleInstruction {
    InitPresale(InitPresaleArgs),
    StartPresale,
    StopPresale,
    StopWhiteList,
    SetAuthority,
    // WithdrawFunds(WithdrawFundsArgs),
    // WithdrawUnsoldTokens(WithdrawUnsoldTokens),
    Buy(BuyArgs),
    DistributeToken(DistributeTokenArgs),
    AddToWhitelist,
}
