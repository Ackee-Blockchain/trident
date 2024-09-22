# Fuzz Instructions

Trident defines `FuzzInstruction` enum containing all available **Instructions** within your program.

The enum variants additionally contains their corresponding structures for **Accounts** and **Input** arguments.

```rust
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    Update(Update),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct Update {
    pub accounts: UpdateAccounts,
    pub data: UpdateData,
}
// ...
```

## Instruction behavior

Each Instruction variant has to define `IxOps` trait containing the following methods:

- `get_program_id()` (required)
- `get_data()` (required)
- `get_accounts()` (required)
- `check()` (optional)
- `tx_error_handler()` (optional)
- `deserialize_accounts()` (automatically implemented)


## Get Program ID

This method specifies **program ID** to which the Instruction corresponds.

In case you have only one program in the Anchor Workspace it is not really important. The importance occurs when you have multiple programs in the Workspace and you want to call Instructions of every Program. In that case each Instruction Variant corresponds to its program by the Program ID.

## Get Data

This method specifies what the Instruction Input Data should look like. You can use completely random data generated by the fuzzer, such as:

```rust
fn get_data(
    &self,
    _client: &mut impl FuzzClient,
    _fuzz_accounts: &mut FuzzAccounts,
) -> Result<Self::IxData, FuzzingError> {
    let data = hello_world::instruction::InitializeFn {
        input: self.data.input,
    };
    Ok(data)
}
```

You can also use always constant values

```rust
fn get_data(
    &self,
    _client: &mut impl FuzzClient,
    _fuzz_accounts: &mut FuzzAccounts,
) -> Result<Self::IxData, FuzzingError> {
    let data = hello_world::instruction::InitializeFn {
        input: 5,
    };
    Ok(data)
}
```

Or you can customize the Data using the Arbitrary crate. Check [Arbitrary Data](./arbitrary-data.md).

## Get Accounts

This method specifies how the **Accounts** for the corresponding Instruction should be resolved. You can use accounts stored within the **FuzzAccounts Account Storages**, or you can define custom Account using the **client**.


!!! important

    Source Code below

    - Take the author from the `FuzzAccounts` Account Storage author on `self.accounts.author` index. If Account on that index does not exist yet, it will be created and returned. In case it is already created, the corresponding Account will be returned.
    - `hello_world_account` Account Storage is of type PdaStore, in this case `get_or_create_account` function do the same as for the authority, but you need to specify seeds to derive its PDA.
    - Next, you need to specify signers if there should be any.
    - Lastly, specify the Account Metas of the corresponding Instruction.
        - for example `<program_name>::accounts::<context_name> {}.to_account_metas(None);`

```rust
fn get_accounts(
    &self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut FuzzAccounts,
) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    let author = fuzz_accounts.author.get_or_create_account(
        self.accounts.author,
        client,
        5 * LAMPORTS_PER_SOL,
    );

    let hello_world_account = fuzz_accounts
        .hello_world_account
        .get_or_create_account(
            self.accounts.hello_world_account,
            &[b"hello_world_seed"],
            &hello_world::ID,
        )
        .unwrap();
    let signers = vec![author.clone()];
    let acc_meta = hello_world::accounts::InitializeContext {
        author: author.pubkey(),
        hello_world_account: hello_world_account.pubkey(),
        system_program: solana_sdk::system_program::ID,
    }
    .to_account_metas(None);
    Ok((signers, acc_meta))
}
```


### Create an arbitrary account
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

## Check

This method provides Invariant Check for the corresponding Instruction. Check [Invariant Checks](./invariant-checks.md).

## Tx Error Handler

This method provides Tx Error Handler for the corresponding Instruction. Check [Error Handler](./error-handlers.md).


## Example

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.