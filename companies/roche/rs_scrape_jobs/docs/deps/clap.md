# Clap Documentation & Examples

`clap` (Command Line Argument Parser) is the standard for building command-line interfaces in Rust.

## Core Features
- **Derive API**: Define your CLI using structs.
- **Subcommands**: Support for commands like `git checkout`.
- **Validation**: Automatic type checking and error messages.

## Examples

### Subcommands with Derive
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-scrape")]
#[command(about = "Roche Job Scraper CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Collect job links and download pages
    Collect {
        #[arg(short, long, default_value = "20260307")]
        date: String,
    },
    /// Process pages and match against a candidate
    Match {
        profile: String,
        #[arg(long)]
        skip_ocr: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Collect { date } => {
            println!("Collecting for date: {}", date);
        }
        Commands::Match { profile, .. } => {
            println!("Matching profile: {}", profile);
        }
    }
}
```

## Relevant Tasks in This Project
- Creating the unified `rs-scrape` binary.
- Defining subcommands for the different pipeline phases.
- Handling configuration flags (models, chunk sizes, dates).
