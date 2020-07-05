use crate::errors::*;
use crate::value::definitions::*;

/// Value Validation
///
/// Returns either an Ok(()) result (if validation has been successfull) or
/// a filled out ValueValidationError.
///
pub trait ValidatesValues {
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}

/// Value to Value Conversion
pub trait CoercesValues {
    fn convert(&self, value: &Value, to_vtype: &ValueType) -> Result<Value, CoercionError>;
}

pub trait ParsesValues {
    fn parse(&self, s: &str) -> Result<Value, ParsingError>;
}
