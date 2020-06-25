mod common_traits {
    use datakit::value::definitions::*;
    use datakit::value::primitives::*;

    macro_rules! from_impl_tests {
        ( $($description:ident : [$x:expr, $type:ty] => $e:expr),+ ) => {
            $(
                #[test]
                fn $description() {
                    let x: Value = ($x).into();
                    assert_eq!(x, $e);

                    let y: Value = Value::from($x);
                    assert_eq!(y, $e);
                }
            )+
        };
    }

    from_impl_tests! {
        i32_from_into_value : [16, i32] => Value::Number(Numeric::Integer(16)),
        i64_from_into_value : [16, i64] => Value::Number(Numeric::Integer(16)),
        f32_from_into_value : [1.6, f32] => Value::Number(Numeric::Real(1.6)),
        f64_from_into_value : [3.14, f64] => Value::Number(Numeric::Real(3.14)),
        strref_from_into_value : ["hello", &str] => Value::Text(String::from("hello")),
        string_from_into_value : ["hello", String] => Value::Text(String::from("hello")),
        option_i32_from_into_value : [16, Option<i32>] => Value::Number(Numeric::Integer(16)),
        option_i64_from_into_value : [16, Option<i64>] => Value::Number(Numeric::Integer(16))
    }

    #[test]
    fn ordering() {
        let a = Value::from(2);
        let b = Value::from(7);
        assert!(b > a);
    }
}

mod api {
    use datakit::value::definitions::*;

    #[test]
    fn is_of_type_is_correct() {
        let x = Value::from(137);
        assert!(x.is_of_type(&ValueType::Number));
        assert!(!x.is_of_type(&ValueType::Text));
    }
}

pub mod value_parsing {
    use datakit::value::definitions::*;
    use datakit::value::parsing::Parser;
    use datakit::value::primitives::*;
    use datakit::value::traits::*;

    macro_rules! test_literal_parsing {
        ( $($desc:ident : $lit:expr => $val:expr),+ ) => {
            $(
                #[test]
                fn $desc() {
                    let parser = Parser::new();
                    let literal: &str = $lit;
                    let value = parser.parse(literal).unwrap();
                    assert_eq!(value, $val);
                }
            )+
        };
    }

    test_literal_parsing! {
        integer_literals : "137" => Value::Number(Numeric::Integer(137)),
        float_literals : "13.7" => Value::Number(Numeric::Real(13.7)),
        bool_literals : "true" => Value::Boolean(true),
        null_literals : "null" => Value::Missing(Empty::Expected),
        array_literal : "[1, 2, 3]" => Value::Composite(
            Collection::Array(vec![
                Value::Number(Numeric::Integer(1)),
                Value::Number(Numeric::Integer(2)),
                Value::Number(Numeric::Integer(3))
            ])
        ),
        obj_literal : "{ \"a\": 1, \"b\": [2, 3], \"c\": { \"d\": \"foo\" }}" =>
            Value::Composite(
                Collection::Object(vec![
                    ("a".to_string(), Value::Number(Numeric::Integer(1))),
                    ("b".to_string(), Value::Composite(
                        Collection::Array(vec![
                            Value::Number(Numeric::Integer(2)),
                            Value::Number(Numeric::Integer(3))
                        ])
                    )),
                    ("c".to_string(), Value::Composite(
                        Collection::Object(vec![
                            ("d".to_string(), Value::Text("foo".to_string()))
                        ])
                    ))
                ])
            )
    }

    #[test]
    fn failed_parsing_throws_error() {
        let parser = Parser::new();
        let bad_literal = "-@(#$*";
        parser.parse(bad_literal).unwrap_err();
    }
}
