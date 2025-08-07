use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template engine error: {0}")]
    Tera(#[from] tera::Error),
}
