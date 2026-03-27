use crate::errors::ExtensionError;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub path: String,
    pub line: Option<usize>,
    pub title: Option<String>,
    pub snippet: Option<String>,
    pub score: Option<f64>,
}

pub fn parse_search_hits(stdout: &str) -> Result<Vec<SearchHit>, ExtensionError> {
    let value: Value =
        serde_json::from_str(stdout).map_err(|err| ExtensionError::InvalidJson(err.to_string()))?;

    let hits_value = if let Some(array) = value.as_array() {
        array
    } else if let Some(array) = value.get("hits").and_then(Value::as_array) {
        array
    } else {
        return Err(ExtensionError::InvalidJson(
            "Expected top-level array or object with `hits` array".to_string(),
        ));
    };

    Ok(hits_value.iter().filter_map(parse_hit).collect())
}

fn parse_hit(value: &Value) -> Option<SearchHit> {
    let path = find_string(value, &["path", "file_path", "file"])?;
    let line = find_usize(value, &["line", "line_start", "line_number"]);
    let title = find_string(value, &["title", "section_title", "symbol"]);
    let snippet = find_string(value, &["snippet", "body_text", "text"]);
    let score = find_f64(value, &["score", "rank"]);

    Some(SearchHit {
        path,
        line,
        title,
        snippet,
        score,
    })
}

fn find_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(key))
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn find_usize(value: &Value, keys: &[&str]) -> Option<usize> {
    keys.iter().find_map(|key| {
        value.get(key).and_then(|value| {
            value
                .as_u64()
                .map(|number| number as usize)
                .or_else(|| value.as_str().and_then(|s| s.parse::<usize>().ok()))
        })
    })
}

fn find_f64(value: &Value, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|key| {
        value.get(key).and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_str().and_then(|s| s.parse::<f64>().ok()))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hits_from_array_json() {
        let json = r#"[
            {"file_path":"src/lib.rs","line_start":12,"symbol":"run","snippet":"does work","score":4.2}
        ]"#;

        let hits = parse_search_hits(json).expect("expected parsed hits");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "src/lib.rs");
        assert_eq!(hits[0].line, Some(12));
    }

    #[test]
    fn parses_hits_from_object_json() {
        let json = r#"{"hits":[{"path":"docs/readme.md","line":3,"title":"Overview"}]}"#;

        let hits = parse_search_hits(json).expect("expected parsed hits");
        assert_eq!(hits[0].path, "docs/readme.md");
        assert_eq!(hits[0].title.as_deref(), Some("Overview"));
    }
}
