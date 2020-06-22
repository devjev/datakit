use crate::value::definition::*;

pub struct Parser {}

impl ParsesToValue for Parser {
    fn parse(&self, s: &str) -> Result<Value, ()> {
        // Using a Scanner, try to parse the value from the most
        // difficult one, to the most straightforward.
        // So, the order should be:
        // f32, f64, i32, i64, DateTime, Composite Objects, String
        //
        // N.B! Composite objects are assumed to be JSON notation.
        Err(())
    }
}
