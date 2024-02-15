#![feature(prelude_import)]
//! An example of an escrow program, inspired by PaulX tutorial seen here
//! https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
//! This example has some changes to implementation, but more or less should be the same overall
//! Also gives examples on how to use some newer anchor features and CPI
//!
//! User (Initializer) constructs an escrow deal:
//! - SPL token (X) they will offer and amount
//! - SPL token (Y) count they want in return and amount
//! - Program will take ownership of initializer's token X account
//!
//! Once this escrow is initialised, either:
//! 1. User (Taker) can call the exchange function to exchange their Y for X
//! - This will close the escrow account and no longer be usable
//! OR
//! 2. If no one has exchanged, the initializer can close the escrow account
//! - Initializer will get back ownership of their token X account
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, SetAuthority, Token,
    TokenAccount, Transfer,
};

pub mod innerstate {
    use super::*;

    pub enum EnumInputInner {
        Variant1,
        Variant2,
        Variant3,
        Variant4,
        Variant5,
    }
    impl borsh::de::BorshDeserialize for EnumInputInner {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let tag =
                <u8 as
                            borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for EnumInputInner {
        fn deserialize_variant<R: borsh::maybestd::io::Read>(reader: &mut R,
            variant_idx: u8)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let mut return_value =
                match variant_idx {
                    0u8 => EnumInputInner::Variant1,
                    1u8 => EnumInputInner::Variant2,
                    2u8 => EnumInputInner::Variant3,
                    3u8 => EnumInputInner::Variant4,
                    4u8 => EnumInputInner::Variant5,
                    _ =>
                        return Err(borsh::maybestd::io::Error::new(borsh::maybestd::io::ErrorKind::InvalidInput,










                                    // Transferring from initializer to taker














                                    {
                                        let res =
                                            ::alloc::fmt::format(format_args!("Unexpected variant index: {0:?}",
                                                    variant_idx));
                                        res
                                    })),
                };
            Ok(return_value)
        }
    }
    impl borsh::ser::BorshSerialize for EnumInputInner {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 =
                match self {
                    EnumInputInner::Variant1 => 0u8,
                    EnumInputInner::Variant2 => 1u8,
                    EnumInputInner::Variant3 => 2u8,
                    EnumInputInner::Variant4 => 3u8,
                    EnumInputInner::Variant5 => 4u8,
                };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                EnumInputInner::Variant1 => {}
                EnumInputInner::Variant2 => {}
                EnumInputInner::Variant3 => {}
                EnumInputInner::Variant4 => {}
                EnumInputInner::Variant5 => {}
            }
            Ok(())
        }
    }
}
pub use crate::innerstate::*;
pub mod state {
    use anchor_lang::prelude::*;
    pub enum EnumInput { Variant1, Variant2, Variant3, Variant4, Variant5, }
    impl borsh::de::BorshDeserialize for EnumInput {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let tag =
                <u8 as
                            borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for EnumInput {
        fn deserialize_variant<R: borsh::maybestd::io::Read>(reader: &mut R,
            variant_idx: u8)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let mut return_value =
                match variant_idx {
                    0u8 => EnumInput::Variant1,
                    1u8 => EnumInput::Variant2,
                    2u8 => EnumInput::Variant3,
                    3u8 => EnumInput::Variant4,
                    4u8 => EnumInput::Variant5,
                    _ =>
                        return Err(borsh::maybestd::io::Error::new(borsh::maybestd::io::ErrorKind::InvalidInput,
                                    {
                                        let res =
                                            ::alloc::fmt::format(format_args!("Unexpected variant index: {0:?}",
                                                    variant_idx));
                                        res
                                    })),
                };
            Ok(return_value)
        }
    }
    impl borsh::ser::BorshSerialize for EnumInput {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 =
                match self {
                    EnumInput::Variant1 => 0u8,
                    EnumInput::Variant2 => 1u8,
                    EnumInput::Variant3 => 2u8,
                    EnumInput::Variant4 => 3u8,
                    EnumInput::Variant5 => 4u8,
                };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                EnumInput::Variant1 => {}
                EnumInput::Variant2 => {}
                EnumInput::Variant3 => {}
                EnumInput::Variant4 => {}
                EnumInput::Variant5 => {}
            }
            Ok(())
        }
    }
    pub struct StructInput {
        pub field1: u8,
        pub field2: String,
        pub field3: StructInputInner,
    }
    impl borsh::de::BorshDeserialize for StructInput where
        u8: borsh::BorshDeserialize, String: borsh::BorshDeserialize,
        StructInputInner: borsh::BorshDeserialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                    field1: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    field2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    field3: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
        }
    }
    impl borsh::ser::BorshSerialize for StructInput where
        u8: borsh::ser::BorshSerialize, String: borsh::ser::BorshSerialize,
        StructInputInner: borsh::ser::BorshSerialize {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.field1, writer)?;
            borsh::BorshSerialize::serialize(&self.field2, writer)?;
            borsh::BorshSerialize::serialize(&self.field3, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for StructInput {
        #[inline]
        fn default() -> StructInput {
            StructInput {
                field1: ::core::default::Default::default(),
                field2: ::core::default::Default::default(),
                field3: ::core::default::Default::default(),
            }
        }
    }
    pub struct StructInputInner {
        pub field1: Pubkey,
        pub field2: String,
    }
    impl borsh::de::BorshDeserialize for StructInputInner where
        Pubkey: borsh::BorshDeserialize, String: borsh::BorshDeserialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                    field1: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    field2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
        }
    }
    impl borsh::ser::BorshSerialize for StructInputInner where
        Pubkey: borsh::ser::BorshSerialize, String: borsh::ser::BorshSerialize
        {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.field1, writer)?;
            borsh::BorshSerialize::serialize(&self.field2, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for StructInputInner {
        #[inline]
        fn default() -> StructInputInner {
            StructInputInner {
                field1: ::core::default::Default::default(),
                field2: ::core::default::Default::default(),
            }
        }
    }
}
pub use crate::state::*;
#[doc = r" The static program ID"]
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([5u8, 214u8,
                204u8, 101u8, 166u8, 163u8, 239u8, 244u8, 13u8, 110u8, 64u8,
                106u8, 230u8, 81u8, 141u8, 186u8, 208u8, 155u8, 78u8, 83u8,
                194u8, 215u8, 103u8, 17u8, 94u8, 15u8, 137u8, 68u8, 170u8,
                153u8, 74u8, 59u8]);
