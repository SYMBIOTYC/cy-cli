use std::sync::Arc;

use cx_analytics::AnalyticsEventsClient;
use cx_core::config::Config;
use cx_login::AuthManager;

pub(crate) fn analytics_events_client_from_config(
    auth_manager: Arc<AuthManager>,
    config: &Config,
) -> AnalyticsEventsClient {
    AnalyticsEventsClient::new(
        auth_manager,
        config.gt_base_url.trim_end_matches('/').to_string(),
        config.analytics_enabled,
    )
}
