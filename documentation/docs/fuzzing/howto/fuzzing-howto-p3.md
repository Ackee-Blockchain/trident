# Specify instruction accounts

{{ config.site_name }} fuzzer generates random indexes of accounts to use in each instruction. Each created account is saved in the global `FuzzAccounts` structure which helps you to reuse already existing accounts across all instructions.

You are required to define, how these accounts should be created and which accounts should be passed to an instruction. It is done using the `IxOps` trait and its method `get_accounts`.

- Go to the `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/fuzz_instructions.rs` file and complete the pre-generated `get_accounts` methods for each instruction such as:

```rust
fn get_accounts(
    &self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut FuzzAccounts,
) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    let author = fuzz_accounts.author.get_or_create_account(
        self.accounts.author,
        client,
        5000000000000,
    );
    let signers = vec![author.clone()];
    let state = fuzz_accounts
        .state
        .get_or_create_account(
            self.accounts.state,
            &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &fuzz_example1::ID,
        )
        .ok_or(FuzzingError::CannotGetAccounts)?
        .pubkey();
    let acc_meta = fuzz_example1::accounts::EndRegistration {
        author: author.pubkey(),
        state,
    }
    .to_account_metas(None);
    Ok((signers, acc_meta))
}
```
Notice especially the helper method `fuzz_accounts.<account_name>.get_or_create_account` that is used to create a Keypair or retrieve the Public key of the already existing account.

## Create an arbitrary account
The `AccountsStorage<T>` type provides an implementation of the `get_or_create_account` method that helps you create new or read already existing accounts. There are different implementations for different types of storage (`Keypair`, `TokenStore`, `MintStore`, `PdaStore`) to simplify the creation of new accounts.

However, there are cases when the provided implementation is not sufficient and it is necessary to create an account manually. These cases can be (but are not limited to) for example:

- you need to create a new account with a predefined address
- you need to create a new account that is not owned by the system program
- you need to create and initialize a new PDA account
- your program expects an account to be initialized in a previous instruction

In that case, you can use the `storage` method of the `AccountsStorage<T>` struct that exposes the underlying `HashMap<AccountId, T>` and you can add new accounts directly to it.

It is possible to create and store any kind of account. For example:

- to add an account that uses the `#[account(zero)]` anchor constraint (must be rent exempt, owned by your program, with empty data):

```rust
let state = fuzz_accounts
    .state
    // gets the storage of all `state` account variants
    .storage()
    // returns the Keypair of the `state` account with
    // the given `AccountId` if it has been added previously
    .entry(self.accounts.state)
    .or_insert_with(|| {
        let space = State::SIZE;
        let rent_exempt_lamports = client.get_rent().unwrap()
                            .minimum_balance(space);
        let keypair = Keypair::new();
        let account = AccountSharedData::new_data_with_space::<[u8; 0]>(
            rent_exempt_lamports,
            &[],
            space,
            &my_program::id(),
        ).unwrap();
        // insert the custom account also into the client
        client.set_account_custom(&keypair.pubkey(), &account);
        keypair
    });
```

- to add a new system-owned account with a specific PDA (address):

```rust
let rent_exempt_for_token_acc = client
    .get_rent()
    .unwrap()
    .minimum_balance(anchor_spl::token::spl_token::state::Account::LEN);

let my_pda = fuzz_accounts
    .my_pda
    // gets the storage of all `my_pda` account variants
    .storage()
    // returns the PdaStore struct of the `my_pda` account with
    // the given `AccountId` if it has been added previously
    .entry(self.accounts.my_pda)
    .or_insert_with(|| {
        let seeds = &[b"some-seeds"];
        let pda = Pubkey::find_program_address(seeds, &my_program::id()).0;
        let account = AccountSharedData::new_data_with_space::<[u8; 0]>(
            rent_exempt_for_token_acc,
            &[],
            0,
            &SYSTEM_PROGRAM_ID,
        ).unwrap();
        // insert the custom account also into the client
        client.set_account_custom(&pda, &account);
        let vec_of_seeds: Vec<Vec<u8>> = seeds.iter().map(|&seed| seed.to_vec())
                            .collect();
        PdaStore {
            pubkey: pda,
            seeds: vec_of_seeds,
        }
    }).pubkey();
```
