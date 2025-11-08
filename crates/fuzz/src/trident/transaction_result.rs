use solana_sdk::transaction::TransactionError;
use trident_svm::processor::InstructionError;

/// Result of a transaction execution containing both the result and logs
///
/// This struct encapsulates the outcome of executing a transaction,
/// including whether it succeeded or failed, and any logs generated
/// during execution.
pub struct TransactionResult {
    transaction_result: solana_sdk::transaction::Result<()>,
    transaction_logs: Vec<String>,
}

impl TransactionResult {
    /// Creates a new TransactionResult (internal use only)
    ///
    /// # Arguments
    /// * `transaction_result` - The result of the transaction execution
    /// * `transaction_logs` - Logs generated during transaction execution
    pub(crate) fn new(
        transaction_result: solana_sdk::transaction::Result<()>,
        transaction_logs: Vec<String>,
    ) -> Self {
        Self {
            transaction_result,
            transaction_logs,
        }
    }

    /// Returns true if the transaction executed successfully
    ///
    /// # Returns
    /// `true` if the transaction completed without errors, `false` otherwise
    pub fn is_success(&self) -> bool {
        self.transaction_result.is_ok()
    }

    /// Returns true if the transaction failed with an error
    ///
    /// # Returns
    /// `true` if the transaction failed, `false` if it succeeded
    pub fn is_error(&self) -> bool {
        self.transaction_result.is_err()
    }

    /// Returns the transaction logs
    ///
    /// Gets all log messages generated during transaction execution,
    /// including program logs and system messages.
    ///
    /// # Returns
    /// A slice of log message strings
    pub fn logs(&self) -> &[String] {
        &self.transaction_logs
    }
    /// Returns the raw transaction result
    ///
    /// Gets the underlying Solana transaction result, which contains
    /// detailed error information if the transaction failed.
    ///
    /// # Returns
    /// A reference to the transaction result
    pub fn get_result(&self) -> &solana_sdk::transaction::Result<()> {
        &self.transaction_result
    }
    /// Extracts the custom error code if the transaction failed with a custom error
    ///
    /// If the transaction failed due to a program's custom error, this method
    /// returns the numeric error code. Returns None for other error types.
    ///
    /// # Returns
    /// Some(error_code) if the transaction failed with a custom error, None otherwise
    pub fn get_custom_error_code(&self) -> Option<u32> {
        self.transaction_result
            .as_ref()
            .err()
            .and_then(|result| match result {
                TransactionError::InstructionError(
                    _error_code,
                    InstructionError::Custom(error_code),
                ) => Some(*error_code),
                _ => None,
            })
    }
    /// Checks if the transaction failed with a specific custom error code
    ///
    /// This is a convenience method to check if the transaction failed
    /// with a particular program-defined error code.
    ///
    /// # Arguments
    /// * `error_code` - The expected custom error code
    ///
    /// # Returns
    /// `true` if the transaction failed with the specified custom error code
    pub fn is_custom_error_with_code(&self, error_code: u32) -> bool {
        self.get_custom_error_code() == Some(error_code)
    }
}
