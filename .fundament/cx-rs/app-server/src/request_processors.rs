use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use cx_analytics::AnalyticsEventsClient;
use cx_analytics::AnalyticsJsonRpcError;
use cx_analytics::InputError;
use cx_analytics::TurnSteerRequestError;
use cx_app_server_protocol::Account;
use cx_app_server_protocol::AccountLoginCompletedNotification;
use cx_app_server_protocol::AccountTokenUsageDailyBucket;
use cx_app_server_protocol::AccountTokenUsageSummary;
use cx_app_server_protocol::AccountUpdatedNotification;
use cx_app_server_protocol::AddCreditsNudgeCreditType;
use cx_app_server_protocol::AddCreditsNudgeEmailStatus;
use cx_app_server_protocol::AdditionalContextEntry;
use cx_app_server_protocol::AdditionalContextKind;
use cx_app_server_protocol::AppListUpdatedNotification;
use cx_app_server_protocol::AppSummary;
use cx_app_server_protocol::AppTemplateSummary;
use cx_app_server_protocol::AppTemplateUnavailableReason;
use cx_app_server_protocol::AppsInstalledParams;
use cx_app_server_protocol::AppsInstalledResponse;
use cx_app_server_protocol::AppsListParams;
use cx_app_server_protocol::AppsListResponse;
use cx_app_server_protocol::AppsReadParams;
use cx_app_server_protocol::AppsReadResponse;
use cx_app_server_protocol::AskForApproval;
use cx_app_server_protocol::AuthMode;
use cx_app_server_protocol::CancelLoginAccountParams;
use cx_app_server_protocol::CancelLoginAccountResponse;
use cx_app_server_protocol::CancelLoginAccountStatus;
use cx_app_server_protocol::ClientInfo;
use cx_app_server_protocol::ClientRequest;
use cx_app_server_protocol::ClientResponsePayload;
use cx_app_server_protocol::CodexErrorInfo;
use cx_app_server_protocol::CollaborationModeListParams;
use cx_app_server_protocol::CollaborationModeListResponse;
use cx_app_server_protocol::CommandExecParams;
use cx_app_server_protocol::CommandExecResizeParams;
use cx_app_server_protocol::CommandExecTerminateParams;
use cx_app_server_protocol::CommandExecWriteParams;
use cx_app_server_protocol::ConfigWarningNotification;
use cx_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use cx_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use cx_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use cx_app_server_protocol::ConversationGitInfo;
use cx_app_server_protocol::ConversationSummary;
use cx_app_server_protocol::DeprecationNoticeNotification;
use cx_app_server_protocol::DynamicToolFunctionSpec;
use cx_app_server_protocol::DynamicToolNamespaceTool;
use cx_app_server_protocol::DynamicToolSpec;
use cx_app_server_protocol::EnvironmentAddParams;
use cx_app_server_protocol::EnvironmentAddResponse;
use cx_app_server_protocol::EnvironmentInfoParams;
use cx_app_server_protocol::EnvironmentInfoResponse;
use cx_app_server_protocol::EnvironmentShellInfo;
use cx_app_server_protocol::EnvironmentStatusKind;
use cx_app_server_protocol::EnvironmentStatusParams;
use cx_app_server_protocol::EnvironmentStatusResponse;
use cx_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use cx_app_server_protocol::ExperimentalFeatureListParams;
use cx_app_server_protocol::ExperimentalFeatureListResponse;
use cx_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use cx_app_server_protocol::FeedbackUploadParams;
use cx_app_server_protocol::FeedbackUploadResponse;
use cx_app_server_protocol::GetAccountParams;
use cx_app_server_protocol::GetAccountRateLimitsResponse;
use cx_app_server_protocol::GetAccountResponse;
use cx_app_server_protocol::GetAccountTokenUsageParams;
use cx_app_server_protocol::GetAccountTokenUsageResponse;
use cx_app_server_protocol::GetAuthStatusParams;
use cx_app_server_protocol::GetAuthStatusResponse;
use cx_app_server_protocol::GetConversationSummaryParams;
use cx_app_server_protocol::GetConversationSummaryResponse;
use cx_app_server_protocol::GetWorkspaceMessagesResponse;
use cx_app_server_protocol::GitDiffToRemoteParams;
use cx_app_server_protocol::GitDiffToRemoteResponse;
use cx_app_server_protocol::GitInfo as ApiGitInfo;
use cx_app_server_protocol::HookHandlerMetadata;
use cx_app_server_protocol::HookMetadata;
use cx_app_server_protocol::HooksListParams;
use cx_app_server_protocol::HooksListResponse;
use cx_app_server_protocol::InitializeParams;
use cx_app_server_protocol::InitializeResponse;
use cx_app_server_protocol::InstalledApp;
use cx_app_server_protocol::JSONRPCErrorError;
use cx_app_server_protocol::ListMcpServerStatusParams;
use cx_app_server_protocol::ListMcpServerStatusResponse;
use cx_app_server_protocol::LoginAccountParams;
use cx_app_server_protocol::LoginAccountResponse;
use cx_app_server_protocol::LoginApiKeyParams;
use cx_app_server_protocol::LoginAppBrand;
use cx_app_server_protocol::LogoutAccountResponse;
use cx_app_server_protocol::MarketplaceAddParams;
use cx_app_server_protocol::MarketplaceAddResponse;
use cx_app_server_protocol::MarketplaceInterface;
use cx_app_server_protocol::MarketplaceRemoveParams;
use cx_app_server_protocol::MarketplaceRemoveResponse;
use cx_app_server_protocol::MarketplaceUpgradeErrorInfo;
use cx_app_server_protocol::MarketplaceUpgradeParams;
use cx_app_server_protocol::MarketplaceUpgradeResponse;
use cx_app_server_protocol::McpResourceReadParams;
use cx_app_server_protocol::McpResourceReadResponse;
use cx_app_server_protocol::McpServerOauthClientRegistration;
use cx_app_server_protocol::McpServerOauthLoginCompletedNotification;
use cx_app_server_protocol::McpServerOauthLoginParams;
use cx_app_server_protocol::McpServerOauthLoginResponse;
use cx_app_server_protocol::McpServerRefreshResponse;
use cx_app_server_protocol::McpServerStatus;
use cx_app_server_protocol::McpServerStatusDetail;
use cx_app_server_protocol::McpServerToolCallParams;
use cx_app_server_protocol::McpServerToolCallResponse;
use cx_app_server_protocol::MemoryResetResponse;
use cx_app_server_protocol::MockExperimentalMethodParams;
use cx_app_server_protocol::MockExperimentalMethodResponse;
use cx_app_server_protocol::ModelListParams;
use cx_app_server_protocol::ModelListResponse;
use cx_app_server_protocol::PermissionProfileListParams;
use cx_app_server_protocol::PermissionProfileListResponse;
use cx_app_server_protocol::PermissionProfileSummary;
use cx_app_server_protocol::PluginDetail;
use cx_app_server_protocol::PluginInstallParams;
use cx_app_server_protocol::PluginInstallResponse;
use cx_app_server_protocol::PluginInstalledParams;
use cx_app_server_protocol::PluginInstalledResponse;
use cx_app_server_protocol::PluginInterface;
use cx_app_server_protocol::PluginListMarketplaceKind;
use cx_app_server_protocol::PluginListParams;
use cx_app_server_protocol::PluginListResponse;
use cx_app_server_protocol::PluginMarketplaceEntry;
use cx_app_server_protocol::PluginReadParams;
use cx_app_server_protocol::PluginReadResponse;
use cx_app_server_protocol::PluginShareCheckoutParams;
use cx_app_server_protocol::PluginShareCheckoutResponse;
use cx_app_server_protocol::PluginShareContext;
use cx_app_server_protocol::PluginShareDeleteParams;
use cx_app_server_protocol::PluginShareDeleteResponse;
use cx_app_server_protocol::PluginShareDiscoverability;
use cx_app_server_protocol::PluginShareListItem;
use cx_app_server_protocol::PluginShareListParams;
use cx_app_server_protocol::PluginShareListResponse;
use cx_app_server_protocol::PluginSharePrincipal;
use cx_app_server_protocol::PluginSharePrincipalType;
use cx_app_server_protocol::PluginShareSaveParams;
use cx_app_server_protocol::PluginShareSaveResponse;
use cx_app_server_protocol::PluginShareTarget;
use cx_app_server_protocol::PluginShareUpdateDiscoverability;
use cx_app_server_protocol::PluginShareUpdateTargetsParams;
use cx_app_server_protocol::PluginShareUpdateTargetsResponse;
use cx_app_server_protocol::PluginSkillReadParams;
use cx_app_server_protocol::PluginSkillReadResponse;
use cx_app_server_protocol::PluginSource;
use cx_app_server_protocol::PluginSummary;
use cx_app_server_protocol::PluginUninstallParams;
use cx_app_server_protocol::PluginUninstallResponse;
use cx_app_server_protocol::RateLimitResetCredit;
use cx_app_server_protocol::RateLimitResetCreditStatus;
use cx_app_server_protocol::RateLimitResetCreditsSummary;
use cx_app_server_protocol::RateLimitResetType;
use cx_app_server_protocol::RequestId;
use cx_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use cx_app_server_protocol::ReviewStartParams;
use cx_app_server_protocol::ReviewStartResponse;
use cx_app_server_protocol::ReviewTarget as ApiReviewTarget;
use cx_app_server_protocol::SandboxMode;
use cx_app_server_protocol::SendAddCreditsNudgeEmailParams;
use cx_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use cx_app_server_protocol::ServerNotification;
use cx_app_server_protocol::ServerRequestResolvedNotification;
use cx_app_server_protocol::SkillSummary;
use cx_app_server_protocol::SkillsConfigWriteParams;
use cx_app_server_protocol::SkillsConfigWriteResponse;
use cx_app_server_protocol::SkillsExtraRootsSetParams;
use cx_app_server_protocol::SkillsExtraRootsSetResponse;
use cx_app_server_protocol::SkillsListParams;
use cx_app_server_protocol::SkillsListResponse;
use cx_app_server_protocol::SortDirection;
use cx_app_server_protocol::Thread;
use cx_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use cx_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use cx_app_server_protocol::ThreadArchiveParams;
use cx_app_server_protocol::ThreadArchiveResponse;
use cx_app_server_protocol::ThreadArchivedNotification;
use cx_app_server_protocol::ThreadBackgroundTerminal;
use cx_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use cx_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use cx_app_server_protocol::ThreadBackgroundTerminalsListParams;
use cx_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use cx_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use cx_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use cx_app_server_protocol::ThreadClosedNotification;
use cx_app_server_protocol::ThreadCompactStartParams;
use cx_app_server_protocol::ThreadCompactStartResponse;
use cx_app_server_protocol::ThreadDecrementElicitationParams;
use cx_app_server_protocol::ThreadDecrementElicitationResponse;
use cx_app_server_protocol::ThreadDeleteParams;
use cx_app_server_protocol::ThreadDeleteResponse;
use cx_app_server_protocol::ThreadDeletedNotification;
use cx_app_server_protocol::ThreadForkParams;
use cx_app_server_protocol::ThreadForkResponse;
use cx_app_server_protocol::ThreadGoal;
use cx_app_server_protocol::ThreadGoalClearParams;
use cx_app_server_protocol::ThreadGoalClearResponse;
use cx_app_server_protocol::ThreadGoalClearedNotification;
use cx_app_server_protocol::ThreadGoalGetParams;
use cx_app_server_protocol::ThreadGoalGetResponse;
use cx_app_server_protocol::ThreadGoalSetParams;
use cx_app_server_protocol::ThreadGoalSetResponse;
use cx_app_server_protocol::ThreadGoalStatus;
use cx_app_server_protocol::ThreadGoalUpdatedNotification;
use cx_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use cx_app_server_protocol::ThreadHistoryMode;
use cx_app_server_protocol::ThreadIncrementElicitationParams;
use cx_app_server_protocol::ThreadIncrementElicitationResponse;
use cx_app_server_protocol::ThreadInjectItemsParams;
use cx_app_server_protocol::ThreadInjectItemsResponse;
use cx_app_server_protocol::ThreadItem;
use cx_app_server_protocol::ThreadItemEntry;
use cx_app_server_protocol::ThreadItemsListParams;
use cx_app_server_protocol::ThreadItemsListResponse;
use cx_app_server_protocol::ThreadListCwdFilter;
use cx_app_server_protocol::ThreadListParams;
use cx_app_server_protocol::ThreadListResponse;
use cx_app_server_protocol::ThreadLoadedListParams;
use cx_app_server_protocol::ThreadLoadedListResponse;
use cx_app_server_protocol::ThreadMemoryModeSetParams;
use cx_app_server_protocol::ThreadMemoryModeSetResponse;
use cx_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use cx_app_server_protocol::ThreadMetadataUpdateParams;
use cx_app_server_protocol::ThreadMetadataUpdateResponse;
use cx_app_server_protocol::ThreadNameUpdatedNotification;
use cx_app_server_protocol::ThreadProjectUpdatedNotification;
use cx_app_server_protocol::ThreadReadParams;
use cx_app_server_protocol::ThreadReadResponse;
use cx_app_server_protocol::ThreadRealtimeAppendAudioParams;
use cx_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use cx_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use cx_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use cx_app_server_protocol::ThreadRealtimeAppendTextParams;
use cx_app_server_protocol::ThreadRealtimeAppendTextResponse;
use cx_app_server_protocol::ThreadRealtimeListVoicesResponse;
use cx_app_server_protocol::ThreadRealtimeStartParams;
use cx_app_server_protocol::ThreadRealtimeStartResponse;
use cx_app_server_protocol::ThreadRealtimeStartTransport;
use cx_app_server_protocol::ThreadRealtimeStopParams;
use cx_app_server_protocol::ThreadRealtimeStopResponse;
use cx_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use cx_app_server_protocol::ThreadResumeParams;
use cx_app_server_protocol::ThreadResumeResponse;
use cx_app_server_protocol::ThreadRollbackParams;
use cx_app_server_protocol::ThreadSearchOccurrence;
use cx_app_server_protocol::ThreadSearchOccurrencesParams;
use cx_app_server_protocol::ThreadSearchOccurrencesResponse;
use cx_app_server_protocol::ThreadSearchParams;
use cx_app_server_protocol::ThreadSearchResponse;
use cx_app_server_protocol::ThreadSearchResult;
use cx_app_server_protocol::ThreadSearchSortKey;
use cx_app_server_protocol::ThreadSearchTextRange;
use cx_app_server_protocol::ThreadSetNameParams;
use cx_app_server_protocol::ThreadSetNameResponse;
use cx_app_server_protocol::ThreadSettings;
use cx_app_server_protocol::ThreadSettingsUpdateParams;
use cx_app_server_protocol::ThreadSettingsUpdateResponse;
use cx_app_server_protocol::ThreadShellCommandParams;
use cx_app_server_protocol::ThreadShellCommandResponse;
use cx_app_server_protocol::ThreadSortKey;
use cx_app_server_protocol::ThreadSourceKind;
use cx_app_server_protocol::ThreadStartParams;
use cx_app_server_protocol::ThreadStartResponse;
use cx_app_server_protocol::ThreadStartedNotification;
use cx_app_server_protocol::ThreadStatus;
use cx_app_server_protocol::ThreadTurnsListParams;
use cx_app_server_protocol::ThreadTurnsListResponse;
use cx_app_server_protocol::ThreadUnarchiveParams;
use cx_app_server_protocol::ThreadUnarchiveResponse;
use cx_app_server_protocol::ThreadUnarchivedNotification;
use cx_app_server_protocol::ThreadUnsubscribeParams;
use cx_app_server_protocol::ThreadUnsubscribeResponse;
use cx_app_server_protocol::ThreadUnsubscribeStatus;
use cx_app_server_protocol::Turn;
use cx_app_server_protocol::TurnEnvironmentParams;
use cx_app_server_protocol::TurnError;
use cx_app_server_protocol::TurnInterruptParams;
use cx_app_server_protocol::TurnInterruptResponse;
use cx_app_server_protocol::TurnItemsView;
use cx_app_server_protocol::TurnStartParams;
use cx_app_server_protocol::TurnStartResponse;
use cx_app_server_protocol::TurnStatus;
use cx_app_server_protocol::TurnSteerParams;
use cx_app_server_protocol::TurnSteerResponse;
use cx_app_server_protocol::UserInput as V2UserInput;
use cx_app_server_protocol::WindowsSandboxReadiness;
use cx_app_server_protocol::WindowsSandboxReadinessResponse;
use cx_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use cx_app_server_protocol::WindowsSandboxSetupMode;
use cx_app_server_protocol::WindowsSandboxSetupStartParams;
use cx_app_server_protocol::WindowsSandboxSetupStartResponse;
use cx_app_server_protocol::WorkspaceMessage;
use cx_app_server_protocol::WorkspaceMessageType;
use cx_arg0::Arg0DispatchPaths;
use cx_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use cx_backend_client::Client as BackendClient;
use cx_backend_client::CodexWorkspaceMessage as BackendWorkspaceMessage;
use cx_backend_client::CodexWorkspaceMessageType as BackendWorkspaceMessageType;
use cx_backend_client::CodexWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use cx_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use cx_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use cx_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use cx_backend_client::RequestError as BackendRequestError;
use cx_backend_client::TokenUsageProfile;
use cx_config::CloudConfigBundleLoadError;
use cx_config::CloudConfigBundleLoadErrorCode;
use cx_config::ConfigLayerStack;
use cx_config::loader::project_trust_key;
use cx_config::types::McpServerTransportConfig;
use cx_connectors::AppInfo;
use cx_core::CodexThread;
use cx_core::CodexThreadSettingsOverrides;
use cx_core::ForkSnapshot;
use cx_core::McpManager;
use cx_core::NewThread;
use cx_core::NotSubmittedReason;
#[cfg(test)]
use cx_core::SessionMeta;
use cx_core::StartThreadOptions;
use cx_core::SteerSubmission;
use cx_core::ThreadConfigSnapshot;
use cx_core::ThreadManager;
use cx_core::TurnInput;
use cx_core::TurnInputRequest;
use cx_core::TurnInputSubmission;
use cx_core::TurnStartOptions;
use cx_core::config::Config;
use cx_core::config::ConfigOverrides;
use cx_core::config::NetworkProxyAuditMetadata;
use cx_core::config::edit::ConfigEdit;
use cx_core::config::edit::ConfigEditsBuilder;
use cx_core::connectors::AccessibleConnectorsStatus;
use cx_core::exec::ExecCapturePolicy;
use cx_core::exec::ExecExpiration;
use cx_core::exec::ExecParams;
use cx_core::exec_env::create_env;
use cx_core::path_utils;
#[cfg(test)]
use cx_core::read_head_for_summary;
use cx_core::sandboxing::SandboxPermissions;
use cx_core::truncate_rollout_after_turn_id;
use cx_core::truncate_rollout_before_turn_id;
use cx_core::windows_sandbox::WindowsSandboxLevelExt;
use cx_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use cx_core::windows_sandbox::WindowsSandboxSetupRequest;
use cx_core::windows_sandbox::sandbox_setup_is_complete;
use cx_core_plugins::PluginInstallError as CorePluginInstallError;
use cx_core_plugins::PluginInstallRequest;
use cx_core_plugins::PluginReadRequest;
use cx_core_plugins::PluginUninstallError as CorePluginUninstallError;
use cx_core_plugins::PluginsManager;
use cx_core_plugins::loader::load_plugin_apps;
use cx_core_plugins::manifest::PluginManifestInterface;
use cx_core_plugins::marketplace::MarketplaceError;
use cx_core_plugins::marketplace::MarketplacePluginSource;
use cx_core_plugins::marketplace_add::MarketplaceAddError;
use cx_core_plugins::marketplace_add::MarketplaceAddRequest;
use cx_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_cx_home;
use cx_core_plugins::marketplace_remove::MarketplaceRemoveError;
use cx_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use cx_core_plugins::marketplace_remove::remove_marketplace;
use cx_core_plugins::remote::RemoteMarketplace;
use cx_core_plugins::remote::RemoteMarketplaceSource;
use cx_core_plugins::remote::RemotePluginCatalogError;
use cx_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use cx_core_plugins::remote::RemotePluginServiceConfig;
use cx_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use cx_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use cx_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use cx_exec_server::EnvironmentManager;
use cx_exec_server::EnvironmentObservedStatus;
use cx_exec_server::LOCAL_ENVIRONMENT_ID;
use cx_exec_server::LOCAL_FS;
use cx_features::FEATURES;
use cx_features::Feature;
use cx_features::Stage;
use cx_feedback::CodexFeedback;
use cx_feedback::FeedbackAttachmentPath;
use cx_feedback::FeedbackUploadOptions;
use cx_git_utils::git_diff_to_remote;
use cx_git_utils::resolve_root_git_project_for_trust;
use cx_gt::connectors;
use cx_login::AuthManager;
use cx_login::CX_OPEN_APP_URL;
use cx_login::CodexAuth;
use cx_login::LoginSuccessPage;
use cx_login::LoginSuccessPageBrand;
use cx_login::ServerOptions as LoginServerOptions;
use cx_login::ShutdownHandle;
use cx_login::complete_device_code_login;
use cx_login::login_with_api_key;
use cx_login::login_with_bedrock_api_key;
use cx_login::oauth_client_id;
use cx_login::request_device_code;
use cx_login::run_login_server;
use cx_mcp::McpRuntimeContext;
use cx_mcp::McpServerStatusSnapshot;
use cx_mcp::McpSnapshotDetail;
use cx_mcp::collect_mcp_server_status_snapshot_with_detail;
use cx_mcp::discover_supported_scopes;
use cx_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use cx_mcp::resolve_oauth_scopes;
use cx_memories_write::clear_memory_roots_contents;
use cx_model_provider::create_model_provider;
use cx_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use cx_protocol::ThreadId;
use cx_protocol::config_types::CollaborationMode;
use cx_protocol::config_types::ForcedLoginMethod;
use cx_protocol::config_types::Personality;
use cx_protocol::config_types::ReasoningSummary;
use cx_protocol::config_types::TrustLevel;
use cx_protocol::config_types::WindowsSandboxLevel;
use cx_protocol::error::CxErr;
use cx_protocol::error::Result as CodexResult;
#[cfg(test)]
use cx_protocol::items::TurnItem;
use cx_protocol::models::ResponseItem;
use cx_protocol::openai_models::ReasoningEffort;
use cx_protocol::protocol::AgentStatus;
use cx_protocol::protocol::ConversationAudioParams;
use cx_protocol::protocol::ConversationSpeechParams;
use cx_protocol::protocol::ConversationStartParams;
use cx_protocol::protocol::ConversationStartTransport;
use cx_protocol::protocol::ConversationTextParams;
use cx_protocol::protocol::EnvironmentConfigState;
use cx_protocol::protocol::EventMsg;
#[cfg(test)]
use cx_protocol::protocol::GitInfo as CoreGitInfo;
use cx_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use cx_protocol::protocol::Op;
use cx_protocol::protocol::RealtimeVoicesList;
use cx_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use cx_protocol::protocol::ReviewRequest;
use cx_protocol::protocol::ReviewTarget as CoreReviewTarget;
use cx_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use cx_protocol::protocol::SessionMetaLine;
use cx_protocol::protocol::TurnEnvironmentSelection;
use cx_protocol::protocol::TurnEnvironmentSelections;
use cx_protocol::protocol::W3cTraceContext;
use cx_protocol::protocol::strip_user_message_prefix;
use cx_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use cx_protocol::user_input::UserInput as CoreInputItem;
use cx_rmcp_client::McpOAuthClientRegistration;
use cx_rmcp_client::StreamableHttpRedirectMode;
use cx_rmcp_client::perform_oauth_login_return_url;
use cx_rollout::InitialHistory;
use cx_rollout::ResumedHistory;
use cx_rollout::RolloutItem;
use cx_rollout::is_persisted_rollout_item;
use cx_rollout::state_db::StateDbHandle;
use cx_rollout::state_db::reconcile_rollout;
use cx_state::ThreadMetadata;
use cx_state::log_db::LogDbLayer;
use cx_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use cx_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use cx_thread_store::ClearableField as StoreClearableField;
use cx_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use cx_thread_store::GitInfoPatch as StoreGitInfoPatch;
use cx_thread_store::ItemSortKey as StoreItemSortKey;
use cx_thread_store::ListItemsParams as StoreListItemsParams;
use cx_thread_store::ListThreadsParams as StoreListThreadsParams;
use cx_thread_store::ListTurnsParams as StoreListTurnsParams;
use cx_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use cx_thread_store::LocalThreadStore;
use cx_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use cx_thread_store::ReadThreadParams as StoreReadThreadParams;
use cx_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use cx_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use cx_thread_store::SortDirection as StoreSortDirection;
use cx_thread_store::StoredThread;
use cx_thread_store::StoredTurn;
use cx_thread_store::StoredTurnItemsView;
use cx_thread_store::StoredTurnStatus;
use cx_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use cx_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use cx_thread_store::ThreadSortKey as StoreThreadSortKey;
use cx_thread_store::ThreadStore;
use cx_thread_store::ThreadStoreError;
use cx_utils_absolute_path::AbsolutePathBuf;
use cx_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use cx_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod diagnostics;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod persisted_resume_settings;
mod plugins;
mod process_exec_processor;
mod projects;
mod remote_control_processor;
mod search;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_processor;
mod thread_queue_processor;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use diagnostics::read_server_diagnostics;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use projects::ProjectRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use thread_queue_processor::ThreadQueueRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
            config: EnvironmentConfigState::FromThread,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(item, cx_protocol::protocol::ThreadHistoryMode::Legacy) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
