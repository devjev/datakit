//! Value-to-Value Conversion
//!
//! This module deals with coercing values to different ValueTypes.
//!
//! # TODO
//!
//! 1. Use the `datakit::value::parsing` module to handle to coercion
//!    from `ValueType::Text` to anything else.
//! 2. Clean the code up a bit, since it looks like hot trash.

use crate::errors::*;
use crate::value::definitions::*;
use crate::value::parsing::*;
use crate::value::primitives::*;
use crate::value::traits::*;
//use chrono::{DateTime, Local, Utc};

pub struct Coercion {
    parser: Parser,
}

impl Coercion {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    // This needs to be harmonized with the Parser
    fn number_to_text(&self, value: &Value) -> Result<Value, CoercionError> {
        match value {
            Value::Number(Numeric::Integer(i)) => Ok(Value::Text(i.to_string())),
            Value::Number(Numeric::Real(r)) => Ok(Value::Text(r.to_string())),
            Value::Number(Numeric::Complex(_, _)) => Err(CoercionError::DomainError(String::from(
                "Conversion for complex numbers is currently not supported.",
            ))),
            _ => Err(CoercionError::UnexpectedType),
        }
    }

    fn boolean_to_text(&self, value: &Value) -> Result<Value, CoercionError> {
        if let Value::Boolean(b) = value {
            Ok(Value::Text(b.to_string()))
        } else {
            Err(CoercionError::UnexpectedType)
        }
    }

    fn datetime_to_text(&self, value: &Value) -> Result<Value, CoercionError> {
        if let Value::DateTime(t) = value {
            Ok(Value::Text(t.to_string()))
        } else {
            Err(CoercionError::UnexpectedType)
        }
    }

    fn boolean_to_number(&self, value: &Value) -> Result<Value, CoercionError> {
        if let Value::Boolean(b) = value {
            match b {
                true => Ok(Value::Number(Numeric::Integer(1))),
                false => Ok(Value::Number(Numeric::Integer(0))),
            }
        } else {
            Err(CoercionError::UnexpectedType)
        }
    }

    fn number_to_boolean(&self, value: &Value) -> Result<Value, CoercionError> {
        match value {
            Value::Number(Numeric::Integer(i)) => match i {
                0 => Ok(Value::Boolean(false)),
                1 => Ok(Value::Boolean(true)),
                _ => Err(CoercionError::DomainError(format!(
                    "Boolean values accepted only as 0 or 1 for integers. Got {}.",
                    i
                ))),
            },
            _ => Err(CoercionError::UnexpectedType),
        }
    }
}

impl CoercesValues for Coercion {
    fn convert(&self, value: &Value, to_vtype: &ValueType) -> Result<Value, CoercionError> {
        use ValueType::*;

        match (value.get_value_type(), to_vtype) {
            (Number, Number) => Ok(value.clone()), // TODO deal with sub-types
            (DateTime, DateTime) => Ok(value.clone()),
            (Boolean, Boolean) => Ok(value.clone()),
            (Text, Text) => Ok(value.clone()),
            (Composite, Composite) => Ok(value.clone()),
            (Text, Composite) => Err(CoercionError::CoercionImpossible {
                from: ValueType::Text,
                to: ValueType::Composite,
            }),
            (Number, Text) => self.number_to_text(value),
            (Boolean, Text) => self.boolean_to_text(value),
            (DateTime, Text) => self.datetime_to_text(value),
            (Number, Boolean) => self.number_to_boolean(value),
            (Boolean, Number) => self.boolean_to_number(value),
            (Text, b) => match value {
                Value::Text(s) => match self.parser.parse(s) {
                    Err(_) => Err(CoercionError::CoercionFailed {
                        target_type: b.clone(),
                        source_value: value.clone(),
                    }),
                    Ok(x) => Ok(x),
                },
                _ => Err(CoercionError::CoercionImpossible {
                    from: Text,
                    to: b.clone(),
                }),
            },
            (a, Missing) => Err(CoercionError::CoercionImpossible {
                from: a.clone(),
                to: ValueType::Missing,
            }),
            (Missing, b) => Err(CoercionError::CoercionImpossible {
                from: ValueType::Missing,
                to: b.clone(),
            }),
            (Composite, b) => Err(CoercionError::CoercionImpossible {
                from: ValueType::Composite,
                to: b.clone(),
            }),
            (a, Composite) => Err(CoercionError::CoercionImpossible {
                from: a.clone(),
                to: ValueType::Composite,
            }),
            (a, DateTime) => Err(CoercionError::CoercionImpossible {
                from: a.clone(),
                to: ValueType::DateTime,
            }),
            (DateTime, b) => Err(CoercionError::CoercionImpossible {
                from: ValueType::DateTime,
                to: b.clone(),
            }),
        }
    }
}
