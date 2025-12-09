use crate::config::ModelConfig;
use crate::models::InferenceRequest;
use anyhow::Result as AnyResult;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures_util::Stream;
use std::sync::Arc;

// another type name for TokenStream
pub type TokenStream = std::pin::Pin<Box<dyn Stream<Item = AnyResult<String>> + Send>>;

/// inference engine abtract between service and base
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    /// get available model list
    async fn get_available_models(&self) -> Vec<String>;

    /// run streaming inference and return TokenStream
    async fn run_streaming_inference(&self, request: InferenceRequest) -> AnyResult<TokenStream>;
}

use mistralrs::{Device, Model, PagedAttentionMetaBuilder, TextModelBuilder};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// M1 engine adapter realization
pub struct M1EngineAdapter {
    // cache loaded model canonical_id -> TextModel
    models: Mutex<HashMap<String, Arc<Model>>>,
    // canonical id -> ModelConfig
    model_configs: HashMap<String, ModelConfig>,
    // alias (id/name) -> canonical id
    model_aliases: HashMap<String, String>,
    // model name list for display
    model_names: Vec<String>,
}

impl M1EngineAdapter {
    pub fn new(configs: Vec<ModelConfig>) -> Self {
        let mut model_configs = HashMap::new();
        let mut model_aliases = HashMap::new();
        let mut model_names = Vec::new();

        for config in configs {
            model_aliases.insert(config.id.clone(), config.id.clone());
            model_aliases.insert(config.name.clone(), config.id.clone());
            model_names.push(config.name.clone());
            model_configs.insert(config.id.clone(), config);
        }

        Self {
            models: Mutex::new(HashMap::new()),
            model_configs,
            model_aliases,
            model_names,
        }
    }

    /// Pre-warm the model by loading it into cache
    pub async fn warmup(&self, model_id: &str, device: &str) -> AnyResult<()> {
        let (canonical_id, config) = self.resolve_model(model_id)?;
        tracing::info!(
            "🔥 Pre-warming model: {} ({}) on device: {}",
            config.name,
            canonical_id,
            device
        );
        self.get_or_load_model(&canonical_id, device).await?;
        tracing::info!("✅ Model pre-warmed and cached: {}", config.name);
        Ok(())
    }

    /// load model and cache
    async fn get_or_load_model(&self, model_id: &str, device: &str) -> AnyResult<Arc<Model>> {
        let (canonical_id, config) = self.resolve_model(model_id)?;

        // check cache first
        {
            let guard = self.models.lock().await;
            if let Some(m) = guard.get(&canonical_id) {
                return Ok(m.clone());
            }
        }

        // not found -> build
        let dev = match device.to_lowercase().as_str() {
            "cuda" => {
                #[cfg(not(feature = "cuda"))]
                tracing::warn!("⚠️ 'cuda' device requested but 'cuda' feature is NOT enabled. This will likely cause CPU fallback. Run with '--features cuda'.");

                match Device::cuda_if_available(0) {
                    Ok(d) => {
                        tracing::info!("✅ Successfully initialized CUDA device.");
                        d
                    }
                    Err(e) => {
                        tracing::warn!(
                            "⚠️ CUDA requested but not available: {:?}. Falling back to CPU.",
                            e
                        );
                        Device::Cpu
                    }
                }
            }
            "metal" => Device::new_metal(0).unwrap_or(Device::Cpu),
            _ => Device::Cpu,
        };

        let identifier = config
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| config.name.clone());

        let builder = TextModelBuilder::new(&identifier)
            .with_device(dev)
            .with_logging()
            .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?;

        let model = builder
            .build()
            .await
            .context("failed to build/load model")?;
        let arc = Arc::new(model);
        let mut guard = self.models.lock().await;
        guard.insert(canonical_id, arc.clone());
        Ok(arc)
    }

    fn resolve_model(&self, model_id: &str) -> AnyResult<(String, ModelConfig)> {
        let canonical_id = self
            .model_aliases
            .get(model_id)
            .cloned()
            .ok_or_else(|| anyhow!("Model '{}' not configured", model_id))?;
        let config = self
            .model_configs
            .get(&canonical_id)
            .cloned()
            .ok_or_else(|| anyhow!("Model '{}' not configured", model_id))?;
        Ok((canonical_id, config))
    }
}

#[async_trait]
impl InferenceEngine for M1EngineAdapter {
    async fn get_available_models(&self) -> Vec<String> {
        self.model_names.clone()
    }

    async fn run_streaming_inference(&self, request: InferenceRequest) -> AnyResult<TokenStream> {
        // Use cached model (or load) and create a stream using the model directly. This avoids
        // rebuilding models for every request and makes `get_or_load_model` actually used.
        let model_id = request.model_name.clone();
        let device = request.device.clone();

        let model = self.get_or_load_model(&model_id, &device).await?;

        let mut messages = mistralrs::TextMessages::new();

        if let Some(msgs) = &request.messages {
            for msg in msgs {
                let role = match msg.role.to_lowercase().as_str() {
                    "user" => mistralrs::TextMessageRole::User,
                    "assistant" => mistralrs::TextMessageRole::Assistant,
                    "system" => mistralrs::TextMessageRole::System,
                    _ => mistralrs::TextMessageRole::User,
                };
                messages = messages.add_message(role, &msg.content);
            }
        } else {
            messages = messages.add_message(mistralrs::TextMessageRole::User, &request.prompt);
        }

        let mut req = mistralrs::RequestBuilder::from(messages)
            .set_sampler_max_len(request.max_token)
            .set_sampler_temperature(request.temperature);

        if request.top_k > 0 {
            req = req.set_sampler_topk(request.top_k as usize);
        }
        if (0.0..1.0).contains(&request.top_p) {
            req = req.set_sampler_topp(request.top_p);
        }
        if request.repeat_penalty != 1.0 {
            let mut sp = mistralrs::SamplingParams::deterministic();
            sp.max_len = Some(request.max_token);
            sp.temperature = Some(request.temperature);
            if request.top_k > 0 {
                sp.top_k = Some(request.top_k as usize);
            }
            if (0.0..1.0).contains(&request.top_p) {
                sp.top_p = Some(request.top_p);
            }
            sp.repetition_penalty = Some(request.repeat_penalty);
            if !request.stop.is_empty() {
                sp.stop_toks = Some(mistralrs::StopTokens::Seqs(request.stop.clone()));
            }
            req = req.set_sampling(sp);
        } else if !request.stop.is_empty() {
            req = req.set_sampler_stop_toks(mistralrs::StopTokens::Seqs(request.stop.clone()));
        }

        use async_stream::try_stream;

        let model_clone = model.clone();
        let req_clone = req;

        let s = try_stream! {
            let mut inner = model_clone.stream_chat_request(req_clone).await?;
            while let Some(chunk) = inner.next().await {
                match chunk {
                    mistralrs::Response::Chunk(mistralrs::ChatCompletionChunkResponse { choices, .. }) => {
                        if let Some(mistralrs::ChunkChoice { delta: mistralrs::Delta { content: Some(c), .. }, .. }) = choices.first() {
                            yield c.clone();
                        } else {
                            yield String::new();
                        }
                    }
                    _ => continue,
                }
            }
        };

        let boxed: TokenStream = Box::pin(s);
        Ok(boxed)
    }
}
