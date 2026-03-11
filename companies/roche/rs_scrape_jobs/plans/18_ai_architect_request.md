# Architektur- und Implementierungsplan: rs_scrape_jobs AI-Optimierung

**Projektkontext für die Worker-KI (MCP-Referenz):**
*   **GitHub Organisation:** `slide-tag` (bzw. Roche interner Workspace)
*   **Projektname:** `rs_scrape_jobs`
*   **Pfad:** `/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs/`

## TEIL 1: ANALYSE DES AKTUELLEN KI-WORKFLOWS (20%)

### 1.1 Aktueller Workflow: Job-Scraping → AI-Processing
1.  **Trigger:** `stage6_ai_test.rs` lädt unannotierte Jobs aus der DB.
2.  **Batching:** `GeminiProvider::annotate_jobs` nutzt den `BatchBuilder`. Derzeit werden Jobs in einen Batch gestopft, bis **80% des TPM-Limits** (Tokens Per Minute) erreicht sind.
3.  **Processing:** Ein riesiger Prompt mit den vollständigen Job-Beschreibungen (limitiert auf 500 Chars) wird an Gemini gesendet.
4.  **Ergebnis:** JSON-Antwort wird geparst, nach `idx` gemappt und via `update_job_ai` in die DB geschrieben.

### 1.2 Aktueller Workflow: Candidate Matching
1.  **Trigger:** User klickt auf Match im Web-UI (`trigger_match`). Ein asynchroner Task startet.
2.  **Datenbeschaffung:** Alle `latest_jobs` werden geladen.
3.  **Batching & Matching:** Ähnlich wie oben. Es wird das Kandidaten-Profil gegen Batches von Jobs (wieder limitiert auf 800 Chars der vollen Beschreibung) gematcht.
4.  **Ergebnis:** Matches > 0.3 Score werden in der DB gespeichert.

### 1.3 Engpässe und Optimierungspotenziale (Bottlenecks)
*   **Kritischer Engpass (RPD - Requests Per Day):** Gemini Flash Lite Preview erlaubt nur 500 Requests/Tag. Wenn das Batching nur durch TPM limitiert wird, senden wir ggf. zu viele kleine Requests und erreichen das RPD-Limit vorzeitig.
*   **Ineffizientes Matching:** Das Candidate Matching nutzt die Rohtexte der Beschreibungen. Das frisst Tokens, ist langsam und verschlechtert das Signal-to-Noise Ratio für die KI.
*   **Fehlendes Logging:** Bei großen Datenmengen "hängt" die Konsole scheinbar in `process_jobs_in_batches` ohne Fortschrittsanzeige.
*   **Fehlende Retry-Logik:** Bei API-Fehlern (z. B. HTTP 429 oder 500) bricht der Batch komplett ab.

---

## TEIL 2: ARCHITEKTUR-ENTWURF (80%)

### Antworten auf deine Klärungsfragen:
*   **ATS-Felder:** Neben `job_level` nehmen wir `job_family`, `grade_profile` und `employment_type` mit in den Prompt auf.
*   **Sprache:** Die Prompt-Anweisung lautet: *"Write the summary in the same language as the original job description."*
*   **Retry-Logik:** Wir implementieren ein einfaches Exponential Backoff (3 Retries) für die Rig-Client Aufrufe.

---

### 2.1 Konfigurations-Schema (YAML/JSON Repräsentation)
Die bestehende `AiModelConfig` muss erweitert werden.

```rust
// In src/07d_ai_rate_limiter.rs
pub struct AiModelConfig {
    pub name: String,
    pub rpm_limit: u32,               // 15
    pub tpm_limit: u32,               // 250_000
    pub rpd_limit: u32,               // 500
    pub assumed_words_per_token: f32, // 0.75
    pub words_per_request: u32,       // NEU: Max Wörter pro Request (Ziel: 25.000)
}
```

---

### 2.2 Dual-Control Rate Limiting & Batching (Architektur)
Der `BatchBuilder` muss modifiziert werden, um **sowohl** TPM **als auch** das neue `words_per_request` Limit zu respektieren. Wir wollen die 25.000 Wörter pro Request maximal ausreizen, um das RPD-Limit von 500/Tag zu schonen.

