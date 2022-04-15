use {
  crate::{
    error::NFTInterfaceError,
    instruction::{
      NFTInterfaceInstruction
    },
    state::{
      NFTInterface,
      NFTINTERFACEPREFIX,
      NFTACCOUNT_LENGTH,
      WHITELISTACCOUNT_LENGTH,
      WHITELISTPREFIX,
      Whitelist,
    },
    utils::{
      create_or_allocate_account_raw,
    },
  },
  borsh::{
    BorshDeserialize,
    BorshSerialize,
  },
  solana_program::{
    account_info::{
      next_account_info,
      AccountInfo,
    },
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke,},
    sysvar:: {
      rent::Rent,
      Sysvar
    }
  },
};

pub fn process_instruction<'a>(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  input: &[u8],
) -> ProgramResult {
  let instruction = NFTInterfaceInstruction::try_from_slice(input)?;
  match instruction {

    NFTInterfaceInstruction::CreateNFTInterfaceAccount(args) => {
      msg!("Create Nft Interface Account");
      process_create_nftinterface_account(
        program_id,
        accounts,
        args.token_price_per_nft,
        args.max_supply,
        args.is_sealed,
      )
    },
    NFTInterfaceInstruction::ModifyNFTInterfaceAccount(args) => {
      msg!("modify Nft Interface Account");
      process_modify_nftinterface_accounts(
        program_id,
        accounts,
        args.token_price_per_nft,
        args.max_supply,
        args.total_supply,
        args.is_sealed,
      )
    },
    NFTInterfaceInstruction::MintNFTInterfaceAccount(args) => {
      msg!("Mint Nft Interface Account");
      process_mint_nftinterface_accounts(
        program_id,
        accounts,
      )
    },
    NFTInterfaceInstruction::GetFeeNftInterfaceAccount(args) => {
      msg!("Get Fee Nft Interface Account");
      process_get_fee_nftinterface_accounts(
        program_id,
        accounts,
        args.wanted_supply,
      )
    },
    NFTInterfaceInstruction::CreateWhitelistAccount(args) => {
      msg!("Create Whitelist Account");
      process_create_whitelist_accounts(
        program_id,
        accounts,
        args.is_sealed,
      )
    },
    NFTInterfaceInstruction::ModifyWhitelistAccount(args) => {
      msg!("Modify Whitelist Account");
      process_modify_whitelist_accounts(
        program_id,
        accounts,
        args.is_sealed,
      )
    },
  }
}

pub fn process_create_nftinterface_account<'a> (
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  token_price_per_nft: u64,
  max_supply: u16,
  is_sealed: u8,
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let new_account_info = next_account_info(account_info_iter)?;
  let fee_receiver_account_info = next_account_info(account_info_iter)?;
  let payer_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  let system_program_info = next_account_info(account_info_iter)?;
  let rent_info = next_account_info(account_info_iter)?;

  let nft_interface_seed = &[
    NFTINTERFACEPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
  ];
  let (nft_interface_key, nft_interface_bump_seed) = Pubkey::find_program_address(nft_interface_seed, program_id);
  
  if new_account_info.key != &nft_interface_key {
    return Err(NFTInterfaceError::InvalidNFTAccountKey.into());
  }

  let nft_interface_authority_signer_seeds = &[
    NFTINTERFACEPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
    &[nft_interface_bump_seed],
  ];

  create_or_allocate_account_raw(
    *program_id,
    new_account_info,
    rent_info,
    system_program_info,
    payer_info,
    NFTACCOUNT_LENGTH,
    nft_interface_authority_signer_seeds,
  )?;

  let mut nft_interface_data = NFTInterface::from_account_info(new_account_info)?;

  let size = 0;
  let rent = &Rent::from_account_info(&rent_info)?;
  let required_lamports = rent
      .minimum_balance(size)
      .max(1)
      .saturating_sub(payer_info.lamports());

  invoke(
    &system_instruction::create_account(
        payer_info.key,
        fee_receiver_account_info.key,
        required_lamports,
        size as u64,
        &system_program_info.key
    ),
    &[payer_info.clone(), fee_receiver_account_info.clone(), system_program_info.clone()]
  )?;

  nft_interface_data.token_price_per_nft = token_price_per_nft;
  nft_interface_data.max_supply = max_supply;
  nft_interface_data.total_supply = 0;
  nft_interface_data.update_authority_key = *update_authority_info.key;
  nft_interface_data.fee_receiver_key = *fee_receiver_account_info.key;
  nft_interface_data.is_sealed = is_sealed;

  nft_interface_data.serialize(&mut *new_account_info.data.borrow_mut())?;
  msg!("Create account success.");
  Ok(())
}

