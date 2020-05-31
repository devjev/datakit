//! Dynamic runtime values.
//!
//! The purpose of this library is to model data that is being input by the user. As such,
//! user input values should have:
//!
//! - A dynamic type (we don't want to error out on type errors at input),
//! - A way to validate the input type and any other contract/scheme defined by the
//!   application (we want to notify the user of any invalid input), and
//! - A way to gracefully handle missing or empty values (see "Rich Null Type" below).
//!
//! # Rich Null Type
//!
//! We take a different approach to handling missing or null values in this crate.
//! The common approaches is to either allow or disallow the use of nulls. However,
//! we recognize that real world data is messy and sometimes, even though we wouldn't
//! expect a value to be missing, it still does.
//!
//! To handle such cases with transparency and explicity, this crate differentiates between
//! expected and unexpected missing values. This gives us more information about the quality
//! of the data and allows clearer data handling and data cleaning routines.
//!
//! # TODO
//!
//! 1. `Empty::Unexpected` should wrap an Error and Display type.
//! 2. ~~`TypeContract` - added a macro-based implementation of values and value types.~~
//! 3. `OneOfContract`
//! 4. `NumericRangeContract`
//! 5. Combinations of Values, like Addition, Multiplication, etc.
//!

use chrono::{DateTime, FixedOffset, Local, Utc};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::error::Error;

// Errors --------------------------------------------------------------------

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

// Traits --------------------------------------------------------------------

/// Validates values.
pub trait ValidatesValues {
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError>;
}

// Value and ValueType implementation macros ---------------------------------

macro_rules! value_types {
    ( $( $i:ident($t:ty) ),+ ) => {
        /// Dynamic runtime value.
        ///
        ///
        #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Value {
            $(
                $i($t),
            )+
        }

        /// The type of a dynamic runtime value.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum ValueType {
            $(
                $i,
            )+
        }

        impl Value {
            pub fn is_of_type(&self, value_type: &ValueType) -> bool {
               match self {
                   $(
                       Value::$i(_) => ValueType::$i == *value_type,
                   )+
               }
            }

            /// Returns the value type of the value.
            pub fn get_value_type(&self) -> &ValueType {
                match self {
                    $(
                        Value::$i(_) => &ValueType::$i,
                    )+
                }
            }
        }

    };
}

macro_rules! impl_from_t_to_value {
    ( $( $type:ty => $fun:expr ),+ ) => {

        $(
            impl From<$type> for Value {
                fn from(x: $type) -> Self {
                    $fun(&x)
                }
            }
        )+
    };
}

#[allow(unused_macros)]
macro_rules! impl_from_value_to_t_option {
    ( $( $type:ty => $p:pat => $exp:expr ),+ ) => {

        $(
            impl From<Value> for Option<$type> {
                fn from(value: Value) -> Self {
                    match value {
                        Value::Missing(_) => None,
                        $p => Some($exp),
                        _ => None,
                    }
                }
            }
        )+

    };
}

// Types ---------------------------------------------------------------------

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

value_types! {
    Number(Numeric),
    Text(String),
    DateTime(DateTime<Utc>),
    Missing(Empty)
}

impl_from_t_to_value! {
    i32 => |value: &i32| { Value::Number(Numeric::Integer(value.clone() as i64)) },
    i64 => |value: &i64| { Value::Number(Numeric::Integer(value.clone())) },
    f32 => |value: &f32| { Value::Number(Numeric::Real(value.clone() as f64))},
    f64 => |value: &f64| { Value::Number(Numeric::Real(value.clone())) },
    (f64, f64) => |value: &(f64, f64)| {
        let real = value.0;
        let imaginary = value.1;
        Value::Number(Numeric::Complex(real, imaginary))
    },
    String => |value: &String| {
        if value.len() == 0 {
            Value::Missing(Empty::Unexpected)
        } else {
            Value::Text(value.clone())
        }
    },
    &str => |value: &&str| {
        let contents = String::from(*value);
        Value::Text(contents)
    },
    DateTime<Utc> => |value: &DateTime<Utc>| {
        Value::DateTime(value.clone())
    },
    DateTime<Local> => |value: &DateTime<Local>| {
        let utc_time = value.with_timezone(&Utc);
        Value::DateTime(utc_time)
    },
    DateTime<FixedOffset> => |value: &DateTime<FixedOffset>| {
        let utc_time = value.with_timezone(&Utc);
        Value::DateTime(utc_time)
    }
}

