
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

// use anyhow::{Context, Result};
// use regex::Regex;
// use serde::Deserialize;

// /// Extract phApp.ddo JSON from HTML content robustly using Serde
// pub fn extract_phapp_json(html: &str) -> Result<String> {
//     // 1. Find where the JSON assignment starts. 
//     // This simple regex is extremely fast and avoids parsing the whole DOM.
//     let re = Regex::new(r"phApp\.ddo\s*=\s*")?;
//     let mat = re.find(html).context("phApp.ddo not found in HTML")?;
    
//     // 2. Slice the HTML so it begins exactly at the `{` of the JSON object.
//     let json_start_str = &html[mat.end()..];
    
//     // 3. Initialize a serde_json Deserializer.
//     let mut deserializer = serde_json::Deserializer::from_str(json_start_str);
    
//     // 4. Ask Serde to parse exactly ONE valid JSON value. 
//     // It will natively handle all nested brackets, escaped quotes, and strings.
//     // It will stop automatically at the final closing `}`.
//     let _parsed_value: serde_json::Value = Deserialize::deserialize(&mut deserializer)
//         .context("Failed to parse the embedded JSON structure")?;
        
//     // 5. The deserializer tracks exactly how many bytes it consumed.
//     // We can use this offset to slice the exact, perfectly valid JSON string!
//     let json_length = deserializer.byte_offset();
//     let valid_json_str = &json_start_str[..json_length];
    
//     Ok(valid_json_str.to_string())
// }