#![cfg(feature = "ai")]

use std::sync::Arc;
use crate::db_traits::DatabaseProvider;
use crate::ai_core::AiProvider;

/// Holt unannotierte Jobs, generiert per AI Zusammenfassungen und speichert diese.
pub async fn annotate_unannotated_jobs(
    db: Arc<dyn DatabaseProvider>,
    ai: Arc<dyn AiProvider>,
    limit: usize,
) -> anyhow::Result<usize> {
    let unannotated = db.get_unannotated_jobs(limit).await?;
    if unannotated.is_empty() {
        return Ok(0);
    }
    
    let annotations = ai.annotate_jobs(unannotated.clone()).await?;
    let mut count = 0;
    
    for annotation in annotations {
        // Finde den passenden Job über den `idx` Index der AI Antwort
        if let Some(job) = unannotated.get(annotation.idx as usize) {
            // Konvertiere das Array aus Strings in eine formatierte Aufzählungsliste
            let formatted_summary = annotation.job_summary
                .iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            
            db.update_job_ai(&job.identifier, &formatted_summary).await?;
            count += 1;
        }
    }
    
    Ok(count)
}
