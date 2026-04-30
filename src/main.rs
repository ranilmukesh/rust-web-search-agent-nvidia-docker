use anyhow::{Context, Result};
use rig::client::CompletionClient;
use rig::completion::{Prompt, ToolDefinition};
use rig::providers::openai;
use rig::tool::{Tool, ToolError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct DuckDuckGoArgs {
    query: String,
    max_results: Option<usize>,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[derive(Debug, Serialize)]
struct DuckDuckGoOutput {
    query: String,
    results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
struct DuckDuckGoResponse {
    #[serde(rename = "Heading")]
    heading: Option<String>,
    #[serde(rename = "AbstractText")]
    abstract_text: Option<String>,
    #[serde(rename = "AbstractURL")]
    abstract_url: Option<String>,
    #[serde(rename = "RelatedTopics", default)]
    related_topics: Vec<DuckDuckGoTopic>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DuckDuckGoTopic {
    Topic {
        #[serde(rename = "Text")]
        text: String,
        #[serde(rename = "FirstURL")]
        first_url: String,
    },
    Category {
        #[serde(rename = "Topics", default)]
        topics: Vec<DuckDuckGoTopic>,
    },
}

#[derive(Clone)]
struct DuckDuckGoSearch;

impl DuckDuckGoSearch {
    fn new() -> Self {
        Self
    }
}

impl Tool for DuckDuckGoSearch {
    const NAME: &'static str = "duckduckgo_search";
    type Error = ToolError;
    type Args = DuckDuckGoArgs;
    type Output = DuckDuckGoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web via DuckDuckGo and return top results with titles, URLs, and snippets."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query."
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of results to return (1-10).",
                        "default": 5
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let max_results = args.max_results.unwrap_or(5).clamp(1, 10);

        info!(query = %args.query, max_results, "duckduckgo search");

        let mut url =
            reqwest::Url::parse("https://api.duckduckgo.com/").map_err(tool_call_error)?;
        url.query_pairs_mut()
            .append_pair("q", &args.query)
            .append_pair("format", "json")
            .append_pair("no_redirect", "1")
            .append_pair("no_html", "1")
            .append_pair("t", "rig-ddg-agent");

        debug!(url = %url, "duckduckgo request url");

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(tool_call_error)?
            .error_for_status()
            .map_err(tool_call_error)?
            .json::<DuckDuckGoResponse>()
            .await
            .map_err(tool_call_error)?;

        let DuckDuckGoResponse {
            heading,
            abstract_text,
            abstract_url,
            related_topics,
        } = response;

        let mut results = Vec::new();

        if let (Some(abstract_text), Some(abstract_url)) = (abstract_text, abstract_url) {
            if !abstract_text.trim().is_empty() && !abstract_url.trim().is_empty() {
                let title = heading.unwrap_or_else(|| "DuckDuckGo Abstract".to_string());
                results.push(SearchResult {
                    title,
                    url: abstract_url,
                    snippet: abstract_text,
                });
            }
        }

        for topic in related_topics {
            collect_topics(topic, &mut results, max_results);
            if results.len() >= max_results {
                break;
            }
        }

        info!(results = results.len(), "duckduckgo results ready");

        Ok(DuckDuckGoOutput {
            query: args.query,
            results,
        })
    }
}

fn collect_topics(topic: DuckDuckGoTopic, results: &mut Vec<SearchResult>, max: usize) {
    if results.len() >= max {
        return;
    }

    match topic {
        DuckDuckGoTopic::Topic { text, first_url } => {
            let (title, snippet) = split_text(&text);
            results.push(SearchResult {
                title,
                url: first_url,
                snippet,
            });
        }
        DuckDuckGoTopic::Category { topics } => {
            for child in topics {
                collect_topics(child, results, max);
                if results.len() >= max {
                    break;
                }
            }
        }
    }
}

fn tool_call_error<E: std::fmt::Display>(err: E) -> ToolError {
    ToolError::ToolCallError(err.to_string().into())
}

fn split_text(text: &str) -> (String, String) {
    if let Some((title, rest)) = text.split_once(" - ") {
        (title.to_string(), rest.to_string())
    } else {
        let trimmed = text.trim();
        (trimmed.to_string(), trimmed.to_string())
    }
}

fn normalize_base_url(base_url: String) -> String {
    let trimmed = base_url.trim_end_matches('/');
    if trimmed.ends_with("/v1") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/v1")
    }
}

fn init_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init()
        .map_err(|e| anyhow::anyhow!("failed to initialize logging: {e}"))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let api_key = env::var("NVIDIA_API_KEY")
        .or_else(|_| env::var("OPENAI_API_KEY"))
        .context("Missing NVIDIA_API_KEY or OPENAI_API_KEY")?;
    let base_url = normalize_base_url(
        env::var("RIG_OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://integrate.api.nvidia.com/v1".to_string()),
    );
    let model = env::var("RIG_MODEL").unwrap_or_else(|_| "deepseek-ai/deepseek-v4-pro".to_string());

    info!(base_url = %base_url, model = %model, "openai client configured");

    let client = openai::Client::builder()
        .api_key(&api_key)
        .base_url(&base_url)
        .build()?
        .completions_api();

    let agent = client
        .agent(&model)
        .preamble(
            "You are a web search assistant. Use the duckduckgo_search tool to fetch results. \
Return a concise answer and include source URLs.",
        )
        .max_tokens(1024)
        .tool(DuckDuckGoSearch::new())
        .build();

    let input = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if input.trim().is_empty() {
        "What is Rig and how do I get started?".to_string()
    } else {
        input
    };

    info!(prompt = %prompt, "sending prompt");

    let response = agent.prompt(&prompt).max_turns(40).await?;
    info!(response_len = response.len(), "received response");
    println!("{response}");
    Ok(())
}
