use crate::commander::Error as CommanderError;
use crate::idl_loader::IdlError;
use std::io;
use std::num::ParseIntError;
use std::path::StripPrefixError;
use thiserror::Error;

use trident_template::error::TemplateError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    StripPrefix(#[from] StripPrefixError),
    #[error("{0:?}")]
    TridentVersionsConfig(#[from] serde_json::Error),
    #[error("{0:?}")]
    ParseInt(#[from] ParseIntError),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
    #[error("{0:?}")]
    Commander(#[from] CommanderError),
    #[error("The Anchor project does not contain any programs")]
    NoProgramsFound,
    #[error("parsing Cargo.toml dependencies failed")]
    ParsingCargoTomlDependenciesFailed,
    #[error("{0:?}")]
    TemplateEngine(#[from] TemplateError),
    #[error("{0}")]
    IdlLoader(#[from] IdlError),
}
