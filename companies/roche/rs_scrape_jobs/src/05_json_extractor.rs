use anyhow::Result;
use regex::Regex;

/// Extract phApp.ddo JSON from HTML content
pub fn extract_phapp_json(html: &str) -> Result<String> {
    let re = Regex::new(r"phApp\.ddo\s*=\s*([^;]+);")?;
    if let Some(captures) = re.captures(html) {
        if let Some(json_match) = captures.get(1) {
            let json_str = json_match.as_str().trim();
            Ok(json_str.to_string())
        } else {
            anyhow::bail!("No JSON found in phApp.ddo assignment");
        }
    } else {
        anyhow::bail!("phApp.ddo not found in HTML");
    }
}