**Code-Beispiel für den Architekturentwurf des BatchBuilders:**
```rust
// In src/07e_ai_batch_builder.rs
pub struct BatchBuilder {
    // ... bestehende felder ...
    pub current_words: u32,
    pub max_words_per_request: u32,
}

impl BatchBuilder {
    pub fn try_add_job(&mut self, job: Job) -> bool {
        let job_tokens = self.estimate_job_tokens(&job);
        let job_words = (job_tokens as f32 * self.assumed_words_per_token) as u32;

        // Dual-Control Check
        if self.current_tokens + job_tokens > self.token_threshold || 
           self.current_words + job_words > self.max_words_per_request {
            return false;
        }

        self.jobs.push(job);
        self.current_tokens += job_tokens;
        self.current_words += job_words;
        true
    }
}
```

---

### 2.3 Fortschritts-Logging-System
Das Logging muss in die Iteration der Provider eingebaut werden. Dazu benötigen wir die Gesamtzahl der Jobs im Vorfeld.

**Format-Anforderung:** `"Processing job descriptions 18..23 out of 180"`

**Code-Beispiel:**
```rust
// In src/07b_ai_gemini.rs -> annotate_jobs und match_candidate
let total_jobs = jobs.len();
let mut processed_count = 0;

// Innerhalb der while loop:
if let Some(batch) = batch_builder.take_batch() {
    let batch_size = batch.len();
    let start_idx = processed_count + 1;
    let end_idx = processed_count + batch_size;
    
    let start_time = Instant::now();
    tracing::info!("Processing job descriptions {}..{} out of {}", start_idx, end_idx, total_jobs);
    
    // ... API Call ...
    
    let elapsed = start_time.elapsed();
    tracing::info!("Batch completed in {:.2?} (Tokens used: {})", elapsed, batch_tokens);
    processed_count += batch_size;
}
```

---

### 2.4 Optimiertes Candidate-Matching
Anstatt die volle `job.description` in `process_candidate_matching_batch` zu verwenden, nutzen wir den neuen `job.job_summary`. Das reduziert die Token-Größe massiv.

**Geänderter Prompt-Aufbau:**
```rust
// In process_candidate_matching_batch
input.push_str(&format!(
    "Summary: {}\n",
    job.job_summary
        .as_deref()
        .unwrap_or("No summary available")
));
```

---

### 2.5 Admin-Only Job-Level Display
Wir müssen den `jobs` Endpoint anpassen, um die Admin-Rolle zu prüfen und an Askama weiterzugeben.

**Code-Beispiel für den Controller (src/13_web_ui.rs):**
```rust
pub async fn get_jobs(...) -> Result<Html<String>, WebError> {
    // ...
    let user_name = session.get::<String>("user_name").await.unwrap_or(None);
    
    // Prüfe ob Admin (Logic angelehnt an 17_admin.rs)
    let is_admin = match &user_name {
        Some(name) => name.to_lowercase() == state.config.admin_username.to_lowercase(),
        None => false,
    };

    let template = JobsTemplate {
        // ...
        is_admin, // <-- Neues Feld im struct JobsTemplate
    };
}
```

**Template-Änderung (templates/jobs.html):**
```html
{% if is_admin %}
    {% if let Some(job_level) = job.job_level %}
    <div class="meta-item">📊 {{ job_level }}</div>
    {% endif %}
{% endif %}
```

---

## MIGRATION-PLAN & PERFORMANCE-BENCHMARK

**Vorher (Current):**
* Full Text Annotation: ca. 10 Jobs pro Batch (Token Limit).
* Matching: Full Text, max 5-8 Jobs pro Request. Hohe Latenz, schnelles Erreichen von Token-Limits.

**Nachher (New Architecture):**
* Full Text Annotation: ca. 25-30 Jobs pro Batch durch `words_per_request = 25000`.
* Matching: Summaries statt Full Text. ca. 50-100 Jobs pro Batch! 
* **Performance Benchmark Erwartung:** API Requests für Matching sinken um 80%. Zeit für Gesamt-Matching sinkt von ~60s auf <15s.

