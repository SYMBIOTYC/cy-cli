//! Registry of model providers supported by CX.
//!
//! Providers can be defined in two places:
//!   1. Built-in defaults compiled into the binary so CX works out-of-the-box.
//!   2. User-defined entries inside `~/.cx/config.toml` under the `model_providers`
//!      key. These override or extend the defaults at runtime.
//!
//! SYMBIOTYC: Modified to add CY cyborg provider as default.
//! Brand: CY Cyborg | https://cy.symbiotyc.workers.dev

use cx_api::Provider as ApiProvider;
use cx_api::RetryConfig as ApiRetryConfig;
use cx_protocol::auth::AuthMode;
use cx_protocol::config_types::ModelProviderAuthInfo;
use cx_protocol::error::CxErr;
use cx_protocol::error::EnvVarError;
use cx_protocol::error::Result as CodexResult;
use http::HeaderMap;
use http::header::HeaderName;
use http::header::HeaderValue;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU64;
use std::time::Duration;

const DEFAULT_STREAM_IDLE_TIMEOUT_MS: u64 = 300_000;
const DEFAULT_STREAM_MAX_RETRIES: u64 = 5;
const DEFAULT_REQUEST_MAX_RETRIES: u64 = 4;
const DEFAULT_AWS_AUTH_REFRESH_TIMEOUT_MS: u64 = 300_000;
pub const DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS: u64 = 15_000;
/// Hard cap for user-configured `stream_max_retries`.
const MAX_STREAM_MAX_RETRIES: u64 = 100;
/// Hard cap for user-configured `request_max_retries`.
const MAX_REQUEST_MAX_RETRIES: u64 = 100;

pub const CHATGPT_CODEX_BASE_URL: &str = "https://cy.symbiotyc.workers.dev/v1";
const CY_PROVIDER_NAME: &str = "CY Cyborg";
pub const CY_PROVIDER_ID: &str = "cy";
// CY-CLI ships with a local `cy_bridge.py` that translates OpenAI-compatible
// requests into the SYMBIOTYC Cloud chat endpoint. The bridge runs on
// 127.0.0.1:8790 by default and is started by the macOS .app launcher; it can
// also be started manually with `python3 cy_bridge.py &` before invoking
// `cy exec` from a terminal.
//
// Set `CY_BASE_URL` to override the
// bridge endpoint, e.g. to point at a remote bridge, a staging deployment,
// or directly at `https://cy.symbiotyc.workers.dev/v1` if you want to skip
// the bridge entirely.
const CY_PROVIDER_DEFAULT_BASE_URL: &str = "http://127.0.0.1:8790/v1";
const CY_PROVIDER_BASE_URL_ENV_VAR: &str = "CY_BASE_URL";
const CY_PROVIDER_ENV_KEY: &str = "CY_API_KEY";
const CY_PROVIDER_ENV_KEY_INSTRUCTIONS: &str = "Set the CY_API_KEY environment variable to your SYMBIOTYC Cloud API key. \
     `cy login` writes the key to ~/.cy/auth.json and the launcher exports \
     it before starting the TUI or the bridge.";
const CHAT_WIRE_API_REMOVED_ERROR: &str = "`wire_api = \"chat\"` is no longer supported.\nHow to fix: set `wire_api = \"responses\"` in your provider config.\nMore info: https://github.com/SYMBIOTYC/cy-cli/discussions/7782";
pub const LEGACY_OLLAMA_CHAT_PROVIDER_ID: &str = "ollama-chat";
pub const OLLAMA_CHAT_PROVIDER_REMOVED_ERROR: &str = "`ollama-chat` is no longer supported.\nHow to fix: replace `ollama-chat` with `ollama` in `model_provider`, `oss_provider`, or `--local-provider`.\nMore info: https://github.com/SYMBIOTYC/cy-cli/discussions/7782";

/// Wire protocol that the provider speaks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WireApi {
    /// The Responses API exposed by oi at `/v1/responses`.
    #[default]
    Responses,
}

impl fmt::Display for WireApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Responses => "responses",
        };
        f.write_str(value)
    }
}

impl<'de> Deserialize<'de> for WireApi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "responses" => Ok(Self::Responses),
            "chat" => Err(serde::de::Error::custom(CHAT_WIRE_API_REMOVED_ERROR)),
            _ => Err(serde::de::Error::unknown_variant(&value, &["responses"])),
        }
    }
}

