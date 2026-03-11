# ARCHITEKTUR- UND IMPLEMENTIERUNGSPLAN (v2)
**PROJEKT:** `rs_scrape_jobs`
**ZIEL:** Optimierung des AI-Workflows für Job-Summaries und Candidate-Matching.

**ANWEISUNG AN DIE IMPLEMENTIERENDE KI (WORKER AI):**
Lies diesen Plan vollständig durch, bevor du beginnst. Du bist die ausführende Instanz. Halte dich exakt an die Code-Strukturen, die Limits und die Reihenfolge der Schritte. Nichts wird geraten.

---

## 🛠️ DEEPWIKI MCP & DEPENDENCIES
Falls du während der Implementierung unsicher über die API einer externen Bibliothek bist (insbesondere für das AI-Framework Rig oder Axum), **musst** du die DeepWiki MCP-Schnittstelle nutzen, um die Dokumentation zu lesen oder fragen zu stellen (beispielsweise nach Code-Beispielen).

**Liste der relevanten Dependencies (Github-Organisation / Projektname):**
*   **Rig (AI Framework):** `0xPlaygrounds/rig` *(Sehr wichtig für die Retry-Logik und den CompletionClient)*
*   **Axum (Web Framework):** `tokio-rs/axum`
*   **Tokio (Async Runtime):** `tokio-rs/tokio`
*   **Askama (Templating):** `djc/askama`
*   **Tower Sessions (Auth/Session):** `maxcountryman/tower-sessions`
*   **Serde (Serialization):** `serde-rs/serde` und `serde-rs/json`
*   **Anyhow (Error Handling):** `dtolnay/anyhow`

---

## TEIL 1: ANALYSE DES AKTUELLEN KI-WORKFLOWS & KORREKTUREN

### 1.1 Identifizierte Engpässe (Bottlenecks)
*   **Context Window Degradation (Das Hauptproblem):** Aktuell werden Batches nur durch das TPM-Limit (Tokens Per Minute = 250.000) geblockt. Das führt dazu, dass gigantische Requests (bis zu 250.000 Tokens) an Gemini gesendet werden. Bei solch riesigen Prompts verliert das Modell den Fokus und liefert fehlerhaften oder unvollständigen strukturierten Output (JSON).
*   **Informationsverlust durch Abschneiden:** Die aktuelle Code-Basis nutzt `.chars().take(500)` für Job-Beschreibungen. Das ist ein fataler Fehler, da die KI so niemals die volle Beschreibung lesen und zusammenfassen kann.
*   **Ineffizientes Candidate Matching:** Das Matching nutzt derzeit die riesigen, rohen Job-Beschreibungen. Das macht das Matching langsam, teuer und inakkurat.

---

## TEIL 2: ARCHITEKTUR-ENTWURF

### 2.1 Dual-Control Rate Limiting (TPM + Words per Request)
Um die Qualität des JSON-Outputs zu sichern, führen wir ein hartes Limit von **25.000 Wörtern pro Request** ein.
Da wir 15 RPM (Requests per Minute) zur Verfügung haben, können wir problemlos mehrere kleinere 25k-Wörter-Requests pro Minute senden, bis das globale TPM-Limit (250.000) greift.

**Architektur des BatchBuilders:**
*   Wir tracken neben `current_tokens` nun auch `current_words`.
*   Ein Job wird einem Batch nur hinzugefügt, wenn weder das Token-Treshold (80% von TPM) NOCH das neue `max_words_per_request` Limit überschritten wird.

### 2.2 Vollständige Daten für Summaries & ATS-Felder
*   Beim Erstellen der `job_summary` (in `process_batch`) darf **nichts** mehr abgeschnitten werden.
*   Zusätzlich zur Beschreibung müssen die ATS-Daten `job_level`, `job_family`, `grade_profile` und `employment_type` in den Prompt integriert werden.

### 2.3 Optimiertes Matching über Summaries
*   In `process_candidate_matching_batch` ersetzen wir die Übergabe der vollen `job.description` durch das Feld `job.job_summary`.
*   Dadurch wird der Prompt drastisch verkleinert und das Signal-to-Noise-Ratio für die KI steigt enorm.

---

## 🚀 IMPLEMENTIERUNGSPLAN FÜR DIE WORKER KI (Schritt-für-Schritt)

Führe diese Schritte streng sequenziell aus.

### SCHRITT 1: Rate Limiter & Batch Builder erweitern
**Dateien:** `src/07d_ai_rate_limiter.rs`, `src/07e_ai_batch_builder.rs`

1.  Füge in `src/07d_ai_rate_limiter.rs` dem Struct `AiModelConfig` das Feld `pub words_per_request: u32` hinzu.
2.  Setze in `src/07b_ai_gemini.rs` in der `GeminiProvider::new` Funktion den Wert `words_per_request: 25000`.
3.  Öffne `src/07e_ai_batch_builder.rs`. Erweitere das `BatchBuilder` Struct:
    ```rust
    pub struct BatchBuilder {
        // ... bestehende Felder ...
        current_words: u32,
        max_words_per_request: u32,
    }
    ```
