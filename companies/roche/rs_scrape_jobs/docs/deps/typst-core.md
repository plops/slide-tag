# Typst (Core) Documentation & Examples

The `typst` crate is the heart of the Typst typesetting system. When used as a library, it allows for a four-phase compilation process: parsing, evaluation, layout, and export.

## Core Compilation Workflow

1.  **World Setup**: Implement the `World` trait to provide the compiler with access to files, fonts, and packages.
2.  **Compilation**: Use `typst::compile::<PagedDocument>(world)` to transform source code into a laid-out document.
3.  **Export**: Pass the resulting `PagedDocument` to specialized export crates (`typst_pdf`, `typst_svg`, `typst_render`).

## Core Types

*   **`Source`**: A parsed Typst file.
*   **`Content`**: The hierarchical, styled representation of the document after evaluation.
*   **`PagedDocument`**: The final laid-out document, containing fixed-position `Frames` (pages).

## Examples

### Basic Compilation and PDF Export
```rust
use typst::model::PagedDocument;
use typst_pdf::PdfOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. You need a World implementation (e.g., TypstWrapperWorld from typst-as-library)
    let world = MyWorld::new(); 

    // 2. Compile to a PagedDocument
    let warned = typst::compile::<PagedDocument>(&world);
    let document = warned.output.map_err(|e| format!("Compile error: {:?}", e))?;

    // 3. Export to PDF
    let pdf_bytes = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| format!("Export error: {:?}", e))?;

    std::fs::write("output.pdf", pdf_bytes)?;
    Ok(())
}
```

### Exporting to Images (PNG)
To render to an image, you use the `typst_render` crate.

```rust
use typst_render::render;

// ... after compiling to 'document' ...

// Render the first page at 2.0x scale (144 DPI)
let page = &document.pages[0];
let pixmap = render(&page.frame, 2.0);
pixmap.save_png("page_1.png")?;
```

## Relevant Tasks in This Project
- **Custom Reporting**: Using the core API to manipulate `Content` before layout for custom dynamic report generation.
- **Multi-format Output**: Generating both PDF for downloads and PNG/SVG for web previews.
- **Deep Integration**: Understanding the `World` trait to inject "virtual" files (like dynamically generated job data) into the Typst environment without writing temporary files.

## References
- See `typst-pdf` for PDF-specific options.
- See `typst-render` for rasterizing frames.
- See `typst-svg` for vector image output.
