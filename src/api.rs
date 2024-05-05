use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

const GPT: &str = "gpt-3.5-turbo";
const MISTRAL: &str = "mistral-large-2402";

const ANSWER_ERROR: &str = "It seems like model unavailable.";
pub fn get_completion(client: Client, prompt: String, model: String) -> String {
    let mut model_choice: &str = "";
    if model == "GPT".to_string() {
        model_choice = GPT;
    } else if model == "MISTRAL".to_string() {
        model_choice = MISTRAL;
    }

    let req = ChatCompletionRequest::new(
        String::from(model_choice),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(prompt),
            name: None,
        }],
    );

    match client.chat_completion(req) {
        Ok(result) => {
            if let Some(content) = result
                .choices
                .get(0)
                .and_then(|choice| choice.message.content.as_ref())
            {
                content.clone()
            } else {
                "No completion response found".to_string()
            }
        }
        Err(_) => ANSWER_ERROR.to_string(),
    }
}

pub fn client(api: &str) -> Client {
    let api_data = String::from(api);
    const BASE_URL: &str = "https://api.naga.ac/v1";
    let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| BASE_URL.to_owned());
    Client::new_with_endpoint(endpoint, api_data)
}
