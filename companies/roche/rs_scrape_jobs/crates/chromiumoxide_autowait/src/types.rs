use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    Visible,
    Stable,
    Enabled,
    Editable,
}

#[derive(Debug, Error)]
pub enum ActionabilityError {
    #[error("Element state missing: {0:?}")]
    MissingState(ElementState),
    #[error("Element is not connected to the DOM")]
    NotConnected,
    #[error("Timeout reached while waiting for actionability")]
    Timeout,
    #[error("Chromiumoxide protocol error: {0}")]
    ProtocolError(String),
}

pub struct AutoWaitOptions {
    pub timeout: Duration,
}

impl Default for AutoWaitOptions {
    fn default() -> Self {
        Self { timeout: Duration::from_secs(10) } // Default 10s timeout
    }
}
