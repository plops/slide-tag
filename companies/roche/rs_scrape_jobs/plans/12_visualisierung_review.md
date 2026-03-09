
### 1. Detaillierte Analyse der aktuellen Probleme

#### A. Warum Profile und Summaries verschwinden (Kritischer Bug)
Das ist kein Server-Neustart-Problem an sich, sondern eine Kombination aus zwei fatalen Logikfehlern in der Datenbank-Interaktion:
1. **Profile verschwinden:** In `src/12_auth.rs` (Zeile 76+) wird bei *jedem* Login ein neues `Candidate`-Objekt mit `profile_text: "".to_string()` erstellt und an `upsert_candidate` übergeben. Da in `01b_db_repo.rs` ein `INSERT OR REPLACE` ausgeführt wird, wird der mühsam eingegebene Profiltext bei jedem erneuten Login durch einen leeren String überschrieben!
2. **Summaries verschwinden:** In `01b_db_repo.rs` speichert die KI die Zusammenfassung via `update_job_ai` in der Tabelle `jobs`. Das Frontend (Dashboard, Jobs-Liste) liest aber aus `job_history` (via `get_latest_jobs` und `get_jobs_paginated`). Die AI-Updates erreichen die Historien-Tabelle nie.
3. **Sessions gehen verloren:** `tower-sessions` verwendet aktuell den `MemoryStore`. Bei jedem Neustart des Rust-Servers fliegen alle Nutzer raus und müssen sich neu einloggen (was durch Bug 1 dann ihr Profil löscht).

#### B. Fehlende Job-Detailseiten
Das System hat zwar eine `/match/:id` Route (die einen Job im Kontext eines Kandidaten-Matches anzeigt), aber keine generische `/job/:identifier` Route. Nutzer können keine Jobs ansehen, für die die KI noch kein Match generiert hat.

#### C. Architektur- & Navigations-Schwächen
* **Navigation:** Die Links in `base.html` sind in einem `{% block nav_links %}` gekapselt, der auf jeder Seite neu definiert wird. Das führt zu Inkonsistenzen. Die Navigation sollte zentral in `base.html` über eine `is_logged_in` Variable gesteuert werden.
* **Landing Page:** Die `/` Route leitet nicht eingeloggte Nutzer hart auf `/auth/login` (und damit sofort zu GitHub) weiter. Das ist aus UX-Sicht ein "No-Go".
* **Tech Debt:** Manche DB-Abfragen sind ineffizient (z. B. das Laden *aller* Jobs in `get_latest_jobs` ohne Pagination in den Hintergrund-Tasks).

---

### 2. Priorisierter Implementierungsplan

#### Phase 1: Kritische Fehler beheben (Must Have)
*Fokus: Datenintegrität herstellen und Basis-UX reparieren.*
1. **Auth-Logik reparieren:** `auth_callback` darf existierende Profile nicht überschreiben.
2. **AI-Summary Sync reparieren:** `update_job_ai` muss auch die Tabelle `job_history` aktualisieren.
3. **Session Store anpassen:** Einbindung von Sessions, die Neustarts überleben (Cookie-basiert oder DB-basiert).
4. **Landing Page erstellen:** Eine echte `index.html` anstelle des harten Redirects.
5. **Globale Navigation:** Umbau der `base.html` für eine konsistente Top-Bar.

#### Phase 2: User Experience verbessern (Should Have)
1. **Generische Job-Detailseite:** Eigene Route `/job/:identifier` implementieren.
2. **Vollständige Metadaten:** Template `job_detail.html` erstellen, das `pay_grade`, `hiring_manager`, Daten etc. anzeigt.
3. **Mobile Optimierung:** CSS Flexbox/Grid in `base.html` für Smartphones anpassen.

#### Phase 3: Advanced Features (Nice to Have)
1. Filter & Sortierung im Backend (derzeit nur Vanilla JS Frontend).
2. Cron-Job Optimierung (Nur Deltas an die KI senden).
3. Admin-Rollen.

---

### 3. Konkrete Lösungsansätze & Code-Beispiele (Für das LLM)

Hier sind die exakten Architekturanweisungen und Code-Snippets, die das LLM Schritt-für-Schritt umsetzen muss.

#### Schritt 1.1: Persistenz der Profile fixen (`src/12_auth.rs`)
Das LLM muss den `auth_callback` anpassen. Anstatt blind zu überschreiben, muss zuerst geprüft werden, ob der User existiert.

```rust
// in src/12_auth.rs - auth_callback Funktion:
if let Ok(user_json) = resp.json::<serde_json::Value>().await {
    let oauth_sub = user_json["id"].as_i64().unwrap().to_string();
    let name = user_json["name"].as_str().unwrap_or("Unknown").to_string();

    // NEU: Zuerst prüfen, ob der Kandidat schon existiert!
    let existing_candidate = state.db_provider.get_candidate_by_oauth_sub(&oauth_sub).await.unwrap_or(None);
    
    let candidate = if let Some(existing) = existing_candidate {
        // Name updaten, aber Profiltext behalten!
        Candidate {
            name,
            ..existing
        }
    } else {
        Candidate {
            id: None,
            oauth_sub: oauth_sub.clone(),
            name: name.clone(),
            profile_text: "".to_string(),
        }
    };

    match state.db_provider.upsert_candidate(&candidate).await { ... }
}
```

#### Schritt 1.2: KI-Zusammenfassungen in der Historie speichern (`src/01b_db_repo.rs`)
Das LLM muss `update_job_ai` umschreiben, damit Frontend-Routen die KI-Daten sehen können.

