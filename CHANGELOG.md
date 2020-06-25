# Changelog

## Version 0.2

- **0.2.0**: Datakit `Value`s can now be parsed from literals following JSON
  syntax. For example, a string `"137"` can be parsed into a
  `Value::Number(Numeric::(137))`. Parsing of arrays and objects/dictionaries is
  also supported, as long as dictionary keys are strings.

## Version 0.1

- **0.1.0**: Added `datakit::value`.
- **0.1.0**: Added `datakit::table`.

# Roadmap

## Version 0.2

- **0.2.0**: Refine schema compatibility checks for Table.
- **0.2.0**: ~~Value contract and column contract checks need to be parallel.~~
  No performance gain there, needs more work.

## Version 0.3

- **0.3.0**: Implement custom `serde` Serializer/Deserializer for
  `datakit::table`. DSV and JSON at least. For JSON there should be options to either serialize
  "raw", i.e. default serialization of Datakit values to JSON, or a "d3.js compatible" version,
  where a table is a list of objects with the same properties/fields.
