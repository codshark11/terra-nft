use {
  crate::{
    error::NFTInterfaceError,
  },
  arrayref::{
    array_ref,
    array_refs,
  },
  solana_program::{
    account_info::{
      AccountInfo,
    },
    program_option::COption,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke, invoke_signed},
    sysvar::{rent::Rent, Sysvar},
    program_pack::{ IsInitialized, Pack },
  },
  std::convert::TryInto,
};

pub fn get_mint_authority(account_info: &AccountInfo) -> Result<COption<Pubkey>, ProgramError> {
  // In token program, 36, 8, 1, 1 is the layout, where the first 36 is mint_authority
  // so we start at 0.
  let data = account_info.try_borrow_data().unwrap();
  let authority_bytes = array_ref![data, 0, 36];

  Ok(unpack_coption_key(&authority_bytes)?)
}

pub fn create_or_allocate_account_raw<'a>(
  program_id: Pubkey,
  new_account_info: &AccountInfo<'a>,
  rent_sysvar_info: &AccountInfo<'a>,
  system_program_info: &AccountInfo<'a>,
  payer_info: &AccountInfo<'a>,
  size: usize,
  signer_seeds: &[&[u8]],
) -> ProgramResult {
  let rent = &Rent::from_account_info(rent_sysvar_info)?;
  let required_lamports = rent
      .minimum_balance(size)
      .max(1)
      .saturating_sub(new_account_info.lamports());
  if required_lamports > 0 {
      msg!("Transfer {} lamports to the new account", required_lamports);
      invoke(
          &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
          &[
              payer_info.clone(),
              new_account_info.clone(),
              system_program_info.clone(),
          ],
      )?;
  }

  let accounts = &[new_account_info.clone(), system_program_info.clone()];

  msg!("Allocate space for the account");
  invoke_signed(
      &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
      accounts,
      &[&signer_seeds],
  )?;

  msg!("Assign the account to the owning program");
  invoke_signed(
      &system_instruction::assign(new_account_info.key, &program_id),
      accounts,
      &[&signer_seeds],
  )?;

  Ok(())
}


pub fn assert_mint_authority_matches_mint(
  mint_authority: &COption<Pubkey>,
  mint_authority_info: &AccountInfo,
) -> ProgramResult {
  match mint_authority {
      COption::None => {
          return Err(NFTInterfaceError::InvalidMintAuthority.into());
      }
      COption::Some(key) => {
          if mint_authority_info.key != key {
              return Err(NFTInterfaceError::InvalidMintAuthority.into());
          }
      }
  }

  if !mint_authority_info.is_signer {
      return Err(NFTInterfaceError::NotMintAuthority.into());
  }

  Ok(())
}

fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
  let (tag, body) = array_refs![src, 4, 32];
  match *tag {
      [0, 0, 0, 0] => Ok(COption::None),
      [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
      _ => Err(ProgramError::InvalidAccountData),
  }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
  if account.owner != owner {
      Err(NFTInterfaceError::IncorrectOwner.into())
  } else {
      Ok(())
  }
}

pub fn assert_token_program_matches_package(token_program_info: &AccountInfo) -> ProgramResult {
  if *token_program_info.key != spl_token::id() {
      return Err(NFTInterfaceError::InvalidTokenProgram.into());
  }
  Ok(())
}

pub fn assert_initialized<T: Pack + IsInitialized>(
  account_info: &AccountInfo,
) -> Result<T, ProgramError> {
  let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
  if !account.is_initialized() {
      Err(NFTInterfaceError::Uninitialized.into())
  } else {
      Ok(account)
  }
}
