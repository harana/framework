// Harana Actions - Cloudflare AI Module
// This module provides Cloudflare Workers AI actions for running ML models.

pub mod output;

use output::*;
use serde::Serialize;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("Workers AI error: {e}")
}

/// Run Workers AI Model
pub async fn run(
    env: &Env,
    binding: &str,
    model: &str,
    params: serde_json::Value,
    _stream: Option<bool>,
) -> Result<RunOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;
    let response: serde_json::Value = ai.run(model, params).await.map_err(to_err)?;
    Ok(RunOutput { response })
}

/// Run Workers AI Text Generation
#[allow(clippy::too_many_arguments)]
pub async fn text_generation(
    env: &Env,
    binding: &str,
    model: &str,
    prompt: Option<&str>,
    messages: Option<Vec<WorkersAiMessage>>,
    max_tokens: Option<i32>,
    temperature: Option<f64>,
    top_k: Option<i32>,
    top_p: Option<f64>,
    frequency_penalty: Option<f64>,
    presence_penalty: Option<f64>,
    repetition_penalty: Option<f64>,
    _stream: Option<bool>,
) -> Result<TextGenerationOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        messages: Option<Vec<WorkersAiMessage>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tokens: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        temperature: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        top_k: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        top_p: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        frequency_penalty: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        presence_penalty: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        repetition_penalty: Option<f64>,
    }

    let input = Input {
        prompt: prompt.map(|s| s.to_string()),
        messages,
        max_tokens,
        temperature,
        top_k,
        top_p,
        frequency_penalty,
        presence_penalty,
        repetition_penalty,
    };

    #[derive(serde::Deserialize)]
    struct AiResponse {
        response: String,
    }

    let result: AiResponse = ai.run(model, input).await.map_err(to_err)?;
    Ok(TextGenerationOutput {
        response: result.response,
    })
}

/// Run Workers AI Text Embeddings
pub async fn text_embeddings(
    env: &Env,
    binding: &str,
    model: &str,
    text: Vec<String>,
) -> Result<TextEmbeddingsOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        text: Vec<String>,
    }

    #[derive(serde::Deserialize)]
    struct AiResponse {
        data: Vec<Vec<f32>>,
        shape: Vec<i32>,
    }

    let result: AiResponse = ai.run(model, Input { text }).await.map_err(to_err)?;
    Ok(TextEmbeddingsOutput {
        data: result.data,
        shape: result.shape,
    })
}

/// Run Workers AI Text Classification
pub async fn text_classification(
    env: &Env,
    binding: &str,
    model: &str,
    text: &str,
) -> Result<TextClassificationOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        text: String,
    }

    let result: Vec<WorkersAiClassification> = ai.run(model, Input { text: text.to_string() }).await.map_err(to_err)?;
    Ok(TextClassificationOutput { labels: result })
}

/// Run Workers AI Translation
pub async fn translation(
    env: &Env,
    binding: &str,
    model: &str,
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<TranslationOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        text: String,
        source_lang: String,
        target_lang: String,
    }

    #[derive(serde::Deserialize)]
    struct AiResponse {
        translated_text: String,
    }

    let result: AiResponse = ai
        .run(
            model,
            Input {
                text: text.to_string(),
                source_lang: source_lang.to_string(),
                target_lang: target_lang.to_string(),
            },
        )
        .await
        .map_err(to_err)?;
    Ok(TranslationOutput {
        translated_text: result.translated_text,
    })
}

/// Run Workers AI Image Classification
pub async fn image_classification(
    env: &Env,
    binding: &str,
    model: &str,
    image: Vec<u8>,
) -> Result<ImageClassificationOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    let result: Vec<WorkersAiClassification> = ai.run(model, image).await.map_err(to_err)?;
    Ok(ImageClassificationOutput { labels: result })
}

/// Run Workers AI Object Detection
pub async fn object_detection(
    env: &Env,
    binding: &str,
    model: &str,
    image: Vec<u8>,
) -> Result<ObjectDetectionOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    let result: Vec<WorkersAiDetection> = ai.run(model, image).await.map_err(to_err)?;
    Ok(ObjectDetectionOutput { detections: result })
}

/// Run Workers AI Text To Image
pub async fn text_to_image(
    env: &Env,
    binding: &str,
    model: &str,
    prompt: &str,
    guidance: Option<f64>,
    height: Option<i32>,
    width: Option<i32>,
    num_steps: Option<i32>,
) -> Result<TextToImageOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        prompt: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        guidance: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        num_steps: Option<i32>,
    }

    let result: Vec<u8> = ai
        .run(
            model,
            Input {
                prompt: prompt.to_string(),
                guidance,
                height,
                width,
                num_steps,
            },
        )
        .await
        .map_err(to_err)?;
    Ok(TextToImageOutput { image: result })
}

/// Run Workers AI Speech Recognition
pub async fn speech_recognition(
    env: &Env,
    binding: &str,
    model: &str,
    audio: Vec<u8>,
    source_lang: Option<&str>,
) -> Result<SpeechRecognitionOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        audio: Vec<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        source_lang: Option<String>,
    }

    let result: SpeechRecognitionOutput = ai
        .run(
            model,
            Input {
                audio,
                source_lang: source_lang.map(|s| s.to_string()),
            },
        )
        .await
        .map_err(to_err)?;
    Ok(result)
}

/// Run Workers AI Text To Speech
pub async fn text_to_speech(env: &Env, binding: &str, model: &str, text: &str) -> Result<TextToSpeechOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        text: String,
    }

    let result: Vec<u8> = ai.run(model, Input { text: text.to_string() }).await.map_err(to_err)?;
    Ok(TextToSpeechOutput { audio: result })
}

/// Run Workers AI Summarization
pub async fn summarization(
    env: &Env,
    binding: &str,
    model: &str,
    input_text: &str,
    max_length: Option<i32>,
) -> Result<SummarizationOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        input_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_length: Option<i32>,
    }

    #[derive(serde::Deserialize)]
    struct AiResponse {
        summary: String,
    }

    let result: AiResponse = ai
        .run(
            model,
            Input {
                input_text: input_text.to_string(),
                max_length,
            },
        )
        .await
        .map_err(to_err)?;
    Ok(SummarizationOutput {
        summary: result.summary,
    })
}

/// Run Workers AI Image To Text
pub async fn image_to_text(
    env: &Env,
    binding: &str,
    model: &str,
    image: Vec<u8>,
    prompt: Option<&str>,
    max_tokens: Option<i32>,
) -> Result<ImageToTextOutput, String> {
    let ai = env.ai(binding).map_err(to_err)?;

    #[derive(Serialize)]
    struct Input {
        image: Vec<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tokens: Option<i32>,
    }

    #[derive(serde::Deserialize)]
    struct AiResponse {
        description: String,
    }

    let result: AiResponse = ai
        .run(
            model,
            Input {
                image,
                prompt: prompt.map(|s| s.to_string()),
                max_tokens,
            },
        )
        .await
        .map_err(to_err)?;
    Ok(ImageToTextOutput {
        description: result.description,
    })
}
