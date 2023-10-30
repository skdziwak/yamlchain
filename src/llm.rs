use openai::chat::{ChatCompletionMessageRole, ChatCompletionMessage};

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    UserMessage(String),
    SystemMessage(String),
    AiMessage(String),
}

#[derive(Debug)]
pub struct Response {
    pub text: String,
}

impl Message {
    pub fn system<S: Into<String>>(s: S) -> Self {
        Self::SystemMessage(s.into())
    }
    pub fn user<S: Into<String>>(s: S) -> Self {
        Self::UserMessage(s.into())
    }
    pub fn ai<S: Into<String>>(s: S) -> Self {
        Self::AiMessage(s.into())
    }
    fn text(&self) -> &str {
        match self {
            Message::UserMessage(s) => s,
            Message::SystemMessage(s) => s,
            Message::AiMessage(s) => s,
        }
    }
}

impl Message {
    fn role(&self) -> ChatCompletionMessageRole {
        match self {
            Message::UserMessage(_) => ChatCompletionMessageRole::User,
            Message::SystemMessage(_) => ChatCompletionMessageRole::System,
            Message::AiMessage(_) => ChatCompletionMessageRole::Assistant,
        }
    }
}

pub fn load_token() -> Result<(), Error> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        Error::InvalidEnvironment(String::from("Missing OPENAI_API_KEY environment variable"))
    })?;
    openai::set_key(openai_api_key);
    Ok(())
}

pub async fn call_openai(messages: Vec<Message>, model: &str) -> Result<Response, Error> {
    log::debug!("Calling OpenAI with messages: {:?}", messages);
    let messages = messages
        .into_iter()
        .map(|m| ChatCompletionMessage {
            role: m.role(),
            content: Some(m.text().to_string()),
            name: None,
            function_call: None,
        })
        .collect::<Vec<_>>();
    let request = openai::chat::ChatCompletionBuilder::default()
        .model(model)
        .messages(messages)
        .build()
        .map_err(|e| {
            log::error!("Failed to build completion request: {:?}", e);
            Error::OpenAIError("Failed to build completion request".to_string())
        })?;
    let completion = openai::chat::ChatCompletion::create(&request)
        .await
        .map_err(|e| {
            log::error!("Failed to create completion: {:?}", e);
            Error::OpenAIError("Failed to create completion".to_string())
        })?;
    let choice = completion.choices.first().ok_or(Error::OpenAIError("No reply from OpenAI".to_string()))?;
    let response = choice
        .message
        .content
        .as_ref()
        .ok_or(Error::OpenAIError("No reply from OpenAI".to_string()))?;
    log::debug!("OpenAI response: {:?}", response);
    Ok(Response {
        text: response.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_openai() {
        dotenv::dotenv().ok();
        load_token().unwrap();
        let messages = vec![
            Message::system("You only respond with yes or no. Just one lowercase word. No capital letters or interpunction."),
            Message::user("Is 7 a prime number?"),
        ];
        let response = call_openai(messages, "gpt-4").await.unwrap();
        assert_eq!(response.text, "yes");
    }
}
