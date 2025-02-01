use reqwest::blocking::{Client, RequestBuilder};
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Role {
    User,
    Assistant,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    seed: i64,
    max_completion_tokens: Option<u32>,
}

#[derive(Hash, Clone)]
pub struct QuerySetting<'a> {
    pub api_key: &'a str,
    pub model: &'a str,
    pub seed: i64,
    pub max_completion_tokens: Option<u32>,
}

impl QuerySetting<'_> {
    fn common_header(&self) -> RequestBuilder {
        let api_key_field = format!("Bearer {}", self.api_key);

        Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", api_key_field.as_str())
    }

    fn make_request_body(&self, messages: Vec<Message>) -> RequestBody {
        let Self {
            model,
            seed,
            max_completion_tokens,
            ..
        } = self.clone();

        RequestBody {
            model: model.to_string(),
            messages,
            seed,
            max_completion_tokens,
        }
    }

    pub fn query(&self, input_messages: &[&str]) -> eyre::Result<String> {
        let messages = input_messages
            .iter()
            .map(|s| Message {
                role: Role::User,
                content: s.to_string(),
            })
            .collect();

        let response_body = self
            .common_header()
            .json(&self.make_request_body(messages))
            .send()?;

        let response_body = response_body.text()?;

        let body: Value = serde_json::from_slice(response_body.as_bytes())?;

        match &body["choices"][0]["message"]["content"] {
            Value::String(s) => Ok(s.clone()),
            _ => Ok(format!("[Unexpected response]\n{}", body)),
        }
    }
}
