use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::{try_from_slice_unchecked},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
};
use spl_token::{
    state::{Account,Mint},
    check_program_account,
    ID as SPL_TOKEN_PROGRAM_ID,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    pub is_initialized: bool,
    pub key_mint: Pubkey,
    pub accounts: Vec<Pubkey>,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);


pub fn lock(
    program_id: &Pubkey, bump: u8,
    accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Bundling...");
        let account_info_iter = &mut accounts.iter();

        // First account is the transaction signer.
        let owner_account = next_account_info(account_info_iter)?;
        if !owner_account.is_signer {
            msg!("Instruction: lock: wrong signer");
            return Err(ProgramError::MissingRequiredSignature)
        }

        // Second account is mint for KEY_NFT account and we need to ensure it is locked and there is only one token supply.
        // Why? Because tokens inside mint are not distinguishable, so the token owner may print more tokens
        //      and use them to unlock out vault.
        let mint_account = next_account_info(account_info_iter)?;
        if mint_account.owner != &SPL_TOKEN_PROGRAM_ID {
            msg!("Instruction: lock: bad owner of mint account");
            return Err(ProgramError::InvalidInstructionData)
        }
        msg!("Mint address: {}", mint_account.key);
        let mint = Mint::unpack(&mint_account.data.borrow())?;
        if mint.decimals != 0 || mint.supply != 1 || mint.mint_authority.is_some() {
            msg!("Instruction: lock: bad mint. Decimals={}, Supply={}, HasAuthority={}", mint.decimals, mint.supply, mint.mint_authority.is_some());
            return Err(ProgramError::InvalidInstructionData)
        }

        // Third account is an account for KEYHOLDER. We need to make sure it belongs to the signer. And it has this 1 NFT.
        let vault_key_account = next_account_info(account_info_iter)?;
        if vault_key_account.owner != &SPL_TOKEN_PROGRAM_ID {
            msg!("Instruction: lock: bad owner of supernft account");
            return Err(ProgramError::InvalidInstructionData)
        }
        let vault_key = Account::unpack(&vault_key_account.data.borrow())?;
        if vault_key.mint != *mint_account.key {
            msg!("Instruction: lock: key does not belong to mint");
            return Err(ProgramError::InvalidInstructionData)
        }
        if vault_key.owner != *owner_account.key {
            msg!("Instruction: lock: key does not belong to signer {}, it belongs to {}", owner_account.key, vault_key.owner);
            return Err(ProgramError::InvalidInstructionData)
        }
        if vault_key.amount != 1u64 {
            msg!("Instruction: lock: only 1 token of key should belong to signer, but {} found", vault_key.amount);
            return Err(ProgramError::InvalidInstructionData)
        }

        let vaults_pda = &Pubkey::create_program_address(
            &[&mint_account.key.to_bytes(), &[bump]],
            program_id
        )?;

        let token_program = next_account_info(account_info_iter)?;
        check_program_account(token_program.key)?;

        let rent = Rent::get()?;

        let vault_account = next_account_info(account_info_iter)?;
        if vault_account.owner != program_id {
            msg!("valut_account.owner ({}) != program_id ({})", vault_account.owner, program_id);
            return Err(ProgramError::InvalidInstructionData)
        }
        if !rent.is_exempt(vault_account.lamports(), vault_account.data_len()) {
            msg!("vault_account {} is not rent excempt", vault_account.key);
            return Err(ProgramError::InvalidInstructionData)
        }

        let mut vault: Vault = try_from_slice_unchecked(&vault_account.data.borrow())?;
        if vault.is_initialized {
            msg!("vault_account {} should be uninitialized", vault_account.key);
            return Err(ProgramError::InvalidInstructionData)
        }
        vault.is_initialized = true;
        vault.key_mint = vault_key.mint;

        // Then for this vault we are generating a PDA with seed: mint + bump.
        // All of the rest accounts are provided in N pairs. Where first item is the source NFT account, 
        // about which we don't care, and the second is a destination NFT account. We need to make sure
        // that the owner of the destination account is set to be a `vaults_pda`.
        // We need to check that the second account is rent-excempt.
        let mut accounts_counter = 0;
        while 1 == 1 {
            let maybe_next_source_account = next_account_info(account_info_iter);
            if maybe_next_source_account.is_err() {
                msg!("Placing {} NFTs into the zip", accounts_counter);
                break
            }
            let next_source_account = maybe_next_source_account.unwrap();
            let next_destination_account = next_account_info(account_info_iter)?;
            let next_destination = Account::unpack(&next_destination_account.data.borrow())?;
            if next_destination.owner != *vaults_pda {
                msg!("Account {} to transfer {} has wrong owner. Should be {}", next_destination_account.key, next_source_account.key, vaults_pda);
                return Err(ProgramError::InvalidInstructionData)
            }
            if !rent.is_exempt(next_destination_account.lamports(), next_destination_account.data_len()) {
                msg!("Account {} is not rent excempt", next_destination_account.key);
                return Err(ProgramError::InvalidInstructionData)
            }

            msg!("Transferring (maybe)");
            let ix = spl_token::instruction::transfer(
                &SPL_TOKEN_PROGRAM_ID,
                next_source_account.key,
                next_destination_account.key,
                owner_account.key,
                &[&owner_account.key],
                1,
            )?;
            invoke(&ix, &[
                next_source_account.clone(),
                next_destination_account.clone(),
                owner_account.clone(),
                token_program.clone(),
            ])?;
            vault.accounts.push(*next_destination_account.key);

            accounts_counter += 1;
        }

        if accounts_counter > (vault_account.data_len() - 32 - 4) / 32 {
            msg!("Account {} is too small to remember all sub-accounts {}", vault_account.key, accounts_counter);
            return Err(ProgramError::InvalidInstructionData)
        }

        vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;
        Ok(())
    }

