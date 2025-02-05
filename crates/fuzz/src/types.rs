pub type AccountId = u8;
pub type FuzzerData<'a> = arbitrary::Unstructured<'a>;
pub type SequenceResult<T> = arbitrary::Result<Vec<T>>;
