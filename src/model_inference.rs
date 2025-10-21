// M1: Model inference code (original CLI implementation).
// This module loads a mistral model and provides two entrypoints:
// - `run(args)` which preserves the original CLI behavior by printing tokens to stdout.
// - `stream_from_args(args)` a new async API that returns a boxed token stream (Stream<Item = Result<String, Error>>).
//
// Recent modifications: added `stream_from_args` to allow server adapters (M1EngineAdapter)
// to call the model and forward tokens as SSE without capturing stdout.
use crate::parse;
use anyhow::{Context, Result};
use mistralrs::{
    Device, IsqType, PagedAttentionMetaBuilder,
    RequestBuilder, StopTokens, TextMessageRole, TextMessages, TextModelBuilder,
};
use mistralrs::{ChatCompletionChunkResponse, ChunkChoice, Delta, Response};
use parse::Args;
use tokio::io::{stdout, AsyncWriteExt};
use async_stream::try_stream;
use futures_util::StreamExt as _;
use std::sync::Arc;
use std::pin::Pin;
use anyhow::Error;

fn parse_device(s: &str) -> Device {
    match s.to_lowercase().as_str() {
        "cuda" => Device::cuda_if_available(0).unwrap_or(Device::Cpu),
        "metal" => Device::new_metal(0).unwrap_or(Device::Cpu),
        _ => Device::Cpu,
    }
}

pub async fn run(args: Args) -> Result<()> {
    // reuse new stream_from_args to get a token stream, then print to stdout (preserve CLI behavior)
    let mut stream = stream_from_args(args).await?;
    let mut out = stdout();
    while let Some(chunk) = stream.as_mut().next().await {
        if let Ok(c) = chunk {
            out.write_all(c.as_bytes()).await?;
            out.flush().await?;
        }
    }
    out.write_all(b"\n").await?;
    Ok(())
}

/// 公共函数：给定 parse::Args，返回一个 boxed 的 token 流（Item = anyhow::Result<String>）
pub async fn stream_from_args(args: Args) -> Result<Pin<Box<dyn futures_util::Stream<Item = Result<String, Error>> + Send + 'static>>> {
    let model_id = &args.model_name;

    let builder = TextModelBuilder::new(model_id)
        .with_device(parse_device(&args.device))
        .with_isq(IsqType::Q4_0)
        .with_logging()
        .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?;

    let model = builder
        .build()
        .await
        .context("failed to build/load model")?;

    let model = Arc::new(model);

    let messages = TextMessages::new().add_message(TextMessageRole::User, &args.prompt);

    let mut req = RequestBuilder::from(messages)
        .set_sampler_max_len(args.max_token)
        .set_sampler_temperature(args.temperature);

    if args.top_k > 0 {
        req = req.set_sampler_topk(args.top_k as usize);
    }
    if (0.0..1.0).contains(&args.top_p) {
        req = req.set_sampler_topp(args.top_p);
    }
    if args.repeat_penalty != 1.0 {
        let mut sp = mistralrs::SamplingParams::deterministic();
        sp.max_len = Some(args.max_token);
        sp.temperature = Some(args.temperature);
        if args.top_k > 0 {
            sp.top_k = Some(args.top_k as usize);
        }
        if (0.0..1.0).contains(&args.top_p) {
            sp.top_p = Some(args.top_p);
        }
        sp.repetition_penalty = Some(args.repeat_penalty);
        if !args.stop.is_empty() {
            sp.stop_toks = Some(StopTokens::Seqs(args.stop.clone()));
        }
        req = req.set_sampling(sp);
    } else if !args.stop.is_empty() {
        req = req.set_sampler_stop_toks(StopTokens::Seqs(args.stop.clone()));
    }

    let model_clone = model.clone();
    let req_clone = req;

    let s = try_stream! {
        let mut inner = model_clone.stream_chat_request(req_clone).await?;
        while let Some(chunk) = inner.next().await {
            match chunk {
                Response::Chunk(ChatCompletionChunkResponse { choices, .. }) => {
                    if let Some(ChunkChoice { delta: Delta { content: Some(c), .. }, .. }) = choices.first() {
                        yield c.clone();
                    } else {
                        yield String::new();
                    }
                }
                _ => continue,
            }
        }
    };

    Ok(Box::pin(s))
}
