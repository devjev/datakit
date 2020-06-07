use chrono::prelude::*;
use chrono::FixedOffset;
use datakit::table::*;
use datakit::value::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    name: Value,
    height: Value,
    date_of_birth: Value,
    favorite_color: Value,
    favorite_cake: Value,
}

fn main() {
    println!("============= Struct =============");

    // Example with a struct and its contracts
    let person = Person {
        name: "Jim".into(),
        height: 1.83.into(),
        date_of_birth: FixedOffset::east(3 * 3600)
            .ymd(1985, 3, 10)
            .and_hms(3, 11, 0)
            .into(),
        favorite_cake: Value::Missing(Empty::Expected),
        favorite_color: Value::Missing(Empty::Unexpected),
    };

    let height_constraint = ValueContract::new(
        TypeConstraint::IsType(ValueType::Number),
        vec![
            ValueConstraint::Minimum(40.5.into()),
            ValueConstraint::Maximum(0.2.into()),
        ],
    );

    let obj_json = serde_json::to_string_pretty(&person).unwrap();
    let height_constraint_json = serde_json::to_string_pretty(&height_constraint).unwrap();

    println!("Person struct: {}", obj_json);
    println!("Height constraint: {}", height_constraint_json);
    println!(
        "Is height valid? {:?}",
        height_constraint.validate(&person.height)
    );

    println!("============= Table =============");

    // Example with a table
    let mut schema_definition: HashMap<String, ValueContract> = HashMap::new();
    schema_definition.insert(
        String::from("Name"),
        ValueContract {
            expected_type: TypeConstraint::IsType(ValueType::Text),
            value_constraints: vec![ValueConstraint::MaximumLength(100)],
        },
    );
    schema_definition.insert(
        String::from("FavoritePie"),
        ValueContract {
            expected_type: TypeConstraint::IsType(ValueType::Text),
            value_constraints: vec![ValueConstraint::OneOf(vec![
                Value::Text(String::from("Apple")),
                Value::Text(String::from("Cherry")),
                Value::Text(String::from("Blueberry")),
            ])],
        },
    );
    schema_definition.insert(
        String::from("PiesEaten"),
        ValueContract {
            expected_type: TypeConstraint::IsType(ValueType::Number),
            value_constraints: vec![ValueConstraint::MaximumLength(255)],
        },
    );

    let schema = Schema::from_tuples(vec![
        (
            "Name",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Text),
                value_constraints: vec![ValueConstraint::MaximumLength(100)],
            },
        ),
        (
            "FavoritePie",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Text),
                value_constraints: vec![ValueConstraint::OneOf(vec![
                    Value::Text(String::from("Apple")),
                    Value::Text(String::from("Cherry")),
                    Value::Text(String::from("Blueberry")),
                ])],
            },
        ),
        (
            "PiesEaten",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Number),
                value_constraints: Vec::new(),
            },
        ),
    ]);

    let mut table = Table::from_schema(&schema);

    table
        .add_row(&vec![
            Value::Text(String::from("Jim")),
            Value::Text(String::from("Apple")),
            Value::Number(Numeric::Integer(1)),
        ])
        .unwrap();

    table
        .add_row(&vec![
            Value::Text(String::from("Jenny")),
            Value::Text(String::from("Cherry")),
            Value::Number(Numeric::Integer(10)),
        ])
        .unwrap();

    table
        .add_row(&vec![
            Value::Text(String::from("Derek")),
            Value::Text(String::from("Beef")),
            Value::Number(Numeric::Integer(99)),
        ])
        .unwrap();

    let validation = table.validate_table().unwrap_err();

    let table_json = serde_json::to_string_pretty(&table).unwrap();
    let validation_result_json = serde_json::to_string_pretty(&validation).unwrap();
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    println!(".... Table");
    println!("{}", table_json);
    println!(".... Validation Result");
    println!("{}", validation_result_json);
    println!(".... Schema");
    println!("{}", schema_json);
}