impl_from_value_to_t_option! {
    i32 => Value::Number(Numeric::Integer(x)) => x as i32,
    i64 => Value::Number(Numeric::Integer(x)) => x,
    f32 => Value::Number(Numeric::Real(r)) => r as f32,
    f64 => Value::Number(Numeric::Real(r)) => r,
    String => Value::Text(text) => text,
    DateTime<Utc> => Value::DateTime(t) => t,
    DateTime<Local> => Value::DateTime(t) => t.with_timezone(&Local)
    // TODO &str and DateTime<FixedOffset>
}

// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TypeConstraint {
    IsType(ValueType),
}

impl ValidatesValues for TypeConstraint {
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError> {
        match (self, value.get_value_type()) {
            (TypeConstraint::IsType(expected), received) => {
                if expected == received {
                    Ok(())
                } else {
                    Err(ValueValidationError {
                        offending_value: value.clone(),
                        failed_constraints: vec![ConstraintError::TypeError {
                            expected: expected.clone(),
                            received: received.clone(),
                        }],
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueConstraint {
    Any,
    Not(Box<ValueConstraint>),
    OneOf(Vec<Value>),
    Maximum(Value),
    Minimum(Value),
    MaximumLength(usize),
    MinimumLength(usize),
}

macro_rules! _to_valueconstraint_err {
    ( $($value:expr, $constraint:expr)? ) => {
        $(
            Err(ValueValidationError {
                offending_value: $value.clone(),
                failed_constraints: vec![
                    ConstraintError::ValueError($constraint.clone())
                ]
            })
        )?
    };
}

impl ValidatesValues for ValueConstraint {
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError> {
        match (self, value) {
            (ValueConstraint::Any, _) => Ok(()),
            (ValueConstraint::Not(c), v) => match c.validate(v) {
                Ok(()) => _to_valueconstraint_err!(v, self),
                Err(_) => Ok(()),
            },
            (ValueConstraint::OneOf(allowed_values), v) => {
                for allowed in allowed_values.iter() {
                    if v != allowed {
                        return _to_valueconstraint_err!(v, self);
                    }
                }
                Ok(())
            }
            (ValueConstraint::Maximum(mv), v) => {
                if v <= mv {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(v, self)
                }
            }
            (ValueConstraint::Minimum(mv), v) => {
                if v >= mv {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(v, self)
                }
            }
            (ValueConstraint::MaximumLength(len), Value::Text(text)) => {
                if text.len() <= *len {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(value.clone(), self)
                }
            }
            (ValueConstraint::MinimumLength(len), Value::Text(text)) => {
                if text.len() >= *len {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(value.clone(), self)
                }
            }
            (ValueConstraint::MaximumLength(_), _) => Err(ValueValidationError {
                offending_value: value.clone(),
                failed_constraints: vec![ConstraintError::InvalidConstraintError],
            }),
            (ValueConstraint::MinimumLength(_), _) => Err(ValueValidationError {
                offending_value: value.clone(),
                failed_constraints: vec![ConstraintError::InvalidConstraintError],
            }),
        }
    }
}

impl<T> ValidatesValues for Vec<T>
where
    T: ValidatesValues,
{
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError> {
        let mut errors: Vec<ConstraintError> = Vec::new();
        let mut errors_found = false;
        for v in self.iter() {
            if let Err(error) = v.validate(value) {
                errors_found = true;
                errors.extend(error.failed_constraints);
            }
        }
        if errors_found {
            Err(ValueValidationError {
                offending_value: value.clone(),
                failed_constraints: errors,
            })
        } else {
            Ok(())
        }
    }
}
