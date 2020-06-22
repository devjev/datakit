use crate::errors::*;
use crate::value::definition::*;
use crate::value::primitives::*;
use crate::value::traits::*;
use chrono::{DateTime, Local, Utc};

// Value Conversion

pub struct ValueConversion {}

impl ValueConversion {
    pub fn new() -> Self {
        Self {}
    }

    fn text_to_number(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::Text(s) = value {
            match s.parse::<i64>() {
                Ok(x) => Ok(Value::Number(Numeric::Integer(x))),
                Err(_) => match s.parse::<f64>() {
                    Ok(f) => Ok(Value::Number(Numeric::Real(f))),
                    Err(_) => Err(ValueConversionError::ParseError {
                        target_type: ValueType::Number,
                        source_text: s.clone(),
                    }),
                },
            }
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn text_to_boolean(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::Text(s) = value {
            match s.parse::<bool>() {
                Ok(b) => Ok(Value::Boolean(b)),
                Err(_) => Err(ValueConversionError::ParseError {
                    target_type: ValueType::Boolean,
                    source_text: s.clone(),
                }),
            }
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn text_to_datetime(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::Text(s) = value {
            match s.parse::<DateTime<Local>>() {
                Ok(t) => {
                    let utc = t.with_timezone(&Utc);
                    Ok(Value::DateTime(utc))
                }
                Err(_) => Err(ValueConversionError::ParseError {
                    target_type: ValueType::DateTime,
                    source_text: s.clone(),
                }),
            }
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn number_to_text(&self, value: &Value) -> Result<Value, ValueConversionError> {
        match value {
            Value::Number(Numeric::Integer(i)) => Ok(Value::Text(i.to_string())),
            Value::Number(Numeric::Real(r)) => Ok(Value::Text(r.to_string())),
            Value::Number(Numeric::Complex(_, _)) => Err(ValueConversionError::DomainError(
                String::from("Conversion for complex numbers is currently not supported."),
            )),
            _ => Err(ValueConversionError::UnexpectedType),
        }
    }

    fn boolean_to_text(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::Boolean(b) = value {
            Ok(Value::Text(b.to_string()))
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn datetime_to_text(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::DateTime(t) = value {
            Ok(Value::Text(t.to_rfc3339()))
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn boolean_to_number(&self, value: &Value) -> Result<Value, ValueConversionError> {
        if let Value::Boolean(b) = value {
            match b {
                true => Ok(Value::Number(Numeric::Integer(1))),
                false => Ok(Value::Number(Numeric::Integer(0))),
            }
        } else {
            Err(ValueConversionError::UnexpectedType)
        }
    }

    fn number_to_boolean(&self, value: &Value) -> Result<Value, ValueConversionError> {
        match value {
            Value::Number(Numeric::Integer(i)) => match i {
                0 => Ok(Value::Boolean(false)),
                1 => Ok(Value::Boolean(true)),
                _ => Err(ValueConversionError::DomainError(format!(
                    "Boolean values accepted only as 0 or 1 for integers. Got {}.",
                    i
                ))),
            },
            _ => Err(ValueConversionError::UnexpectedType),
        }
    }
}

impl ConvertsValues for ValueConversion {
    fn convert(&self, value: &Value, to_vtype: &ValueType) -> Result<Value, ValueConversionError> {
        use ValueType::*;

        match (value.get_value_type(), to_vtype) {
            (Number, Number) => Ok(value.clone()), // TODO deal with sub-types
            (DateTime, DateTime) => Ok(value.clone()),
            (Boolean, Boolean) => Ok(value.clone()),
            (Text, Text) => Ok(value.clone()),
            (Composite, Composite) => Ok(value.clone()),
            (Text, Number) => self.text_to_number(value),
            (Text, Boolean) => self.text_to_boolean(value),
            (Text, Composite) => Err(ValueConversionError::ConversionUnavailable {
                from: ValueType::Text,
                to: ValueType::Composite,
            }),
            (Text, DateTime) => self.text_to_datetime(value),
            (Number, Text) => self.number_to_text(value),
            (Boolean, Text) => self.boolean_to_text(value),
            (DateTime, Text) => self.datetime_to_text(value),
            (Number, Boolean) => self.number_to_boolean(value),
            (Boolean, Number) => self.boolean_to_number(value),
            (a, Missing) => Err(ValueConversionError::ConversionUnavailable {
                from: a.clone(),
                to: ValueType::Missing,
            }),
            (Missing, b) => Err(ValueConversionError::ConversionUnavailable {
                from: ValueType::Missing,
                to: b.clone(),
            }),
            (Composite, b) => Err(ValueConversionError::ConversionUnavailable {
                from: ValueType::Composite,
                to: b.clone(),
            }),
            (a, Composite) => Err(ValueConversionError::ConversionUnavailable {
                from: a.clone(),
                to: ValueType::Composite,
            }),
            (a, DateTime) => Err(ValueConversionError::ConversionUnavailable {
                from: a.clone(),
                to: ValueType::DateTime,
            }),
            (DateTime, b) => Err(ValueConversionError::ConversionUnavailable {
                from: ValueType::DateTime,
                to: b.clone(),
            }),
        }
    }
}
