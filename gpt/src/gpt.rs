use crate::imports::*;

/// A selectable OpenAI model; its [`Display`](std::fmt::Display)
/// representation yields the API model identifier sent in requests.
#[derive(Debug)]
pub enum Model {
    /// Codex Cushman model (`cushman-codex`).
    CushmanCodex,
    /// Codex Davinci model (`davinci-codex`).
    DavinciCodex,
    /// GPT-3.5 Turbo chat model (`gpt-3.5-turbo`).
    Gpt35Turbo,
    /// GPT-4 chat model (`gpt-4`).
    Gpt4,
    /// GPT-4o chat model (`gpt-4o`).
    Gpt4o,
    /// Text Ada completion model (`text-ada-001`).
    TextAda001,
    /// Text Babbage completion model (`text-babbage-001`).
    TextBabbage001,
    /// Text Curie completion model (`text-curie-001`).
    TextCurie001,
    /// Text Davinci completion model, version 2 (`text-davinci-002`).
    TextDavinci002,
    /// Text Davinci completion model, version 3 (`text-davinci-003`).
    TextDavinci003,
    /// An arbitrary model identifier passed through verbatim.
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

struct Inner {
    api_key: String,
    model: Model,
    client: Client,
}

/// Cheaply clonable handle to an OpenAI chat completions client, holding the
/// API key, target model, and shared HTTP client.
#[derive(Clone)]
pub struct ChatGPT {
    inner: Arc<Inner>,
}

impl ChatGPT {
    /// Creates a client that authenticates with the given API key and issues
    /// requests against the specified `model`.
    pub fn new(api_key: String, model: Model) -> Self {
        ChatGPT {
            inner: Arc::new(Inner {
                api_key,
                model,
                client: Client::new(),
            }),
        }
    }

    /// Calls [`query`](Self::query) repeatedly until it succeeds or `retries`
    /// attempts have failed, waiting `delay` between attempts. Returns
    /// [`Error::RetryFailure`] carrying the last error if all attempts fail.
    pub async fn query_with_retries(
        &self,
        text: String,
        retries: usize,
        delay: Duration,
    ) -> Result<String> {
        let mut attempt = 0;
        loop {
            match self.query(text.clone()).await {
                Ok(response) => {
                    return Ok(response);
                }
                Err(err) => {
                    workflow_core::task::sleep(delay).await;
                    attempt += 1;
                    if attempt >= retries {
                        return Err(Error::RetryFailure(retries, err.to_string()));
                    }
                }
            }
        }
    }

    /// Sends a single user message to the chat completions endpoint and
    /// returns the content of the first response choice, or an empty string
    /// if the model returned no choices.
    pub async fn query(&self, text: String) -> Result<String> {
        let response = self
            .inner
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.inner.api_key))
            .json(&Request {
                model: self.inner.model.to_string(),
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

    /// Translates each entry into `target_language`, returning each original
    /// string paired with its translation. The entries are submitted as a
    /// single line-by-line request and the response lines are zipped back
    /// onto the inputs.
    pub async fn translate(
        &self,
        entries: Vec<String>,
        target_language: &str,
    ) -> Result<Vec<(String, String)>> {
        // Construct a single message with all texts to be translated
        let message_content = entries.clone().join("\n");
        let message_content = format!(
            "Translate the following text line by line to {}\n{}",
            target_language, message_content
        );

        let response = self
            .inner
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.inner.api_key))
            .json(&Request {
                model: self.inner.model.to_string(),
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
