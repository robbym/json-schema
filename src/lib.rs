pub mod validator;

#[cfg(test)]
mod tests {
    use super::validator;
    use serde_derive::Deserialize;
    use serde_json::Value;
    use std::fs::File;

    #[derive(Deserialize)]
    struct SchemaTest {
        description: String,
        schema: Value,
        tests: Vec<Test>,
    }

    #[derive(Deserialize)]
    struct Test {
        description: String,
        data: Value,
        valid: bool,
    }

    #[derive(Debug)]
    struct TestFailures {
        description: String,
        passes: Vec<String>,
        failures: Vec<String>,
    }

    fn read_tests(file: &str) -> Vec<SchemaTest> {
        serde_json::from_reader(File::open(file).unwrap()).unwrap()
    }

    fn run_tests(file: &str) {
        for schema_test in read_tests(file) {
            let mut passes = vec![];
            let mut failures = vec![];

            match validator::generate_validator(&schema_test.schema) {
                Ok(validator) => {
                    for test in schema_test.tests {
                        if validator.validate(&test.data) != test.valid {
                            failures.push(format!("FAILED {}", test.description));
                        } else {
                            passes.push(format!("PASSED {}", test.description));
                        }
                    }
                }
                Err(value) => {
                    failures.push(format!("UNIMPLEMENTED '{}'", value));
                }
            }

            if failures.len() > 0 {
                panic!(
                    "{:#?}",
                    TestFailures {
                        description: schema_test.description,
                        passes,
                        failures
                    }
                );
            }
        }
    }

    #[test]
    fn test_additional_items() {
        run_tests("test-suite/tests/draft7/additionalItems.json");
    }

    #[test]
    fn test_additional_properties() {
        run_tests("test-suite/tests/draft7/additionalProperties.json");
    }

    #[test]
    fn test_all_of() {
        run_tests("test-suite/tests/draft7/allOf.json");
    }

    #[test]
    fn test_any_of() {
        run_tests("test-suite/tests/draft7/anyOf.json");
    }

    #[test]
    fn test_boolean_schema() {
        run_tests("test-suite/tests/draft7/boolean_schema.json");
    }

    #[test]
    fn test_const() {
        run_tests("test-suite/tests/draft7/const.json");
    }

    #[test]
    fn test_contains() {
        run_tests("test-suite/tests/draft7/contains.json");
    }

    #[test]
    fn test_default() {
        run_tests("test-suite/tests/draft7/default.json");
    }

    #[test]
    fn test_definitions() {
        run_tests("test-suite/tests/draft7/definitions.json");
    }

    #[test]
    fn test_dependencies() {
        run_tests("test-suite/tests/draft7/dependencies.json");
    }

    #[test]
    fn test_enum() {
        run_tests("test-suite/tests/draft7/enum.json");
    }

    #[test]
    fn test_exclusive_maximum() {
        run_tests("test-suite/tests/draft7/exclusiveMaximum.json");
    }

    #[test]
    fn test_exclusive_minimum() {
        run_tests("test-suite/tests/draft7/exclusiveMinimum.json");
    }

    #[test]
    fn test_if_then_else() {
        run_tests("test-suite/tests/draft7/if-then-else.json");
    }

    #[test]
    fn test_items() {
        run_tests("test-suite/tests/draft7/items.json");
    }

    #[test]
    fn test_maximum() {
        run_tests("test-suite/tests/draft7/maximum.json");
    }

    #[test]
    fn test_max_items() {
        run_tests("test-suite/tests/draft7/maxItems.json");
    }

    #[test]
    fn test_max_length() {
        run_tests("test-suite/tests/draft7/maxLength.json");
    }

    #[test]
    fn test_max_properties() {
        run_tests("test-suite/tests/draft7/maxProperties.json");
    }

    #[test]
    fn test_minimum() {
        run_tests("test-suite/tests/draft7/minimum.json");
    }

    #[test]
    fn test_min_items() {
        run_tests("test-suite/tests/draft7/minItems.json");
    }

    #[test]
    fn test_min_length() {
        run_tests("test-suite/tests/draft7/minLength.json");
    }

    #[test]
    fn test_min_properties() {
        run_tests("test-suite/tests/draft7/minProperties.json");
    }

    #[test]
    fn test_multiple_of() {
        run_tests("test-suite/tests/draft7/multipleOf.json");
    }

    #[test]
    fn test_not() {
        run_tests("test-suite/tests/draft7/not.json");
    }

    #[test]
    fn test_one_of() {
        run_tests("test-suite/tests/draft7/oneOf.json");
    }

    #[test]
    fn test_pattern() {
        run_tests("test-suite/tests/draft7/pattern.json");
    }

    #[test]
    fn test_pattern_properties() {
        run_tests("test-suite/tests/draft7/patternProperties.json");
    }

    #[test]
    fn test_properties() {
        run_tests("test-suite/tests/draft7/properties.json");
    }

    #[test]
    fn test_property_names() {
        run_tests("test-suite/tests/draft7/propertyNames.json");
    }

    #[test]
    fn test_ref() {
        run_tests("test-suite/tests/draft7/ref.json");
    }

    #[test]
    fn test_ref_remote() {
        run_tests("test-suite/tests/draft7/refRemote.json");
    }

    #[test]
    fn test_required() {
        run_tests("test-suite/tests/draft7/required.json");
    }

    #[test]
    fn test_type() {
        run_tests("test-suite/tests/draft7/type.json");
    }

    #[test]
    fn test_unique_items() {
        run_tests("test-suite/tests/draft7/uniqueItems.json");
    }

}
