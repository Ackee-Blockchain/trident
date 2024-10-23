#[macro_export]
macro_rules! pre_sequence {
    // Accept a list of FuzzInstruction variants using parentheses `()`
    ($($ix_variant:ident),* $(,)?) => {
        fn pre_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
            let mut instructions = Vec::new();
            $(
                let ix = FuzzInstruction::$ix_variant($ix_variant::arbitrary(_u)?);
                instructions.push(ix);
            )*
            Ok(instructions)
        }
    };
}

#[macro_export]
macro_rules! middle_sequence {
    // Accept a list of FuzzInstruction variants (which may include duplicates)
    ($($ix_variant:ident),* $(,)?) => {
        fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
            #[allow(unused_mut)]
            let mut instructions = Vec::new();
            $(
                let ix = FuzzInstruction::$ix_variant($ix_variant::arbitrary(_u)?);
                instructions.push(ix);
            )*
            Ok(instructions)
        }
    };
}

#[macro_export]
macro_rules! post_sequence {
    // Accept a list of FuzzInstruction variants (which may include duplicates)
    ($($ix_variant:ident),* $(,)?) => {
        fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
            #[allow(unused_mut)]
            let mut instructions = Vec::new();
            $(
                let ix = FuzzInstruction::$ix_variant($ix_variant::arbitrary(_u)?);
                instructions.push(ix);
            )*
            Ok(instructions)
        }
    };
}
