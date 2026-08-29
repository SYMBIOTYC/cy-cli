use crate::config::Config;
pub use cx_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use cx_rollout::Cursor;
pub use cx_rollout::INTERACTIVE_SESSION_SOURCES;
pub use cx_rollout::RolloutRecorder;
pub use cx_rollout::RolloutRecorderParams;
pub use cx_rollout::SESSIONS_SUBDIR;
pub use cx_rollout::SessionMeta;
pub use cx_rollout::SortDirection;
pub use cx_rollout::ThreadItem;
pub use cx_rollout::ThreadSortKey;
pub use cx_rollout::ThreadsPage;
pub use cx_rollout::append_thread_name;
pub use cx_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use cx_rollout::find_conversation_path_by_id_str;
pub use cx_rollout::find_thread_meta_by_name_str;
pub use cx_rollout::find_thread_name_by_id;
pub use cx_rollout::find_thread_names_by_ids;
pub use cx_rollout::find_thread_path_by_id_str;
pub use cx_rollout::parse_cursor;
pub use cx_rollout::read_head_for_summary;
pub use cx_rollout::read_session_meta_line;
pub use cx_rollout::rollout_date_parts;

impl cx_rollout::RolloutConfigView for Config {
    fn cx_home(&self) -> &std::path::Path {
        self.cx_home.as_path()
    }

    fn sqlite_config(&self) -> &cx_state::SqliteConfig {
        self.sqlite_config()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub(crate) mod list {
    pub use cx_rollout::find_thread_path_by_id_str;
}

#[cfg(test)]
pub(crate) mod recorder {
    pub use cx_rollout::RolloutRecorder;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
