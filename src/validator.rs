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