pub fn process_modify_nftinterface_accounts<'a>(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  token_price_per_nft: Option<u64>,
  max_supply: Option<u16>,
  total_supply: Option<u16>,
  is_sealed: Option<u8>,
) -> ProgramResult {

  let account_info_iter = &mut accounts.iter();
  let nft_interface_account_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  
  let nft_interface_seed = &[
    NFTINTERFACEPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
  ];
 
  let (nft_interface_key, _) = Pubkey::find_program_address(nft_interface_seed, program_id);

  if nft_interface_account_info.key != &nft_interface_key {
    return Err(NFTInterfaceError::InvalidNFTAccountKey.into());
  }

  let mut nft_interface_data = NFTInterface::from_account_info(nft_interface_account_info)?;

  if let Some(val) = token_price_per_nft {
    nft_interface_data.token_price_per_nft = val;
  }

  if let Some(val) = max_supply {
    nft_interface_data.max_supply = val;
  }
  if let Some(val) = total_supply {
    nft_interface_data.total_supply = val;
  }
  if let Some(val) = is_sealed {
    nft_interface_data.is_sealed = val;
  }

  nft_interface_data.serialize(&mut *nft_interface_account_info.data.borrow_mut())?;

  msg!("Finished modify.");
  Ok(())
}

pub fn process_mint_nftinterface_accounts<'a>(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let nft_interface_account_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  let fee_receiver_account_info = next_account_info(account_info_iter)?;
  let payer_info = next_account_info(account_info_iter)?;
  let system_program_info = next_account_info(account_info_iter)?;
  
  let nft_interface_seed = &[
    NFTINTERFACEPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
  ];
  let (nft_interface_key, _) = Pubkey::find_program_address(nft_interface_seed, program_id);
  
  if nft_interface_account_info.key != &nft_interface_key {
    return Err(NFTInterfaceError::InvalidNFTAccountKey.into());
  }

  msg!("Sending Sol to fee receiver.");
  let mut nft_interface_data = NFTInterface::from_account_info(nft_interface_account_info)?;
  let mut fee_amount = nft_interface_data.token_price_per_nft as u64;
  if payer_info.key == update_authority_info.key {
    fee_amount = 0;
  }
  if payer_info.lamports() < fee_amount {
    return Err(NFTInterfaceError::NotEnoughSOL.into());
  }
  if nft_interface_data.total_supply + 1 > nft_interface_data.max_supply {
    return Err(NFTInterfaceError::ExceedMaxSupply.into())
  }
  invoke(
    &system_instruction::transfer(
        &payer_info.key,
        &fee_receiver_account_info.key,
        fee_amount,
    ),
    &[
        payer_info.clone(),
        fee_receiver_account_info.clone(),
        system_program_info.clone(),
    ],
  )?; 
  nft_interface_data.total_supply += 1;
  nft_interface_data.serialize(&mut *nft_interface_account_info.data.borrow_mut())?;
  Ok(())
}

