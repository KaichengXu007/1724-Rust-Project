use std::sync::Arc;
use crate::engine::InferenceEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<dyn InferenceEngine>,
}

impl AppState {
    pub fn new(engine: Arc<dyn InferenceEngine>) -> Self {
        Self { engine }
    }
}
