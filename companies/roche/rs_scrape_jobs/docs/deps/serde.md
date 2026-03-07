# Serde Documentation & Examples

`serde` is a framework for serializing and deserializing Rust data structures efficiently and generically.

## Core Features
- **Derive API**: Simply add `#[derive(Serialize, Deserialize)]`.
- **Format Agnostic**: Works with JSON, YAML, TOML, Bincode, and more.
- **Highly Configurable**: Custom renaming, default values, and skipping fields.

## Examples

### JSON Serialization/Deserialization
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    id: String,
    title: String,
    #[serde(rename = "jobDescription")]
    description: String,
    #[serde(default)]
    relevance_score: u32,
}

fn main() {
    let json = r#"{
        "id": "123",
        "title": "Rust Dev",
        "jobDescription": "Build awesome things."
    }"#;

    // From JSON
    let job: Job = serde_json::from_str(json).unwrap();
    println!("{:?}", job);

    // To JSON
    let serialized = serde_json::to_string(&job).unwrap();
    println!("{}", serialized);
}
```

## Relevant Tasks in This Project
- Mapping the job JSON schema extracted from HTML.
- Handling complex nested structures in AI responses (`RESPONSES.json`).
- Communicating data between the Rust tool and the filesystem-based agents.
