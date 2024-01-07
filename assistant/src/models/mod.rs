use llm::{Model, InferenceSession};
use std::error::Error;
use std::env;
use flume::Sender;
use crate::actions;
use crate::messaging::{Message, MessageBus};

mod llama_2_7b_chat_ggml;
mod llama_2_13b_chat_ggml;

pub fn initialize(bus: &mut MessageBus) {
    let inference_model = InferenceModel::new();

    loop {
        todo!();
    }
}

struct InferenceModel {
    pub model: Box<dyn Model>,
    pub session: InferenceSession,
    response_string: String,
}

impl InferenceModel {
    pub fn new() {
        let model_file: String = get_model_filename();
        let model = llm::load_dynamic(
            Some(llm::ModelArchitecture::Llama),
            // Path to GGML file
            std::path::Path::new(format!("src/models/{}", model_file).as_str()),
            // tokenizer source
            llm::TokenizerSource::Embedded,
            // llm::ModelParameters
            llm::ModelParameters::default(),
            // load progress callback
            llm::load_progress_callback_stdout
        ).unwrap_or_else(|e| {
            panic!("Failed to load model: {}", e);
        });

        println!("Loaded model: {}", model_file);

        let session = model.start_session(Default::default());

        InferenceModel {
            model,
            session,
            response_string: String::new(),
        }
    }

    pub fn infer(&mut self, prompt: String) -> Result<(), Box<dyn Error>> {
        let response = session.infer::<std::convert::Infallible>(
            model,
            // RNG provider
            &mut rand::thread_rng(),
            // the prompt to use for generation, as well as other
            // inference parameters
            &llm::InferenceRequest {
                prompt: llm::Prompt::Text(get_prompt(prompt).as_str()),
                parameters: &llm::InferenceParameters::default(), // InferenceParameters
                play_back_previous_tokens: false,
                maximum_token_count: Some(20),
            },
            // output request
            &mut llm::OutputRequest::default(),
            // output callback
            |output| {
                // handle_response parses the response JSON and dispatches
                // any necessary actions / side effects
                match output {
                    llm::InferenceResponse::SnapshotToken(token) => {
                        println!("SnapshotToken: {}", token);
                        Ok(llm::InferenceFeedback::Continue)
                    }
                    llm::InferenceResponse::PromptToken(token) => {
                        println!("PromptToken: {}", token);
                        response_string.push_str(token);
                        Ok(llm::InferenceFeedback::Continue)
                    }
                    llm::InferenceResponse::InferredToken(result) => {
                        println!("InferredToken: {}", result);
                        Ok(llm::InferenceFeedback::Continue)
                    }
                    llm::InferenceResponse::EotToken => {
                        println!("EotToken");
                        println!("Final response: {}", self.response_string);
                        actions::handle_response(self.response_string.clone());
                        self.response_string.clear();
                        Ok(llm::InferenceFeedback::Halt)
                    }
                }
            },
        );

        match response {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Inference Error: {}", e);
                Err(Box::new(e))
            }
        }
    }
}

fn get_model_filename() -> String {
    let model_file: String = env::var("LLM_MODEL").expect("LLM_MODEL must be set");

    match model_file.as_str() {
        "llama-2-7b-chat-ggml" => llama_2_7b_chat_ggml::FILENAME.to_string(),
        "llama-2-13b-chat-ggml" => llama_2_13b_chat_ggml::FILENAME.to_string(),
        _ => panic!("Unknown model file: {}", model_file)
    }
}

fn get_prompt(prompt: String) -> String {
    let model_file: String = env::var("LLM_MODEL").expect("LLM_MODEL must be set");

    match model_file.as_str() {
        "llama-2-7b-chat-ggml" => llama_2_7b_chat_ggml::get_prompt(prompt),
        "llama-2-13b-chat-ggml" => llama_2_13b_chat_ggml::get_prompt(prompt),
        _ => panic!("Unknown model file: {}", model_file)
    }
}
