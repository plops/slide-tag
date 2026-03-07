# Tera Documentation & Examples

Tera is a powerful, fast, and easy-to-use template engine for Rust, inspired by Jinja2 and Django templates.

## Core Features
- **Jinja2-like Syntax**: Uses `{{ }}` for variables and `{% %}` for control flow.
- **Serde Integration**: Define context using standard Rust structs with `#[derive(Serialize)]`.
- **Inheritance**: Supports `extends` and `block` for complex layouts.
- **Runtime Rendering**: Templates are parsed and rendered at runtime.

## Examples

### Setup and Context
```rust
use tera::{Tera, Context};
use serde::Serialize;

#[derive(Serialize)]
struct Job {
    title: String,
    score: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tera = Tera::default();
    tera.add_raw_template("job_report", "Job: {{ job.title }} (Score: {{ job.score }})")?;

    let mut context = Context::new();
    let job = Job { title: "Rust Developer".into(), score: 95 };
    context.insert("job", &job);

    let rendered = tera.render("job_report", &context)?;
    println!("{}", rendered);
    Ok(())
}
```

### Template Syntax
```jinja2
{% if job.score > 80 %}
  <li class="high-relevance">{{ job.title }}</li>
{% else %}
  <li>{{ job.title }}</li>
{% endif %}

<ul>
{% for skill in job.skills %}
  <li>{{ skill }}</li>
{% endfor %}
</ul>
```

## Relevant Tasks in This Project
- Generating the Typst or HTML reports for job matches (Phase 4).
- Creating Markdown summaries for the "Inbox/Outbox" workflow.
- Dynamic report generation where templates might be updated without recompiling the binary.
