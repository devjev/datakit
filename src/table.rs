use crate::errors::*;
use crate::value::constraints::*;
use crate::value::definitions::*;
use crate::value::traits::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "experimental")]
use rayon::prelude::*;

pub type Column = Vec<Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnContract {
    pub name: String,
    pub value_contract: ValueContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub column_contracts: Vec<ColumnContract>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            column_contracts: Vec::new(),
        }
    }

    pub fn from_tuples(tuples: Vec<(&str, ValueContract)>) -> Self {
        let mut new = Self::new();
        for (name, vc) in tuples.iter() {
            new.column_contracts.push(ColumnContract {
                name: String::from(*name),
                value_contract: vc.clone(),
            })
        }
        new
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    columns: Vec<Column>,
    column_contracts: Vec<ColumnContract>,
    col_length: usize,
    row_length: usize,
    // TODO row_contract -- Note, a table can have only one row contract
    // TODO table_contract -- Things like table dimensions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColumnId {
    Ordinal(usize),
    Name(String),
}

impl Table {
    pub fn new() -> Self {
        let columns: Vec<Column> = Vec::new();
        let column_contracts: Vec<ColumnContract> = Vec::new();
        let col_length: usize = 0;
        let row_length: usize = 0;
        Self {
            columns,
            column_contracts,
            col_length,
            row_length,
        }
    }

    pub fn len(&self) -> usize {
        self.row_length
    }

    pub fn from_schema(schema: &Schema) -> Self {
        let mut new = Self::new();
        new.column_contracts = schema.column_contracts.clone();
        new.col_length = schema.column_contracts.len();
        for _ in 0..new.col_length {
            new.columns.push(Vec::new());
        }
        new.row_length = 0;
        new
    }

    fn column_order(&self, col_name: &str) -> Option<usize> {
        self.column_contracts
            .iter()
            .position(|c| &c.name == col_name)
    }

    fn resolve_column_id(&self, col_id: &ColumnId) -> Result<usize, TableError> {
        match col_id {
            ColumnId::Name(name) => match self.column_order(&name) {
                Some(id) => Ok(id),
                None => Err(TableError::ColumnError(ColumnError::Unknown(
                    col_id.clone(),
                ))),
            },
            ColumnId::Ordinal(ord) => Ok(*ord),
        }
    }

    pub fn add_empty_column(&mut self, col_contract: ColumnContract) -> Result<(), TableError> {
        match self.column_order(&col_contract.name) {
            Some(ordinal) => Err(TableError::ColumnError(ColumnError::AlreadyExists {
                ordinal: ordinal,
                name: col_contract.name.clone(),
            })),
            None => {
                self.column_contracts.push(col_contract);
                self.columns.push(Vec::new());
                self.col_length += 1;
                Ok(())
            }
        }
    }

