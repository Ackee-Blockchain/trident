#![allow(dead_code)]

use solana_banks_client::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FuzzClientError {
    #[error("Custom fuzzing error: {0}")]
    Custom(u32),
    #[error("Not able to initialize client: {0}")]
    ClientInitError(#[from] std::io::Error),
    // Box for Error variant too Long warnings
    #[error("Banks Client Error: {0}")]
    BanksError(Box<BanksClientError>),
}

#[derive(Debug, Error)]
pub enum FuzzingError {
    #[error("Custom fuzzing error: {0}\n")]
    Custom(u32),
    #[error("Not able to deserialize account: {0}\n")]
    CannotDeserializeAccount(String),
    #[error("Optional Account not provided: {0}\n")]
    OptionalAccountNotProvided(String),
    #[error("Not enough Accounts: {0}\n")]
    NotEnoughAccounts(String),
    #[error("Account not Found: {0}\n")]
    AccountNotFound(String),
    #[error("Not Able To Obtain AccountInfos\n")]
    NotAbleToObtainAccountInfos,
    #[error("Balance Mismatch\n")]
    BalanceMismatch,
    #[error("Data Mismatch\n")]
    DataMismatch,
    #[error("Unable to obtain Data\n")]
    UnableToObtainData,
}

impl From<BanksClientError> for FuzzClientError {
    fn from(value: BanksClientError) -> Self {
        Self::BanksError(Box::new(value))
    }
}

impl FuzzClientError {
    pub fn with_origin(self, origin: Origin) -> FuzzClientErrorWithOrigin {
        let mut error_with_origin = FuzzClientErrorWithOrigin::from(self);
        error_with_origin.origin = Some(origin);
        error_with_origin
    }
    pub fn with_context(self, context: Context) -> FuzzClientErrorWithOrigin {
        let mut error_with_origin = FuzzClientErrorWithOrigin::from(self);
        error_with_origin.context = Some(context);
        error_with_origin
    }
}

impl FuzzingError {
    pub fn with_origin(self, origin: Origin) -> FuzzingErrorWithOrigin {
        let mut error_with_origin = FuzzingErrorWithOrigin::from(self);
        error_with_origin.origin = Some(origin);
        error_with_origin
    }
    pub fn with_context(self, context: Context) -> FuzzingErrorWithOrigin {
        let mut error_with_origin = FuzzingErrorWithOrigin::from(self);
        error_with_origin.context = Some(context);
        error_with_origin
    }
}

#[derive(Debug)]
pub enum Origin {
    Instruction(String),
    Account(Pubkey),
}

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Origin: {:#?}", self)
    }
}

#[derive(Debug)]
pub enum Context {
    Pre,
    Post,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Context: {:#?}", self)
    }
}

#[derive(Debug)]
pub struct FuzzClientErrorWithOrigin {
    pub client_error: FuzzClientError,
    pub origin: Option<Origin>,
    pub context: Option<Context>,
}

#[derive(Debug)]
pub struct FuzzingErrorWithOrigin {
    pub fuzzing_error: FuzzingError,
    pub origin: Option<Origin>,
    pub context: Option<Context>,
}

impl From<FuzzClientError> for FuzzClientErrorWithOrigin {
    fn from(client_error: FuzzClientError) -> Self {
        Self {
            client_error,
            origin: None,
            context: None,
        }
    }
}

impl From<FuzzingError> for FuzzingErrorWithOrigin {
    fn from(fuzzing_error: FuzzingError) -> Self {
        Self {
            fuzzing_error,
            origin: None,
            context: None,
        }
    }
}
impl Display for FuzzClientErrorWithOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.client_error, f)?;
        if let Some(o) = &self.origin {
            Display::fmt(o, f)?;
        }
        if let Some(c) = &self.context {
            Display::fmt(c, f)?;
        }
        Ok(())
    }
}
impl Display for FuzzingErrorWithOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.fuzzing_error, f)?;
        if let Some(o) = &self.origin {
            Display::fmt(o, f)?;
        }
        if let Some(c) = &self.context {
            Display::fmt(c, f)?;
        }
        Ok(())
    }
}

impl FuzzClientErrorWithOrigin {
    pub fn with_origin(mut self, origin: Origin) -> Self {
        self.origin = Some(origin);
        self
    }
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}
impl FuzzingErrorWithOrigin {
    pub fn with_origin(mut self, origin: Origin) -> Self {
        self.origin = Some(origin);
        self
    }
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}