/// Serializable representation of a provider definition.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelProviderInfo {
    /// Friendly display name.
    #[serde(default)]
    pub name: String,
    /// Base URL for the provider's oi-compatible API.
    pub base_url: Option<String>,
    /// Environment variable that stores the user's API key for this provider.
    pub env_key: Option<String>,

    /// Optional instructions to help the user get a valid value for the
    /// variable and set it.
    pub env_key_instructions: Option<String>,
    /// Value to use with `Authorization: Bearer <token>` header. Use of this
    /// config is discouraged in favor of `env_key` for security reasons, but
    /// this may be necessary when using this programmatically.
    pub experimental_bearer_token: Option<String>,
    /// Command-backed bearer-token configuration for this provider.
    pub auth: Option<ModelProviderAuthInfo>,
    /// AWS SigV4 auth configuration for this provider.
    pub aws: Option<ModelProviderAwsAuthInfo>,
    /// Which wire protocol this provider expects.
    #[serde(default)]
    pub wire_api: WireApi,
    /// Optional query parameters to append to the base URL.
    pub query_params: Option<HashMap<String, String>>,
    /// Additional HTTP headers to include in requests to this provider where
    /// the (key, value) pairs are the header name and value.
    pub http_headers: Option<HashMap<String, String>>,
    /// Optional HTTP headers to include in requests to this provider where the
    /// (key, value) pairs are the header name and _environment variable_ whose
    /// value should be used. If the environment variable is not set, or the
    /// value is empty, the header will not be included in the request.
    pub env_http_headers: Option<HashMap<String, String>>,
    /// Maximum number of times to retry a failed HTTP request to this provider.
    pub request_max_retries: Option<u64>,
    /// Number of times to retry reconnecting a dropped streaming response before failing.
    pub stream_max_retries: Option<u64>,
    /// Idle timeout (in milliseconds) to wait for activity on a streaming response before treating
    /// the connection as lost.
    pub stream_idle_timeout_ms: Option<u64>,
    /// Maximum time (in milliseconds) to wait for a websocket connection attempt before treating
    /// it as failed.
    pub websocket_connect_timeout_ms: Option<u64>,
    /// Does this provider require an oi API Key or gt login token? If true,
    /// user is presented with login screen on first run, and login preference and token/key
    /// are stored in auth.json. If false (which is the default), login screen is skipped,
    /// and API key (if needed) comes from the "env_key" environment variable.
    #[serde(default)]
    pub requires_openai_auth: bool,
    /// Whether this provider supports the Responses API WebSocket transport.
    #[serde(default)]
    pub supports_websockets: bool,
    /// Whether this provider supports the standalone web-search endpoint.
    #[serde(default)]
    pub supports_standalone_web_search: bool,
}

/// AWS SigV4 auth configuration for a model provider.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelProviderAwsAuthInfo {
    /// AWS profile name to use. When unset, the AWS SDK default chain decides.
    pub profile: Option<String>,
    /// AWS region to use for provider-specific endpoints.
    pub region: Option<String>,
    /// Optional command used to reauthenticate after a refreshable AWS auth failure.
    pub auth_refresh: Option<AwsAuthRefreshConfig>,
}

/// Command used to refresh AWS credentials for a model provider.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct AwsAuthRefreshConfig {
    /// Executable to invoke directly, without a shell.
    pub command: String,
    /// Arguments passed to the refresh command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Maximum time to wait for the refresh command to complete.
    #[serde(default = "default_aws_auth_refresh_timeout_ms")]
    pub timeout_ms: NonZeroU64,
}

impl AwsAuthRefreshConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms.get())
    }
}

fn default_aws_auth_refresh_timeout_ms() -> NonZeroU64 {
    match NonZeroU64::new(DEFAULT_AWS_AUTH_REFRESH_TIMEOUT_MS) {
        Some(timeout_ms) => timeout_ms,
        None => panic!("AWS auth refresh timeout must be non-zero"),
    }
}

