use anyhow::Result;
use regex::Regex;

pub fn extract_phapp_json_regex(html: &str) -> Result<String> {
    // (?s) is Rust's version of Python's re.DOTALL
    // It looks for the exact sequence `};` to stop, just like the Python script.
    let re = Regex::new(r"(?s)phApp\.ddo\s*=\s*(\{.*?\});")?;

    if let Some(captures) = re.captures(html) {
        if let Some(json_match) = captures.get(1) {
            return Ok(json_match.as_str().to_string());
        }
    }

    anyhow::bail!("phApp.ddo not found in HTML");
}
