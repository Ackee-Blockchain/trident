use crate::___private::{Commander, Error, LocalnetHandle};
use fehler::throws;
use log::debug;
use std::{borrow::Cow, mem};

/// `Tester` is used primarily by [`#[trident_test]`](trident_test::trident_test) macro.
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
        let commander = Commander::with_root(mem::take(&mut self.root));
        commander.start_localnet().await?
    }

    #[throws]
    pub async fn after(&self, localnet_handle: LocalnetHandle) {
        debug!("____ AFTER TEST ____");
        localnet_handle.stop_and_remove_ledger().await?;
        debug!("_____________________");
    }
}