impl ModelProviderInfo {
    pub fn validate(&self) -> std::result::Result<(), String> {
        if self.aws.is_some() {
            if self.supports_websockets {
                // TODO(celia-oai): Support AWS SigV4 signing for WebSocket
                // upgrade requests before allowing AWS-authenticated providers
                // to enable Responses-over-WebSocket.
                return Err("provider aws cannot be combined with supports_websockets".to_string());
            }

            let mut conflicts = Vec::new();
            if self.env_key.is_some() {
                conflicts.push("env_key");
            }
            if self.experimental_bearer_token.is_some() {
                conflicts.push("experimental_bearer_token");
            }
            if self.auth.is_some() {
                conflicts.push("auth");
            }
            if self.requires_openai_auth {
                conflicts.push("requires_openai_auth");
            }

            if !conflicts.is_empty() {
                return Err(format!(
                    "provider aws cannot be combined with {}",
                    conflicts.join(", ")
                ));
            }

            if let Some(auth_refresh) = self.aws.as_ref().and_then(|aws| aws.auth_refresh.as_ref())
            {
                if auth_refresh.command.trim().is_empty() {
                    return Err("provider aws.auth_refresh.command must not be empty".to_string());
                }
                if auth_refresh.command != "aws" {
                    return Err("provider aws.auth_refresh.command must be `aws`".to_string());
                }
            }
        }

        let Some(auth) = self.auth.as_ref() else {
            return Ok(());
        };

        if auth.command.trim().is_empty() {
            return Err("provider auth.command must not be empty".to_string());
        }

        let mut conflicts = Vec::new();
        if self.env_key.is_some() {
            conflicts.push("env_key");
        }
        if self.experimental_bearer_token.is_some() {
            conflicts.push("experimental_bearer_token");
        }
        if self.requires_openai_auth {
            conflicts.push("requires_openai_auth");
        }

        if conflicts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "provider auth cannot be combined with {}",
                conflicts.join(", ")
            ))
        }
    }

    fn build_header_map(&self) -> CodexResult<HeaderMap> {
        let capacity = self.http_headers.as_ref().map_or(0, HashMap::len)
            + self.env_http_headers.as_ref().map_or(0, HashMap::len);
        let mut headers = HeaderMap::with_capacity(capacity);
        if let Some(extra) = &self.http_headers {
            for (k, v) in extra {
                if let (Ok(name), Ok(value)) = (HeaderName::try_from(k), HeaderValue::try_from(v)) {
                    headers.insert(name, value);
                }
            }
        }

        if let Some(env_headers) = &self.env_http_headers {
            for (header, env_var) in env_headers {
                if let Ok(val) = std::env::var(env_var)
                    && !val.trim().is_empty()
                    && let (Ok(name), Ok(value)) =
                        (HeaderName::try_from(header), HeaderValue::try_from(val))
                {
                    headers.insert(name, value);
                }
            }
        }

        Ok(headers)
    }

    pub fn to_api_provider(&self, auth_mode: Option<AuthMode>) -> CodexResult<ApiProvider> {
        let default_base_url = if matches!(
            auth_mode,
            Some(
                AuthMode::Chatgpt
                    | AuthMode::ChatgptAuthTokens
                    | AuthMode::Headers
                    | AuthMode::AgentIdentity
                    | AuthMode::PersonalAccessToken
            )
        ) {
            CHATGPT_CODEX_BASE_URL
        } else {
            "https://api.cy.symbiotyc.workers.dev/v1"
        };
        let base_url = self
            .base_url
            .clone()
            .unwrap_or_else(|| default_base_url.to_string());

        let headers = self.build_header_map()?;
        let retry = ApiRetryConfig {
            max_attempts: self.request_max_retries(),
            base_delay: Duration::from_millis(200),
            retry_429: false,
            retry_5xx: true,
            retry_transport: true,
        };

        Ok(ApiProvider {
            name: self.name.clone(),
            base_url,
            query_params: self.query_params.clone(),
            headers,
            retry,
            stream_idle_timeout: self.stream_idle_timeout(),
        })
    }

    /// If `env_key` is Some, returns the API key for this provider if present
    /// (and non-empty) in the environment. If `env_key` is required but
    /// cannot be found, returns an error.
    ///
    /// SYMBIOTYC: for the `cy` provider, fall back to reading
    /// `~/.cy/auth.json` (or `$CY_HOME/auth.json`) when the env var is not
    /// set, so the macOS launcher-spawned shell and direct `cy exec` from a
    /// plain terminal both work without an extra `export CY_API_KEY=...`.
    pub fn api_key(&self) -> CodexResult<Option<String>> {
        match &self.env_key {
            Some(env_key) => {
                let api_key = std::env::var(env_key)
                    .ok()
                    .filter(|v| !v.trim().is_empty())
                    .or_else(read_cy_auth_json_key)
                    .ok_or_else(|| {
                        CxErr::EnvVar(EnvVarError {
                            var: env_key.clone(),
                            instructions: self.env_key_instructions.clone(),
                        })
                    })?;
                Ok(Some(api_key))
            }
            None => Ok(None),
        }
    }

    /// Effective maximum number of request retries for this provider.
    pub fn request_max_retries(&self) -> u64 {
        self.request_max_retries
            .unwrap_or(DEFAULT_REQUEST_MAX_RETRIES)
            .min(MAX_REQUEST_MAX_RETRIES)
    }

    /// Effective maximum number of stream reconnection attempts for this provider.
    pub fn stream_max_retries(&self) -> u64 {
        self.stream_max_retries
            .unwrap_or(DEFAULT_STREAM_MAX_RETRIES)
            .min(MAX_STREAM_MAX_RETRIES)
    }

    /// Effective idle timeout for streaming responses.
    pub fn stream_idle_timeout(&self) -> Duration {
        self.stream_idle_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_STREAM_IDLE_TIMEOUT_MS))
    }

    /// Effective timeout for websocket connect attempts.
    pub fn websocket_connect_timeout(&self) -> Duration {
        self.websocket_connect_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS))
    }

    pub fn create_cy_provider() -> ModelProviderInfo {
        // Prefer an explicit `CY_BASE_URL`, then any value the user wrote
        // into `~/.cy/config.toml` under `[model_providers.cy] base_url`,
        // and only fall back to the built-in default (the local bridge) if
        // neither is set. This keeps `cy exec` pointed at the bridge that
        // the .app launcher starts, but lets power users point at a remote
        // bridge or straight at the SYMBIOTYC Cloud endpoint.
        let base_url = std::env::var(CY_PROVIDER_BASE_URL_ENV_VAR)
            .ok()
            .unwrap_or_else(|| CY_PROVIDER_DEFAULT_BASE_URL.to_string());
        ModelProviderInfo {
            name: CY_PROVIDER_NAME.into(),
            base_url: Some(base_url),
            env_key: Some(CY_PROVIDER_ENV_KEY.into()),
            env_key_instructions: Some(CY_PROVIDER_ENV_KEY_INSTRUCTIONS.into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
            supports_standalone_web_search: false,
        }
    }

    pub fn is_openai(&self) -> bool {
        self.name == CY_PROVIDER_NAME
    }

    pub fn has_command_auth(&self) -> bool {
        self.auth.is_some()
    }
}

