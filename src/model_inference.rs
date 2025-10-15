use crate::parse;
use anyhow::{Context, Result};
use mistralrs::{
    AutoDeviceMapParams, Device, DeviceMapSetting, IsqType, Model, PagedAttentionMetaBuilder,
    RequestBuilder, StopTokens, TextMessageRole, TextMessages, TextModelBuilder,
};
use mistralrs::{ChatCompletionChunkResponse, ChunkChoice, Delta, Response};
use parse::Args;
use tokio::io::{stdout, AsyncWriteExt};

fn parse_device(s: &str) -> Device {
    match s.to_lowercase().as_str() {
        "cuda" => Device::cuda_if_available(0).unwrap_or(Device::Cpu),
        "metal" => Device::new_metal(0).unwrap_or(Device::Cpu),
        _ => Device::Cpu,
    }
}

pub async fn run(args: Args) -> Result<()> {
    // format : hf id
    // if exist, load from local cache else download and load
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

    println!("model loaded");

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

    // stdout
    let mut stream = model.stream_chat_request(req).await?;
    let mut out = stdout();

    while let Some(chunk) = stream.next().await {
        if let Response::Chunk(ChatCompletionChunkResponse { choices, .. }) = chunk {
            if let Some(ChunkChoice {
                delta: Delta {
                    content: Some(c), ..
                },
                ..
            }) = choices.first()
            {
                out.write_all(c.as_bytes()).await?;
                out.flush().await?;
            }
        }
    }
    out.write_all(b"\n").await?;
    Ok(())
}
