use {
  borsh::{
    BorshSerialize,
    BorshDeserialize,
  },
  solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
  },
};

pub const NFTINTERFACEPREFIX: &str = "nftinterface";
pub const NFTACCOUNT_LENGTH: usize = 8 + 2 + 2 + 32 + 32 + 1;

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub struct NFTInterface {
  pub token_price_per_nft: u64,
  pub max_supply: u16,
  pub total_supply: u16,
  pub update_authority_key: Pubkey,
  pub fee_receiver_key: Pubkey,
  pub is_sealed: u8,
}

impl NFTInterface {
  pub fn from_account_info(a: &AccountInfo) -> Result<NFTInterface, ProgramError> {
    let ni: NFTInterface = try_from_slice_unchecked(&a.data.borrow_mut())?;
    msg!("Read NFTInterface Account data: {}", true);
    Ok(ni)
  }
}

pub const WHITELISTPREFIX: &str = "whitelist";
pub const WHITELISTACCOUNT_LENGTH: usize = 1;

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub struct Whitelist {
  pub is_sealed: u8,
}

impl Whitelist {
  pub fn from_account_info(a: &AccountInfo) -> Result<Whitelist, ProgramError> {
    let ni: Whitelist = try_from_slice_unchecked(&a.data.borrow_mut())?;
    msg!("Read Whitelist Account data: {}", true);
    Ok(ni)
  }
}
