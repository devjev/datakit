use serde::{Deserialize, Serialize};

/// *Primitive*: A type for rich null values.
///
/// Differentiates between missing/empty data that is missing as expected
/// and data that is missing due to some error.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Empty {
    Unexpected,
    Expected,
}

/// *Primitive*: Numeric value type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Numeric {
    Integer(i64),
    Real(f64),
    Complex(f64, f64),
}
