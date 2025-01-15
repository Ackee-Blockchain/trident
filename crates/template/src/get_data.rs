use quote::format_ident;
use std::collections::HashMap;
use syn::{parse_quote, Block};

use trident_idl_spec::{IdlField, IdlInstruction, IdlType};

use crate::instruction_account::InstructionAccount;

pub(crate) fn get_data(
    instruction: &IdlInstruction,
    _instructions_accounts: &HashMap<String, InstructionAccount>,
) -> Vec<syn::Block> {
    let mut args_implementations = vec![];

    for argument in &instruction.args {
        match argument.ty {
            IdlType::Pubkey => {
                process_pubkey(argument, &mut args_implementations);
            }
            IdlType::PublicKey => {
                process_pubkey(argument, &mut args_implementations);
            }
            _ => {
                process_argument(argument, &mut args_implementations);
            }
        }
    }

    args_implementations
}

fn process_pubkey(argument: &IdlField, arguments_implementation: &mut Vec<Block>) {
    let argument_name = &argument.name;
    let argument_ident = format_ident!("{}", argument_name);

    // TODO: maybe generated more code so it is easier to fill
    let argument_implementation = parse_quote!({
        let #argument_ident: Pubkey = todo!();
        args.extend(borsh::to_vec(&#argument_ident).unwrap());
    });

    arguments_implementation.push(argument_implementation);
}

fn process_argument(argument: &IdlField, arguments_implementation: &mut Vec<Block>) {
    let argument_name = &argument.name;
    let argument_ident = format_ident!("{}", argument_name);

    // Take argument and append it to vector
    let other_arg = parse_quote!({
        args.extend(borsh::to_vec(&self.data.#argument_ident).unwrap());
    });
    arguments_implementation.push(other_arg);
}
