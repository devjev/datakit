use crate::value::primitives::*;
use serde::{Deserialize, Serialize};

macro_rules! value_type_definition {
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

value_type_definition! {
    Number(Numeric),
    Text(String),
    DateTime(DateTime),
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

    /* TODO conditional on chrono
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

    */

    bool => |value: &bool| { Value::Boolean(value.clone()) }

    // TODO ensure coverage
}

impl_from_value_to_t_option! {
    i32 => Value::Number(Numeric::Integer(x)) => x as i32,
    i64 => Value::Number(Numeric::Integer(x)) => x,
    f32 => Value::Number(Numeric::Real(r)) => r as f32,
    f64 => Value::Number(Numeric::Real(r)) => r,
    String => Value::Text(text) => text,
    /* conditional on chrono
    DateTime<Utc> => Value::DateTime(t) => t,
    DateTime<Local> => Value::DateTime(t) => t.with_timezone(&Local),
    */
    bool => Value::Boolean(b) => b

    // TODO &str and DateTime<FixedOffset>
    // TODO ensure coverage
}
