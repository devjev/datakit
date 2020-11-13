use datakit::table::*;
use datakit::value::constraints::*;
use datakit::value::definitions::*;

fn main() {
    let mut schema = Schema::new();
    schema.column_contracts.push(ColumnContract {
        name: "First Column".into(),
        value_contract: ValueContract::new(TypeConstraint::IsType(ValueType::Text), vec![]),
    });

    schema.column_contracts.push(ColumnContract {
        name: "Second Column".into(),
        value_contract: ValueContract::new(TypeConstraint::IsType(ValueType::Text), vec![]),
    });

    schema.column_contracts.push(ColumnContract {
        name: "Third Column".into(),
        value_contract: ValueContract::new(TypeConstraint::IsType(ValueType::Text), vec![]),
    });

    let mut table = Table::from_schema(&schema);
    table
        .add_row(&vec![
            "First value".into(),
            "Second value".into(),
            "Third value".into(),
        ])
        .unwrap();
    table
        .add_row(&vec![
            "First value, second row".into(),
            "Second value, second row".into(),
            "Third value, second row".into(),
        ])
        .unwrap();

    let table_json = serde_json::to_string_pretty(&table).unwrap();
    println!("{}", table_json);
}
