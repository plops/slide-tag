# LLM-Chain Documentation & Examples

`llm-chain` is a toolbox for building LLM applications, focusing on chaining prompts and multi-step execution.

## Core Features
- **Chains**: Sequence of prompts where output of one can feed the next.
- **Templates**: Powerful prompt templating system.
- **Map-Reduce**: Useful for processing large files or datasets in chunks.

## Examples

### Creating a Chain
```rust
use llm_chain::prelude::*;
use llm_chain::step::Step;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?;
    
    let chain = Chain::new(vec![
        Step::new(
            "Summarize the following job description: {{text}}",
            None,
        ),
        Step::new(
            "Based on this summary: {{summary}}, identify the top 3 required skills.",
            None,
        ),
    ]);
    
    // Execute chain with initial data
    let res = chain.run(parameters!("text" => "Full job description..."), &exec).await?;
    println!("Skills: {}", res.to_immediate().await?.as_content());
    
    Ok(())
}
```

### Map-Reduce for Large Batches
```rust
// Useful for "packing" 20k tokens as requested in the workflow strategy
let map_reduce = chain.map_reduce(
    parameters!("text" => long_text_chunks),
    &exec
).await?;
```

## Relevant Tasks in This Project
- Orchestrating the complex "Inbox/Outbox" workflow.
- Batching 20k tokens of job data into a single summary or matching prompt.
- Creating sophisticated multi-step matching algorithms.
