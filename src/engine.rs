use std::sync::Arc;
use async_trait::async_trait;
use futures_util::Stream;
use anyhow::Result as AnyResult;
use anyhow::Context;
use crate::models::InferenceRequest;

// Token 流的类型别名
pub type TokenStream = std::pin::Pin<Box<dyn Stream<Item = AnyResult<String>> + Send>>;

/// 推理引擎抽象 - 作为服务与底层推理逻辑之间的边界
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    /// 获取可用模型列表（初期可以返回硬编码信息）
    async fn get_available_models(&self) -> Vec<String>;

    /// 执行流式推理，返回 token 流
    async fn run_streaming_inference(&self, request: InferenceRequest) -> AnyResult<TokenStream>;
}

use mistralrs::{Device, PagedAttentionMetaBuilder, TextModelBuilder, Model};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// M1 的适配器实现：将原先 CLI 风格的推理逻辑封装为服务可用的引擎
pub struct M1EngineAdapter {
    // 缓存已加载的模型实例：model_id -> TextModel
    models: Mutex<HashMap<String, Arc<Model>>>,
    // 可用模型（初期硬编码或从配置加载）
    available: Vec<String>,
}

impl M1EngineAdapter {
    pub fn new(available: Vec<String>) -> Self {
        Self {
            models: Mutex::new(HashMap::new()),
            available,
        }
    }

    /// 内部：根据 model_id 懒加载模型并缓存
    async fn get_or_load_model(&self, model_id: &str, device: &str) -> AnyResult<Arc<Model>> {
        // check cache first
        {
            let guard = self.models.lock().await;
            if let Some(m) = guard.get(model_id) {
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
                        tracing::warn!("⚠️ CUDA requested but not available: {:?}. Falling back to CPU.", e);
                        Device::Cpu
                    }
                }
            },
            "metal" => Device::new_metal(0).unwrap_or(Device::Cpu),
            _ => Device::Cpu,
        };

        let builder = TextModelBuilder::new(model_id)
            .with_device(dev)
            .with_logging()
            .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?;

        let model = builder.build().await.context("failed to build/load model")?;
        let arc = Arc::new(model);
        let mut guard = self.models.lock().await;
        guard.insert(model_id.to_string(), arc.clone());
        Ok(arc)
    }
}

#[async_trait]
impl InferenceEngine for M1EngineAdapter {
    async fn get_available_models(&self) -> Vec<String> {
        self.available.clone()
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