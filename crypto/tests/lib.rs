use donft::{process_instruction, Vault};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    borsh::{try_from_slice_unchecked},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    program_pack::{Pack},
    signer::keypair::Keypair,
    signature::Signer,
    transaction::Transaction,
};
use spl_token::{
    state::{Account as TokenAccount},
    ID as SPL_TOKEN_PROGRAM_ID,
};

#[tokio::test]
async fn test_locking() {
    let program_id = Pubkey::new_unique();
    let owner = Keypair::new();
    let owner2 = Keypair::new();

    // Mint account for token which will be acting as a key
    let key_mint_pubkey = Pubkey::new_unique();
    let owner_key_pubkey = Pubkey::new_unique();
    let owner2_key_pubkey = Pubkey::new_unique();

    // Mint accounts for tokens which will be stored
    let mint1_pubkey = Pubkey::new_unique();
    let mint2_pubkey = Pubkey::new_unique();
    let owners_account_for_mint1_pubkey = Pubkey::new_unique();
    let owners2_account_for_mint1_pubkey = Pubkey::new_unique();
    let pdas_account_for_mint1_pubkey = Pubkey::new_unique();
    let owners_account_for_mint2_pubkey = Pubkey::new_unique();
    let owners2_account_for_mint2_pubkey = Pubkey::new_unique();
    let pdas_account_for_mint2_pubkey = Pubkey::new_unique();

    let vault_pubkey = Pubkey::new_unique();
    let (vaults_pda, bump_seed) = Pubkey::find_program_address(&[&key_mint_pubkey.to_bytes()], &program_id);

    // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    let token_program = Pubkey::new(&[
        0x06, 0xdd, 0xf6, 0xe1, 0xd7, 0x65, 0xa1, 0x93, 0xd9, 0xcb, 0xe1, 0x46, 0xce, 0xeb, 0x79, 0xac,
        0x1c, 0xb4, 0x85, 0xed, 0x5f, 0x5b, 0x37, 0x91, 0x3a, 0x8c, 0xf5, 0x85, 0x7e, 0xff, 0x00, 0xa9]);

    let mut program_test = ProgramTest::new(
        "donft", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        key_mint_pubkey,
        Account {
            lamports: 1,
            data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ].to_vec(),
            owner: token_program,
            ..Account::default()
        }
    );

    let mut key_account_data = vec![];
    key_account_data.extend_from_slice(&key_mint_pubkey.to_bytes());
    key_account_data.extend_from_slice(&owner.pubkey().to_bytes());
    key_account_data.extend_from_slice(&[
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owner_key_pubkey,
        Account {
            lamports: 1,
            data: key_account_data,
            owner: token_program,
            ..Account::default()
        }
    );

    let mut owner2_key_account_data = vec![];
    owner2_key_account_data.extend_from_slice(&key_mint_pubkey.to_bytes());
    owner2_key_account_data.extend_from_slice(&owner2.pubkey().to_bytes());
    owner2_key_account_data.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owner2_key_pubkey,
        Account {
            lamports: 1,
            data: owner2_key_account_data,
            owner: token_program,
            ..Account::default()
        }
    );

    program_test.add_account(
        vault_pubkey,
        Account {
            lamports: 1000000000,
            data: [0u8;32+8+2*32].to_vec(),
            owner: program_id,
            ..Account::default()
        }
    );
    
    // The next 4 blocks may seem a bit nonsence but they are just showing
    // 4 accounts, 2 owned by owner, 2 owned by vault_pda, first 2 has some
    // tokens on it, second 2 don't
    
    let mut owners_account_for_mint1_data = vec![];
    owners_account_for_mint1_data.extend_from_slice(&mint1_pubkey.to_bytes());
    owners_account_for_mint1_data.extend_from_slice(&owner.pubkey().to_bytes());
    owners_account_for_mint1_data.extend_from_slice(&[
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owners_account_for_mint1_pubkey,
        Account {
            lamports: 1,
            data: owners_account_for_mint1_data,
            owner: token_program,
            ..Account::default()
        }
    );

    let mut owners2_account_for_mint1_data = vec![];
    owners2_account_for_mint1_data.extend_from_slice(&mint1_pubkey.to_bytes());
    owners2_account_for_mint1_data.extend_from_slice(&owner2.pubkey().to_bytes());
    owners2_account_for_mint1_data.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owners2_account_for_mint1_pubkey,
        Account {
            lamports: 1000000000,
            data: owners2_account_for_mint1_data,
            owner: token_program,
            ..Account::default()
        }
    );
    
    let mut pdas_account_for_mint1_data = vec![];
    pdas_account_for_mint1_data.extend_from_slice(&mint1_pubkey.to_bytes());
    pdas_account_for_mint1_data.extend_from_slice(&vaults_pda.to_bytes());
    pdas_account_for_mint1_data.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        pdas_account_for_mint1_pubkey,
        Account {
            lamports: 1000000000,
            data: pdas_account_for_mint1_data,
            owner: token_program,
            ..Account::default()
        }
    );
    
    let mut owners_account_for_mint2_data = vec![];
    owners_account_for_mint2_data.extend_from_slice(&mint2_pubkey.to_bytes());
    owners_account_for_mint2_data.extend_from_slice(&owner.pubkey().to_bytes());
    owners_account_for_mint2_data.extend_from_slice(&[
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owners_account_for_mint2_pubkey,
        Account {
            lamports: 1,
            data: owners_account_for_mint2_data,
            owner: token_program,
            ..Account::default()
        }
    );

    let mut owners2_account_for_mint2_data = vec![];
    owners2_account_for_mint2_data.extend_from_slice(&mint2_pubkey.to_bytes());
    owners2_account_for_mint2_data.extend_from_slice(&owner2.pubkey().to_bytes());
    owners2_account_for_mint2_data.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        owners2_account_for_mint2_pubkey,
        Account {
            lamports: 1000000000,
            data: owners2_account_for_mint2_data,
            owner: token_program,
            ..Account::default()
        }
    );

    let mut pdas_account_for_mint2_data = vec![];
    pdas_account_for_mint2_data.extend_from_slice(&mint2_pubkey.to_bytes());
    pdas_account_for_mint2_data.extend_from_slice(&vaults_pda.to_bytes());
    pdas_account_for_mint2_data.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00
    ]);
    program_test.add_account(
        pdas_account_for_mint2_pubkey,
        Account {
            lamports: 1000000000,
            data: pdas_account_for_mint2_data,
            owner: token_program,
            ..Account::default()
        }
    );
        
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Bundle
    let bundle_instruction_data: [u8;2] = [0u8, bump_seed];
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &bundle_instruction_data,
            vec![
                AccountMeta::new_readonly(owner.pubkey(), true),
                AccountMeta::new_readonly(key_mint_pubkey, false),
                AccountMeta::new_readonly(owner_key_pubkey, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new(owners_account_for_mint1_pubkey, false),
                AccountMeta::new(pdas_account_for_mint1_pubkey, false),
                AccountMeta::new(owners_account_for_mint2_pubkey, false),
                AccountMeta::new(pdas_account_for_mint2_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check that transfer succeeded
    let vault_account_after_locking = banks_client
        .get_account(vault_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");

    let vault_after_locking: Vault = try_from_slice_unchecked(&vault_account_after_locking.data).unwrap();
    assert_eq!(
        vault_after_locking.key_mint,
        key_mint_pubkey
    );
    assert_eq!(
        vault_after_locking.accounts.len(),
        2
    );
    assert_eq!(
        vault_after_locking.accounts[0],
        pdas_account_for_mint1_pubkey
    );
    assert_eq!(
        vault_after_locking.accounts[1],
        pdas_account_for_mint2_pubkey
    );

    let pdas_account_for_mint1_pubkey_after_bundling_raw = banks_client
        .get_account(pdas_account_for_mint1_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let pdas_account_for_mint1_pubkey_after_bundling = TokenAccount::unpack(&pdas_account_for_mint1_pubkey_after_bundling_raw.data).unwrap();
    assert_eq!(
        pdas_account_for_mint1_pubkey_after_bundling.amount,
        1u64
    );

    let pdas_account_for_mint2_pubkey_after_bundling_raw = banks_client
        .get_account(pdas_account_for_mint2_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let pdas_account_for_mint2_pubkey_after_bundling = TokenAccount::unpack(&pdas_account_for_mint2_pubkey_after_bundling_raw.data).unwrap();
    assert_eq!(
        pdas_account_for_mint2_pubkey_after_bundling.amount,
        1u64
    );


    // Transafer bundle key to owner2.
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::transfer(
            &SPL_TOKEN_PROGRAM_ID,
            &owner_key_pubkey,
            &owner2_key_pubkey,
            &owner.pubkey(),
            &[&owner.pubkey()],
            1,
        ).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


    // Unbundle for owner2
    let unbundle_instruction_data: [u8;2] = [1u8, bump_seed];
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &unbundle_instruction_data,
            vec![
                AccountMeta::new_readonly(owner2.pubkey(), true),
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(owner2_key_pubkey, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new_readonly(vaults_pda, false),
                AccountMeta::new(pdas_account_for_mint1_pubkey, false),
                AccountMeta::new(owners2_account_for_mint1_pubkey, false),
                AccountMeta::new(pdas_account_for_mint2_pubkey, false),
                AccountMeta::new(owners2_account_for_mint2_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner2], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


    let owners2_account_for_mint1_pubkey_after_unbundling_raw = banks_client
        .get_account(owners2_account_for_mint1_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let owners2_account_for_mint1_pubkey_after_unbundling = TokenAccount::unpack(&owners2_account_for_mint1_pubkey_after_unbundling_raw.data).unwrap();
    assert_eq!(
        owners2_account_for_mint1_pubkey_after_unbundling.amount,
        1u64
    );

    let owners2_account_for_mint2_pubkey_after_unbundling_raw = banks_client
        .get_account(owners2_account_for_mint2_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let owners2_account_for_mint2_pubkey_after_unbundling = TokenAccount::unpack(&owners2_account_for_mint2_pubkey_after_unbundling_raw.data).unwrap();
    assert_eq!(
        owners2_account_for_mint2_pubkey_after_unbundling.amount,
        1u64
    );

    let pdas_account_for_mint1_pubkey_after_unbundling_raw = banks_client
        .get_account(pdas_account_for_mint1_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let pdas_account_for_mint1_pubkey_after_unbundling = TokenAccount::unpack(&pdas_account_for_mint1_pubkey_after_unbundling_raw.data).unwrap();
    assert_eq!(
        pdas_account_for_mint1_pubkey_after_unbundling.amount,
        0u64
    );

    let pdas_account_for_mint2_pubkey_after_unbundling_raw = banks_client
        .get_account(pdas_account_for_mint2_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    let pdas_account_for_mint2_pubkey_after_unbundling = TokenAccount::unpack(&pdas_account_for_mint2_pubkey_after_unbundling_raw.data).unwrap();
    assert_eq!(
        pdas_account_for_mint2_pubkey_after_unbundling.amount,
        0u64
    );
    
    //let amount_after_staking = u64::from_le_bytes(pool_token_account_after_staking.data.get(64..72).unwrap().try_into().unwrap());
    //assert_eq!(18374686479671623685u64, amount_after_staking);
    /*
    let greeted_pubkey = Pubkey::new_unique();

    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );

    // Verify account has zero greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        0
    );

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greeted_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one greeting
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        1
    );

    // Greet again
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greeted_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        2
    );
    */
}
