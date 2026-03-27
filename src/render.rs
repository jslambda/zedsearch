use crate::parse::SearchHit;

const MAX_RESULTS: usize = 10;

pub fn render_index_success(project_root: &str, index_dir: &str) -> String {
    format!(
        "srcsearch index created successfully.\n\nProject root: {project_root}\nIndex dir: {index_dir}"
    )
}

pub fn render_update_success(project_root: &str, index_dir: &str) -> String {
    format!(
        "srcsearch index refreshed successfully.\n\nPhase 1 note: /srcupdate currently performs a full reindex.\nProject root: {project_root}\nIndex dir: {index_dir}"
    )
}

pub fn render_search_results(query: &str, docs_only: bool, hits: &[SearchHit]) -> String {
    if hits.is_empty() {
        return format!(
            "No results found for: {query}\n\nTry a broader query, or run /srcindex if your index is stale."
        );
    }

    let mut lines = Vec::new();
    if docs_only {
        lines.push(format!("Top documentation results for: {query}"));
    } else {
        lines.push(format!("Top results for: {query}"));
    }
    lines.push(String::new());

    for (idx, hit) in hits.iter().take(MAX_RESULTS).enumerate() {
        let location = if let Some(line) = hit.line {
            format!("{}:{}", hit.path, line)
        } else {
            hit.path.clone()
        };

        lines.push(format!("{}. {}", idx + 1, location));
        if let Some(title) = &hit.title {
            lines.push(format!("   Title: {title}"));
        }
        if let Some(score) = hit.score {
            lines.push(format!("   Score: {:.2}", score));
        }
        if let Some(snippet) = &hit.snippet {
            lines.push(format!("   Snippet: {}", normalize_whitespace(snippet)));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_no_results_state() {
        let rendered = render_search_results("parser", false, &[]);
        assert!(rendered.contains("No results found for: parser"));
    }

    #[test]
    fn renders_ranked_results() {
        let hits = vec![SearchHit {
            path: "src/parser.rs".to_string(),
            line: Some(42),
            title: Some("parse_expression".to_string()),
            snippet: Some("Parses  the next expression".to_string()),
            score: Some(12.3),
        }];

        let rendered = render_search_results("parser", false, &hits);
        assert!(rendered.contains("1. src/parser.rs:42"));
        assert!(rendered.contains("Score: 12.30"));
    }
}
