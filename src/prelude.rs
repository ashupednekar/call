/*use thiserror::Error;
#[derive(Debug, Error)]
pub enum CallError{
    #[error("unexpected")]
    Unexpected
}*/

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
