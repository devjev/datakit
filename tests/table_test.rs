mod table {
    use datakit::table::*;
    use datakit::value::constraints::*;
    use datakit::value::definitions::*;
    //use datakit::value::primitives::*;

    #[test]
    fn validate_table_ok() -> Result<(), TableError> {
        let schema = Schema::from_tuples(vec![
            (
                "Name",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Text),
                    vec![ValueConstraint::Any],
                ),
            ),
            (
                "NumberOfPiesEaten",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Number),
                    vec![ValueConstraint::Maximum(10.into())],
                ),
            ),
        ]);

        let mut table = Table::from_schema(&schema);
        table.add_row(&vec![Value::Text("Jim".into()), 2.into()])?;
        table.validate_table()
    }

    #[test]
    fn validate_table_err() -> Result<(), String> {
        let schema = Schema::from_tuples(vec![
            (
                "Name",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Text),
                    vec![ValueConstraint::Any],
                ),
            ),
            (
                "NumberOfPiesEaten",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Number),
                    vec![ValueConstraint::Maximum(10.into())],
                ),
            ),
        ]);

        let mut table = Table::from_schema(&schema);
        table
            .add_row(&vec![Value::Text("Jim".into()), 12.into()])
            .unwrap();
        match table.validate_table() {
            Ok(()) => Err("Table validation didn't catch errors".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn validate_table_against_other_schema_err() -> Result<(), String> {
        let schema = Schema::from_tuples(vec![
            (
                "Name",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Text),
                    vec![ValueConstraint::Any],
                ),
            ),
            (
                "NumberOfPiesEaten",
                ValueContract::new(
                    TypeConstraint::IsType(ValueType::Number),
                    vec![ValueConstraint::Maximum(10.into())],
                ),
            ),
        ]);

        let other_schema = Schema::from_tuples(vec![(
            "Name",
            ValueContract::new(
                TypeConstraint::IsType(ValueType::Text),
                vec![ValueConstraint::MaximumLength(2)],
            ),
        )]);

        let mut table = Table::from_schema(&schema);
        table
            .add_row(&vec![Value::Text("Jim".into()), 12.into()])
            .unwrap();

        match table.validate_table_against_schema(&other_schema, false) {
            Ok(()) => Err("Table validation didn't catch errors".into()),
            Err(_) => Ok(()),
        }
    }
}
