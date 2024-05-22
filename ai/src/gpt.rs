use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Model {
    CushmanCodex,
    DavinciCodex,
    Gpt35Turbo,
    Gpt4,
    Gpt4o,
    TextAda001,
    TextBabbage001,
    TextCurie001,
    TextDavinci002,
    TextDavinci003,
    Custom(String),
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Model::CushmanCodex => write!(f, "cushman-codex"),
            Model::DavinciCodex => write!(f, "davinci-codex"),
            Model::Gpt35Turbo => write!(f, "gpt-3.5-turbo"),
            Model::Gpt4 => write!(f, "gpt-4"),
            Model::Gpt4o => write!(f, "gpt-4o"),
            Model::TextAda001 => write!(f, "text-ada-001"),
            Model::TextBabbage001 => write!(f, "text-babbage-001"),
            Model::TextCurie001 => write!(f, "text-curie-001"),
            Model::TextDavinci002 => write!(f, "text-davinci-002"),
            Model::TextDavinci003 => write!(f, "text-davinci-003"),
            Model::Custom(model) => write!(f, "{model}"),
        }
    }
}

pub struct ChatGPT {
    api_key: String,
    model: Model,
    client: Client,
}

impl ChatGPT {
    pub fn new(api_key: String, model: Model) -> Self {
        ChatGPT {
            api_key,
            model,
            client: Client::new(),
        }
    }

    pub async fn query(&self, text: String) -> Result<String, reqwest::Error> {
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&Request {
                model: self.model.to_string(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: text,
                }],
            })
            .send()
            .await?
            .json::<Response>()
            .await?;

        Ok(response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default())
    }

    pub async fn translate(
        &self,
        entries: Vec<String>,
        target_language: &str,
    ) -> Result<Vec<(String, String)>, reqwest::Error> {
        let client = Client::new();

        // Construct a single message with all texts to be translated
        let message_content = entries.clone().join("\n");
        let message_content = format!(
            "Translate the following text line by line to {}\n{}",
            target_language, message_content
        );

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&Request {
                model: self.model.to_string(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: message_content,
                }],
            })
            .send()
            .await?
            .json::<Response>()
            .await?;

        // Extract the translations from the response
        let translations = response
            .choices
            .first()
            .map(|choice| {
                choice
                    .message
                    .content
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        // Pair each original text with its translation
        let result: Vec<(String, String)> = entries.into_iter().zip(translations).collect();

        Ok(result)
    }
}

#[derive(Serialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}
