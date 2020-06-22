use crate::errors::*;
use crate::value::definition::*;
use crate::value::traits::ValidatesValues;
use serde::{Deserialize, Serialize};

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

// Contracts & Constraints

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
