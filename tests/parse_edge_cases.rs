use llm_inference::parse;

#[test]
fn missing_fields() {
    let json = r#"{"model-name":"a"}"#;
    let res = parse::parse_args(json);
    assert!(res.is_err());
}

#[test]
fn default_values_present() {
    let json = r#"{"model-name":"a","model-dir":"models/","prompt":"hi","repeat-penalty":1.0,"stop":[]}"#;
    let args = parse::parse_args(json).expect("parse");
    // these fields have defaults; ensure they are present
    assert_eq!(args.max_token, 128);
    assert_eq!(args.temperature, 0.7);
}
