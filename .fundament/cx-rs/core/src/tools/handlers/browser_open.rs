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

const TOOL_NAME: &str = "browser_open";

#[derive(Debug)]
struct BrowserOpenOutput {
    url: String,
}

impl crate::tools::context::ToolOutput for BrowserOpenOutput {
    fn log_preview(&self) -> String {
        format!("Opened {}", self.url)
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
        FunctionToolOutput::from_text(format!("Opened {} in system browser", self.url), Some(true))
            .to_response_item(call_id, _payload)
    }

    fn code_mode_result(&self, _payload: &crate::tools::context::ToolPayload) -> serde_json::Value {
        serde_json::json!({ "opened": self.url })
    }
}

pub struct BrowserOpenHandler;

impl ToolExecutor<ToolInvocation> for BrowserOpenHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Open a URL in the system browser.".to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                BTreeMap::from([(
                    "url".to_string(),
                    JsonSchema::string(Some(
                        "The URL to open (e.g. https://example.com).".to_string(),
                    )),
                )]),
                Some(vec!["url".to_string()]),
                Some(false.into()),
            ),
            output_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "opened": {
                        "type": "string",
                        "description": "The URL that was opened."
                    }
                },
                "required": ["opened"],
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

            let output = Command::new("open").arg(&url).output();

            match output {
                Ok(_) => Ok(boxed_tool_output(BrowserOpenOutput { url })),
                Err(e) => Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        format!("Failed to open URL: {}", e),
                        Some(false),
                    ),
                )),
            }
        })
    }
}

impl CoreToolRuntime for BrowserOpenHandler {}
