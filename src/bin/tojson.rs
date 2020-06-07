use chrono::prelude::*;
use chrono::FixedOffset;
use datakit::value::*;
use serde::{Deserialize, Serialize};

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

    let obj_json = serde_json::to_string(&person).unwrap();
    let height_constraint_json = serde_json::to_string(&height_constraint).unwrap();

    println!("Person struct: {}", obj_json);
    println!("Height constraint: {}", height_constraint_json);
    println!(
        "Is height valid? {:?}",
        height_constraint.validate(&person.height)
    );
}
