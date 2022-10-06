use clap::error::{ContextKind, ContextValue, ErrorKind};
use inflector::Inflector;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Internal io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Internal error: no name in validation")]
    NoValidationName,
    #[error("Internal match error: {0}")]
    MatchError(clap::Error),
    #[error("Internal error: no child stdout or stderr")]
    NoStdoutOrStderr,
    #[error("Validation error in {}: '{}'", .name, .message)]
    ValidationError { name: String, message: String },
    #[error("{0}")]
    GuiError(String),
}

impl From<clap::Error> for ExecutionError {
    fn from(err: clap::Error) -> Self {
        match clap::Error::kind(&err) {
            ErrorKind::ValueValidation => {
                // TODO parse out the actual name of the option
                let invalid_arg = err.get(ContextKind::InvalidArg);
                let invalid_value = err.get(ContextKind::InvalidValue);
                if let (
                    Some(ContextValue::String(invalid_arg)),
                    Some(ContextValue::String(invalid_value)),
                ) = (invalid_arg, invalid_value)
                {
                    ExecutionError::ValidationError {
                        name: invalid_arg.to_sentence_case(),
                        message: invalid_value.clone(),
                    }
                } else {
                    ExecutionError::NoValidationName
                }
            }
            _ => ExecutionError::MatchError(err),
        }
    }
}

impl From<String> for ExecutionError {
    fn from(str: String) -> Self {
        Self::GuiError(str)
    }
}

impl From<&str> for ExecutionError {
    fn from(str: &str) -> Self {
        Self::GuiError(str.to_string())
    }
}
