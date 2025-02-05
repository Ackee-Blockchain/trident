#[macro_export]
macro_rules! sequence {
    // Handle array of instructions with explicit context parameters
    ([$($ix_variant:ident),+ $(,)?], $fuzzer_data:expr) => {{
        let mut batch = Vec::new();
        $(
            batch.push(FuzzTransactions::$ix_variant($ix_variant::arbitrary($fuzzer_data)?));
        )*
        batch
    }};
}
