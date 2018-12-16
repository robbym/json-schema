use matches::assert_matches;
use serde_json::{Number, Value};

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

#[derive(Debug)]
struct MinimumValidator {
    value: Number,
}
impl Validator for MinimumValidator {
    fn validate(&self, value: &Value) -> bool {
        if let Value::Number(num) = value {
            if let (Some(n1), Some(n2)) = (self.value.as_f64(), num.as_f64()) {
                n1 <= n2
            } else {
                false
            }
        } else {
            true
        }
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
                if let Value::Number(val) = val {
                    return Ok(Box::new(MinimumValidator {
                        value: val.clone(),
                    }));
                }
            }

            if let Some(val) = obj.get("type") {
                assert_matches!(val, Value::String(_));
                return Err(schema);
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
            return Err(schema);
        }
    }
}