---
---

## 🛠️ IMPLEMENTIERUNGSPLAN FÜR DIE KI (SCHRITT-FÜR-SCHRITT)

**WICHTIG AN DIE IMPLEMENTIERENDE KI:** Führe diese Schritte streng sequenziell aus. Weiche nicht von den Namenskonventionen ab.

### SCHRITT 1: Konfiguration & BatchBuilder anpassen (Dateien: `07d_ai_rate_limiter.rs`, `07e_ai_batch_builder.rs`)
1. Füge `words_per_request: u32` zu `AiModelConfig` hinzu.
2. In `GeminiProvider::new` (`07b_ai_gemini.rs`), setze `words_per_request: 25000`.
3. Erweitere den `BatchBuilder` (`07e_ai_batch_builder.rs`) um `current_words` und `max_words_per_request`.
4. Passe `try_add_job` an, sodass es *false* zurückgibt, wenn Token-Limit **oder** Wort-Limit (`max_words_per_request`) überschritten wird.
*🏁 Aktion nach diesem Schritt: `cargo check` ausführen, um Typfehler zu vermeiden.*

### SCHRITT 2: Fortschritts-Logging implementieren (Datei: `07b_ai_gemini.rs`)
1. Modifiziere `annotate_jobs` und `match_candidate`. Speichere `total_jobs = jobs.len()`.
2. Füge eine Variable `processed_count = 0` ein.
3. Füge vor dem API-Call (`process_batch` / `process_candidate_matching_batch`) das geforderte `tracing::info!` oder `println!` Log-Statement ein: `"Processing job descriptions X..Y out of Z"`.
4. Messe die Zeit mit `std::time::Instant` und logge Latenz & Token nach dem Call.
*🏁 Aktion nach diesem Schritt: `cargo build`.*

### SCHRITT 3: Job-Summary Prompt & Retry-Logik (Datei: `07b_ai_gemini.rs`)
1. In `process_batch`: Füge ATS-Daten in den Input-String ein (`job_level`, `job_family`, `grade_profile`, `employment_type`). Verwende `.as_deref().unwrap_or("N/A")`.
2. Passe den Preamble/Prompt an: *"Generate structured bullet points... Respond in the language of the job description."*
3. Implementiere eine simple `for`-Schleife (max 3 Iterationen) um den `completion_model.completion(request).await` Aufruf. Bei `Err` 2 Sekunden mit `tokio::time::sleep` warten und neu versuchen. Bei Erfolg `break`.

### SCHRITT 4: Candidate-Matching Optimierung (Datei: `07b_ai_gemini.rs`)
1. In `process_candidate_matching_batch`: Ersetze `job.description` durch `job.job_summary`. 
2. Entferne den `chars().take(800)` Limitierer, da die Zusammenfassung bereits kurz genug ist.
3. Passe den Prompt leicht an, da er nun strukturierte Summaries liest.
*🏁 Aktion nach diesem Schritt: Führe `cargo run --bin stage6_ai_test` aus, um die Annotationen lokal zu testen.*

### SCHRITT 5: Admin-Only UI (Dateien: `13_web_ui.rs`, `templates/jobs.html`, `templates/job_detail.html`)
1. Füge `pub is_admin: bool` zu `JobsTemplate` und `JobDetailTemplate` in `13_web_ui.rs` hinzu.
2. In `get_jobs` und `get_job_detail` implementiere den Logik-Check:
   ```rust
   let is_admin = user_name.as_ref().map_or(false, |name| name.to_lowercase() == state.config.admin_username.to_lowercase());
   ```
3. Aktualisiere die `JobsTemplate` und `JobDetailTemplate` Instanziierungen.
4. Öffne `templates/jobs.html` und umschließe die Anzeige von `job_level` mit `{% if is_admin %}`. Wiederhole das für `job_detail.html`.
*🏁 Aktion nach diesem Schritt: `cargo run` um den Server zu starten und das Web-UI als Admin und als Gast zu verifizieren.*