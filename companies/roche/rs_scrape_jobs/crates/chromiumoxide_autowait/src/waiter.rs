use crate::types::{ActionabilityError, AutoWaitOptions, ElementState};
use crate::scripts::{CHECK_STATES_JS, CHECK_STABLE_JS};
use chromiumoxide::Page;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const BACKOFF_DELAYS_MS: &[u64] = &[0, 20, 100, 100, 500];

pub async fn wait_for_states(
    page: &Page,
    selector: &str,
    states: &[ElementState],
    options: &AutoWaitOptions,
) -> Result<(), ActionabilityError> {
    let deadline = Instant::now() + options.timeout;
    let mut retry = 0usize;

    loop {
        if Instant::now() >= deadline {
            return Err(ActionabilityError::Timeout);
        }

        if retry > 0 {
            let delay_ms = BACKOFF_DELAYS_MS[retry.min(BACKOFF_DELAYS_MS.len() - 1)];
            sleep(Duration::from_millis(delay_ms)).await;
        }

        let mut all_passed = true;
        
        // Erst Stabilität prüfen (Async via requestAnimationFrame)
        if states.contains(&ElementState::Stable) {
            let js = format!("({CHECK_STABLE_JS})(`{selector}`)");
            match page.evaluate(js).await {
                Ok(res) => {
                    let val = res.into_value::<serde_json::Value>().unwrap_or_default();
                    if val.get("error").is_some() || val.get("missingState").is_some() {
                        all_passed = false;
                    }
                },
                Err(_) => { all_passed = false; }
            }
        }

        // Dann synchrone States prüfen
        if all_passed {
            let sync_states: Vec<&str> = states.iter()
                .filter(|&&s| s != ElementState::Stable)
                .map(|s| match s {
                    ElementState::Visible => "visible",
                    ElementState::Enabled => "enabled",
                    ElementState::Editable => "editable",
                    _ => "",
                })
                .collect();
            
            if !sync_states.is_empty() {
                let states_json = serde_json::to_string(&sync_states).unwrap();
                let js = format!("({CHECK_STATES_JS})(`{selector}`, {states_json})");
                match page.evaluate(js).await {
                    Ok(res) => {
                        let val = res.into_value::<serde_json::Value>().unwrap_or_default();
                        if val.get("error").is_some() || val.get("missingState").is_some() {
                            all_passed = false;
                        }
                    },
                    Err(_) => { all_passed = false; }
                }
            }
        }

        if all_passed { return Ok(()); }
        retry += 1;
    }
}
