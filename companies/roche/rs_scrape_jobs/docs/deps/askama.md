# Askama Documentation & Examples

Askama is a type-safe, compiled templating engine for Rust that performs **compile-time template validation** by linking templates to user-defined structs.

## Core Features
- **Compile-time Validation**: Ensures that template variables correspond to the fields of the context struct, catching errors during compilation.
- **High Performance**: Renders are converted into optimized Rust code via procedural macros.
- **Inheritance**: Supports `extends` and `block` syntax.

## How it Works
Askama leverages Rust's type system through its `#[derive(Template)]` procedural macro. When you annotate a struct, the macro processes the template file and generates Rust code that implements the `Template` trait.

### Example: Type-Safe Template

**`templates/job.html`**:
```jinja
Hello, {{ company_name }}!
Relevance Score: {{ score }}%
```

**`src/main.rs`**:
```rust
use askama::Template;

#[derive(Template)]
#[template(path = "job.html")]
struct JobTemplate<'a> {
    company_name: &'a str,
    score: u32,
}

fn main() {
    let tpl = JobTemplate { company_name: "Roche", score: 95 };
    
    // This is validated at compile-time. 
    // If you changed the template to use {{ username }}, 
    // it would fail to compile!
    println!("{}", tpl.render().unwrap());
}
```

## Advanced Validation
- **Control Flow**: Askama validates the syntax of `{% if %}`, `{% for %}`, and `{% match %}`. A missing `{% endif %}` results in a compile-time error.
- **Filter Arguments**: Custom filters undergo signature validation, ensuring correct argument types.
- **Reserved Keywords**: Prevents conflicts with Rust keywords.

## Relevant Tasks in This Project
- Generating highly performant, type-safe reports for large batches of jobs.
- Enforcing that report templates are always in sync with underlying Data models (caught by the compiler).
- Use-case for Phase 4 where static, high-speed rendering is prioritized.
