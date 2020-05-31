mod value;

// Tests ---------------------------------------------------------------------

#[cfg(test)]
mod conversion_tests {
    macro_rules! from_impl_tests {
        ( $($description:ident [$x:expr, $type:ty] => $e:expr);+ ) => {
            $(
                #[test]
                fn $description() {
                    use crate::value::*;
                    let x: Value = ($x).into();
                    assert_eq!(x, $e)
                }
            )+
        };
    }

    from_impl_tests! {
        converts_i32_to_value [16, i32] => Value::Number(Numeric::Integer(16));
        converts_i64_to_value [16, i64] => Value::Number(Numeric::Integer(16));
        converts_f32_to_value [1.6, f32] => Value::Number(Numeric::Real(1.6));
        converts_f64_to_value [3.14, f64] => Value::Number(Numeric::Real(3.14));
        converts_strref_to_value ["hello", &str] => Value::Text(String::from("hello"));
        converts_string_to_value ["hello", String] => Value::Text(String::from("hello"));
        converts_option_i32_to_value [16, Option<i32>] => Value::Number(Numeric::Integer(16));
        converts_option_i64_to_value [16, Option<i64>] => Value::Number(Numeric::Integer(16))
        //converts_empty_option_i32_to_value [None, Option<i32>] => Value::Missing(Empty::Expected)
    }
}

#[cfg(test)]
mod api_tests {
    #[test]
    fn value_from_creation_works() {
        use crate::value::*;
        let x = Value::from(72.1);
        assert_eq!(x, Value::Number(Numeric::Real(72.1)));
    }

    #[test]
    fn is_of_type_is_correct() {
        use crate::value::*;
        let x = Value::from(137);
        assert!(x.is_of_type(&ValueType::Number));
        assert!(!x.is_of_type(&ValueType::Text));
    }

    #[test]
    fn ordering_test() {
        use crate::value::*;
        let a = Value::from(2);
        let b = Value::from(7);
        assert!(b > a);
    }
}
