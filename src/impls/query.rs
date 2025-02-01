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
pub struct QuerySetting {
    pub api_key: String,
    pub model: String,
    pub seed: i64,
    pub max_completion_tokens: Option<u32>,
}

impl QuerySetting {
    fn header(&self) -> RequestBuilder {
        let api_key_field = format!("Bearer {}", self.api_key);

        Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", api_key_field.as_str())
    }

    fn make_request_body(&self, messages: &[&str]) -> RequestBody {
        let messages = messages
            .iter()
            .map(|s| Message {
                role: Role::User,
                content: s.to_string(),
            })
            .collect();

        let Self {
            model,
            seed,
            max_completion_tokens,
            ..
        } = self.clone();

        RequestBody {
            model,
            messages,
            seed,
            max_completion_tokens,
        }
    }

    pub fn query(&self, input_messages: &[&str]) -> eyre::Result<String> {
        let body: Value = self
            .header()
            .json(&self.make_request_body(input_messages))
            .send()?
            .json()?;

        match &body["choices"][0]["message"]["content"] {
            Value::String(s) => Ok(s.clone()),
            _ => Ok(format!("[Unexpected response]\n{}", body)),
        }
    }
}
