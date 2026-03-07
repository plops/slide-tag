# GenAI Documentation & Examples

`genai` is a unified interface for multiple LLM providers (OpenAI, Anthropic, Gemini, etc.) in Rust.

## Core Features
- **Unified API**: One client for many providers.
- **Provider Adapters**: Handles specific API formats internally.
- **Streaming Support**: Supports real-time response processing.

## Examples

### Multi-Provider Request
```rust
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    
    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("You are a helpful assistant."),
        ChatMessage::user("What is the best way to port Python to Rust?"),
    ]);

    // Use a specific model; genai resolves the provider automatically
    let model = "gemini-1.5-pro"; 
    let res = client.exec_chat(model, chat_req, None).await?;
    
    println!("Answer: {}", res.content_text_as_str().unwrap_or(""));
    Ok(())
}
```

### Custom Chat Options
```rust
use genai::chat::ChatOptions;

let options = ChatOptions::new().with_temperature(0.2).with_max_tokens(1000);
let res = client.exec_chat("gpt-4o", chat_req, Some(options)).await?;
```

## Relevant Tasks in This Project
- Interfacing with multiple LLM providers without rewriting the core logic.
- Providing a fallback mechanism if one API is unavailable.
- Standardizing prompt formatting across different models.
