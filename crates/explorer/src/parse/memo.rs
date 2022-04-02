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