    pub fn remove_column(&mut self, col_id: &ColumnId) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        self.column_contracts.remove(ordinal);
        self.columns.remove(ordinal);
        self.col_length -= 1;
        Ok(())
    }

    pub fn add_row(&mut self, row: &Vec<Value>) -> Result<(), TableError> {
        if row.len() != self.col_length {
            Err(TableError::DimensionError)
        } else {
            for (col_index, value) in row.iter().enumerate() {
                self.columns[col_index].push(value.clone());
            }
            self.row_length += 1;
            Ok(())
        }
    }

    pub fn column_contracts(&self) -> &Vec<ColumnContract> {
        &self.column_contracts
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn column(&self, col_id: &ColumnId) -> Result<&Column, TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        Ok(&self.columns[ordinal])
    }

    pub fn column_contract(&self, col_id: &ColumnId) -> Result<&ColumnContract, TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        Ok(&self.column_contracts[ordinal])
    }

    pub fn alter_column(
        &mut self,
        col_id: &ColumnId,
        col_contract: ColumnContract,
    ) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        self.column_contracts[ordinal] = col_contract;
        Ok(())
    }

    #[cfg(feature = "experimental")]
    pub fn validate_column_par(&self, col_id: &ColumnId) -> Result<(), TableError> {
        let column_contract = self.column_contract(col_id)?;
        let column = self.column(col_id)?;

        let errors: Vec<(usize, ValidationError)> = column
            .par_iter()
            .enumerate()
            .filter_map(
                |(rowno, value)| match column_contract.value_contract.validate(value) {
                    Ok(()) => None,
                    Err(error) => Some((rowno, error)),
                },
            )
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(TableError::ColumnError(
                ColumnError::ContainsInvalidValues {
                    contract: column_contract.clone(),
                    errors: errors,
                },
            ))
        }
    }

    pub fn validate_column_against_contract(
        &self,
        col_id: &ColumnId,
        column_contract: &ColumnContract,
    ) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        let column = &self.columns[ordinal];

        let mut result: Vec<(usize, ValidationError)> = Vec::new();
        for (rowno, value) in column.iter().enumerate() {
            match column_contract.value_contract.validate(value) {
                Ok(()) => (),
                Err(error) => {
                    result.push((rowno, error));
                }
            }
        }

        if result.len() == 0 {
            Ok(())
        } else {
            Err(TableError::ColumnError(
                ColumnError::ContainsInvalidValues {
                    contract: column_contract.clone(),
                    errors: result,
                },
            ))
        }
    }

    pub fn validate_column(&self, col_id: &ColumnId) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        let column_contract = &self.column_contracts[ordinal];
        self.validate_column_against_contract(&ColumnId::Ordinal(ordinal), column_contract)
    }

    #[cfg(feature = "experimental")]
    pub fn validate_table_par(&self) -> Result<(), TableError> {
        let column_results: Vec<(ColumnContract, Vec<(usize, ValidationError)>)> = self
            .columns()
            .par_iter()
            .enumerate()
            .filter_map(|(ordinal, _)| {
                match self.validate_column_par(&ColumnId::Ordinal(ordinal)) {
                    Ok(()) => None,
                    Err(TableError::ColumnError(ColumnError::ContainsInvalidValues {
                        contract,
                        errors,
                    })) => Some((contract, errors)),
                    _ => None,
                }
            })
            .collect();

        if column_results.is_empty() {
            Ok(())
        } else {
            let mut error_map: HashMap<String, Vec<(usize, ValidationError)>> = HashMap::new();
            for (contract, errors) in column_results.iter() {
                error_map.insert(contract.name.clone(), errors.clone());
            }
            Err(TableError::InvalidData(error_map))
        }
    }

    pub fn validate_table(&self) -> Result<(), TableError> {
        self.validate_table_against_contracts(&self.column_contracts, true)
    }

    pub fn validate_table_against_schema(
        &self,
        schema: &Schema,
        strict: bool,
    ) -> Result<(), TableError> {
        self.validate_table_against_contracts(&schema.column_contracts, strict)
    }

    pub(crate) fn validate_table_against_contracts(
        &self,
        col_contracts: &Vec<ColumnContract>,
        strict: bool,
    ) -> Result<(), TableError> {
        let mut result: HashMap<String, Vec<(usize, ValidationError)>> = HashMap::new();

        for (ordinal, _) in self.columns.iter().enumerate() {
            if !strict && (ordinal > col_contracts.len() - 1) {
                break;
            } else if strict && (ordinal > col_contracts.len() - 1) {
                return Err(TableError::ColumnError(ColumnError::Unknown(
                    ColumnId::Ordinal(ordinal),
                )));
            }

            if let Err(table_error) = self.validate_column_against_contract(
                &ColumnId::Ordinal(ordinal),
                &col_contracts[ordinal],
            ) {
                if let TableError::ColumnError(ColumnError::ContainsInvalidValues {
                    contract: _,
                    errors,
                }) = table_error
                {
                    let key = self.column_contracts[ordinal].name.clone();
                    result.insert(key, errors);
                }
            }
        }

        if result.is_empty() {
            Ok(())
        } else {
            Err(TableError::InvalidData(result))
        }
    }

    pub fn map_column_if<F: Fn(&Value) -> Value, P: Fn(&Value) -> bool>(
        &mut self,
        col_id: &ColumnId,
        func: F,
        predicates: &Vec<(ColumnId, P)>,
    ) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(&col_id)?;
        for rowno in 0..self.row_length {
            let old_value = &self.columns[ordinal][rowno];

            for (other_col_id, predicate) in predicates {
                let other_col_ordinal = self.resolve_column_id(other_col_id)?;
                let other_col_value = &self.columns[other_col_ordinal][rowno];
                if !predicate(other_col_value) {
                    continue;
                }
            }

            let new_value = func(old_value);
            self.columns[ordinal][rowno] = new_value;
        }
        Ok(())
    }

    pub fn map_column<F: Fn(&Value) -> Value>(
        &mut self,
        col_id: &ColumnId,
        func: F,
    ) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(&col_id)?;
        for rowno in 0..self.row_length {
            let old_value = &self.columns[ordinal][rowno];
            let new_value = func(old_value);
            self.columns[ordinal][rowno] = new_value;
        }
        Ok(())
    }

    pub fn check_compatibility(&self, schema: &Schema) -> Result<(), SchemaValidationError> {
        let mut result: Vec<SchemaError> = Vec::new();

        'outer: for their_cc in schema.column_contracts.iter() {
            for our_cc in self.column_contracts.iter() {
                if their_cc.name == our_cc.name {
                    if their_cc.value_contract != our_cc.value_contract {
                        result.push(SchemaError::ConflictingConstraints {
                            expected: their_cc.clone(),
                            received: our_cc.clone(),
                        });
                    };
                    continue 'outer;
                }
            }
            result.push(SchemaError::MissingColumn(their_cc.name.clone()));
        }

        if result.len() > 0 {
            Err(SchemaValidationError {
                schema_errors: result,
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaValidationError {
    pub schema_errors: Vec<SchemaError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchemaError {
    ConflictingConstraints {
        expected: ColumnContract,
        received: ColumnContract,
    },
    MissingColumn(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColumnError {
    Unknown(ColumnId),
    AlreadyExists {
        ordinal: usize,
        name: String,
    },
    ContainsInvalidValues {
        contract: ColumnContract,
        errors: Vec<(usize, ValidationError)>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableError {
    DimensionError, // TODO
    ColumnError(ColumnError),
    InvalidData(HashMap<String, Vec<(usize, ValidationError)>>),
}
