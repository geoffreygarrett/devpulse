use serde_json::{Value, Map};
use std::collections::HashMap;

trait Flattener {
    fn flatten(&self, value: &Value) -> HashMap<String, Value>;
}

struct JsonFlattener;

impl Flattener for JsonFlattener {
    fn flatten(&self, value: &Value) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        self.flatten_recursive(value, "", &mut result);
        result
    }
}

impl JsonFlattener {
    fn flatten_recursive(&self, value: &Value, prefix: &str, result: &mut HashMap<String, Value>) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.to_string()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    self.flatten_recursive(v, &new_prefix, result);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    self.flatten_recursive(v, &new_prefix, result);
                }
            }
            _ => {
                result.insert(prefix.to_string(), value.clone());
            }
        }
    }
}

pub fn flatten_json(value: &Value) -> Value {
    let flattener = JsonFlattener;
    let flattened = flattener.flatten(value);
    Value::Object(flattened.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_flatten_json() {
        let value = json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": [3, 4, 5],
            },
        });

        let flattened = flatten_json(&value);

        let expected = json!({
            "a": 1,
            "b.c": 2,
            "b.d[0]": 3,
            "b.d[1]": 4,
            "b.d[2]": 5
        });

        assert_eq!(flattened, expected);
    }
}