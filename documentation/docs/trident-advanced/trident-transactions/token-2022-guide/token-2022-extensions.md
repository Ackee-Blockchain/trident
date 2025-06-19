# Token 2022 Extensions

Trident currently covers 100% of Token 2022 extensions. 

Use Trident's user-friendly data structures for each extension.

## Importing:

```rust
use trident_fuzz::accounts_storage::{
    extensions::*, // Token 2022 Mint and Account extensions from extensions::*;
    ParamValue, // Enum parameter that holds every extension type as type of its own.

};
```

The above import gives you access to the Enum used as parameter to hold every extension struct available in Trident.

## Example:

```rust
// For MetadataPointer extension
let metadata_params = ParamValue::MetadataPointer(
    MetadataPointer {
        authority: Some(authority),
        metadata_address: Some(receiver),
    }
);

// For InterestBearing extension
let interest_bearing_params = ParamValue::InterestBearingConfig(
    InterestBearingConfig {
        rate_authority: Some(authority),
        current_rate: Some(500), // 5% APR
        initialization_timestamp: Some(1640995200), // Jan 1, 2022
    }
);
```
`ParamValue` Enum has a matching type for every single extension type available on SPL-Token-2022 v6.0.0.

Make sure you associate the correct extension struct to the matching ParamValue Enum field (They will have matching names, as the above example portrays).

Note: All extension struct fields have been made optional (`Option<T>`) for added flexibility. You can provide `Some(value)` for fields you want to set, or `None` to use default values.

Once the extension is prepared, it's time to place it in a vector.

```rust
let mut extensions = Vec::<ParamValue>::new();
extensions.push(metadata_params); //<- In the Vector is possible to accumulate as many extensions you want.
extensions.push(interest_bearing_params);
```

Next, use the last parameter of either `get_or_create_token_2022_mint` or `get_or_create_token_2022_account`, to pass in the desired extension to be loaded:

```rust
let mint = fuzz_accounts.mint.get_or_create_token_2022_mint(
        self.accounts.mint.account_id,
        client,
        None,
        9,
        &authority,
        Some(authority),
        Some(extensions) // <- Place the extension or extensions
);
```
    Some extensions should not be combined due to either security concerns or straight incompatibility.
    


!!! warning "Mint known extensions Incompatibilities"
    Confirmed Incompatible Combinations:

    - NonTransferable + Transfer-related Extensions, NonTransferable cannot be combined with:

        `TransferFeeConfig` - since NonTransferable tokens can't be transferred, transfer fees make no sense
        
        `TransferHook` - transfer hooks are pointless if tokens can't be transferred
        
        `ConfidentialTransfer` - confidential transfers are irrelevant for non-transferable tokens
    
    - TransferHook + ConfidentialTransfer
        
        `Transfer hooks` and `confidential transfers` do not currently work together (these transfers can only see source / destination accounts, therefore cannot act on the amount transferred)
    
    Logical Incompatibilities (implied):

    - MintCloseAuthority + Extensions requiring ongoing mint state:

        `InterestBearingConfig` - can't accrue interest if mint is closed

        `TransferFeeConfig` - can't collect fees if mint is closed

    Multiple "Pointer" Extensions pointing to different accounts:

    - `MetadataPointer` + `GroupPointer` - both can exist but should point to compatible accounts

    
!!! warning "Token Account known extension Incompatibilities"

    Logical Conflicts Based on NonTransferable Tokens. 
    
    When a mint has NonTransferable extension enabled, the corresponding token accounts automatically get:

    - `NonTransferableAccount` extension

    - `ImmutableOwner` extension (automatically added)

    This means NonTransferableAccount inherently conflicts with any transfer-related account extensions since the tokens can't be transferred.

    TransferHook Requirements:

    When a mint has `TransferHook` extension, token accounts automatically require:

    - `TransferHookAccount` extension

    TransferFeeConfig Requirements:

    When a mint has `TransferFeeConfig` extension, token accounts automatically require:

    - `TransferFeeAmount` extension

!!! warning "Specific Account Extension Incompatibilities:"

    ImmutableOwner Conflicts:

    The `ImmutableOwner` extension is automatically enabled by default for all Associated Token Accounts in Token-2022, and "there's no way to turn it off" through the standard ATA program.

    This creates conflicts when you need to:

    - Change ownership after account creation

    - Use multisig ownership patterns with existing ATAs

    CpiGuard Conflicts:

    `CpiGuard` can cause issues when:

    - Programs expect to use token accounts in CPI contexts

    - Must follow delegation flow instead of direct transfers

    - Can be enabled/disabled dynamically but affects program behavior

    MemoTransfer Conflicts:

    `MemoTransfer` requires all incoming transfers to have a memo instruction, which can cause:

    - Transaction failures when memo is missing

    - Issues with programs that don't expect memo requirements
    
    - Problems in multi-transaction intents 