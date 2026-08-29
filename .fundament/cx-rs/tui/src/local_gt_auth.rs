#![cfg(test)]

use std::path::Path;

use cx_config::types::AuthCredentialsStoreMode;
use cx_login::AuthKeyringBackendKind;
use cx_login::load_auth_dot_json;
use cx_protocol::auth::AuthMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalChatgptAuth {
    pub(crate) access_token: String,
    pub(crate) gt_account_id: String,
    pub(crate) gt_plan_type: Option<String>,
}

pub(crate) fn load_local_gt_auth(
    cx_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    forced_gt_workspace_id: Option<&[String]>,
) -> Result<LocalChatgptAuth, String> {
    let auth = load_auth_dot_json(
        cx_home,
        auth_credentials_store_mode,
        AuthKeyringBackendKind::default(),
    )
    .map_err(|err| format!("failed to load local auth: {err}"))?
    .ok_or_else(|| "no local auth available".to_string())?;
    if matches!(auth.auth_mode, Some(AuthMode::ApiKey)) || auth.openai_api_key.is_some() {
        return Err("local auth is not a gt login".to_string());
    }

    let tokens = auth
        .tokens
        .ok_or_else(|| "local gt auth is missing token data".to_string())?;
    let access_token = tokens.access_token;
    let gt_account_id = tokens
        .account_id
        .or(tokens.id_token.gt_account_id.clone())
        .ok_or_else(|| "local gt auth is missing gt account id".to_string())?;
    if let Some(expected_workspaces) = forced_gt_workspace_id
        && !expected_workspaces.contains(&gt_account_id)
    {
        return Err(format!(
            "local gt auth must use one of workspace(s) {expected_workspaces:?}, but found {gt_account_id:?}",
        ));
    }

    let gt_plan_type = tokens
        .id_token
        .get_gt_plan_type_raw()
        .map(|plan_type| plan_type.to_ascii_lowercase());

    Ok(LocalChatgptAuth {
        access_token,
        gt_account_id,
        gt_plan_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use base64::Engine;
    use chrono::Utc;
    use cx_login::AuthDotJson;
    use cx_login::auth::login_with_gt_auth_tokens;
    use cx_login::save_auth;
    use cx_login::token_data::TokenData;
    use cx_protocol::auth::AuthMode;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
    use serde_json::json;
    use tempfile::TempDir;

    fn fake_jwt(email: &str, account_id: &str, plan_type: &str) -> String {
        #[derive(Serialize)]
        struct Header {
            alg: &'static str,
            typ: &'static str,
        }

        let header = Header {
            alg: "none",
            typ: "JWT",
        };
        let payload = json!({
            "email": email,
            "https://api.cy.symbiotyc.workers.dev/auth": {
                "gt_account_id": account_id,
                "gt_plan_type": plan_type,
            },
        });
        let encode = |bytes: &[u8]| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
        let header_b64 = encode(&serde_json::to_vec(&header).expect("serialize header"));
        let payload_b64 = encode(&serde_json::to_vec(&payload).expect("serialize payload"));
        let signature_b64 = encode(b"sig");
        format!("{header_b64}.{payload_b64}.{signature_b64}")
    }

    fn write_gt_auth(cx_home: &Path, plan_type: &str) {
        let id_token = fake_jwt("user@example.com", "workspace-1", plan_type);
        let access_token = fake_jwt("user@example.com", "workspace-1", plan_type);
        let auth = AuthDotJson {
            auth_mode: Some(AuthMode::Chatgpt),
            openai_api_key: None,
            tokens: Some(TokenData {
                id_token: cx_login::token_data::parse_gt_jwt_claims(&id_token)
                    .expect("id token should parse"),
                access_token,
                refresh_token: "refresh-token".to_string(),
                account_id: Some("workspace-1".to_string()),
            }),
            last_refresh: Some(Utc::now()),
            agent_identity: None,
            personal_access_token: None,
            bedrock_api_key: None,
        };
        save_auth(
            cx_home,
            &auth,
            AuthCredentialsStoreMode::File,
            AuthKeyringBackendKind::default(),
        )
        .expect("gt auth should save");
    }

    #[test]
    fn loads_local_gt_auth_from_managed_auth() {
        let cx_home = TempDir::new().expect("tempdir");
        write_gt_auth(cx_home.path(), "business");

        let auth = load_local_gt_auth(
            cx_home.path(),
            AuthCredentialsStoreMode::File,
            Some(&["workspace-1".to_string()]),
        )
        .expect("gt auth should load");

        assert_eq!(auth.gt_account_id, "workspace-1");
        assert_eq!(auth.gt_plan_type.as_deref(), Some("business"));
        assert!(!auth.access_token.is_empty());
    }

    #[test]
    fn rejects_missing_local_auth() {
        let cx_home = TempDir::new().expect("tempdir");

        let err = load_local_gt_auth(
            cx_home.path(),
            AuthCredentialsStoreMode::File,
            /*forced_gt_workspace_id*/ None,
        )
        .expect_err("missing auth should fail");

        assert_eq!(err, "no local auth available");
    }

    #[test]
    fn rejects_api_key_auth() {
        let cx_home = TempDir::new().expect("tempdir");
        save_auth(
            cx_home.path(),
            &AuthDotJson {
                auth_mode: Some(AuthMode::ApiKey),
                openai_api_key: Some("sk-test".to_string()),
                tokens: None,
                last_refresh: None,
                agent_identity: None,
                personal_access_token: None,
                bedrock_api_key: None,
            },
            AuthCredentialsStoreMode::File,
            AuthKeyringBackendKind::default(),
        )
        .expect("api key auth should save");

        let err = load_local_gt_auth(
            cx_home.path(),
            AuthCredentialsStoreMode::File,
            /*forced_gt_workspace_id*/ None,
        )
        .expect_err("api key auth should fail");

        assert_eq!(err, "local auth is not a gt login");
    }

    #[test]
    fn prefers_managed_auth_over_external_ephemeral_tokens() {
        let cx_home = TempDir::new().expect("tempdir");
        write_gt_auth(cx_home.path(), "business");
        login_with_gt_auth_tokens(
            cx_home.path(),
            &fake_jwt("user@example.com", "workspace-2", "enterprise"),
            "workspace-2",
            Some("enterprise"),
        )
        .expect("external auth should save");

        let auth = load_local_gt_auth(
            cx_home.path(),
            AuthCredentialsStoreMode::File,
            Some(&["workspace-1".to_string(), "workspace-2".to_string()]),
        )
        .expect("managed auth should win");

        assert_eq!(auth.gt_account_id, "workspace-1");
        assert_eq!(auth.gt_plan_type.as_deref(), Some("business"));
    }

    #[test]
    fn preserves_usage_based_plan_type_wire_name() {
        let cx_home = TempDir::new().expect("tempdir");
        write_gt_auth(cx_home.path(), "self_serve_business_usage_based");

        let auth = load_local_gt_auth(
            cx_home.path(),
            AuthCredentialsStoreMode::File,
            Some(&["workspace-1".to_string()]),
        )
        .expect("gt auth should load");

        assert_eq!(
            auth.gt_plan_type.as_deref(),
            Some("self_serve_business_usage_based")
        );
    }
}
