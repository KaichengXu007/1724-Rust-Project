use anyhow::Result;
use llm_inference::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");



    //  TinyLlama/TinyLlama-1.1B-Chat-v1.0
    let json = r#"
{
  "model-name": "Qwen/Qwen3-0.6B",
  "model-dir": "models/",
  "prompt": "Explain ownership",
  "max_token": 64,
  "temperature": 0.7,
  "top-p": 0.95,
  "top-k": 10,
  "repeat-penalty": 1.1,
  "stop": [],
  "device": "metal"
}
"#;

    let args = parse::parse_args(json)?;
    println!("{args:#?}");

    model_inference::run(args).await?;
    Ok(())
}
