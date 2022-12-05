use anchor_client::solana_client::client_error::ClientErrorKind;
use anchor_client::solana_client::rpc_request::RpcError::RpcResponseError;
use anchor_client::solana_client::rpc_request::RpcResponseErrorData;
use anchor_client::solana_client::rpc_response::RpcSimulateTransactionResult;
use anchor_client::ClientError;
use anyhow::Error;

trait ErrorReporter<T> {
    fn report(error: &T);
}

impl ErrorReporter<ClientError> for ClientError {
    fn report(error: &ClientError) {
        if let ClientError::SolanaClientError(err) = error {
            if let ClientErrorKind::RpcError(RpcResponseError {
                data,
                code,
                message,
            }) = err.kind()
            {
                let formatted_data = match data {
                    RpcResponseErrorData::SendTransactionPreflightFailure(
                        RpcSimulateTransactionResult {
                            logs: Some(logs), ..
                        },
                    ) => logs.join("\n   "),
                    _ => "".to_string(),
                };
                println!("RpcResponseError [{code}]: {message}\n{formatted_data}");
            }
        }
    }
}

pub fn report_error(error: &Error) {
    if let Some(err) = error.downcast_ref::<ClientError>() {
        ClientError::report(err);
    };
}
