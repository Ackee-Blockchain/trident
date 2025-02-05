#[macro_export]
macro_rules! pre_sequence {
    // Handle mix of single instructions and arrays
    ($($element:tt),* $(,)?) => {
        fn pre_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<FuzzInstruction>>> {
            let mut instructions = Vec::new();
            $(
                match_element!($element, instructions, _u);
            )*
            Ok(instructions)
        }
    };
}

#[macro_export]
macro_rules! match_element {
    // Handle single instruction
    ($ix_variant:ident, $instructions:ident, $u:ident) => {
        let ix = FuzzInstruction::$ix_variant($ix_variant::arbitrary($u)?);
        $instructions.push(TransactionInstructions { instructions: vec![ix] });
    };
    // Handle array of instructions
    ([$($ix_variant:ident),+ $(,)?], $instructions:ident, $u:ident) => {
        let mut batch = Vec::new();
        $(
            batch.push(FuzzInstruction::$ix_variant($ix_variant::arbitrary($u)?));
        )*
        $instructions.push(TransactionInstructions { instructions: batch });
    };
}

#[macro_export]
macro_rules! middle_sequence {
    ($($element:tt),* $(,)?) => {
        fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<FuzzInstruction>>> {
            #[allow(unused_mut)]
            let mut instructions = Vec::new();
            $(
                match_element!($element, instructions, _u);
            )*
            Ok(instructions)
        }
    };
}

#[macro_export]
macro_rules! post_sequence {
    ($($element:tt),* $(,)?) => {
        fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<FuzzInstruction>>> {
            #[allow(unused_mut)]
            let mut instructions = Vec::new();
            $(
                match_element!($element, instructions, _u);
            )*
            Ok(instructions)
        }
    };
}
