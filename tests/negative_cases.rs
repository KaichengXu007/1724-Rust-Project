use llm_inference::parse;
use serde_json::json;

#[test]
fn malformed_json() {
    let json = "{not a json}";
    let res = parse::parse_args(json);
    assert!(res.is_err());
}

#[test]
fn invalid_param_values() {
    // negative max_token should be rejected by type (serde) or be coerced
    let body = json!({"model-name":"a","model-dir":"models/","prompt":"p","repeat-penalty":1.0,"stop":[],"max-token": -5});
    let res = parse::parse_args(&body.to_string());
    assert!(res.is_err());
}
