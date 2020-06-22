use crate::errors::*;
use crate::value::definition::*;

/// Value Validation
///
/// Returns either an Ok(()) result (if validation has been successfull) or
/// a filled out ValueValidationError.
///
pub trait ValidatesValues {
    fn validate(&self, value: &Value) -> Result<(), ValueValidationError>;
}

/// Value to Value Conversion
pub trait ConvertsValues {
    fn convert(&self, value: &Value, to_vtype: &ValueType) -> Result<Value, ValueConversionError>;
}

pub trait ParsesToValue {
    fn parse(&self, s: &str) -> Result<Value, ()>;
}
