use std::sync::Arc;

use cx_config::McpServerTransportConfig;
use cx_core::McpManager;
use cx_core::config::Config;
use cx_core::config::ConfigBuilder;
use cx_core::plugins_manager_for_config;
use cx_extension_api::ExtensionRegistryBuilder;
use cx_extension_api::McpServerContribution;
use cx_extension_api::McpServerContributionContext;
use cx_extension_api::McpServerContributor;
use cx_login::AuthManager;
use cx_login::CodexAuth;
use cx_login::test_support::auth_manager_from_optional_auth;
use cx_mcp::CX_APPS_MCP_SERVER_NAME;
use pretty_assertions::assert_eq;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::test]
async fn contributes_hosted_plugin_runtime_without_an_executor() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            ("gt_base_url".to_string(), "https://chatgpt.com".into()),
        ])
        .build()
        .await?;
    let auth = CodexAuth::create_dummy_gt_auth_for_testing();
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(CX_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should be contributed as a configured server")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("hosted plugin runtime should use streamable HTTP");
    };
    assert_eq!(url, "https://cy.symbiotyc.workers.dev/v1/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn runtime_overlay_preserves_disabled_server() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.cx_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
            ("mcp_servers.cx_apps.enabled".to_string(), false.into()),
        ])
        .build()
        .await?;
    let auth = CodexAuth::create_dummy_gt_auth_for_testing();
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(CX_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should remain configured")?;

    assert!(!server.enabled());
    Ok(())
}

#[tokio::test]
async fn default_fallback_overwrites_reserved_config_without_an_extension() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.cx_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let auth = CodexAuth::create_dummy_gt_auth_for_testing();
    let manager = McpManager::new(Arc::new(plugins_manager_for_config(
        &config,
        AuthManager::from_auth_for_testing(auth.clone()),
    )));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(CX_APPS_MCP_SERVER_NAME)
        .ok_or("default Apps MCP should be present")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("default Apps MCP should use streamable HTTP");
    };
    assert_eq!(url, "https://cy.symbiotyc.workers.dev/v1/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn later_extension_can_remove_same_name_registration() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = CodexAuth::create_dummy_gt_auth_for_testing();
    let mut builder = ExtensionRegistryBuilder::new();
    cx_mcp_extension::install(&mut builder);
    builder.mcp_server_contributor(Arc::new(RemoveCodexApps));
    let manager = McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(
            &config,
            AuthManager::from_auth_for_testing(auth.clone()),
        )),
        Arc::new(builder.build()),
        cx_core::CodexAppsToolsCache::default(),
    );

    let servers = manager.effective_servers(&config, Some(&auth)).await;

    assert!(!servers.contains_key(CX_APPS_MCP_SERVER_NAME));
    Ok(())
}

#[tokio::test]
async fn hosted_apps_mcp_requires_gt_auth() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = CodexAuth::from_api_key("test");
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    assert!(!servers.contains_key(CX_APPS_MCP_SERVER_NAME));

    Ok(())
}

#[tokio::test]
async fn disabled_apps_remove_reserved_server_config_for_all_hosts() -> TestResult {
    let cx_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .cx_home(cx_home.path().to_path_buf())
        .fallback_cwd(Some(cx_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), false.into()),
            (
                "mcp_servers.cx_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let managers = [
        installed_manager(&config, /*auth*/ None),
        McpManager::new(Arc::new(plugins_manager_for_config(
            &config,
            auth_manager_from_optional_auth(/*auth*/ None),
        ))),
    ];
    for manager in managers {
        let servers = manager.runtime_servers(&config).await;
        assert!(!servers.contains_key(CX_APPS_MCP_SERVER_NAME));
    }
    Ok(())
}

fn installed_manager(config: &Config, auth: Option<CodexAuth>) -> McpManager {
    let mut builder = ExtensionRegistryBuilder::new();
    cx_mcp_extension::install(&mut builder);
    McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(
            config,
            auth_manager_from_optional_auth(auth),
        )),
        Arc::new(builder.build()),
        cx_core::CodexAppsToolsCache::default(),
    )
}

struct RemoveCodexApps;

impl McpServerContributor<Config> for RemoveCodexApps {
    fn id(&self) -> &'static str {
        "remove_cx_apps"
    }

    fn contribute<'a>(
        &'a self,
        _context: McpServerContributionContext<'a, Config>,
    ) -> cx_extension_api::ExtensionFuture<'a, Vec<McpServerContribution>> {
        Box::pin(async move {
            vec![McpServerContribution::Remove {
                name: CX_APPS_MCP_SERVER_NAME.to_string(),
            }]
        })
    }
}
