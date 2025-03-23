pub mod afl;
pub mod constants;
pub mod honggfuzz;
use thiserror::Error;
use tokio::process::Child;

#[derive(Error, Debug, Clone)]
pub enum CoverageError {
    #[error("Cleaning of coverage directory failed")]
    CleaningFailed,
    #[error("Coverage report generation failed")]
    GeneratingReportFailed,
}

pub trait Coverage {
    fn new(cargo_target_dir: &str, fuzzer_loopcount: u64) -> Self
    where
        Self: Sized;

    fn get_profraw_file(&self) -> String;
    fn get_coverage_file(&self) -> String;
    fn get_coverage_target_dir(&self) -> String;
    fn get_ignore_regex(&self) -> String;
    fn get_rustflags(&self) -> String;
    fn get_fuzzer_loopcount(&self) -> String;

    async fn handle_child(child: &mut Child, error: CoverageError) -> Result<(), CoverageError> {
        tokio::select! {
            res = child.wait() => {
                if !res.map_err(|_| error.clone())?.success() {
                    return Err(error);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                if let Ok(status) = child.wait().await {
                    if !status.success() {
                        return Err(error);
                    }
                }
            }
        }
        Ok(())
    }

    async fn clean(&self) -> Result<(), CoverageError> {
        let profraw_list_path =
            std::path::Path::new(&self.get_coverage_target_dir()).join(constants::PROFRAW_LIST);
        if let Err(e) = tokio::fs::remove_file(profraw_list_path).await {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(CoverageError::CleaningFailed);
            }
        }

        let mut child = tokio::process::Command::new("cargo")
            .env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_coverage_target_dir())
            .arg("llvm-cov")
            .arg("clean")
            .arg("--workspace")
            .spawn()
            .map_err(|_| CoverageError::CleaningFailed)?;

        Self::handle_child(&mut child, CoverageError::CleaningFailed).await
    }
}
