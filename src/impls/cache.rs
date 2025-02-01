use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use crate::impls::query::QuerySetting;

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedContent {
    model: String,
    seed: i64,
    max_completion_tokens: Option<u32>,
    response: String,
}

fn get_cache_file_path(setting: &QuerySetting<'_>, content: &str) -> eyre::Result<PathBuf> {
    let out_dir = env::var("CARGO_MANIFEST_DIR")?;
    let cache_dir = format!("{}/gpt_responses", out_dir);

    if !fs::exists(&cache_dir)? {
        fs::create_dir_all(&cache_dir)?;
    }

    Ok(PathBuf::from(out_dir).join(format!(
        "gpt_responses/cache_{}.toml",
        hash_content(&(setting, content))
    )))
}

pub fn load_cache(setting: &QuerySetting<'_>, content: &str) -> eyre::Result<Option<String>> {
    let cache_file = get_cache_file_path(setting, content)?;

    // キャッシュを読み込む
    let response = fs::read_to_string(cache_file);

    match response {
        Ok(response) => {
            let response: CachedContent = toml::from_str(&response)?;
            Ok(Some(response.response.to_string()))
        }
        // 存在しない場合
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn cache_result(setting: &QuerySetting<'_>, content: &str, response: &str) -> eyre::Result<()> {
    let cache_file = get_cache_file_path(setting, content)?;

    let QuerySetting {
        model,
        seed,
        max_completion_tokens,
        ..
    } = setting;

    let contents = CachedContent {
        model: model.to_string(),
        seed: *seed,
        max_completion_tokens: *max_completion_tokens,
        response: response.to_string(),
    };

    let contents = toml::to_string(&contents)?;

    // 結果を保存
    fs::write(cache_file, contents)?;

    Ok(())
}

pub fn hash_content<H: Hash>(key: &H) -> i64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() % (i64::MAX as u64)) as _
}