/// Built-in default provider list: SYMBIOTYC only.
pub fn built_in_model_providers() -> HashMap<String, ModelProviderInfo> {
    use ModelProviderInfo as P;

    // SYMBIOTYC ships a single built-in provider. Users can add their own
    // providers via `model_providers` in config.toml.
    [(CY_PROVIDER_ID, P::create_cy_provider())]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

/// Merge configured providers into the built-in provider catalog.
///
/// Configured providers extend the built-in set.
pub fn merge_configured_model_providers(
    mut model_providers: HashMap<String, ModelProviderInfo>,
    configured_model_providers: HashMap<String, ModelProviderInfo>,
) -> Result<HashMap<String, ModelProviderInfo>, String> {
    for (key, provider) in configured_model_providers {
        model_providers.entry(key).or_insert(provider);
    }

    Ok(model_providers)
}

#[cfg(test)]
#[path = "model_provider_info_tests.rs"]
mod tests;

/// SYMBIOTYC: read the API key from `~/.cy/auth.json` so direct shell
/// invocations of `cy exec` work without `CY_API_KEY` being exported. The
/// file shape is the standard one written by the CY-CLI macOS launcher
/// (`{"auth_mode": "apiKey", "cy_api_key": "..."}`). Respects
/// `$CY_HOME` so tests / sandboxed runs can point at a fixture.
fn read_cy_auth_json_key() -> Option<String> {
    use std::path::PathBuf;

    let home = std::env::var_os("CY_HOME").map(PathBuf::from).or_else(|| {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
    })?;
    let path = home.join(".cy").join("auth.json");
    let bytes = std::fs::read(&path).ok()?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    let key = value.get("cy_api_key")?.as_str()?.trim();
    if key.is_empty() {
        None
    } else {
        Some(key.to_string())
    }
}