```rust
// in src/01b_db_repo.rs
pub async fn update_job_ai(&self, identifier: &str, summary: &str) -> anyhow::Result<()> {
    // Update base table
    self.conn.execute(
        "UPDATE jobs SET job_summary = ? WHERE identifier = ?",
        params![summary, identifier],
    ).await?;
    
    // WICHTIG: Update neusten Eintrag in der History Tabelle!
    self.conn.execute(
        "UPDATE job_history SET job_summary = ? 
         WHERE identifier = ? 
         AND id = (SELECT MAX(id) FROM job_history WHERE identifier = ?)",
        params![summary, identifier, identifier],
    ).await?;
    
    Ok(())
}
```

#### Schritt 1.3: Echte Startseite implementieren (`src/11_web_server.rs` & `13_web_ui.rs`)
Die harte Weiterleitung muss weg.
1. Erstelle `templates/index.html` (Landing Page mit "Sign in with GitHub" Button).
2. Passe die Root-Route an:

```rust
// in src/11_web_server.rs oder 13_web_ui.rs
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub is_logged_in: bool,
}

async fn root(session: Session) -> Result<Html<String>, WebError> {
    if let Some(_user_name) = session.get::<String>("user_name").await.unwrap_or(None) {
        // Wenn eingeloggt, zeige Dashboard oder Landing Page mit "Gehe zu Dashboard"
        Ok(Html(IndexTemplate { is_logged_in: true }.render()?))
    } else {
        // Wenn nicht eingeloggt, zeige schöne Startseite
        Ok(Html(IndexTemplate { is_logged_in: false }.render()?))
    }
}
```

#### Schritt 1.4: Navigation konsistent machen (`templates/base.html`)
Das LLM muss das struct-übergreifende Feld `is_logged_in` in Askama einführen (oder das `user_name` `Option<String>` nutzen) und den Header in `base.html` fixieren:

```html
<!-- In templates/base.html -->
<nav>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/jobs">Jobs</a></li>
        {% if user_name.is_some() %}
            <li><a href="/dashboard">Dashboard</a></li>
            <li><a href="/profile">Profil</a></li>
            <li><a href="/auth/logout">Logout</a></li>
        {% else %}
            <li><a href="/auth/login" class="btn btn-primary">Sign in with GitHub</a></li>
        {% endif %}
    </ul>
</nav>
```
*Anweisung an das LLM:* Du musst `user_name: Option<String>` zu **jedem** Askama-Template-Struct in `13_web_ui.rs` hinzufügen.

#### Schritt 1.5: Job-Detailseite hinzufügen (`src/13_web_ui.rs` & DB Traits)
Das LLM muss eine saubere Detailseite für Jobs einbauen.

1. **Trait erweitern (`01c_db_traits.rs`):**
   `async fn get_job_by_identifier(&self, identifier: &str) -> Result<Option<JobHistory>>;`
2. **Route erstellen (`13_web_ui.rs`):**
   ```rust
   #[derive(Template)]
   #[template(path = "job_detail.html")]
   pub struct JobDetailTemplate {
       pub user_name: Option<String>,
       pub job: JobHistory,
   }

   pub async fn get_job_detail(
       Path(identifier): Path<String>,
       session: Session,
       State(state): State<Arc<AppState>>,
   ) -> Result<Html<String>, WebError> {
       let user_name = session.get::<String>("user_name").await.unwrap_or(None);
       let job = state.db.get_job_by_identifier(&identifier)
           .await
           .map_err(WebError::Database)?
           .ok_or_else(|| WebError::Database(anyhow::anyhow!("Job not found")))?;

       Ok(Html(JobDetailTemplate { user_name, job }.render()?))
   }
   ```

---

### 4. Anweisungen für das Ausführende LLM (System-Prompt-Zusatz)

Wenn du das LLM die Änderungen durchführen lässt, gib ihm exakt diese Anweisungen:

> **WICHTIGE ARBEITSANWEISUNGEN FÜR DEINE IMPLEMENTIERUNG:**
> 
> 1. **Phase 1 hat oberste Priorität:** Behebe den Profil-Überschreibungs-Bug in `12_auth.rs` als Allererstes. Führe dazu ein `get_candidate_by_oauth_sub` aus, bevor du ein `Candidate` Objekt baust.
> 2. **Datenbank-Synchronität:** Wenn die AI Jobs bewertet (`01b_db_repo.rs`), musst du ZWINGEND auch die Tabelle `job_history` updaten, da das UI seine Daten von dort bezieht. 
> 3. **Templates & Typen:** Wenn du die Navigation in `base.html` umbaust, pass auf, dass du in `13_web_ui.rs` alle Askama Template-Structs (z. B. `DashboardTemplate`, `ProfileTemplate`, `JobsTemplate`) so anpasst, dass sie `user_name: Option<String>` nutzen (statt eines fixen Strings). Das verhindert Abstürze bei uneingeloggten Usern auf öffentlichen Seiten wie `/jobs`.
> 4. **Neue Routen:** Implementiere `GET /job/:identifier`. Lege dazu `templates/job_detail.html` an. Präsentiere dort systematisch alle Meta-Daten (Paygrade, Dates, Hiring Manager etc.), die im `JobHistory` Struct existieren.
> 5. **Nutze den MCP Deepwiki:** Wenn du nicht weißt, wie man in `axum` optionale Session-Werte ausliest (z.B. für öffentliche Routen), schlage in der Dokumentation nach. Keine Halluzinationen bei axum extractors!

Mit diesem Plan werden die kritischen Bugs behoben, die Architektur für die Zukunft stabilisiert und alle UX-Probleme (Persistenz, Navigation, fehlende Detailansicht) systematisch abgestellt.