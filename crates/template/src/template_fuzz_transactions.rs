use crate::Template;

use syn::{parse_quote, parse_str};
use trident_idl_spec::{
    IdlInstruction, IdlInstructionAccount, IdlInstructionAccountItem, IdlInstructionAccounts,
};

impl Template {
    pub(crate) fn account_storage(&mut self, instruction: &IdlInstruction) {
        instruction
            .accounts
            .iter()
            .for_each(|account| match account {
                IdlInstructionAccountItem::Single(idl_instruction_account) => {
                    self.process_single_account(idl_instruction_account);
                }
                IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                    self.process_composite_account(idl_instruction_accounts);
                }
            });
    }
    pub(crate) fn process_single_account(
        &mut self,
        idl_instruction_account: &IdlInstructionAccount,
    ) {
        let account_name = idl_instruction_account.name.clone();
        let account_name_ident: syn::Ident = parse_str(&account_name).unwrap();

        let account_item = parse_quote!(
            pub #account_name_ident: AccountsStorage
        );

        self.account_storages.insert(account_name, account_item);
    }
    pub(crate) fn process_composite_account(
        &mut self,
        idl_instruction_accounts: &IdlInstructionAccounts,
    ) {
        idl_instruction_accounts
            .accounts
            .iter()
            .for_each(|account| match account {
                IdlInstructionAccountItem::Single(idl_instruction_account) => {
                    self.process_single_account(idl_instruction_account);
                }
                IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                    self.process_composite_account(idl_instruction_accounts);
                }
            });
    }

    pub(crate) fn fuzz_transaction_variant(&mut self, instruction: &IdlInstruction) {
        let instruction_name = self.get_camel_identifier(instruction);

        let transaction_name = format!("{}Transaction", instruction_name);
        let transaction_struct_name: syn::Ident = parse_str(&transaction_name).unwrap();

        let variant: syn::Variant = parse_quote! {
            #transaction_struct_name(#transaction_struct_name)
        };

        self.fuzz_transactions.push(variant);
    }
}
