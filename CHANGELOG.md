# Changelog

## Version 0.2

- **0.2.0**: Datakit `Value`s can now be parsed from literals following JSON
  syntax. For example, a string `"137"` can be parsed into a
  `Value::Number(Numeric::(137))`. Parsing of arrays and objects/dictionaries is
  also supported, as long as dictionary keys are strings.
- **0.2.0 (feature = experimental)**: Added parallel implementations of column
  and table schema/contract validations. Performance of these implementations
  seems [worse](research/parallel/parallel-routines-datakit.md). Keeping these
  implementations under the _experimental_ feature until I find a clear benefit
  of using them in main code.

## Version 0.1

- **0.1.0**: Added `datakit::value`.
- **0.1.0**: Added `datakit::table`.

# Roadmap

## Version 0.2

- **0.2.2**: Added DateTime primitives. Put `chrono` integration into a separate
  feature.
- **0.2.0**: Validate tables against foreign schemas. There should be a "strict"
  validation, i.e. the table must contain **only and exactly** the columns
  defined in the schema. Alternatively, there must be a "minimal" mode, where
  the validation checks, if a table contains **at least** the columns defined in the
  schema.
- **0.2.2**: Implement custom `serde` Serializer/Deserializer for
  `datakit::table`. DSV and JSON at least. For JSON there should be options to
  either serialize "raw", i.e. default serialization of Datakit values to JSON,
  or a "d3.js compatible" version, where a table is a list of objects with the
  same properties/fields.
