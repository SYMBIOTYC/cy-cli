/// Returns whether `host` is one of the gt hosts CX is allowed to treat
/// as first-party gt HTTP traffic.
pub fn is_allowed_gt_host(host: &str) -> bool {
    const EXACT_HOSTS: &[&str] = &["chatgpt.com", "chat.openai.com", "gt-staging.com"];
    const SUBDOMAIN_SUFFIXES: &[&str] = &[".chatgpt.com", ".gt-staging.com"];

    EXACT_HOSTS.contains(&host)
        || SUBDOMAIN_SUFFIXES
            .iter()
            .any(|suffix| host.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_gt_hosts_without_suffix_tricks() {
        for host in [
            "chatgpt.com",
            "foo.chatgpt.com",
            "staging.chatgpt.com",
            "chat.openai.com",
            "gt-staging.com",
            "api.gt-staging.com",
        ] {
            assert!(is_allowed_gt_host(host));
        }

        for host in [
            "evilchatgpt.com",
            "chatgpt.com.evil.example",
            "api.openai.com",
            "foo.chat.openai.com",
        ] {
            assert!(!is_allowed_gt_host(host));
        }
    }
}
