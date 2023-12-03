use crate::{commander::Error, Commander, LocalnetHandle};
use fehler::throws;
use log::debug;
use std::borrow::Cow;

/// `Tester` is used primarily by [`#[trdelnik_test]`](trdelnik_test::trdelnik_test) macro.
///
/// There should be no need to use `Tester` directly.
#[derive(Default)]
pub struct Tester {
    root: Cow<'static, str>,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            root: "../../".into(),
        }
    }

    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self { root: root.into() }
    }

    #[throws]
    pub async fn before(&mut self) -> LocalnetHandle {
        debug!("_____________________");
        debug!("____ BEFORE TEST ____");
        Commander::start_localnet(&self.root.to_string()).await?
    }

    #[throws]
    pub async fn after(&self, localnet_handle: LocalnetHandle) {
        debug!("____ AFTER TEST ____");
        localnet_handle.stop_and_remove_ledger().await?;
        debug!("_____________________");
    }
}