4.  Passe die `new`-Funktion von `BatchBuilder` an, sodass sie `max_words_per_request` aus der Konfiguration ausliest (ähnlich wie `tpm_limit`).
5.  In der `try_add_job` Funktion:
    *   Zähle die Wörter des Jobs (Nutze die bestehende Logik in `estimate_job_tokens` als Basis oder lagere die Wortzählung in eine eigene Funktion `count_job_words(&job)` aus).
    *   Prüfe: `if self.current_tokens + job_tokens > self.token_threshold || self.current_words + job_words > self.max_words_per_request { return false; }`
    *   Addiere `self.current_words += job_words;` wenn der Job hinzugefügt wird.

*🏁 Aktion: Führe `cargo check` aus.*

### SCHRITT 2: Fortschritts-Logging & Retry-Logik implementieren
**Datei:** `src/07b_ai_gemini.rs`

1.  In `annotate_jobs` und `match_candidate`:
    *   Definiere `let total_jobs = jobs.len();` und `let mut processed_jobs = 0;` VOR der `while`-Schleife.
    *   Nachdem ein Batch gepickt wurde (`if let Some(batch) = batch_builder.take_batch()`), logge den Fortschritt VOR der Verarbeitung:
        `println!("Processing job descriptions {}..{} out of {}", processed_jobs + 1, processed_jobs + batch.len(), total_jobs);`
    *   Messe die Zeit mit `std::time::Instant::now()` und logge nach dem `process_batch` Aufruf die Dauer und den Token-Verbrauch.
    *   Erhöhe `processed_jobs += batch.len();`.
2.  Implementiere in `process_batch` und `process_candidate_matching_batch` eine Retry-Schleife um den `completion_model.completion(request).await` Aufruf.
    *   Versuche es maximal 3 Mal.
    *   Nutze bei einem Error `tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;`, logge eine Warnung und iteriere weiter.

*🏁 Aktion: Führe `cargo check` aus.*

### SCHRITT 3: Volle Job-Beschreibungen & ATS Felder im Prompt (CRITICAL)
**Datei:** `src/07b_ai_gemini.rs`

1.  In `process_batch`, passe die Formatierung von `input` an.
    *   **LÖSCHE** den `.chars().take(500)` Limitierer! Der Code muss lauten: `job.description.as_deref().unwrap_or("N/A")`. Die gesamte Beschreibung MUSS an die KI gehen!
    *   Füge die geforderten ATS-Felder hinzu:
        ```rust
        input.push_str(&format!("Job Level: {}\n", job.job_level.as_deref().unwrap_or("N/A")));
        input.push_str(&format!("Job Family: {}\n", job.job_family.as_deref().unwrap_or("N/A")));
        input.push_str(&format!("Grade Profile: {}\n", job.grade_profile.as_deref().unwrap_or("N/A")));
        input.push_str(&format!("Employment Type: {}\n", job.employment_type.as_deref().unwrap_or("N/A")));
        ```
2.  Passe das `preamble` an: Weise die KI explizit an, in der Originalsprache der Jobbeschreibung zu antworten (DE/EN).

*🏁 Aktion: Führe `cargo build` aus.*

### SCHRITT 4: Candidate-Matching Optimierung (Summaries nutzen)
**Datei:** `src/07b_ai_gemini.rs`

1.  In `process_candidate_matching_batch`:
    *   Wir nutzen hier NICHT mehr `job.description`.
    *   Ersetze das Einlesen der Description durch das Einlesen der Summary:
        ```rust
        input.push_str(&format!(
            "Job Summary: {}\n",
            job.job_summary.as_deref().unwrap_or("No summary available")
        ));
        ```
    *   Entferne logischerweise jegliches `.chars().take(800)`.
2.  Passe das `preamble` für das Matching an, um der KI mitzuteilen, dass sie gegen "Job Summaries" matcht.

*🏁 Aktion: Führe das Test-Script via `cargo run --bin stage6_ai_test` aus. Verifiziere in der Konsole, dass die Batches jetzt kleiner sind, alle Jobs vollständig verarbeitet werden und die Log-Ausgabe ("Processing 1..25 out of 180") korrekt erscheint.*

### SCHRITT 5: Admin-Only Job-Level Display im Web-UI
**Dateien:** `src/13_web_ui.rs`, `templates/jobs.html`, `templates/job_detail.html`

1.  Erweitere die `JobsTemplate` und `JobDetailTemplate` structs in `src/13_web_ui.rs` um ein Feld `pub is_admin: bool`.
2.  Implementiere in `get_jobs` und `get_job_detail` den Check gegen `state.config.admin_username` (Nutze die Funktion `is_admin` aus `17_admin.rs` als logisches Vorbild, aber achte darauf, hier keinen Error zu werfen, wenn jemand kein Admin ist, sondern setze `is_admin` einfach auf `false`).
    ```rust
    let is_admin = match user_name {
        Some(ref name) => name.to_lowercase() == state.config.admin_username.to_lowercase(),
        None => false,
    };
    ```
3.  Umschließe in `templates/jobs.html` und `templates/job_detail.html` das Rendering von `job.job_level` mit `{% if is_admin %}`.

*🏁 Aktion: Führe `cargo run` aus. Logge dich als normaler Nutzer ein -> das Job-Level darf nicht sichtbar sein. Logge dich als Admin ("plops") ein -> das Job-Level muss sichtbar sein.*