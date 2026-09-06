use crate::tools::context::ToolInvocation;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use cx_tools::JsonSchema;
use cx_tools::ResponsesApiTool;
use cx_tools::ToolName;
use cx_tools::ToolSpec;
use std::collections::BTreeMap;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const TOOL_NAME: &str = "browser_screenshot";

#[derive(Debug)]
struct BrowserScreenshotOutput {
    path: String,
}

impl crate::tools::context::ToolOutput for BrowserScreenshotOutput {
    fn log_preview(&self) -> String {
        format!("Screenshot saved to {}", self.path)
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
        FunctionToolOutput::from_text(format!("Screenshot saved to {}", self.path), Some(true))
            .to_response_item(call_id, _payload)
    }

    fn code_mode_result(&self, _payload: &crate::tools::context::ToolPayload) -> serde_json::Value {
        serde_json::json!({ "path": self.path })
    }
}

pub struct BrowserScreenshotHandler;

impl ToolExecutor<ToolInvocation> for BrowserScreenshotHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Take a screenshot of the current screen and save it to a temp file."
                .to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(BTreeMap::new(), None, Some(false.into())),
            output_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the saved screenshot PNG."
                    }
                },
                "required": ["path"],
                "additionalProperties": false
            })),
        })
    }

    fn handle(&self, _invocation: ToolInvocation) -> cx_tools::ToolExecutorFuture<'_> {
        Box::pin(async move {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let path = format!("/tmp/cy-screenshot-{}.png", ts);

            let output = Command::new("screencapture").arg("-x").arg(&path).output();

            match output {
                Ok(out) if out.status.success() => {
                    Ok(boxed_tool_output(BrowserScreenshotOutput { path }))
                }
                Ok(out) => Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        format!(
                            "screencapture failed: {}",
                            String::from_utf8_lossy(&out.stderr)
                        ),
                        Some(false),
                    ),
                )),
                Err(e) => Ok(boxed_tool_output(
                    crate::tools::context::FunctionToolOutput::from_text(
                        format!("Failed to run screencapture: {}", e),
                        Some(false),
                    ),
                )),
            }
        })
    }
}

impl CoreToolRuntime for BrowserScreenshotHandler {}
