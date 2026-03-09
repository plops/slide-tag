# Anweisung für das ausführende LLM: Implementierung der Visualisierungs-Features

Du bist beauftragt, die Frontend- und Visualisierungs-Schicht für das "Roche Job Scraper" Projekt in Rust zu implementieren.

**WICHTIGE GRUNDREGELN FÜR DEINE ARBEIT:**

1. **Nutze das Repository Pattern:** Direkte SQL-Queries in den Web-Routen sind streng verboten. Alle Datenbankzugriffe MÜSSEN über das `DatabaseProvider` Trait in `01c_db_traits.rs` definiert und in `01b_db_repo.rs` implementiert werden.
2. **Nutze Askama für UI:** Wir verwenden Server-Side-Rendering. Du musst HTML-Templates im Ordner `/templates/` anlegen und via `#[derive(Template)]` in Rust einbinden.
3. **Vanilla JS für simple UI-Logik:** Vermeide schwere Frontend-Frameworks. Nutze reines JavaScript für DOM-Manipulationen (wie Filterung auf dem Dashboard).
4. **Dokumentation:** Wenn du bei `axum`, `askama` oder `libsql` nicht weiterweißt, nutze sofort das **deepwiki MCP**, um die Dokumentation abzurufen (z.B. `tokio-rs/axum` oder `askama-rs/askama`). Rate nicht bei API-Signaturen!

---

## Task 1: Detailansicht für Job-Matches implementieren (Priorität 1)

**Ziel:** Ein Klick auf ein Match im Dashboard soll eine Detailseite öffnen, die die komplette Jobbeschreibung (`job.description`) und die detaillierte KI-Erklärung (`explanation`) anzeigt.

### 1.1 Datenbank-Schicht anpassen

Erweitere das Trait in `src/01c_db_traits.rs`:

```rust
// Füge diese Zeile zum DatabaseProvider Trait hinzu:
async fn get_match_detail(&self, match_id: i64) -> Result<Option<(CandidateMatch, JobHistory)>>;

```

Implementiere die Methode in `src/01b_db_repo.rs` via `JOIN` zwischen `candidate_matches` und `job_history`.

### 1.2 Axum Route & Askama Template (`src/13_web_ui.rs`)

Definiere die Datenstruktur für das Template und die Route:

```rust
use axum::extract::Path;

#[derive(Template)]
#[template(path = "match_detail.html")]
pub struct MatchDetailTemplate {
    pub title: String,
    pub user_name: String,
    pub match_data: CandidateMatch,
    pub job: JobHistory,
}

pub async fn get_match_detail(
    Path(match_id): Path<i64>,
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    let candidate = get_current_user(&session, &*state.db).await?;
    
    // Rufe die Daten ab
    let (match_data, job) = state.db.get_match_detail(match_id)
        .await
        .map_err(WebError::Database)?
        .ok_or_else(|| WebError::Database(anyhow::anyhow!("Match not found")))?;

    // Sicherheitscheck: Gehört das Match dem User?
    if match_data.candidate_id != candidate.id.unwrap_or(0) {
        return Err(WebError::Auth("Unauthorized access to match".to_string()));
    }

    let template = MatchDetailTemplate {
        title: format!("Match: {}", job.title),
        user_name: candidate.name,
        match_data,
        job,
    };

    Ok(Html(template.render()?))
}

```

Füge die Route `.route("/match/:id", get(get_match_detail))` zur `web_ui_routes()` Funktion hinzu.

### 1.3 HTML Template erstellen (`templates/match_detail.html`)

Erstelle diese Datei. Erweitere `base.html`. Zeige den `score` farblich hervorgehoben an, liste die `required_topics` als kleine CSS-Badges und rendere die `job.description` (Achtung: Job-Beschreibungen könnten HTML enthalten, nutze den Askama `|safe` Filter, falls angebracht, oder render es in einem `<pre>` Block, falls es reiner Text ist).

