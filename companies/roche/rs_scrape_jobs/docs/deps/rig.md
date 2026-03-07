# Rig Rust Library Documentation & Examples

Rig is a library for building LLM-powered applications in Rust, with a focus on structured outputs and type safety.

## Core Concepts
- **Extractor**: A system for structured data extraction from unstructured text.
- **Type Safety**: Uses `serde` and `schemars` to ensure LLM outputs match Rust structs.
- **Multi-provider**: Supports OpenAI, Gemini, and others.

## Examples

### Structured Data Extraction
```rust
use rig::providers::gemini;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct JobMatch {
    relevance_score: u32,
    explanation: String,
    skills_found: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini::Client::from_env();
    let model = client.model("gemini-1.5-flash");
    
    // Create an extractor for our JobMatch struct
    let extractor = model.extractor::<JobMatch>("Extract job match details from the text.");
    
    let result = extractor.extract("Job description text...").await?;
    println!("Match Score: {}", result.relevance_score);
    
    Ok(())
}
```

## Relevant Tasks in This Project
- Using Gemini via the Developer API for structured job filtering.
- Ensuring AI annotations (relevance, summary) are correctly typed before hitting the database.
- Interfacing with Gemini in Phase 3 of the pipeline.
