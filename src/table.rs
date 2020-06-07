use crate::value::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Column = Vec<Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnContract {
    pub name: String,
    pub value_contract: ValueContract,
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

    pub fn add_row(&mut self, row: Vec<Value>) -> Result<(), TableError> {
        if row.len() != self.col_length {
            Err(TableError::DimensionError)
        } else {
            for (col_index, value) in row.iter().enumerate() {
                self.columns[col_index].push(value.clone());
            }
            Ok(())
        }
    }

    pub fn column_contracts(&self) -> &Vec<ColumnContract> {
        &self.column_contracts
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
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

    pub fn validate_column(&self, col_id: &ColumnId) -> Result<(), TableError> {
        let ordinal = self.resolve_column_id(col_id)?;
        let column_contract = &self.column_contracts[ordinal];
        let column = &self.columns[ordinal];

        let mut result: Vec<(usize, ValueValidationError)> = Vec::new();
        // TODO this can be done in parallel
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

    pub fn validate_table(&self) -> Result<(), TableError> {
        let mut result: HashMap<String, Vec<(usize, ValueValidationError)>> = HashMap::new();
        for (ordinal, _) in self.columns.iter().enumerate() {
            if let Err(table_error) = self.validate_column(&ColumnId::Ordinal(ordinal)) {
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

        if result.len() != 0 {
            Err(TableError::TableDataInvalid(result))
        } else {
            Ok(())
        }
    }

    pub fn map_column_if(
        &mut self,
        col_id: ColumnId,
        func: &dyn Fn(&Value) -> Value,
        predicates: &Vec<(ColumnId, &dyn Fn(&Value) -> bool)>,
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

    pub fn map_column(
        &mut self,
        col_id: ColumnId,
        func: &dyn Fn(&Value) -> Value,
    ) -> Result<(), TableError> {
        self.map_column_if(col_id, func, &Vec::new())
    }
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
        errors: Vec<(usize, ValueValidationError)>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableError {
    DimensionError, // TODO
    ColumnError(ColumnError),
    TableDataInvalid(HashMap<String, Vec<(usize, ValueValidationError)>>),
}