#[doc = r" Confirms that a given pubkey is equivalent to the program ID"]
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
#[doc = r" Returns the program ID"]
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey { ID }
use self::escrow::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
#[doc =
r" The Anchor codegen exposes a programming model where a user defines"]
#[doc = r" a set of methods inside of a `#[program]` module in a way similar"]
#[doc =
r" to writing RPC request handlers. The macro then generates a bunch of"]
#[doc =
r" code wrapping these user defined methods into something that can be"]
#[doc = r" executed on Solana."]
#[doc = r""]
#[doc = r" These methods fall into one categorie for now."]
#[doc = r""]
#[doc = r" Global methods - regular methods inside of the `#[program]`."]
#[doc = r""]
#[doc = r" Care must be taken by the codegen to prevent collisions between"]
#[doc =
r" methods in these different namespaces. For this reason, Anchor uses"]
#[doc = r" a variant of sighash to perform method dispatch, rather than"]
#[doc = r" something like a simple enum variant discriminator."]
#[doc = r""]
#[doc = r" The execution flow of the generated code can be roughly outlined:"]
#[doc = r""]
#[doc = r" * Start program via the entrypoint."]
#[doc =
r" * Strip method identifier off the first 8 bytes of the instruction"]
#[doc = r"   data and invoke the identified method. The method identifier"]
#[doc =
r"   is a variant of sighash. See docs.rs for `anchor_lang` for details."]
#[doc = r" * If the method identifier is an IDL identifier, execute the IDL"]
#[doc = r"   instructions, which are a special set of hardcoded instructions"]
#[doc = r"   baked into every Anchor program. Then exit."]
#[doc = r" * Otherwise, the method identifier is for a user defined"]
#[doc = r"   instruction, i.e., one of the methods in the user defined"]
#[doc = r"   `#[program]` module. Perform method dispatch, i.e., execute the"]
#[doc = r"   big match statement mapping method identifier to method handler"]
#[doc = r"   wrapper."]
#[doc = r" * Run the method handler wrapper. This wraps the code the user"]
#[doc = r"   actually wrote, deserializing the accounts, constructing the"]
#[doc = r"   context, invoking the user's code, and finally running the exit"]
#[doc = r"   routine, which typically persists account changes."]
#[doc = r""]
#[doc = r" The `entry` function here, defines the standard entry to a Solana"]
#[doc = r" program, where execution begins."]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8])
    -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| { e.log(); e.into() })
}
fn try_entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8])
    -> anchor_lang::Result<()> {
    if *program_id != ID {
            return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
        }
    if data.len() < 8 {
            return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
        }
    dispatch(program_id, accounts, data)
}
#[doc = r" Module representing the program."]
pub mod program {
    use super::*;
    #[doc = r" Type representing the program."]
    pub struct Escrow;
    #[automatically_derived]
    impl ::core::clone::Clone for Escrow {
        #[inline]
        fn clone(&self) -> Escrow { Escrow }
    }
    impl anchor_lang::Id for Escrow {
        fn id() -> Pubkey { ID }
    }
}
#[doc = r" Performs method dispatch."]
#[doc = r""]
#[doc =
r" Each method in an anchor program is uniquely defined by a namespace"]
#[doc = r" and a rust identifier (i.e., the name given to the method). These"]
#[doc = r" two pieces can be combined to creater a method identifier,"]
#[doc = r" specifically, Anchor uses"]
#[doc = r""]
#[doc = r#" Sha256("<namespace>:<rust-identifier>")[..8],"#]
#[doc = r""]
#[doc = r#" where the namespace can be one type. "global" for a"#]
#[doc = r" regular instruction."]
#[doc = r""]
#[doc = r" With this 8 byte identifier, Anchor performs method dispatch,"]
#[doc = r" matching the given 8 byte identifier to the associated method"]
#[doc =
r" handler, which leads to user defined code being eventually invoked."]
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8])
    -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] =
        {
            let mut sighash: [u8; 8] = [0; 8];
            sighash.copy_from_slice(&ix_data[..8]);
            ix_data = &ix_data[8..];
            sighash
        };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::InitializeEscrow::DISCRIMINATOR => {
            __private::__global::initialize_escrow(program_id, accounts,
                ix_data)
        }
        instruction::CancelEscrow::DISCRIMINATOR => {
            __private::__global::cancel_escrow(program_id, accounts, ix_data)
        }
        instruction::Exchange::DISCRIMINATOR => {
            __private::__global::exchange(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {

            #[cfg(not(feature = "no-idl"))]
            {
                __private::__idl::__idl_dispatch(program_id, accounts,
                    &ix_data)
            }
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => {
            Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into())
        }
    }
}
#[doc = r" Create a private module to not clutter the program's namespace."]
#[doc = r" Defines an entrypoint for each individual instruction handler"]
#[doc = r" wrapper."]
mod __private {
    use super::*;
    #[doc =
    r" __idl mod defines handlers for injected Anchor IDL instructions."]
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(program_id: &Pubkey, accounts: &[AccountInfo],
            idl_ix_data: &[u8]) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix =
                anchor_lang::idl::IdlInstruction::deserialize(&mut data).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCreateAccounts::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlResizeAccount::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCloseAccount::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCreateBuffer::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlAccounts::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority
                    } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlAccounts::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_set_authority(program_id, &mut accounts,
                            new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlSetBuffer::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(f,
                    "IdlAccount", "authority", &self.authority, "data_len",
                    &&self.data_len)
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize {
            fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
                -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount where
            Pubkey: borsh::BorshDeserialize, u32: borsh::BorshDeserialize {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(reader:
                    &mut R)
                -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                        authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
                -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123,
                                            158]).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }
                if AnchorSerialize::serialize(self, writer).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                        return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                    }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.to_string(),
                                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/escrow/src/lib.rs",
                                                        line: 42u32,
                                                    })),
                                            compared_values: None,
                                        }).with_account_name("IdlAccount"));
                    }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8])
                -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data).map_err(|_|
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] =
                [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer =
                    Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(),
                        program_id).expect("Seed is always valid")
            }
            pub fn seed() -> &'static str { "anchor:idl" }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey { crate::ID }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCreateAccounts<'info>
            where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let from: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("base"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                let program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("program"))?;
                if !from.is_signer {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSigner).with_account_name("from"));
                    }
                if !to.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("to"));
                    }
                let (__pda_address, __bump) =
                    Pubkey::find_program_address(&[], &__program_id);
                __bumps.insert("base".to_string(), __bump);
                if base.key() != __pda_address {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("base").with_pubkeys((base.key(),
                                        __pda_address)));
                    }
                if !program.to_account_info().executable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintExecutable).with_account_name("program"));
                    }
                Ok(IdlCreateAccounts {
                        from,
                        to,
                        base,
                        system_program,
                        program,
                    })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCreateAccounts<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCreateAccounts<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to,
                            program_id).map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCreateAccounts`]."]
            pub struct IdlCreateAccounts {
                pub from: anchor_lang::solana_program::pubkey::Pubkey,
                pub to: anchor_lang::solana_program::pubkey::Pubkey,
                pub base: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.from,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.to,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.base,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCreateAccounts`]."]
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for
                IdlCreateAccounts<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.from),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.to),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.base),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCreateAccounts<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.from));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.base));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.program));
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlAccounts<'info> where
            'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                if !idl.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
            where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlAccounts`]."]
            pub struct IdlAccounts {
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlAccounts`]."]
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlAccounts<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlResizeAccount<'info>
            where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                if !idl.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !authority.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("authority"));
                    }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlResizeAccount { idl, authority, system_program })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlResizeAccount<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlResizeAccount<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority,
                            program_id).map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlResizeAccount`]."]
            pub struct IdlResizeAccount {
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.authority,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlResizeAccount`]."]
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for
                IdlResizeAccount<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlResizeAccount<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCreateBuffer<'info>
            where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                        return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                    }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    {
                        let mut __data: &[u8] = &buffer.try_borrow_data()?;
                        let mut __disc_bytes = [0u8; 8];
                        __disc_bytes.copy_from_slice(&__data[..8]);
                        let __discriminator = u64::from_le_bytes(__disc_bytes);
                        if __discriminator != 0 {
                                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintZero).with_account_name("buffer"));
                            }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(&buffer)
                            {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("buffer")),
                        }
                    };
                if !buffer.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("buffer"));
                    }
                if !__anchor_rent.is_exempt(buffer.to_account_info().lamports(),
                                buffer.to_account_info().try_data_len()?) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("buffer"));
                    }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlCreateBuffer { buffer, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCreateBuffer<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCreateBuffer<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer,
                            program_id).map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCreateBuffer`]."]
            pub struct IdlCreateBuffer {
                pub buffer: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.buffer,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCreateBuffer`]."]
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info>
                {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.buffer),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCreateBuffer<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.buffer));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority == idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlSetBuffer<'info> where
            'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let buffer:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                if !buffer.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("buffer"));
                    }
                if !(buffer.authority == idl.authority) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("buffer"));
                    }
                if !idl.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlSetBuffer { buffer, idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
            where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer,
                            program_id).map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlSetBuffer`]."]
            pub struct IdlSetBuffer {
                pub buffer: anchor_lang::solana_program::pubkey::Pubkey,
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.buffer,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlSetBuffer`]."]
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.buffer),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlSetBuffer<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.buffer));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCloseAccount<'info>
            where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let account:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("account"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("sol_destination"))?;
                if !account.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("account"));
                    }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("account").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                {
                    if account.key() == sol_destination.key() {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintClose).with_account_name("account"));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                if !sol_destination.to_account_info().is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("sol_destination"));
                    }
                Ok(IdlCloseAccount { account, authority, sol_destination })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCloseAccount<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCloseAccount<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(&self.account,
                                sol_destination.to_account_info()).map_err(|e|
                                e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination,
                            program_id).map_err(|e|
                            e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCloseAccount`]."]
            pub struct IdlCloseAccount {
                pub account: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub sol_destination: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
                {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.sol_destination,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCloseAccount`]."]
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info>
                {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.sol_destination),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCloseAccount<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.sol_destination));
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self)
            -> Ref<'info, [u8]>;
            fn trailing_data_mut(self)
            -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for
            &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(),
                    |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts, data_len: u64)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                    return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
                }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix =
                anchor_lang::solana_program::system_instruction::create_account_with_seed(from,
                    &to, &base, seed, lamports, space as u64, owner);
            anchor_lang::solana_program::program::invoke_signed(&ix,
                    &[accounts.from.clone(), accounts.to.clone(),
                                accounts.base.clone(),
                                accounts.system_program.to_account_info().clone()],
                    &[seeds])?;
            let mut idl_account =
                {
                    let mut account_data = accounts.to.try_borrow_data()?;
                    let mut account_data_slice: &[u8] = &account_data;
                    IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
                };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(program_id: &Pubkey,
            accounts: &mut IdlResizeAccount, data_len: u64)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                    return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
                }
            let new_account_space =
                accounts.idl.to_account_info().data_len().checked_add(std::cmp::min(data_len.checked_sub(accounts.idl.to_account_info().data_len()).expect("data_len should always be >= the current account space"),
                            10_000)).unwrap();
            if new_account_space > accounts.idl.to_account_info().data_len() {
                    let sysvar_rent = Rent::get()?;
                    let new_rent_minimum =
                        sysvar_rent.minimum_balance(new_account_space);
                    anchor_lang::system_program::transfer(anchor_lang::context::CpiContext::new(accounts.system_program.to_account_info(),
                                anchor_lang::system_program::Transfer {
                                    from: accounts.authority.to_account_info(),
                                    to: accounts.idl.to_account_info().clone(),
                                }),
                            new_rent_minimum.checked_sub(accounts.idl.to_account_info().lamports()).unwrap())?;
                    accounts.idl.to_account_info().realloc(new_account_space,
                            false)?;
                }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(program_id: &Pubkey,
            accounts: &mut IdlCloseAccount) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(program_id: &Pubkey, accounts: &mut IdlAccounts,
            idl_data: Vec<u8>) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize =
                ::std::convert::TryInto::<usize>::try_into(accounts.idl.data_len).unwrap();
            let new_len: usize =
                prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len =
                accounts.idl.data_len.checked_add(::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap()).unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                        error_name: anchor_lang::error::ErrorCode::RequireEqViolated.name(),
                                        error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated.into(),
                                        error_msg: anchor_lang::error::ErrorCode::RequireEqViolated.to_string(),
                                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/escrow/src/lib.rs",
                                                    line: 42u32,
                                                })),
                                        compared_values: None,
                                    }).with_values((idl_expansion.len(), idl_data.len())));
                };
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(program_id: &Pubkey,
            accounts: &mut IdlAccounts, new_authority: Pubkey)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(program_id: &Pubkey,
            accounts: &mut IdlSetBuffer) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len =
                ::std::convert::TryInto::<usize>::try_into(accounts.buffer.data_len).unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                        error_name: anchor_lang::error::ErrorCode::RequireGteViolated.name(),
                                        error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated.into(),
                                        error_msg: anchor_lang::error::ErrorCode::RequireGteViolated.to_string(),
                                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/escrow/src/lib.rs",
                                                    line: 42u32,
                                                })),
                                        compared_values: None,
                                    }).with_values((target.len(), buffer_len)));
                };
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    #[doc =
    r" __global mod defines wrapped handlers for global instructions."]
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_escrow(__program_id: &Pubkey,
            __accounts: &[AccountInfo], __ix_data: &[u8])
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeEscrow");
            let ix =
                instruction::InitializeEscrow::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitializeEscrow {
                    initializer_amount,
                    taker_amount,
                    _enum_variant,
                    _enum_variant_inner,
                    _struct_variant_inner } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                InitializeEscrow::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                escrow::initialize_escrow(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps),
                        initializer_amount, taker_amount, _enum_variant,
                        _enum_variant_inner, _struct_variant_inner)?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn cancel_escrow(__program_id: &Pubkey,
            __accounts: &[AccountInfo], __ix_data: &[u8])
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CancelEscrow");
            let ix =
                instruction::CancelEscrow::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CancelEscrow = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                CancelEscrow::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                escrow::cancel_escrow(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps))?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn exchange(__program_id: &Pubkey, __accounts: &[AccountInfo],
            __ix_data: &[u8]) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Exchange");
            let ix =
                instruction::Exchange::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Exchange = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                Exchange::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                escrow::exchange(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps))?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod escrow {
    use super::*;
    const ESCROW_PDA_SEED: &[u8] = b"escrow";
    pub fn initialize_escrow(ctx: Context<InitializeEscrow>,
        initializer_amount: u64, taker_amount: u64, _enum_variant: EnumInput,
        _enum_variant_inner: EnumInputInner,
        _struct_variant_inner: StructInput) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key =
            *ctx.accounts.initializer.key;
        ctx.accounts.escrow_account.initializer_deposit_token_account =
            *ctx.accounts.initializer_deposit_token_account.to_account_info().key;
        ctx.accounts.escrow_account.initializer_receive_token_account =
            *ctx.accounts.initializer_receive_token_account.to_account_info().key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;
        let (pda, _bump_seed) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner,
                Some(pda))?;
        Ok(())
    }
    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let (_pda, bump_seed) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];
        token::set_authority(ctx.accounts.into_set_authority_context().with_signer(&[&seeds[..]]),
                AuthorityType::AccountOwner,
                Some(ctx.accounts.escrow_account.initializer_key))?;
        Ok(())
    }
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let (_pda, bump_seed) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];
        token::transfer(ctx.accounts.into_transfer_to_taker_context().with_signer(&[&seeds[..]]),
                ctx.accounts.escrow_account.initializer_amount)?;
        token::transfer(ctx.accounts.into_transfer_to_initializer_context(),
                ctx.accounts.escrow_account.taker_amount)?;
        token::set_authority(ctx.accounts.into_set_authority_context().with_signer(&[&seeds[..]]),
                AuthorityType::AccountOwner,
                Some(ctx.accounts.escrow_account.initializer_key))?;
        Ok(())
    }
}
#[doc = r" An Anchor generated module containing the program's set of"]
#[doc =
r" instructions, where each method handler in the `#[program]` mod is"]
#[doc = r" associated with a struct defining the input arguments to the"]
#[doc =
r" method. These should be used directly, when one wants to serialize"]
#[doc = r" Anchor instruction data, for example, when speciying"]
#[doc = r" instructions on a client."]
pub mod instruction {
    use super::*;
    #[doc = r" Instruction."]
    pub struct InitializeEscrow {
        pub initializer_amount: u64,
        pub taker_amount: u64,
        pub _enum_variant: EnumInput,
        pub _enum_variant_inner: EnumInputInner,
        pub _struct_variant_inner: StructInput,
    }
    impl borsh::ser::BorshSerialize for InitializeEscrow where
        u64: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
        EnumInput: borsh::ser::BorshSerialize,
        EnumInputInner: borsh::ser::BorshSerialize,
        StructInput: borsh::ser::BorshSerialize {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.initializer_amount,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.taker_amount, writer)?;
            borsh::BorshSerialize::serialize(&self._enum_variant, writer)?;
            borsh::BorshSerialize::serialize(&self._enum_variant_inner,
                    writer)?;
            borsh::BorshSerialize::serialize(&self._struct_variant_inner,
                    writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeEscrow where
        u64: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
        EnumInput: borsh::BorshDeserialize,
        EnumInputInner: borsh::BorshDeserialize,
        StructInput: borsh::BorshDeserialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                    initializer_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    taker_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    _enum_variant: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    _enum_variant_inner: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    _struct_variant_inner: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
        }
    }
    impl anchor_lang::Discriminator for InitializeEscrow {
        const DISCRIMINATOR: [u8; 8] = [243, 160, 77, 153, 11, 92, 48, 209];
    }
    impl anchor_lang::InstructionData for InitializeEscrow {}
    impl anchor_lang::Owner for InitializeEscrow {
        fn owner() -> Pubkey { ID }
    }
    #[doc = r" Instruction."]
    pub struct CancelEscrow;
    impl borsh::ser::BorshSerialize for CancelEscrow {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CancelEscrow {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CancelEscrow {
        const DISCRIMINATOR: [u8; 8] = [156, 203, 54, 179, 38, 72, 33, 21];
    }
    impl anchor_lang::InstructionData for CancelEscrow {}
    impl anchor_lang::Owner for CancelEscrow {
        fn owner() -> Pubkey { ID }
    }
    #[doc = r" Instruction."]
    pub struct Exchange;
    impl borsh::ser::BorshSerialize for Exchange {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Exchange {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for Exchange {
        const DISCRIMINATOR: [u8; 8] = [47, 3, 27, 97, 215, 236, 219, 144];
    }
    impl anchor_lang::InstructionData for Exchange {}
    impl anchor_lang::Owner for Exchange {
        fn owner() -> Pubkey { ID }
    }
}
#[doc = r" An Anchor generated module, providing a set of structs"]
#[doc = r" mirroring the structs deriving `Accounts`, where each field is"]
#[doc = r" a `Pubkey`. This is useful for specifying accounts for a client."]
pub mod accounts {
    pub use crate::__client_accounts_initialize_escrow::*;
    pub use crate::__client_accounts_exchange::*;
    pub use crate::__client_accounts_cancel_escrow::*;
}
#[instruction(initializer_amount : u64)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut, constraint = initializer_deposit_token_account.amount >=
    initializer_amount)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = initializer, space = 8 + EscrowAccount :: LEN)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for InitializeEscrow<'info> where
    'info: 'info {
    #[inline(never)]
    fn try_accounts(__program_id:
            &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts:
            &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        __ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs:
            &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
        -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            initializer_amount: u64,
        }
        impl borsh::ser::BorshSerialize for __Args where
            u64: borsh::ser::BorshSerialize {
            fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
                -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.initializer_amount,
                        writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args where
            u64: borsh::BorshDeserialize {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(reader:
                    &mut R)
                -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                        initializer_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
            }
        }
        let __Args { initializer_amount } =
            __Args::deserialize(&mut __ix_data).map_err(|_|
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        let initializer: Signer =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer"))?;
        let initializer_deposit_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer_deposit_token_account"))?;
        let initializer_receive_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer_receive_token_account"))?;
        if __accounts.is_empty() {
                return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
            }
        let escrow_account = &__accounts[0];
        *__accounts = &__accounts[1..];
        let system_program: anchor_lang::accounts::program::Program<System> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("system_program"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("token_program"))?;
        let __anchor_rent = Rent::get()?;
        let escrow_account =
            {
                let actual_field = escrow_account.to_account_info();
                let actual_owner = actual_field.owner;
                let space = 8 + EscrowAccount::LEN;
                let pa:
                        anchor_lang::accounts::account::Account<EscrowAccount> =
                    if !false ||
                                actual_owner ==
                                    &anchor_lang::solana_program::system_program::ID {
                            let __current_lamports = escrow_account.lamports();
                            if __current_lamports == 0 {
                                    let space = space;
                                    let lamports = __anchor_rent.minimum_balance(space);
                                    let cpi_accounts =
                                        anchor_lang::system_program::CreateAccount {
                                            from: initializer.to_account_info(),
                                            to: escrow_account.to_account_info(),
                                        };
                                    let cpi_context =
                                        anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                            cpi_accounts);
                                    anchor_lang::system_program::create_account(cpi_context.with_signer(&[]),
                                            lamports, space as u64, __program_id)?;
                                } else {
                                   if initializer.key() == escrow_account.key() {
                                           return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                               error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.name(),
                                                               error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.into(),
                                                               error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.to_string(),
                                                               error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                                           filename: "programs/escrow/src/lib.rs",
                                                                           line: 123u32,
                                                                       })),
                                                               compared_values: None,
                                                           }).with_pubkeys((initializer.key(), escrow_account.key())));
                                       };
                                   let required_lamports =
                                       __anchor_rent.minimum_balance(space).max(1).saturating_sub(__current_lamports);
                                   if required_lamports > 0 {
                                           let cpi_accounts =
                                               anchor_lang::system_program::Transfer {
                                                   from: initializer.to_account_info(),
                                                   to: escrow_account.to_account_info(),
                                               };
                                           let cpi_context =
                                               anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                                   cpi_accounts);
                                           anchor_lang::system_program::transfer(cpi_context,
                                                   required_lamports)?;
                                       }
                                   let cpi_accounts =
                                       anchor_lang::system_program::Allocate {
                                           account_to_allocate: escrow_account.to_account_info(),
                                       };
                                   let cpi_context =
                                       anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                           cpi_accounts);
                                   anchor_lang::system_program::allocate(cpi_context.with_signer(&[]),
                                           space as u64)?;
                                   let cpi_accounts =
                                       anchor_lang::system_program::Assign {
                                           account_to_assign: escrow_account.to_account_info(),
                                       };
                                   let cpi_context =
                                       anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                           cpi_accounts);
                                   anchor_lang::system_program::assign(cpi_context.with_signer(&[]),
                                           __program_id)?;
                               }
                            match anchor_lang::accounts::account::Account::try_from_unchecked(&escrow_account)
                                {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("escrow_account")),
                            }
                        } else {
                           match anchor_lang::accounts::account::Account::try_from(&escrow_account)
                               {
                               Ok(val) => val,
                               Err(e) => return Err(e.with_account_name("escrow_account")),
                           }
                       };
                if false {
                        if space != actual_field.data_len() {
                                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSpace).with_account_name("escrow_account").with_values((space,
                                                actual_field.data_len())));
                            }
                        if actual_owner != __program_id {
                                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintOwner).with_account_name("escrow_account").with_pubkeys((*actual_owner,
                                                *__program_id)));
                            }
                        {
                            let required_lamports =
                                __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                    return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("escrow_account"));
                                }
                        }
                    }
                pa
            };
        if !escrow_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow_account"));
            }
        if !escrow_account.to_account_info().is_signer {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSigner).with_account_name("escrow_account"));
            }
        if !__anchor_rent.is_exempt(escrow_account.to_account_info().lamports(),
                        escrow_account.to_account_info().try_data_len()?) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("escrow_account"));
            }
        if !initializer.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("initializer"));
            }
        if !initializer_deposit_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("initializer_deposit_token_account"));
            }
        if !(initializer_deposit_token_account.amount >= initializer_amount) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("initializer_deposit_token_account"));
            }
        Ok(InitializeEscrow {
                initializer,
                initializer_deposit_token_account,
                initializer_receive_token_account,
                escrow_account,
                system_program,
                token_program,
            })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeEscrow<'info>
    where 'info: 'info {
    fn to_account_infos(&self)
        ->
            Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.initializer.to_account_infos());
        account_infos.extend(self.initializer_deposit_token_account.to_account_infos());
        account_infos.extend(self.initializer_receive_token_account.to_account_infos());
        account_infos.extend(self.escrow_account.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for InitializeEscrow<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>)
        -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.initializer.to_account_metas(None));
        account_metas.extend(self.initializer_deposit_token_account.to_account_metas(None));
        account_metas.extend(self.initializer_receive_token_account.to_account_metas(None));
        account_metas.extend(self.escrow_account.to_account_metas(Some(true)));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for InitializeEscrow<'info> where
    'info: 'info {
    fn exit(&self, program_id: &anchor_lang::solana_program::pubkey::Pubkey)
        -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.initializer,
                    program_id).map_err(|e|
                    e.with_account_name("initializer"))?;
        anchor_lang::AccountsExit::exit(&self.initializer_deposit_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("initializer_deposit_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.escrow_account,
                    program_id).map_err(|e|
                    e.with_account_name("escrow_account"))?;
        Ok(())
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a struct for a given"]
#[doc =
r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
#[doc = r" instead of an `AccountInfo`. This is useful for clients that want"]
#[doc = r" to generate a list of accounts, without explicitly knowing the"]
#[doc = r" order all the fields should be in."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" `accounts` module (also generated), which re-exports this."]
pub(crate) mod __client_accounts_initialize_escrow {
    use super::*;
    use anchor_lang::prelude::borsh;
    #[doc = " Generated client accounts for [`InitializeEscrow`]."]
    pub struct InitializeEscrow {
        pub initializer: anchor_lang::solana_program::pubkey::Pubkey,
        pub initializer_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for InitializeEscrow where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
        {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.initializer, writer)?;
            borsh::BorshSerialize::serialize(&self.initializer_deposit_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.initializer_receive_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.escrow_account, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for InitializeEscrow {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.initializer,
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.initializer_deposit_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.initializer_receive_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow_account,
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program,
                    false));
            account_metas
        }
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a CPI struct for a given"]
#[doc = r" `#[derive(Accounts)]` implementation, where each field is an"]
#[doc = r" AccountInfo."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" [`cpi::accounts`] module (also generated), which re-exports this."]
pub(crate) mod __cpi_client_accounts_initialize_escrow {
    use super::*;
    #[doc = " Generated CPI struct of the accounts for [`InitializeEscrow`]."]
    pub struct InitializeEscrow<'info> {
        pub initializer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub initializer_deposit_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub initializer_receive_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub escrow_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeEscrow<'info> {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.initializer),
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.initializer_deposit_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.initializer_receive_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow_account),
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.token_program),
                    false));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeEscrow<'info>
        {
        fn to_account_infos(&self)
            ->
                Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer_deposit_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer_receive_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.token_program));
            account_infos
        }
    }
}
pub struct Exchange<'info> {
    #[account(signer)]
    /// CHECK: ...
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: ...
    pub initializer_main_account: AccountInfo<'info>,
    #[account(mut, constraint = escrow_account.taker_amount <=
    taker_deposit_token_account.amount, constraint =
    escrow_account.initializer_deposit_token_account == *
    pda_deposit_token_account.to_account_info().key, constraint =
    escrow_account.initializer_receive_token_account == *
    initializer_receive_token_account.to_account_info().key, constraint =
    escrow_account.initializer_key == * initializer_main_account.key, close =
    initializer_main_account)]
    pub escrow_account: Account<'info, EscrowAccount>,
    /// CHECK: ...
    pub pda_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Exchange<'info> where
    'info: 'info {
    #[inline(never)]
    fn try_accounts(__program_id:
            &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts:
            &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        __ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs:
            &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
        -> anchor_lang::Result<Self> {
        let taker: AccountInfo =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e| e.with_account_name("taker"))?;
        let taker_deposit_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("taker_deposit_token_account"))?;
        let taker_receive_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("taker_receive_token_account"))?;
        let pda_deposit_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("pda_deposit_token_account"))?;
        let initializer_receive_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer_receive_token_account"))?;
        let initializer_main_account: AccountInfo =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer_main_account"))?;
        let escrow_account:
                anchor_lang::accounts::account::Account<EscrowAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("escrow_account"))?;
        let pda_account: AccountInfo =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("pda_account"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("token_program"))?;
        if !taker.is_signer {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSigner).with_account_name("taker"));
            }
        if !taker_deposit_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("taker_deposit_token_account"));
            }
        if !taker_receive_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("taker_receive_token_account"));
            }
        if !pda_deposit_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("pda_deposit_token_account"));
            }
        if !initializer_receive_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("initializer_receive_token_account"));
            }
        if !initializer_main_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("initializer_main_account"));
            }
        if !escrow_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow_account"));
            }
        if !(escrow_account.taker_amount <=
                            taker_deposit_token_account.amount) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        if !(escrow_account.initializer_deposit_token_account ==
                            *pda_deposit_token_account.to_account_info().key) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        if !(escrow_account.initializer_receive_token_account ==
                            *initializer_receive_token_account.to_account_info().key) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        if !(escrow_account.initializer_key == *initializer_main_account.key)
                {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        {
            if escrow_account.key() == initializer_main_account.key() {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintClose).with_account_name("escrow_account"));
                }
        }
        Ok(Exchange {
                taker,
                taker_deposit_token_account,
                taker_receive_token_account,
                pda_deposit_token_account,
                initializer_receive_token_account,
                initializer_main_account,
                escrow_account,
                pda_account,
                token_program,
            })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Exchange<'info> where
    'info: 'info {
    fn to_account_infos(&self)
        ->
            Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.taker.to_account_infos());
        account_infos.extend(self.taker_deposit_token_account.to_account_infos());
        account_infos.extend(self.taker_receive_token_account.to_account_infos());
        account_infos.extend(self.pda_deposit_token_account.to_account_infos());
        account_infos.extend(self.initializer_receive_token_account.to_account_infos());
        account_infos.extend(self.initializer_main_account.to_account_infos());
        account_infos.extend(self.escrow_account.to_account_infos());
        account_infos.extend(self.pda_account.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Exchange<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>)
        -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.taker.to_account_metas(Some(true)));
        account_metas.extend(self.taker_deposit_token_account.to_account_metas(None));
        account_metas.extend(self.taker_receive_token_account.to_account_metas(None));
        account_metas.extend(self.pda_deposit_token_account.to_account_metas(None));
        account_metas.extend(self.initializer_receive_token_account.to_account_metas(None));
        account_metas.extend(self.initializer_main_account.to_account_metas(None));
        account_metas.extend(self.escrow_account.to_account_metas(None));
        account_metas.extend(self.pda_account.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Exchange<'info> where
    'info: 'info {
    fn exit(&self, program_id: &anchor_lang::solana_program::pubkey::Pubkey)
        -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.taker_deposit_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("taker_deposit_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.taker_receive_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("taker_receive_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.pda_deposit_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("pda_deposit_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.initializer_receive_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("initializer_receive_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.initializer_main_account,
                    program_id).map_err(|e|
                    e.with_account_name("initializer_main_account"))?;
        {
            let initializer_main_account = &self.initializer_main_account;
            anchor_lang::AccountsClose::close(&self.escrow_account,
                        initializer_main_account.to_account_info()).map_err(|e|
                        e.with_account_name("escrow_account"))?;
        }
        Ok(())
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a struct for a given"]
#[doc =
r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
#[doc = r" instead of an `AccountInfo`. This is useful for clients that want"]
#[doc = r" to generate a list of accounts, without explicitly knowing the"]
#[doc = r" order all the fields should be in."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" `accounts` module (also generated), which re-exports this."]
pub(crate) mod __client_accounts_exchange {
    use super::*;
    use anchor_lang::prelude::borsh;
    #[doc = " Generated client accounts for [`Exchange`]."]
    pub struct Exchange {
        pub taker: anchor_lang::solana_program::pubkey::Pubkey,
        pub taker_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub taker_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub initializer_main_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Exchange where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
        {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.taker, writer)?;
            borsh::BorshSerialize::serialize(&self.taker_deposit_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.taker_receive_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.pda_deposit_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.initializer_receive_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.initializer_main_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.escrow_account, writer)?;
            borsh::BorshSerialize::serialize(&self.pda_account, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Exchange {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.taker,
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.taker_deposit_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.taker_receive_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.pda_deposit_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.initializer_receive_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.initializer_main_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.pda_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program,
                    false));
            account_metas
        }
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a CPI struct for a given"]
#[doc = r" `#[derive(Accounts)]` implementation, where each field is an"]
#[doc = r" AccountInfo."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" [`cpi::accounts`] module (also generated), which re-exports this."]
pub(crate) mod __cpi_client_accounts_exchange {
    use super::*;
    #[doc = " Generated CPI struct of the accounts for [`Exchange`]."]
    pub struct Exchange<'info> {
        pub taker: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub taker_deposit_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub taker_receive_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pda_deposit_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub initializer_receive_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub initializer_main_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub escrow_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pda_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Exchange<'info> {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.taker),
                    true));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.taker_deposit_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.taker_receive_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.pda_deposit_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.initializer_receive_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.initializer_main_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.pda_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.token_program),
                    false));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Exchange<'info> {
        fn to_account_infos(&self)
            ->
                Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.taker));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.taker_deposit_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.taker_receive_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pda_deposit_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer_receive_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer_main_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pda_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.token_program));
            account_infos
        }
    }
}
pub struct CancelEscrow<'info> {
    /// CHECK: ...
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    /// CHECK: ...
    pub pda_account: AccountInfo<'info>,
    #[account(mut, constraint = escrow_account.initializer_key == *
    initializer.key, constraint =
    escrow_account.initializer_deposit_token_account == *
    pda_deposit_token_account.to_account_info().key, close = initializer)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CancelEscrow<'info> where
    'info: 'info {
    #[inline(never)]
    fn try_accounts(__program_id:
            &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts:
            &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        __ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs:
            &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
        -> anchor_lang::Result<Self> {
        let initializer: AccountInfo =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("initializer"))?;
        let pda_deposit_token_account:
                anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("pda_deposit_token_account"))?;
        let pda_account: AccountInfo =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("pda_account"))?;
        let escrow_account:
                anchor_lang::accounts::account::Account<EscrowAccount> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("escrow_account"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> =
            anchor_lang::Accounts::try_accounts(__program_id, __accounts,
                        __ix_data, __bumps,
                        __reallocs).map_err(|e|
                        e.with_account_name("token_program"))?;
        if !initializer.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("initializer"));
            }
        if !pda_deposit_token_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("pda_deposit_token_account"));
            }
        if !escrow_account.to_account_info().is_writable {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow_account"));
            }
        if !(escrow_account.initializer_key == *initializer.key) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        if !(escrow_account.initializer_deposit_token_account ==
                            *pda_deposit_token_account.to_account_info().key) {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("escrow_account"));
            }
        {
            if escrow_account.key() == initializer.key() {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintClose).with_account_name("escrow_account"));
                }
        }
        Ok(CancelEscrow {
                initializer,
                pda_deposit_token_account,
                pda_account,
                escrow_account,
                token_program,
            })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CancelEscrow<'info> where
    'info: 'info {
    fn to_account_infos(&self)
        ->
            Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.initializer.to_account_infos());
        account_infos.extend(self.pda_deposit_token_account.to_account_infos());
        account_infos.extend(self.pda_account.to_account_infos());
        account_infos.extend(self.escrow_account.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CancelEscrow<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>)
        -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.initializer.to_account_metas(None));
        account_metas.extend(self.pda_deposit_token_account.to_account_metas(None));
        account_metas.extend(self.pda_account.to_account_metas(None));
        account_metas.extend(self.escrow_account.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CancelEscrow<'info> where
    'info: 'info {
    fn exit(&self, program_id: &anchor_lang::solana_program::pubkey::Pubkey)
        -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.initializer,
                    program_id).map_err(|e|
                    e.with_account_name("initializer"))?;
        anchor_lang::AccountsExit::exit(&self.pda_deposit_token_account,
                    program_id).map_err(|e|
                    e.with_account_name("pda_deposit_token_account"))?;
        {
            let initializer = &self.initializer;
            anchor_lang::AccountsClose::close(&self.escrow_account,
                        initializer.to_account_info()).map_err(|e|
                        e.with_account_name("escrow_account"))?;
        }
        Ok(())
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a struct for a given"]
#[doc =
r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
#[doc = r" instead of an `AccountInfo`. This is useful for clients that want"]
#[doc = r" to generate a list of accounts, without explicitly knowing the"]
#[doc = r" order all the fields should be in."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" `accounts` module (also generated), which re-exports this."]
pub(crate) mod __client_accounts_cancel_escrow {
    use super::*;
    use anchor_lang::prelude::borsh;
    #[doc = " Generated client accounts for [`CancelEscrow`]."]
    pub struct CancelEscrow {
        pub initializer: anchor_lang::solana_program::pubkey::Pubkey,
        pub pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CancelEscrow where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize
        {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.initializer, writer)?;
            borsh::BorshSerialize::serialize(&self.pda_deposit_token_account,
                    writer)?;
            borsh::BorshSerialize::serialize(&self.pda_account, writer)?;
            borsh::BorshSerialize::serialize(&self.escrow_account, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CancelEscrow {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.initializer,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.pda_deposit_token_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.pda_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow_account,
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program,
                    false));
            account_metas
        }
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a CPI struct for a given"]
#[doc = r" `#[derive(Accounts)]` implementation, where each field is an"]
#[doc = r" AccountInfo."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" [`cpi::accounts`] module (also generated), which re-exports this."]
pub(crate) mod __cpi_client_accounts_cancel_escrow {
    use super::*;
    #[doc = " Generated CPI struct of the accounts for [`CancelEscrow`]."]
    pub struct CancelEscrow<'info> {
        pub initializer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pda_deposit_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pda_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub escrow_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CancelEscrow<'info> {
        fn to_account_metas(&self, is_signer: Option<bool>)
            -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.initializer),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.pda_deposit_token_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.pda_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow_account),
                    false));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.token_program),
                    false));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CancelEscrow<'info> {
        fn to_account_infos(&self)
            ->
                Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.initializer));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pda_deposit_token_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pda_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_account));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.token_program));
            account_infos
        }
    }
}
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}
impl borsh::ser::BorshSerialize for EscrowAccount where
    Pubkey: borsh::ser::BorshSerialize, Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize {
    fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
        -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.initializer_key, writer)?;
        borsh::BorshSerialize::serialize(&self.initializer_deposit_token_account,
                writer)?;
        borsh::BorshSerialize::serialize(&self.initializer_receive_token_account,
                writer)?;
        borsh::BorshSerialize::serialize(&self.initializer_amount, writer)?;
        borsh::BorshSerialize::serialize(&self.taker_amount, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for EscrowAccount where
    Pubkey: borsh::BorshDeserialize, Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize {
    fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
        -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
                initializer_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initializer_deposit_token_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initializer_receive_token_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initializer_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                taker_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EscrowAccount {
    #[inline]
    fn clone(&self) -> EscrowAccount {
        EscrowAccount {
            initializer_key: ::core::clone::Clone::clone(&self.initializer_key),
            initializer_deposit_token_account: ::core::clone::Clone::clone(&self.initializer_deposit_token_account),
            initializer_receive_token_account: ::core::clone::Clone::clone(&self.initializer_receive_token_account),
            initializer_amount: ::core::clone::Clone::clone(&self.initializer_amount),
            taker_amount: ::core::clone::Clone::clone(&self.taker_amount),
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for EscrowAccount {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
        -> anchor_lang::Result<()> {
        if writer.write_all(&[36, 69, 48, 18, 128, 225, 125, 135]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
        if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for EscrowAccount {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [36, 69, 48, 18, 128, 225, 125, 135].len() {
                return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
            }
        let given_disc = &buf[..8];
        if &[36, 69, 48, 18, 128, 225, 125, 135] != given_disc {
                return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                                    error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                                    error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.to_string(),
                                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                filename: "programs/escrow/src/lib.rs",
                                                line: 189u32,
                                            })),
                                    compared_values: None,
                                }).with_account_name("EscrowAccount"));
            }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8])
        -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data).map_err(|_|
                anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for EscrowAccount {
    const DISCRIMINATOR: [u8; 8] = [36, 69, 48, 18, 128, 225, 125, 135];
}
#[automatically_derived]
impl anchor_lang::Owner for EscrowAccount {
    fn owner() -> Pubkey { crate::ID }
}
impl EscrowAccount {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8;
}
impl<'info> From<&mut InitializeEscrow<'info>> for
    CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
    fn from(accounts: &mut InitializeEscrow<'info>) -> Self {
        let cpi_accounts =
            SetAuthority {
                account_or_mint: accounts.initializer_deposit_token_account.to_account_info().clone(),
                current_authority: accounts.initializer.to_account_info().clone(),
            };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'info> CancelEscrow<'info> {
    fn into_set_authority_context(&self)
        -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts =
            SetAuthority {
                account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
                current_authority: self.pda_account.clone(),
            };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'info> Exchange<'info> {
    fn into_set_authority_context(&self)
        -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts =
            SetAuthority {
                account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
                current_authority: self.pda_account.clone(),
            };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'info> Exchange<'info> {
    fn into_transfer_to_taker_context(&self)
        -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts =
            Transfer {
                from: self.pda_deposit_token_account.to_account_info().clone(),
                to: self.taker_receive_token_account.to_account_info().clone(),
                authority: self.pda_account.clone(),
            };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(&self)
        -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts =
            Transfer {
                from: self.taker_deposit_token_account.to_account_info().clone(),
                to: self.initializer_receive_token_account.to_account_info().clone(),
                authority: self.taker.clone(),
            };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
