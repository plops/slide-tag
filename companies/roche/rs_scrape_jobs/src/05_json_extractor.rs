use anyhow::Result;
use regex::Regex;

/// Extract phApp.ddo JSON from HTML content
pub fn extract_phapp_json(html: &str) -> Result<String> {
    // Use `(?s)` so that `.` matches newlines.
    // We capture the JSON object non-greedily `(\{.*?\})` up until the trailing semicolon 
    // that is immediately followed by the next expected JS token (like `phApp.`, `var`, `/*`, or `</script>`).
    let re = Regex::new(r"(?s)phApp\.ddo\s*=\s*(\{.*?\})\s*;\s*(?:phApp\.|var\b|let\b|const\b|/\*|</script>)")?;
    
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