pub fn process_get_fee_nftinterface_accounts<'a> 
(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  wanted_supply: Option<u64>,
) -> ProgramResult {

  let account_info_iter = &mut accounts.iter();
  let nft_interface_account_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  let fee_receiver_account_info = next_account_info(account_info_iter)?;
  let receiver_account_info = next_account_info(account_info_iter)?;
  let system_program_info = next_account_info(account_info_iter)?;
  
  let nft_interface_seed = &[
    NFTINTERFACEPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
  ];
  let (nft_interface_key, _) = Pubkey::find_program_address(nft_interface_seed, program_id);
  
  if nft_interface_account_info.key != &nft_interface_key {
    return Err(NFTInterfaceError::InvalidNFTAccountKey.into());
  }

  msg!("Sending Sol to fee receiver.");

  let mut fee_amount;
  if let Some(val) = wanted_supply {
    fee_amount = val;
    if fee_receiver_account_info.lamports() < fee_amount {
      fee_amount = fee_receiver_account_info.lamports();
    }
  } else {
    fee_amount = fee_receiver_account_info.lamports();
  }

  invoke(
    &system_instruction::transfer(
        &fee_receiver_account_info.key,
        &receiver_account_info.key,
        fee_amount,
    ),
    &[
        fee_receiver_account_info.clone(),
        receiver_account_info.clone(),
        system_program_info.clone(),
    ]
  )?; 
  Ok(())
}


pub fn process_create_whitelist_accounts<'a> 
(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  is_sealed: u8,
) -> ProgramResult {

  let account_info_iter = &mut accounts.iter();
  let new_account_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  let payer_info = next_account_info(account_info_iter)?;
  let whitelist_account_info = next_account_info(account_info_iter)?;
  let system_program_info = next_account_info(account_info_iter)?;
  let rent_info = next_account_info(account_info_iter)?;
  
  let whitelist_seed = &[
    WHITELISTPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
    whitelist_account_info.key.as_ref(),
  ];
  let (whitelist_key, whitelist_bump_seed) = Pubkey::find_program_address(whitelist_seed, program_id);
  
  if new_account_info.key != &whitelist_key {
    return Err(NFTInterfaceError::InvalidWhitelistAccountKey.into());
  }

  let whitelist_authority_signer_seeds = &[
    WHITELISTPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
    whitelist_account_info.key.as_ref(),
    &[whitelist_bump_seed],
  ];

  create_or_allocate_account_raw(
    *program_id,
    new_account_info,
    rent_info,
    system_program_info,
    payer_info,
    WHITELISTACCOUNT_LENGTH,
    whitelist_authority_signer_seeds,
  )?;

  let mut whitelist_data = Whitelist::from_account_info(new_account_info)?;

  whitelist_data.is_sealed = is_sealed;

  whitelist_data.serialize(&mut *new_account_info.data.borrow_mut())?;
  msg!("Create account success.");
  Ok(())
}


pub fn process_modify_whitelist_accounts<'a> 
(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  is_sealed: u8,
) -> ProgramResult {

  let account_info_iter = &mut accounts.iter();
  let whitelist_data_account_info = next_account_info(account_info_iter)?;
  let update_authority_info = next_account_info(account_info_iter)?;
  let whitelist_account_info = next_account_info(account_info_iter)?;
  
  let whitelist_seed = &[
    WHITELISTPREFIX.as_bytes(),
    program_id.as_ref(),
    update_authority_info.key.as_ref(),
    whitelist_account_info.key.as_ref(),
  ];
  let (whitelist_key, _) = Pubkey::find_program_address(whitelist_seed, program_id);
  
  if whitelist_data_account_info.key != &whitelist_key {
    return Err(NFTInterfaceError::InvalidWhitelistAccountKey.into());
  }

  let mut whitelist_data = Whitelist::from_account_info(whitelist_data_account_info)?;
  whitelist_data.is_sealed = is_sealed;

  whitelist_data.serialize(&mut *whitelist_data_account_info.data.borrow_mut())?;
  msg!("Modify account success.");
  Ok(())
}
