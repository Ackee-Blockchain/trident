use syn::{parse_quote, parse_str};
use trident_idl_spec::IdlInstruction;

use crate::{template::ModDefinition, Template};

impl Template {
    pub(crate) fn transaction(&mut self, instruction: &IdlInstruction) {
        let instruction_name = self.get_camel_identifier(instruction);

        let transaction_name = format!("{}Transaction", instruction_name);
        let instruction_name = format!("{}Instruction", instruction_name);

        let transaction_struct_name: syn::Ident = parse_str(&transaction_name).unwrap();
        let instruction_struct_name: syn::Ident = parse_str(&instruction_name).unwrap();

        let transaction: syn::ItemStruct = parse_quote! {
            /// Customize transaction behavior by adding more instructions.
            ///
            /// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
            #[derive(Arbitrary, Debug, TridentTransaction)]
            pub struct #transaction_struct_name {
                pub instruction: #instruction_struct_name,
            }
        };

        let transaction_custom_methods_impl: syn::ItemImpl = parse_quote! {
        /// Methods for customizing transaction behavior:
        /// - `pre_transaction`: Execute custom logic before transaction execution
        /// - `transaction_invariant_check`: Validate transaction-specific invariants
        /// - `transaction_error_handler`: Custom handling of transaction errors
        /// - `post_transaction`: Execute custom logic after transaction execution
        ///
        /// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
            impl TransactionHooks for #transaction_struct_name {
                type IxAccounts = FuzzAccounts;
            }
        };

        let transaction_file: syn::File = parse_quote! {
            use crate::fuzz_transactions::FuzzAccounts;
            use crate::instructions::*;
            use trident_fuzz::fuzzing::*;

            #transaction

            #transaction_custom_methods_impl
        };

        let transaction_file_name = self.get_snake_identifier(instruction);

        self.transactions
            .insert(transaction_file_name, transaction_file);

        self.transaction_mod(instruction);
    }

    pub(crate) fn transaction_mod(&mut self, instruction: &IdlInstruction) {
        let transaction_file_name = self.get_snake_identifier(instruction);
        let transaction_file_name_ident: syn::Ident = parse_str(&transaction_file_name).unwrap();

        let transactions_mod: syn::ItemMod = parse_quote!(
            pub mod #transaction_file_name_ident;
        );

        let transactions_use_statement: syn::ItemUse = parse_quote!(
            pub use #transaction_file_name_ident::*;
        );

        self.transactions_mod.push(ModDefinition {
            module: transactions_mod,
            use_statement: transactions_use_statement,
        });
    }
}
