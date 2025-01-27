use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

fn get_cache_file_path(content: &str) -> PathBuf {
    let out_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let cache_dir = format!("{}/gpt_responses", out_dir);

    if !fs::exists(&cache_dir).expect("Failed to check if cache directory exists") {
        fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    }

    PathBuf::from(out_dir).join(format!("gpt_responses/cache_{}.txt", hash_content(content)))
}

pub fn load_cache(content: &str) -> Option<String> {
    let cache_file = get_cache_file_path(content);

    // キャッシュが存在するか確認
    if cache_file.exists() {
        // キャッシュが存在する場合は読み込む
        let response = fs::read_to_string(cache_file).expect("Failed to read cache file");
        Some(response)
    } else {
        None
    }
}

pub fn cache_result(content: &str, response: &str) {
    let cache_file = get_cache_file_path(content);

    // 結果を保存
    fs::write(cache_file, response).expect("Failed to write cache file");
}

pub fn hash_content(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
