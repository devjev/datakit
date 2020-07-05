use crate::value::constraints::*;
use crate::value::definitions::*;
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

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub enum CoercionError {
    #[error("Impossible coercion between types")]
    CoercionImpossible { from: ValueType, to: ValueType },

    #[error("Cannot coerce value")]
    CoercionFailed {
        target_type: ValueType,
        source_value: Value,
    },

    #[error("Unexpected type")]
    UnexpectedType,

    #[error("Domain error")] // TODO elaborate on that
    DomainError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub enum ParsingError {
    #[error("Parsing failed")]
    CannotParseValue(String),
}
