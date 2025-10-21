use crate::engine::{InferenceEngine, TokenStream};
use async_trait::async_trait;
use anyhow::Result as AnyResult;
use futures_util::stream;
use crate::models::InferenceRequest;
use std::sync::Arc;

pub struct MockEngine {}

impl MockEngine {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl InferenceEngine for MockEngine {
    async fn get_available_models(&self) -> Vec<String> {
        vec!["mock-model".to_string()]
    }

    async fn run_streaming_inference(&self, request: InferenceRequest) -> AnyResult<TokenStream> {
        let replies: Vec<String> = vec!["hello".to_string(), " ".to_string(), request.prompt.clone(), "\n".to_string(), "done".to_string()];
        let s = stream::iter(replies.into_iter().map(|s| Ok(s)));
        let boxed: TokenStream = Box::pin(s);
        Ok(boxed)
    }
}

pub fn boxed(engine: Arc<dyn InferenceEngine>) -> Arc<dyn InferenceEngine> { engine }
