#[cfg(test)]
mod tests {
    use std::sync::Once;
    use serde_json::Value;
    use rustrict::config::{models::Configuration, deserialize_yaml};


    fn sort_json_value(value: &mut Value) {
        if let Value::Array(ref mut arr) = value {
            arr.sort_by(|a, b| {
                let a_type = a.get("type").unwrap().as_str().unwrap();
                let b_type = b.get("type").unwrap().as_str().unwrap();
                a_type.cmp(b_type)
            });
        }
    }
    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            // Initialize color_eyre
            color_eyre::install().unwrap();
        });
    }

    #[test]
    fn test_deserialize_model() {
        setup();
        let yaml_data = r#"
model:
  schema: 1.1

types:
  user:
  # No relations needed for user as it's a base type

  domain:
    relations:
      member: [ user ]  # Directly relate users as members of a domain

  folder:
    relations:
      can_share: [ user ]
      owner:
        or:
          - [ user, domain#member ]
          - { from: parent_folder }
      parent_folder: [ folder ]
      viewer:
        or:
          - [ user, domain#member, { type: domain, relation: member } ]
          - writer
          - { relation: viewer, from: parent_folder }
        but_not:
          - { relation: blocked }
        "#;

        let model: Configuration = deserialize_yaml(yaml_data).unwrap();
        assert_eq!(model.model.schema, "1.1");
        assert_eq!(model.types.len(), 3);
        assert!(model.types.contains_key("user"));
        assert!(model.types.contains_key("domain"));
    }
}
