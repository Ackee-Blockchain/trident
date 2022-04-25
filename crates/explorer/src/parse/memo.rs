use crate::parse::{ParsableProgram, ParseInstructionError};
use serde_json::Value;
use solana_sdk::instruction::CompiledInstruction;
use std::str::{from_utf8, Utf8Error};

pub fn parse_memo(instruction: &CompiledInstruction) -> Result<Value, ParseInstructionError> {
    parse_memo_data(&instruction.data)
        .map(Value::String)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::SPLMemo))
}

pub fn parse_memo_data(data: &[u8]) -> Result<String, Utf8Error> {
    from_utf8(data).map(|s| s.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_memo() {
        let good_memo = "good memo".to_string();
        assert_eq!(
            parse_memo(&CompiledInstruction {
                program_id_index: 0,
                accounts: vec![],
                data: good_memo.as_bytes().to_vec(),
            })
            .unwrap(),
            Value::String(good_memo),
        );

        let bad_memo = vec![128u8];
        assert!(std::str::from_utf8(&bad_memo).is_err());
        assert!(parse_memo(&CompiledInstruction {
            program_id_index: 0,
            data: bad_memo,
            accounts: vec![],
        })
        .is_err(),);
    }
}
