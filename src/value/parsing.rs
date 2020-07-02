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
use crate::value::definitions::*;
use crate::value::primitives::*;
use crate::value::traits::*;

mod translate_iso8601 {
    use crate::value::definitions::*;
    use crate::value::primitives::*;
    use std::convert::TryInto;

    pub(crate) fn date_to_dk_date(iso8601_date: &iso8601::Date) -> Date {
        match iso8601_date {
            iso8601::Date::YMD { year, month, day } => Date::YearMonthDay {
                year: *year,
                month: (*month).try_into().unwrap(),
                day: (*day).try_into().unwrap(),
            },
            iso8601::Date::Week { year, ww, d } => Date::YearWeekDay {
                year: *year,
                week_in_year: (*ww).try_into().unwrap(),
                day_in_week: (*d).try_into().unwrap(),
            },
            iso8601::Date::Ordinal { year, ddd } => Date::YearDay {
                year: *year,
                day_in_year: (*ddd).try_into().unwrap(),
            },
        }
    }

    pub(crate) fn time_to_dk_time(iso8601_time: &iso8601::Time) -> Time {
        match iso8601_time {
            iso8601::Time {
                hour,
                minute,
                second,
                millisecond,
                tz_offset_hours,
                tz_offset_minutes,
            } => {
                let tz = if *tz_offset_hours == 0 && *tz_offset_minutes == 0 {
                    TimeZone::Utc
                } else {
                    TimeZone::Offset {
                        hours: (*tz_offset_hours).try_into().unwrap(),
                        minutes: (*tz_offset_minutes).try_into().unwrap(),
                    }
                };

                Time {
                    hour: (*hour).try_into().unwrap(),
                    minute: (*minute).try_into().unwrap(),
                    second: (*second).try_into().unwrap(),
                    milli: (*millisecond).try_into().unwrap(),
                    micro: 0,
                    nano: 0,
                    timezone: tz,
                }
            }
        }
    }

    pub(crate) fn datetime_to_dk_datetime(iso8601_struct: &iso8601::DateTime) -> DateTime {
        let date = date_to_dk_date(&iso8601_struct.date);
        let time = time_to_dk_time(&iso8601_struct.time);
        DateTime::Full { date, time }
    }

    pub(crate) fn iso8601_to_dk_value(s: &str) -> Result<Value, ()> {
        if let Ok(iso8601_struct) = iso8601::datetime(s) {
            let datetime = datetime_to_dk_datetime(&iso8601_struct);
            Ok(Value::DateTime(datetime))
        } else if let Ok(iso8601_date) = iso8601::date(s) {
            let date = date_to_dk_date(&iso8601_date);
            Ok(Value::DateTime(DateTime::Date(date)))
        } else if let Ok(iso8601_time) = iso8601::time(s) {
            let time = time_to_dk_time(&iso8601_time);
            Ok(Value::DateTime(DateTime::Time(time)))
        } else {
            Err(())
        }
    }
}

fn jsvalue_to_dkvalue(jsvalue: &serde_json::Value) -> Value {
    match jsvalue {
        serde_json::Value::Null => Value::Missing(Empty::Expected),
        serde_json::Value::Bool(x) => Value::Boolean(*x),
        serde_json::Value::String(s) => {
            if let Ok(datetime) = translate_iso8601::iso8601_to_dk_value(s) {
                datetime
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
