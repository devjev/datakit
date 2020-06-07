//! Dynamic runtime values and value data contracts
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
//! # Example
//!
//! ```json
//! {
//!     "name": {
//!         "text": "Jim"
//!     },
//!     "height": {
//!         "number": {
//!             "real": 1.83
//!         }
//!     },
//!     "dateOfBirth": {
//!         "dateTime": "1985-03-10T00:11:00Z"
//!     },
//!     "favoriteColor": {
//!         "missing": "unexpected"
//!     },
//!     "favoriteCake": {
//!         "missing": "expected"
//!     }
//! }
//! ```

use chrono::{DateTime, FixedOffset, Local, Utc};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::error::Error;

// Errors --------------------------------------------------------------------

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

// Traits --------------------------------------------------------------------

/// Value Validation
///
/// Returns either an Ok(()) result (if validation has been successfull) or
/// a filled out ValueValidationError.
///
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Collection<T> {
    Array(Vec<T>),
    Object(Vec<(String, T)>),
}

value_types! {
    Number(Numeric),
    Text(String),
    DateTime(DateTime<Utc>),
    Missing(Empty),
    Boolean(bool),
    Composite(Collection<Value>)
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
    },
    bool => |value: &bool| { Value::Boolean(value.clone()) }
}

impl_from_value_to_t_option! {
    i32 => Value::Number(Numeric::Integer(x)) => x as i32,
    i64 => Value::Number(Numeric::Integer(x)) => x,
    f32 => Value::Number(Numeric::Real(r)) => r as f32,
    f64 => Value::Number(Numeric::Real(r)) => r,
    String => Value::Text(text) => text,
    DateTime<Utc> => Value::DateTime(t) => t,
    DateTime<Local> => Value::DateTime(t) => t.with_timezone(&Local),
    bool => Value::Boolean(b) => b
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
            (ValueConstraint::Not(c), _) => match c.validate(value) {
                Ok(()) => _to_valueconstraint_err!(value, self),
                Err(_) => Ok(()),
            },
            (ValueConstraint::OneOf(allowed_values), _) => {
                let mut is_one_of_the_allowed = false;
                for allowed in allowed_values.iter() {
                    if value == allowed {
                        is_one_of_the_allowed = true;
                        break;
                    }
                }

                if is_one_of_the_allowed {
                    Ok(())
                } else {
                    return _to_valueconstraint_err!(value, self);
                }
            }
            (ValueConstraint::Maximum(max), _) => {
                if value <= max {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(value, self)
                }
            }
            (ValueConstraint::Minimum(min), _) => {
                if value >= min {
                    Ok(())
                } else {
                    _to_valueconstraint_err!(value, self)
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueContract {
    pub expected_type: TypeConstraint,
    pub value_constraints: Vec<ValueConstraint>,
}

impl ValueContract {
    pub fn new(expected_type: TypeConstraint, value_constraints: Vec<ValueConstraint>) -> Self {
        Self {
            expected_type,
            value_constraints,
        }
    }
}

impl ValidatesValues for ValueContract {
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError> {
        let mut errors_found = false;
        let mut errors: Vec<ConstraintError> = Vec::new();
        if let Err(tce) = self.expected_type.validate(value) {
            errors_found = true;
            errors.extend(tce.failed_constraints);
        };

        for vc in self.value_constraints.iter() {
            if let Err(vce) = vc.validate(value) {
                errors_found = true;
                errors.extend(vce.failed_constraints);
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
