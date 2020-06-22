use crate::value::constraint::*;
use crate::value::definition::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// An error that represents a single instance of a failed Value validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConstraintError {
    TypeError {
        expected: ValueType,
        received: ValueType,
    },
    ValueError(ValueConstraint),
    InvalidConstraintError, // TODO add constraint info
}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintError::TypeError { expected, received } => write!(
                f,
                r#"Expected type: `{:?}`, received type: `{:?}`"#,
                expected, received
            ),
            ConstraintError::ValueError(constraint) => {
                write!(f, "Failed value constraint {:?}", constraint)
            }
            ConstraintError::InvalidConstraintError => write!(f, "Invalid constraint"),
        }
    }
}

impl Error for ConstraintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueValidationError {
    pub offending_value: Value,
    pub failed_constraints: Vec<ConstraintError>,
}

impl std::fmt::Display for ValueValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value `{:?}` is invalid: {:?}",
            self.offending_value, self.failed_constraints
        )
    }
}

impl Error for ValueValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// TODO convert to proper Errors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueConversionError {
    ConversionUnavailable {
        from: ValueType,
        to: ValueType,
    },
    UnexpectedType,
    ParseError {
        target_type: ValueType,
        source_text: String,
    },
    DomainError(String),
}