---

## Task 2: Client-Side Filterung im Dashboard (Vanilla JS)

**Ziel:** Die Filter-Buttons (`data-filter="high"`, etc.) in `dashboard.html` sollen die Liste der angezeigten Matches ohne Server-Reload filtern.

**Aufgabe:** Füge ganz unten in `templates/dashboard.html` (vor dem `{% endblock %}`) folgendes JavaScript ein und passe ggf. die Selektoren an:

```html
<script>
document.addEventListener('DOMContentLoaded', () => {
    const buttons = document.querySelectorAll('.filter-btn');
    const cards = document.querySelectorAll('.match-card');

    buttons.forEach(btn => {
        btn.addEventListener('click', () => {
            // Active state toggeln
            buttons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            const filterType = btn.getAttribute('data-filter');

            cards.forEach(card => {
                const score = parseFloat(card.getAttribute('data-score'));
                
                if (filterType === 'all') {
                    card.style.display = 'block';
                } else if (filterType === 'high' && score >= 0.8) {
                    card.style.display = 'block';
                } else if (filterType === 'medium' && score >= 0.6 && score < 0.8) {
                    card.style.display = 'block';
                } else {
                    card.style.display = 'none';
                }
            });
        });
    });
});
</script>

```

*Vergiss nicht, in `dashboard.html` bei jedem Match-Eintrag einen Link zur neuen Detailseite aus Task 1 (`<a href="/match/{{ match_item.match_data.id.unwrap() }}">Details ansehen</a>`) einzubauen!*

---

## Task 3: Gesamtübersicht aller gescrapten Jobs (Must Have)

**Ziel:** Eine durchsuchbare und paginierte Liste aller Jobs im System unter `/jobs`.

### 3.1 Pagination Model & DB-Logik (`src/00_models.rs` & `01c_db_traits.rs`)

Da große Datenmengen anfallen, müssen wir serverseitig paginieren.

```rust
// In src/01c_db_traits.rs
async fn get_jobs_paginated(&self, limit: i64, offset: i64, search_query: Option<String>) -> Result<(Vec<JobHistory>, i64)>; 
// Returnt (Job-Liste, Totale_Anzahl_für_Pagination)

```

Implementiere dies in `01b_db_repo.rs`. Nutze `LIMIT ? OFFSET ?` in SQL. Wenn `search_query` `Some` ist, nutze `WHERE title LIKE ? OR description LIKE ?`.

### 3.2 Axum Route (`src/13_web_ui.rs`)

Nutze Axums `Query` Extractor für Pagination.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobQuery {
    pub page: Option<i64>,
    pub q: Option<String>,
}

#[derive(Template)]
#[template(path = "jobs.html")]
pub struct JobsTemplate {
    pub title: String,
    pub user_name: String,
    pub jobs: Vec<JobHistory>,
    pub current_page: i64,
    pub total_pages: i64,
    pub search_query: String,
}

// Implementiere get_jobs(Query(params): Query<JobQuery>, ...)
// 20 Items pro Seite. (page - 1) * 20 = offset.

```

### 3.3 UI: `templates/jobs.html`

Erstelle eine tabellarische Ansicht oder ein sauberes Grid-Layout. Oben muss ein `<form method="GET" action="/jobs">` mit einem Text-Input für die Suche sein. Unten die Paginierungs-Buttons (Zurück / Weiter).

---

## Task 4: Performance: SQLite Indizes (`src/01_db_setup.rs`)

**Ziel:** Die Abfragen aus Task 1-3 müssen auch bei 10.000 Jobs performant bleiben.

Ergänze in der `init_db` Funktion:

```rust
conn.execute("CREATE INDEX IF NOT EXISTS idx_matches_candidate_id ON candidate_matches(candidate_id)", ()).await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_matches_score ON candidate_matches(score DESC)", ()).await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_job_hist_identifier ON job_history(identifier)", ()).await?;

```