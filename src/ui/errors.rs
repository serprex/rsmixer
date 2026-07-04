use thiserror::Error;

#[derive(Debug, Error)]
pub enum UIError {
    #[error("terminal window is too small")]
    TerminalTooSmall,
    #[error("terminal io error")]
    IoError(#[from] std::io::Error),
}
