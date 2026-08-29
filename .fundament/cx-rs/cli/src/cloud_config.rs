use anyhow::Context;
use anyhow::Result;
use cx_cloud_config::cloud_config_bundle_loader_for_storage;
use cx_config::CloudConfigBundleLoader;
use cx_config::ConfigLoadOptions;
use cx_core::config::Config;
use cx_core::config::ConfigBuilder;
use cx_core::config::LoaderOverrides;
use cx_core::config::bootstrap_auth_config;
use cx_core::config::find_cx_home;
use cx_core::config::load_config_toml_with_layer_stack;
use cx_utils_absolute_path::AbsolutePathBuf;
use cx_utils_cli::CliConfigOverrides;

pub(crate) async fn load_config(
    config_overrides: &CliConfigOverrides,
    loader_overrides: LoaderOverrides,
) -> Result<Config> {
    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(anyhow::Error::msg)?;
    let cx_home = find_cx_home().context("failed to resolve CX_HOME")?;
    let cwd = AbsolutePathBuf::current_dir().context("failed to resolve current directory")?;
    let bootstrap_config = load_config_toml_with_layer_stack(
        cx_home.as_path(),
        Some(&cwd),
        cli_overrides.clone(),
        ConfigLoadOptions {
            loader_overrides: loader_overrides.clone(),
            strict_config: false,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await
    .context("failed to load bootstrap configuration")?;
    let cloud_config_bundle = cloud_config_bundle_loader_for_storage(
        bootstrap_auth_config(cx_home.as_path(), &bootstrap_config)
            .context("failed to resolve cloud configuration authentication")?,
        /*enable_cx_api_key_env*/ false,
    )
    .await
    .context("failed to initialize cloud configuration authentication")?;

    ConfigBuilder::default()
        .cx_home(cx_home.to_path_buf())
        .cli_overrides(cli_overrides)
        .loader_overrides(loader_overrides)
        .cloud_config_bundle(cloud_config_bundle)
        .build()
        .await
        .context("failed to load configuration")
}
