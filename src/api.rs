use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use rand::Rng;

const GPT: &str = "gpt-3.5-turbo";
const GEMINI: &str = "gemini-pro";
const MISTRAL: &str = "mistral-large-2402";

pub fn get_completion(client: Client, prompt: &str) -> String {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(1..=3);
    let model;

    if random_number == 1 {
        model = GPT;
    } else if random_number == 2 {
        model = GEMINI;
    } else {
        model = MISTRAL;
    }

    let req = ChatCompletionRequest::new(
        String::from(model),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Te xt(String::from(prompt)),
            name: None,
        }],
    );

    let result = client.chat_completion(req).unwrap();
    result.choices[0].message.content.as_ref().unwrap().clone()
}

pub fn client(api: &str) -> Client {
    let api_data = String::from(api);
    const BASE_URL: &str = "https://api.naga.ac/v1";
    let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| BASE_URL.to_owned());
    Client::new_with_endpoint(endpoint, api_data)
}
