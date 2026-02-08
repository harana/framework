// Harana Actions - Cloudflare Queue Module
// This module provides Cloudflare Queue actions for sending, acknowledging,
// retrying, and processing queue messages.

pub mod output;

use output::*;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("Queue error: {e}")
}

/// Send Queue Message
pub async fn send(
    env: &Env,
    queue: &str,
    message: serde_json::Value,
    _content_type: Option<&str>,
    delay_seconds: Option<i32>,
) -> Result<SendOutput, String> {
    let q = env.queue(queue).map_err(to_err)?;

    if let Some(delay) = delay_seconds {
        let msg = worker::MessageBuilder::new(message).delay_seconds(delay as u32).build();
        q.send(msg).await.map_err(to_err)?;
    } else {
        q.send(message).await.map_err(to_err)?;
    }

    Ok(SendOutput { success: true })
}

/// Send Queue Message Batch
pub async fn send_batch(env: &Env, queue: &str, messages: Vec<QueueBatchMessage>) -> Result<SendBatchOutput, String> {
    let q = env.queue(queue).map_err(to_err)?;

    let send_messages: Vec<worker::SendMessage<serde_json::Value>> = messages
        .into_iter()
        .map(|m| {
            let mut builder = worker::MessageBuilder::new(m.body);
            if let Some(delay) = m.delay_seconds {
                builder = builder.delay_seconds(delay as u32);
            }
            builder.build()
        })
        .collect();

    match q.send_batch(send_messages).await {
        Ok(()) => Ok(SendBatchOutput {
            success: true,
            failed_messages: vec![],
        }),
        Err(e) => Ok(SendBatchOutput {
            success: false,
            failed_messages: vec![QueueFailedMessage {
                body: serde_json::Value::Null,
                error: format!("Batch send failed: {e}"),
            }],
        }),
    }
}
