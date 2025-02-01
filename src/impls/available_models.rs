use crate::impls::macro_::IntoSynRes;
use proc_macro2::Span;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use syn::LitStr;

const DEFAULT_MODEL: &str = "gpt-4o";

fn get_available_gpt_models(api_key: &str) -> eyre::Result<Vec<String>> {
    let api_key_field = format!("Bearer {}", api_key);

    let body: Value = Client::new()
        .get("https://api.openai.com/v1/models")
        .header("Authorization", api_key_field.as_str())
        .send()?
        .json()?;

    let model_array = match &body["data"] {
        Value::Array(array) => array,
        _ => eyre::bail!("Unexpected response: {:?}", body),
    };

    let model_names = model_array
        .iter()
        .map(|value| match &value["id"] {
            Value::String(s) => Ok(s.clone()),
            _ => eyre::bail!("Unexpected response: {:?}", value),
        })
        .collect::<eyre::Result<Vec<String>>>()?;

    Ok(model_names)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ModelsCache {
    available_models: Vec<String>,
}

// モデル一覧は基本キャッシュしておき、存在しない場合だけAPIを叩く
fn available_gpt_models(api_key: &str) -> eyre::Result<Vec<String>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let cache_dir = format!("{}/gpt_responses", manifest_dir);

    if !fs::exists(&cache_dir)? {
        fs::create_dir_all(&cache_dir)?;
    }

    let cached_models_file_name = PathBuf::from(cache_dir).join("available_models.toml");

    let available_models = match fs::read_to_string(&cached_models_file_name) {
        Ok(content) => {
            let cache: ModelsCache = toml::from_str(&content)?;
            cache.available_models
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let available_models = get_available_gpt_models(api_key)?;

            let cache = ModelsCache {
                available_models: available_models.clone(),
            };
            // tomlとして保存するために変換
            let cache = toml::to_string(&cache)?;
            // 結果を保存
            fs::write(&cached_models_file_name, cache)?;

            available_models
        }
        Err(e) => return Err(e.into()),
    };

    Ok(available_models)
}

pub fn check_available(api_key: &str, model_lit: Option<LitStr>) -> syn::Result<String> {
    let lit = model_lit.unwrap_or(LitStr::new(DEFAULT_MODEL, Span::call_site()));
    let span = lit.span();
    let model_name = lit.value();

    let available_models = available_gpt_models(api_key).into_syn(span)?;

    if !available_models.contains(&model_name) {
        return Err(syn::Error::new(
            span,
            format!("Model {} is not available", model_name),
        ));
    }

    Ok(model_name)
}