pub fn unlock(
    program_id: &Pubkey, bump: u8,
    accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Unbundling...");
        let account_info_iter = &mut accounts.iter();

        // First account is the transaction signer.
        let owner_account = next_account_info(account_info_iter)?;
        if !owner_account.is_signer {
            msg!("Instruction: unlock: wrong signer");
            return Err(ProgramError::MissingRequiredSignature)
        }

        // Second account is a VAULTINFO account.
        let vault_account = next_account_info(account_info_iter)?;
        if vault_account.owner != program_id {
            msg!("Instruction: unlock: expected valut_account owner {}", vault_account.owner);
            return Err(ProgramError::InvalidInstructionData)
        }
        let vault: Vault = try_from_slice_unchecked(&vault_account.data.borrow())?;
        if !vault.is_initialized {
            msg!("vault_account {} should be initialized", vault_account.key);
            return Err(ProgramError::InvalidInstructionData)
        }

        // Third account is an KEYHOLDER account. It should belong to the signer and have one NFT,
        // and mint should be as specified in the VAULTINFO
        let vault_key_account = next_account_info(account_info_iter)?;
        if vault_key_account.owner != &SPL_TOKEN_PROGRAM_ID {
            msg!("Instruction: unlock: bad owner of mint account");
            return Err(ProgramError::InvalidInstructionData)
        }
        let vault_key = Account::unpack(&vault_key_account.data.borrow())?;
        if vault_key.mint != vault.key_mint {
            msg!("Instruction: unlock: key does not belong to mint");
            return Err(ProgramError::InvalidInstructionData)
        }
        if vault_key.owner != *owner_account.key {
            msg!("Instruction: unlock: key does not belong to signer {}, it belongs to {}", owner_account.key, vault_key.owner);
            return Err(ProgramError::InvalidInstructionData)
        }
        if vault_key.amount != 1u64 {
            msg!("Instruction: unlock: only 1 token of key should belong to signer, but {} found", vault_key.amount);
            return Err(ProgramError::InvalidInstructionData)
        }

        let rent = Rent::get()?;
        let token_program = next_account_info(account_info_iter)?;
        check_program_account(token_program.key)?;
        let vaults_pda = &Pubkey::create_program_address(
            &[&vault.key_mint.to_bytes(), &[bump]],
            program_id
        )?;
        let vaults_pda_account = next_account_info(account_info_iter)?;

        for account in &vault.accounts {
            let next_source_account = next_account_info(account_info_iter)?;
            if next_source_account.key != account {
                msg!("Instruction: unlock: expected to have {} on some position but got {}", account, next_source_account.key);
                return Err(ProgramError::InvalidInstructionData)
            }
            let next_destination_account = next_account_info(account_info_iter)?;
            let next_destination = Account::unpack(&next_destination_account.data.borrow())?;
            if next_destination.owner != *owner_account.key {
                msg!("Instruction: unlock: expected destination account {} to belong to signer {}, but got {}", next_destination_account.key, owner_account.key, next_destination.owner);
                return Err(ProgramError::InvalidInstructionData)
            }
            if !rent.is_exempt(next_destination_account.lamports(), next_destination_account.data_len()) {
                msg!("Instruction: unlock: expected destination account {} to be rent excempt", next_destination_account.key);
                return Err(ProgramError::InvalidInstructionData)
            }

            msg!("Transferring (maybe)");
            let ix = spl_token::instruction::transfer(
                &SPL_TOKEN_PROGRAM_ID,
                next_source_account.key,
                next_destination_account.key,
                vaults_pda,
                &[&vaults_pda],
                1,
            )?;
            invoke_signed(&ix, &[
                vaults_pda_account.clone(),
                next_source_account.clone(),
                next_destination_account.clone(),
                owner_account.clone(),
                token_program.clone(),
            ], &[&[&vault.key_mint.to_bytes(), &[bump]]])?;
        }

        // No reason to destroy anything correctly, because all of the accounts are already empty.

        Ok(())
    }


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    use ProgramError::InvalidInstructionData;
    
    msg!("NFT Vault entrypoint");
    let (&tag, rest) = _instruction_data.split_first().ok_or(InvalidInstructionData)?;
    let (&bump, _) = rest.split_first().ok_or(InvalidInstructionData)?;

    match tag {
        0u8 => lock(program_id, bump, accounts)?,
        1u8 => unlock(program_id, bump, accounts)?,
        _ => unreachable!(),
    }

    Ok(())
}
