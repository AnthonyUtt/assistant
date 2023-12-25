use llm::{Model, InferenceSession};
use std::error::Error;
use std::env;
use crate::actions;

pub fn init() -> (Box<dyn Model>, InferenceSession) {
    let model_file: String = env::var("LLM_MODEL").expect("LLM_MODEL must be set");
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

    let session = model.start_session(Default::default());

    (model, session)
}

pub fn infer(model: &dyn Model, session: &mut InferenceSession, prompt: String) -> Result<(), Box<dyn Error>> {
    let response = session.infer::<std::convert::Infallible>(
        model,
        // RNG provider
        &mut rand::thread_rng(),
        // the prompt to use for generation, as well as other
        // inference parameters
        &llm::InferenceRequest {
            prompt: llm::Prompt::Text(prompt.as_str()),
            parameters: &llm::InferenceParameters::default(), // InferenceParameters
            play_back_previous_tokens: false,
            maximum_token_count: Some(10000),
        },
        // output request
        &mut llm::OutputRequest::default(),
        // output callback
        |output| {
            // handle_response parses the response JSON and dispatches
            // any necessary actions / side effects
            match output {
                llm::InferenceResponse::InferredToken(result) => {
                    actions::handle_response(result);
                }
                _ => {}
            }

            Ok(llm::InferenceFeedback::Halt)
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
