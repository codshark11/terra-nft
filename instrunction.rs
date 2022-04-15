use {
  borsh::{
    BorshDeserialize, BorshSerialize,
  },
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateNFTInterfaceAccountArgs {
  pub token_price_per_nft: u64,
  pub max_supply: u16,
  pub is_sealed: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ModifyNFTInterfaceAccountArgs {
  pub token_price_per_nft: Option<u64>,
  pub max_supply: Option<u16>,
  pub total_supply: Option<u16>,
  pub is_sealed: Option<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MintNFTInterfaceAccountArgs {
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GetFeeNftInterfaceAccountArgs {
 pub wanted_supply: Option<u64>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateWhitelistAccountArgs {
  pub is_sealed: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ModifyWhitelistAccountArgs {
  pub is_sealed: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum NFTInterfaceInstruction {
  CreateNFTInterfaceAccount(CreateNFTInterfaceAccountArgs),
  ModifyNFTInterfaceAccount(ModifyNFTInterfaceAccountArgs),
  MintNFTInterfaceAccount(MintNFTInterfaceAccountArgs),
  GetFeeNftInterfaceAccount(GetFeeNftInterfaceAccountArgs),
  CreateWhitelistAccount(CreateWhitelistAccountArgs),
  ModifyWhitelistAccount(ModifyWhitelistAccountArgs),
}
