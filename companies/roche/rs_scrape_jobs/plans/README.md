# rs-scrape: Roche Job Intelligence System

A production-ready Rust application for scraping, analyzing, and matching job postings from Roche careers website with AI-powered enrichment and candidate matching capabilities.

## Overview

`rs-scrape` is a complete rewrite of the original Python-based job processing system, consolidating web scraping, AI annotation, candidate matching, and web interface into a single modular Rust binary. The system implements a hexagonal architecture with trait-based abstractions for testability and flexibility.  

## Features

- **Job Scraping**: Headless browser automation to extract job postings from Roche careers
- **AI Enrichment**: Generate job summaries and relevance scores using Google Gemini API
- **Candidate Matching**: Score candidate profiles against jobs with personalized explanations
- **Web Interface**: OAuth2 authentication, session management, and dashboard
- **Automated Scheduling**: Nightly cron jobs for pipeline execution
- **Modular Architecture**: Feature-gated compilation for optimized deployments

## Prerequisites

- Rust 1.70+
- Chrome/Chromium browser (for headless scraping)
- Google Gemini API key

## Installation

### Build from Source

```bash
cd companies/roche/rs_scrape_jobs
cargo build --release
```

### Feature Flags

The application uses modular compilation features:  

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `db` | SQLite database operations | `libsql` |
| `scraper` | Web scraping and HTML/JSON extraction | `chromiumoxide`, `reqwest` |
| `ai` | Gemini API integration | `rig-core` |
| `web` | Web server, OAuth, templates, scheduling | `axum`, `askama`, `oauth2` |

Build with specific features:
```bash
# Minimal build (database only)
cargo build --release --features "db"

# Full build (default)
cargo build --release --features "db,scraper,ai,web"
```

## Configuration

Set environment variables (create `.env` file):

```bash
GEMINI_API_KEY=your_gemini_api_key
DB_PATH=jobs.db
OAUTH_CLIENT_ID=your_github_oauth_client_id
OAUTH_CLIENT_SECRET=your_github_oauth_client_secret
OAUTH_REDIRECT_URI=http://127.0.0.1:3000/auth/callback
HOST=127.0.0.1
PORT=3000
SESSION_SECRET=your_session_secret
```

## Usage

### CLI Commands

The application provides three main commands:  

#### Start Web Server
```bash
rs-scrape serve --port 3000 --host 127.0.0.1
```
Starts the web server and background scheduler for automated pipeline execution.

#### Manual Scraping
```bash
rs-scrape trigger-scrape --debug-dump
```
Manually executes the scraping pipeline. Use `--debug-dump` to save HTML/JSON for debugging.

#### Force Candidate Matching
```bash
rs-scrape force-match --candidate-id 123
```
Re-runs AI candidate matching for a specific candidate against all jobs.

### Web Interface

Access the web interface at `http://127.0.0.1:3000`:
- `/login` - OAuth2 authentication (GitHub)
- `/profile` - Manage candidate profile
- `/dashboard` - View matched jobs
- `/jobs` - Browse all job postings

## Architecture

### Numbered File Organization

The codebase uses a numbered naming convention to indicate dependency order:  

| Prefix | Layer | Purpose |
|--------|-------|---------|
| `00_` | Domain Models | Core data structures |
| `01_` | Database Layer | Setup, repository, traits |
| `02_-05_` | Scraping Layer | Browser automation, extraction |
| `06_` | Data Processing | Pipeline orchestration |
| `07_` | AI Layer | Provider traits, implementations |
| `11_-17_` | Web Layer | Server, auth, UI, scheduling |

### Hexagonal Architecture

The system follows ports and adapters pattern with trait-based abstractions:  

- `DatabaseProvider` trait for database operations
- `AiProvider` trait for AI services
- Concrete implementations: `JobRepository` (SQLite), `GeminiProvider` (AI)

### Pipeline Flow

1. **Scraping**: Navigate Roche careers, extract job URLs
2. **Downloading**: Fetch job details with polite delays
3. **Processing**: Parse JSON, store in database with versioning
4. **AI Enrichment**: Generate summaries and relevance scores
5. **Matching**: Score candidates against jobs
6. **Scheduling**: Automated nightly execution

## Development

### Running Tests

Stage binaries for incremental testing:  

```bash
# Test database setup
cargo run --bin stage1_db --features "db"

# Test browser automation
cargo run --bin stage2_browser_test --features "scraper"

# Test web server
cargo run --bin stage10_web --features "web,db"
```

### Code Quality

Run formatting and linting:
```bash
cargo fmt
cargo clippy
```

## Deployment

### Production Build

Optimized for production with minimal binary size:  

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

Binary size: ~10-15 MB (from ~33 MB with debug symbols)

### Docker Deployment

Create a minimal Docker image:
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rs-scrape /usr/local/bin/
CMD ["rs-scrape", "serve"]
```

## Legacy Python System

This Rust application replaces the original Python-based pipeline documented in `companies/roche/scrape_jobs/README.md`  . The Rust version provides:
- Better performance and memory safety
- Unified binary instead of multiple scripts
- Real-time web interface
- Improved error handling and logging

## License

Internal project - see repository for licensing information.

## Notes

The rs_scrape_jobs project represents a complete architectural overhaul of the original Python job scraper, leveraging Rust's performance and safety features while maintaining the core functionality of job scraping, AI enrichment, and candidate matching. The modular feature-gated design allows for flexible deployment scenarios from minimal database-only services to full-featured web applications.

Wiki pages you might want to explore:
- [Job Intelligence System (rs-scrape Application) (plops/slide-tag)](https://deepwiki.com/plops/slide-tag#2)
- [System Architecture and Design Patterns (plops/slide-tag)](https://deepwiki.com/plops/slide-tag#2.1)
