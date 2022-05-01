use fehler::throws;
use std::env::current_dir;
use std::fmt::format;
use std::path::{Path, PathBuf};
use serde::Serialize;
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value, ser::Serializer};

const TESTS_DIRECTORY: &str = "tests";

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
    #[error("cannot initialize dev dependencies")]
    CannotInitDevDeps,
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
    #[error("{0:?}")]
    Io(#[from] std::io::Error),
}

pub struct TestGenerator {
    path: PathBuf,
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            path: current_dir().unwrap(),
        }
    }

    #[throws]
    pub async fn generate(&self) {
        self.check_workspace().await?;
        self.generate_test_files().await?;
        self.add_dev_dependencies().await?;
    }

    #[throws]
    async fn generate_test_files(&self) {
        let path = Path::new(&self.path).join(TESTS_DIRECTORY);
        if fs::metadata(&path).await.is_ok() {
            return;
        }

        fs::create_dir(&path).await?;
        fs::write(path.join("test.rs"), "").await?;
    }

    #[throws]
    async fn add_dev_dependencies(&self) {
        let cargo_toml = Path::new(&self.path).join("Cargo.toml");
        let mut cargo_toml_content: Value = fs::read_to_string(&cargo_toml).await?.parse()?;
        let mut dev_deps = cargo_toml_content
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("dev-dependencies")
            .or_insert(Value::Table(Table::default()))
            .as_table_mut()
            .ok_or(Error::CannotInitDevDeps)?;
        dev_deps = self.initialize_dev_deps(dev_deps);
        println!("{:?}\n\n", dev_deps);
        println!("{:?}", cargo_toml_content.to_string());
        // fs::write(cargo_toml, cargo_toml_content.to_string()).await?;
    }

    fn initialize_dev_deps<'a>(&'a self, dev_deps: &'a mut Table) -> &mut Table {
        [
            r#"fehler = "1.0.0""#,
            r#"trdelnik-client = { path = "../../../../crates/client" }"#,
            r#"program_client = { path = "../../program_client" }"#,
        ]
            .map(|dependency| {
                if let Value::Table(table) = dependency.parse().unwrap() {
                    let (name, value) = table.into_iter().next().unwrap();
                    dev_deps.entry(name).or_insert(value.clone());
                }
            });
        dev_deps
    }

    #[throws]
    async fn check_workspace(&self) {
        // todo: throw error if the workspace is not valid
        // throw!(Error::BadWorkspace);
    }
}
