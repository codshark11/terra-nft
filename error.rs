use {
  num_derive::FromPrimitive,
  solana_program:: {
    decode_error::DecodeError,
    msg,
    program_error::{
      PrintProgramError,
      ProgramError,
    },
  },
  thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NFTInterfaceError {
  #[error("Invalid new account key.")]
  InvalidNFTAccountKey,

  #[error("Invalid fee receiver account key.")]
  InvalidFeeReceiverAccountKey,
  
  #[error("Invalid Mint Authority.")]
  InvalidMintAuthority,

  #[error("Invalid not mint authority.")]
  NotMintAuthority,

  #[error("Incorrect owner.")]
  IncorrectOwner,

  #[error("Incorrect Token program Id.")]
  InvalidTokenProgram,

  #[error("Uninitialized account.")]
  Uninitialized,

  #[error("Not Enough Sol.")]
  NotEnoughSOL,

  #[error("Not Allow to mint.")]
  NotSealed,

  #[error("Exceed Max supply.")]
  ExceedMaxSupply,

  #[error("Invalid Whitelist account key.")]
  InvalidWhitelistAccountKey,
}

impl PrintProgramError for NFTInterfaceError {
  fn print<E>(&self) {
    msg!(&self.to_string());
  }
}

impl From<NFTInterfaceError> for ProgramError {
  fn from(e: NFTInterfaceError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

impl<T> DecodeError<T> for NFTInterfaceError {
  fn type_of() -> &'static str {
    "NFT interface Error"
  }
}
