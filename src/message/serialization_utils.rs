/// Utilities for working with json
/// Author: Andrew Evans
use serde_json::Value;


fn serialize_json_payload_to_string(payload: Value) -> String{
    String::from("")
}


fn deserialize_string_payload_to_json(payload: String) {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_string_payload_to_json() {
        let jstr =  r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    }

    #[test]
    fn should_serialize_json_payload_to_string(){

    }
}
