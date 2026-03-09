Fixe die Session-Verwaltung und behebe den Bug, dass Matches nach einem Logout/Login (oder auch nach dem Aktualisieren des Profils) scheinbar "verschwinden".

### 🔍 Die Ursachenanalyse (Root Cause)

In `src/01b_db_repo.rs` nutzt die Funktion `upsert_candidate` aktuell diesen SQL-Befehl:
`INSERT OR REPLACE INTO candidates ...`

**Was hier unter der Haube in SQLite passiert:**
Wenn ein User sich einloggt, wird geprüft, ob seine `oauth_sub` schon existiert. Wenn ja, greift das `REPLACE`. In SQLite ist `REPLACE` ein Alias für `DELETE` gefolgt von `INSERT`. 
Das bedeutet: Der existierende Datensatz des Kandidaten wird gelöscht und ein exakt gleicher neu angelegt. **Dabei generiert SQLite aber eine völlig neue `id` (Primary Key)!**

Die in `candidate_matches` gespeicherten Job-Matches verweisen aber über `candidate_id` auf die *alte* ID. Nach dem Login hat dein User plötzlich eine neue `id` und seine alten Matches sind als "Datenmüll" verwaist in der Datenbank (Orphaned Records). Deshalb ist das Dashboard leer.

---

### 🛠️ Prompt-Anweisung für die ausführende KI

Kopiere den folgenden Block **exakt so** und gib ihn an das implementierende LLM weiter. Er ist so detailliert geschrieben, dass die KI keine Fehler machen kann.

***

**WICHTIGE ARBEITSANWEISUNGEN FÜR DEINE IMPLEMENTIERUNG:**

Du agierst als ausführender Entwickler. Dein Ziel ist es, einen kritischen "Primary Key Churn"-Bug zu beheben, der dafür sorgt, dass User nach dem Login ihre Job-Matches verlieren. 

Da der User in `12_auth.rs` bereits sauber mit seiner existierenden `id` initialisiert wird (`..existing`), müssen wir nur verhindern, dass die Datenbank diese ID beim Speichern verwirft.

**Schritt 1: Behebe den Bug in `src/01b_db_repo.rs`**
Ersetze die Methode `upsert_candidate` im `JobRepository` komplett. Du darfst **kein** `INSERT OR REPLACE` mehr verwenden! 
Prüfe stattdessen, ob `candidate.id` vorhanden ist (`Some`). Wenn ja, führe ein explizites `UPDATE` aus, ansonsten ein normales `INSERT`.

Implementiere es exakt so:

```rust
// in src/01b_db_repo.rs

async fn upsert_candidate(&self, candidate: &Candidate) -> anyhow::Result<i64> {
    if let Some(existing_id) = candidate.id {
        // User existiert bereits -> Mache ein klassisches UPDATE, um die ID zu erhalten!
        // Das verhindert, dass die Foreign Key Relation zu candidate_matches bricht.
        self.conn
            .execute(
                "UPDATE candidates SET name = ?, profile_text = ? WHERE id = ?",
                params![
                    candidate.name.clone(), 
                    candidate.profile_text.clone(), 
                    existing_id
                ],
            )
            .await?;
        
        // Gebe die existierende und beibehaltene ID zurück
        Ok(existing_id)
    } else {
        // Neuer User -> Normales INSERT
        self.conn
            .execute(
                "INSERT INTO candidates (oauth_sub, name, profile_text) VALUES (?, ?, ?)",
                params![
                    candidate.oauth_sub.clone(), 
                    candidate.name.clone(), 
                    candidate.profile_text.clone()
                ],
            )
            .await?;
            
        Ok(self.conn.last_insert_rowid())
    }
}
```

**Schritt 2: Logik-Verifikation in `src/13_web_ui.rs`**
Wenn du diesen Fix vorgenommen hast, ist auch automatisch der Bug behoben, der beim Ändern des Profil-Textes auftrat! In `post_profile` wird nämlich `updated_candidate` ebenfalls mit der bestehenden `id` an `upsert_candidate` übergeben. Ab sofort mutiert die ID nicht mehr. 

**WICHTIGE REGELN:**
1. Ändere nichts an der `params!` Makro-Syntax von libsql, der obige Code ist bereits korrekt formatiert.
2. Keine Änderungen an `12_auth.rs` nötig, da diese Datei die `Candidate` Struct bereits vorbildlich mit `..existing` zusammenbaut und die `id` somit korrekt transportiert.

***

### 🏛️ Architektonischer Ausblick für dich

Sobald das obige LLM diese Änderung eingespielt hat, bleiben IDs über Logins und Profil-Updates hinweg stabil und die Matches bleiben erhalten. 

**Tech-Debt Hinweis:** Durch das bisherige Fehlverhalten haben sich in der Tabelle `candidate_matches` verwaiste Einträge angesammelt (Matches, die zu nicht mehr existierenden Candidate-IDs gehören). Bei Gelegenheit solltest du ein kleines DB-Migration-Script schreiben, das Fremdschlüssel (`FOREIGN KEY (candidate_id) REFERENCES candidates(id) ON DELETE CASCADE`) in SQLite erzwingt oder einen Cleanup-Job baut, um die alte Historie zu bereinigen. Für das aktuelle Fehlerbild ist das aber nicht zwingend erforderlich.