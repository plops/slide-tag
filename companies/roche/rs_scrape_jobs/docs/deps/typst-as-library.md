# Typst-as-library Documentation & Examples

`typst-as-library` demonstrates how to integrate the Typst compiler directly into a Rust application. It provides the necessary boilerplate to implement the `typst::World` trait, which the compiler needs to resolve fonts, files, and packages.

## Core Concepts
- **TypstWrapperWorld**: The central struct that manages the environment for Typst compilation.
- **Font Discovery**: Automatically searches for system fonts and caches them.
- **Async/Thread-safety**: Uses `Arc` and `Mutex` to allow shared caching and potentially concurrent renders.

## Examples

### Rendering Typst String to PDF
The following example shows how to take a simple Typst string and produce PDF bytes.

```rust
use std::fs;
use typst_as_library::TypstWrapperWorld;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = "= Job Match Report\n\nHello from Rust!";
    
    // Create the world (root directory for assets, and the source content)
    let world = TypstWrapperWorld::new("./assets".to_owned(), content.to_owned());

    // Compile the document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Export to PDF bytes
    let pdf_options = typst_pdf::PdfOptions::default();
    let pdf = typst_pdf::pdf(&document, &pdf_options).expect("Error exporting PDF");
    
    // Save to file
    fs::write("report.pdf", pdf)?;
    
    Ok(())
}
```

## Relevant Tasks in This Project
- **Direct PDF Generation**: Replacing the external `typst compile` CLI call with a native Rust implementation in Phase 4.
- **In-memory Reporting**: Generating reports directly without needing to write `.typ` files to disk first.
- **Dynamic Content**: Injecting job data into a Typst template and rendering it immediately.

## Note on Images
While the library focuses on PDF output using `typst_pdf`, the model returned by `typst::compile` can also be rendered to images (e.g., PNG) using the `typst-render` crate if needed in the future for the website preview.
