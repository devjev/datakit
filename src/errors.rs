use crate::value::constraint::*;
use crate::value::definition::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An error that represents a single instance of a failed Value validation
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub enum ConstraintError {
    #[error("Encountered unexpected value type")]
    TypeError {
        expected: ValueType,
        received: ValueType,
    },

    #[error("Value constraint violated")]
    InvalidValueError(ValueConstraint),

    #[error("Constraint inapplicable")]
    InvalidConstraintError, // TODO add constraint info
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub enum ValidationError {
    #[error("Value violates constraint(s)")]
    ValueValidationError {
        offending_value: Value,
        failed_constraints: Vec<ConstraintError>,
    },
}

// TODO convert to proper Errors
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub enum CoercionError {
    #[error("Impossible coercion between types")]
    CoercionImpossible { from: ValueType, to: ValueType },

    #[error("Cannot coerce to value")]
    CoercionFailed {
        target_type: ValueType,
        source_text: String,
    },

    #[error("Unexpected type")]
    UnexpectedType,

    #[error("Domain error")] // TODO elaborate on that
    DomainError(String),
}
