use crate::tools::context::ToolInvocation;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use cx_tools::JsonSchema;
use cx_tools::ResponsesApiTool;
use cx_tools::ToolName;
use cx_tools::ToolSpec;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::process::Command;

const TOOL_NAME: &str = "browser_fetch";

#[derive(Debug)]
struct BrowserFetchOutput {
    url: String,
    body: String,
}

impl crate::tools::context::ToolOutput for BrowserFetchOutput {
    fn log_preview(&self) -> String {
        format!("Fetched {} ({} chars)", self.url, self.body.len())
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(
        &self,
        call_id: &str,
        _payload: &crate::tools::context::ToolPayload,
    ) -> cx_protocol::models::ResponseInputItem {
        use crate::tools::context::FunctionToolOutput;
        FunctionToolOutput::from_text(self.body.clone(), Some(true))
            .to_response_item(call_id, _payload)
    }

    fn code_mode_result(&self, _payload: &crate::tools::context::ToolPayload) -> serde_json::Value {
        serde_json::json!({
            "url": self.url,
            "length": self.body.len(),
        })
    }
}

pub struct BrowserFetchHandler;

impl ToolExecutor<ToolInvocation> for BrowserFetchHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Fetch a URL and return its content as text.".to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                BTreeMap::from([
                    (
                        "url".to_string(),
                        JsonSchema::string(Some(
                            "The URL to fetch (e.g. https://example.com).".to_string(),
                        )),
                    ),
                    (
                        "method".to_string(),
                        JsonSchema::string(Some("HTTP method, default GET.".to_string())),
                    ),
                    (
                        "body".to_string(),
                        JsonSchema::string(Some("Optional request body for POST.".to_string())),
                    ),
                ]),
                Some(vec!["url".to_string()]),
                Some(false.into()),
            ),
            output_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "length": { "type": "number" }
                },
                "required": ["url", "length"],
                "additionalProperties": false
            })),
        })
    }

    fn handle(&self, invocation: ToolInvocation) -> cx_tools::ToolExecutorFuture<'_> {
        Box::pin(async move {
            let args_str = match &invocation.payload {
                crate::tools::context::ToolPayload::Function { arguments } => arguments,
                _ => "",
            };
            let args: serde_json::Map<String, JsonValue> =
                serde_json::from_str(args_str).unwrap_or_default();
            let url = args
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            if url.is_empty() {
                return Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        "Error: url is required".to_string(),
                        Some(false),
                    ),
                ));
            }

            let method = args.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
            let body = args.get("body").and_then(|v| v.as_str());

            let mut cmd = Command::new("curl");
            cmd.arg("-s")
                .arg("-L")
                .arg("--max-time")
                .arg("15")
                .arg(&url);

            if method == "POST" {
                cmd.arg("-X").arg("POST");
            }

            if let Some(b) = body {
                if !b.is_empty() {
                    cmd.arg("-d").arg(b);
                }
            }

            let output = cmd.output();

            match output {
                Ok(out) if out.status.success() => {
                    let text = String::from_utf8_lossy(&out.stdout).to_string();
                    let truncated = if text.len() > 200_000 {
                        format!("{}... [truncated]", &text[..200_000])
                    } else {
                        text
                    };
                    Ok(boxed_tool_output(BrowserFetchOutput {
                        url,
                        body: truncated,
                    }))
                }
                Ok(out) => Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        format!("curl failed: {}", String::from_utf8_lossy(&out.stderr)),
                        Some(false),
                    ),
                )),
                Err(e) => Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        format!("Failed to run curl: {}", e),
                        Some(false),
                    ),
                )),
            }
        })
    }
}

impl CoreToolRuntime for BrowserFetchHandler {}
