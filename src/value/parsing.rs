//! Value Parsing
//!
//! # Piggybacking On `serde_json`
//!
//! JavaScript/JSON syntax for literal values is very <broad, spread out?> and
//! overlaps with a lot of other textual serialization formats, like strict and
//! quoted CSV. For example, the text `"abc"` describes a text string in JSON,
//! CSV, Python, TOML, etc. Same applies for number literals.
//!
//! See [this](https://docs.serde.rs/serde_json/index.html).

use crate::errors::*;
use crate::value::definition::*;
use crate::value::primitives::*;
use crate::value::traits::*;
use chrono::{DateTime, Local, Utc};

fn jsvalue_to_dkvalue(jsvalue: &serde_json::Value) -> Value {
    match jsvalue {
        serde_json::Value::Null => Value::Missing(Empty::Expected),
        serde_json::Value::Bool(x) => Value::Boolean(*x),
        serde_json::Value::String(s) => {
            // 1. Try to parse a date,
            // 2. Try to parse a complex (TODO)
            // 3. If that fails, return that as string.
            if let Ok(datetime) = s.parse::<DateTime<Local>>() {
                let utc = datetime.with_timezone(&Utc);
                Value::DateTime(utc)
            } else {
                Value::Text(s.clone())
            }
        }
        serde_json::Value::Number(jsnum) => {
            if jsnum.is_i64() {
                if let Some(result) = jsnum.as_i64() {
                    Value::Number(Numeric::Integer(result))
                } else {
                    Value::Missing(Empty::Unexpected)
                }
            } else if jsnum.is_f64() {
                if let Some(result) = jsnum.as_f64() {
                    Value::Number(Numeric::Real(result))
                } else {
                    Value::Missing(Empty::Unexpected)
                }
            } else {
                Value::Missing(Empty::Unexpected) // TODO probably a conversion/parsing error
            }
        }
        serde_json::Value::Array(arr) => {
            let mut result: Vec<Value> = Vec::new();
            for jsvalue_in_arr in arr.iter() {
                let dkvalue = jsvalue_to_dkvalue(&jsvalue_in_arr);
                result.push(dkvalue);
            }
            Value::Composite(Collection::Array(result))
        }
        serde_json::Value::Object(obj) => {
            let mut result: Vec<(String, Value)> = Vec::new();
            for (key, jsvalue_in_obj) in obj.iter() {
                let dkvalue = jsvalue_to_dkvalue(&jsvalue_in_obj);
                result.push((key.clone(), dkvalue));
            }
            Value::Composite(Collection::Object(result))
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParsesValues for Parser {
    fn parse(&self, s: &str) -> Result<Value, ParsingError> {
        if let Ok(jsvalue) = serde_json::from_str::<serde_json::Value>(s) {
            Ok(jsvalue_to_dkvalue(&jsvalue))
        } else {
            Err(ParsingError::CannotParseValue(s.to_string()))
        }
    }
}
