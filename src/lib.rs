use matches::assert_matches;
use serde_json::Value;

pub trait Validator: std::fmt::Debug {
    fn validate(&self, value: &Value) -> bool;
}

#[derive(Debug)]
struct TrueValidator;
impl Validator for TrueValidator {
    fn validate(&self, _value: &Value) -> bool {
        true
    }
}

#[derive(Debug)]
struct FalseValidator;
impl Validator for FalseValidator {
    fn validate(&self, _value: &Value) -> bool {
        false
    }
}

#[derive(Debug)]
struct ConstValidator {
    value: Value,
}
impl Validator for ConstValidator {
    fn validate(&self, value: &Value) -> bool {
        &self.value == value
    }
}

#[derive(Debug)]
struct EnumValidator {
    validators: Vec<Box<dyn Validator>>,
}
impl Validator for EnumValidator {
    fn validate(&self, value: &Value) -> bool {
        self.validators.iter().any(|v| v.validate(value))
    }
}

pub fn generate_validator(schema: &Value) -> Result<Box<dyn Validator>, &Value> {
    match schema {
        Value::Object(obj) => {
            if let Some(val) = obj.get("const") {
                assert!(obj.len() == 1);
                return Ok(Box::new(ConstValidator { value: val.clone() }));
            }

            if let Some(val) = obj.get("enum") {
                assert!(obj.len() == 1);
                assert_matches!(val, Value::Array(_));

                if let Value::Array(items) = val {
                    let validators = items
                        .iter()
                        .map(|val| {
                            Box::new(ConstValidator { value: val.clone() }) as Box<dyn Validator>
                        })
                        .collect();
                    return Ok(Box::new(EnumValidator { validators }));
                }
            }

            if let Some(val) = obj.get("minimum") {
                assert!(obj.len() == 1);
                assert_matches!(val, Value::Number(_));
                return Ok(Box::new(ConstValidator { value: val.clone() }));
            }

            if let Some(val) = obj.get("type") {
                assert_matches!(val, Value::String(_));
                unimplemented!();
            }

            Ok(Box::new(ConstValidator {
                value: schema.clone(),
            }))
        }

        Value::Bool(val) => {
            if *val {
                Ok(Box::new(TrueValidator {}))
            } else {
                Ok(Box::new(FalseValidator {}))
            }
        }

        _ => {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod tests {
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

            match super::generate_validator(&schema_test.schema) {
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
