//! NC/VC/PIE Commander style 4-panel TUI for CY-CLI.
//!
//! Layout (PIE Commander inspired):
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Panel 1 (Top-Left)    │ Panel 2 (Top-Right)                 │
//! │ File Tree / Sessions  │ Agent Output / Chat                 │
//! ├───────────────────────┼─────────────────────────────────────┤
//! │ Panel 3 (Bottom-Left) │ Panel 4 (Bottom-Right)              │
//! │ Model / Config        │ Logs / Debug                        │
//! ├───────────────────────┴─────────────────────────────────────┤
//! │ Command Line: cy > ______________________________________ │
//! ├─────────────────────────────────────────────────────────────┤
//! │ F1 Help  F2 Menu  F3 View  F4 Edit  F5 Copy  F6 Move  F7 MkDir  F8 Del  F9 Menu  F10 Quit │
//! └─────────────────────────────────────────────────────────────┘

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use ratatui::prelude::Stylize;
use std::io::{self, Write};
use std::io::IsTerminal;
use uuid::Uuid;

/// 4-panel layout manager for CY-CLI TUI
pub struct NcView {
    /// Current focused panel (0-3)
    focused_panel: usize,
    /// Panel contents
    panels: [PanelContent; 4],
    /// Command line input buffer
    cmd_buffer: String,
    /// Status message
    status: String,
    /// Show help
    show_help: bool,
    /// Last emitted terminal title to avoid redundant writes.
    last_title: Option<String>,
    /// Chat identifier shown in the terminal title and agent panel.
    chat_id: String,
}

#[derive(Debug, Clone)]
enum PanelContent {
    FileTree { cwd: String, entries: Vec<String>, selected: usize },
    AgentOutput { lines: Vec<String> },
    ModelConfig { models: Vec<String>, current: String },
    Logs { lines: Vec<String> },
}

impl PanelContent {
    fn new_file_tree() -> Self {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let entries = Self::read_dir(&cwd);
        Self::FileTree { cwd, entries, selected: 0 }
    }

    fn read_dir(path: &str) -> Vec<String> {
        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(path) {
            for entry in read_dir.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    entries.push(format!("{}/", name));
                } else {
                    entries.push(name);
                }
            }
        }
        entries.sort();
        entries
    }
}

impl Default for NcView {
    fn default() -> Self {
        Self {
            focused_panel: 1,
            panels: [
                PanelContent::new_file_tree(),
                PanelContent::AgentOutput {
                    lines: vec!["Welcome to CY-CLI".to_string(), "Press F1 for help".to_string()],
                },
                PanelContent::ModelConfig {
                    models: vec!["openrouter/free".to_string(), "openrouter/auto".to_string()],
                    current: "openrouter/free".to_string(),
                },
                PanelContent::Logs {
                    lines: vec!["CY-CLI started".to_string()],
                },
            ],
            cmd_buffer: String::new(),
            status: "Ready".to_string(),
            show_help: false,
            last_title: None,
            chat_id: NcView::default_chat_id(),
        }
    }
}

impl NcView {
    fn default_chat_id() -> String {
        std::env::var("CY_CHAT_ID")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| std::env::var("THREAD_ID").ok().filter(|s| !s.trim().is_empty()))
            .or_else(|| {
                std::env::var("CX_THREAD_ID")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            })
            .unwrap_or_else(|| Uuid::new_v4().to_string())
    }
}

impl NcView {
    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.show_help {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::F(1) => {
                    self.show_help = false;
                }
                _ => {}
            }
            return true;
        }

        match key.code {
            // Panel switching with Tab / Shift+Tab
            KeyCode::Tab => {
                self.focused_panel = (self.focused_panel + 1) % 4;
            }
            KeyCode::BackTab => {
                self.focused_panel = (self.focused_panel + 3) % 4;
            }

            // Function keys F1-F10
            KeyCode::F(1) => self.show_help = true,
            KeyCode::F(2) => self.status = "F2: User Menu (not implemented)".to_string(),
            KeyCode::F(3) => self.status = "F3: View (not implemented)".to_string(),
            KeyCode::F(4) => self.status = "F4: Edit (not implemented)".to_string(),
            KeyCode::F(5) => self.status = "F5: Copy (not implemented)".to_string(),
            KeyCode::F(6) => self.status = "F6: Move (not implemented)".to_string(),
            KeyCode::F(7) => self.status = "F7: Make Dir (not implemented)".to_string(),
            KeyCode::F(8) => self.status = "F8: Delete (not implemented)".to_string(),
            KeyCode::F(9) => self.status = "F9: Menu (not implemented)".to_string(),
            KeyCode::F(10) => return false, // Quit

            // Command line editing
            KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                match c {
                    'c' => self.cmd_buffer.clear(),
                    'u' => self.cmd_buffer.clear(),
                    'k' => {
                        // Clear to end
                    }
                    _ => {}
                }
            }
            KeyCode::Char(c) => {
                if self.focused_panel != 0 || !self.cmd_buffer.is_empty() {
                    self.cmd_buffer.push(c);
                }
            }
            KeyCode::Enter => {
                if self.focused_panel == 0 {
                    self.enter_selected();
                } else if !self.cmd_buffer.trim().is_empty() {
                    self.execute_command();
                }
            }
            KeyCode::Backspace => {
                if self.focused_panel == 0 && self.cmd_buffer.is_empty() {
                    self.go_up();
                } else {
                    self.cmd_buffer.pop();
                }
            }
            KeyCode::Esc => {
                self.cmd_buffer.clear();
            }

            // Arrow keys for panel navigation
            KeyCode::Up => self.navigate_panel(-1),
            KeyCode::Down => self.navigate_panel(1),
            KeyCode::Left => self.navigate_panel(-1),
            KeyCode::Right => self.navigate_panel(1),

            _ => {}
        }
        true
    }

    fn current_chat_id(&self) -> Option<&str> {
        if self.chat_id.is_empty() { return None; }
        Some(self.chat_id.as_str())
    }

    fn sync_terminal_title(&mut self) {
        let chat_id = self.current_chat_id();
        let mut title = String::from("CY");
        if let Some(chat_id) = chat_id {
            title.push_str(" | chat: ");
            title.push_str(chat_id);
        }
        title.push_str(" | Commander");
        if !self.cmd_buffer.is_empty() {
            title.push_str(" | input");
        }

        if self.last_title.as_deref() == Some(title.as_str()) {
            return;
        }

        if let Err(err) = io::stdout().write_all(b"\x1B]0;") {
            tracing::debug!(error = %err, "failed to set ncview terminal title");
        } else if let Err(err) = io::stdout().write_all(title.as_bytes()) {
            tracing::debug!(error = %err, "failed to set ncview terminal title");
        } else if let Err(err) = io::stdout().write_all(b"\x07") {
            tracing::debug!(error = %err, "failed to set ncview terminal title");
        } else {
            self.last_title = Some(title);
        }
    }

    fn navigate_panel(&mut self, delta: i32) {
        match &mut self.panels[self.focused_panel] {
            PanelContent::FileTree { selected, entries, .. } => {
                if !entries.is_empty() {
                    let new = (*selected as i32 + delta).clamp(0, entries.len() as i32 - 1) as usize;
                    *selected = new;
                }
            }
            PanelContent::AgentOutput { .. } | PanelContent::Logs { .. } => {
                // Scroll
            }
            PanelContent::ModelConfig { .. } => {}
        }
    }

    fn execute_command(&mut self) {
        let cmd = self.cmd_buffer.trim().to_string();
        self.add_log(&format!("> {}", cmd));
        self.cmd_buffer.clear();

        // Simple command parsing
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "q" | "quick" => {
                if parts.len() > 1 {
                    let prompt = parts[1..].join(" ");
                    self.add_agent_output(&format!("> {}", prompt));
                    self.run_quick_sync(&prompt);
                }
            }
            "m" | "model" => {
                if parts.len() > 1 {
                    self.set_model(parts[1]);
                } else {
                    // Show current model - read from panel 2
                    let current = match &self.panels[2] {
                        PanelContent::ModelConfig { current, .. } => current.clone(),
                        _ => "unknown".to_string(),
                    };
                    self.add_agent_output(&format!("Current model: {}", current));
                }
            }
            "ls" | "list-models" => {
                let models = match &self.panels[2] {
                    PanelContent::ModelConfig { models, .. } => models.clone(),
                    _ => vec![],
                };
                for m in models {
                    self.add_agent_output(&m);
                }
            }
            "r" | "resume" => {
                self.add_agent_output("Resume: not implemented");
            }
            "hist" | "history" => {
                self.add_agent_output("History: not implemented");
            }
            "b" | "batch" => {
                self.add_agent_output("Batch: not implemented");
            }
            "help" | "?" => {
                self.show_help = true;
            }
            "clear" => {
                if let PanelContent::AgentOutput { lines } = &mut self.panels[1] {
                    lines.clear();
                }
            }
            _ => {
                self.add_agent_output(&format!("Unknown command: {}. Type 'help'", parts[0]));
            }
        }
    }

    fn add_agent_output(&mut self, line: &str) {
        if let PanelContent::AgentOutput { lines } = &mut self.panels[1] {
            lines.push(line.to_string());
        }
    }

    fn run_quick_sync(&mut self, prompt: &str) {
        let cy_exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(_) => {
                self.add_agent_output("Error: cannot find cy executable");
                return;
            }
        };
        let output = match std::process::Command::new(&cy_exe)
            .args(["q", "--skip-git-repo-check", prompt])
            .output()
        {
            Ok(o) => o,
            Err(_) => {
                self.add_agent_output("Error: failed to spawn cy q");
                return;
            }
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            self.add_agent_output(line);
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            self.add_agent_output(&format!("[err] {}", line));
        }
    }

    fn add_log(&mut self, line: &str) {
        if let PanelContent::Logs { lines } = &mut self.panels[3] {
            lines.push(line.to_string());
        }
    }

    fn set_model(&mut self, model: &str) {
        if let PanelContent::ModelConfig { current, .. } = &mut self.panels[2] {
            *current = model.to_string();
            self.add_agent_output(&format!("Model set to: {}", model));
        }
    }

    fn enter_selected(&mut self) {
        if let PanelContent::FileTree { cwd, entries, selected } = &mut self.panels[0] {
            if *selected < entries.len() {
                let entry = &entries[*selected];
                let path = std::path::Path::new(cwd).join(entry.trim_end_matches('/'));
                if entry.ends_with('/') && path.is_dir() {
                    *cwd = path.to_string_lossy().to_string();
                    *entries = PanelContent::read_dir(cwd);
                    *selected = 0;
                    self.status = format!("Entered: {}", cwd);
                } else if path.is_file() {
                    self.status = format!("File: {}", path.display());
                    self.add_agent_output(&format!("Selected file: {}", path.display()));
                }
            }
        }
    }

    fn go_up(&mut self) {
        if let PanelContent::FileTree { cwd, entries, selected } = &mut self.panels[0] {
            if let Some(parent) = std::path::Path::new(cwd).parent() {
                *cwd = parent.to_string_lossy().to_string();
                *entries = PanelContent::read_dir(cwd);
                *selected = 0;
                self.status = format!("Up: {}", cwd);
            }
        }
    }

    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let help_text = vec![
            Line::from("CY-CLI Help (PIE Commander 4-panel TUI)".bold().cyan()),
            Line::from(""),
            Line::from("Navigation:".bold()),
            Line::from("  Tab / Shift+Tab  - Switch panels (4 panels)"),
            Line::from("  ↑/↓/←/→          - Navigate within panel"),
            Line::from(""),
            Line::from("Function Keys:".bold()),
            Line::from("  F1               - This help"),
            Line::from("  F2               - User Menu"),
            Line::from("  F3               - View file"),
            Line::from("  F4               - Edit file"),
            Line::from("  F5               - Copy"),
            Line::from("  F6               - Move"),
            Line::from("  F7               - Make Directory"),
            Line::from("  F8               - Delete"),
            Line::from("  F9               - Menu"),
            Line::from("  F10              - Quit"),
            Line::from(""),
            Line::from("Commands (type in command line):".bold()),
            Line::from("  q <prompt>       - Quick question to agent"),
            Line::from("  m [model]        - Show/set model"),
            Line::from("  ls               - List models"),
            Line::from("  r [id]           - Resume session"),
            Line::from("  hist             - Session history"),
            Line::from("  b <instr> [files]- Batch process"),
            Line::from("  clear            - Clear agent output"),
            Line::from("  help             - This help"),
            Line::from(""),
            Line::from("Press F1, Esc, or q to close help".italic().dark_gray()),
        ];

        let help_widget = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(" Help (F1) ").border_style(Style::default().fg(Color::Cyan)))
            .style(Style::default().bg(Color::Black));

        // Center the help
        let help_area = Rect {
            x: area.x + area.width.saturating_sub(70) / 2,
            y: area.y + area.height.saturating_sub(30) / 2,
            width: 70.min(area.width),
            height: 30.min(area.height),
        };
        help_widget.render(help_area, buf);
    }
}

fn render_panel_static(content: &PanelContent, area: Rect, buf: &mut Buffer, focused: bool) {
    let (title, body) = match content {
        PanelContent::FileTree { cwd, entries, selected } => {
            let title = format!(" Files: {} ", cwd);
            let body: Vec<Line> = entries
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let style = if i == *selected && focused {
                        Style::default().bg(Color::Cyan).fg(Color::Black).bold()
                    } else {
                        Style::default()
                    };
                    Line::from(Span::styled(format!(" {}", e), style))
                })
                .collect();
            (title, body)
        }
        PanelContent::AgentOutput { lines } => {
            let title = " Agent ".to_string();
            let body: Vec<Line> = lines.iter().map(|l| Line::from(l.as_str())).collect();
            (title, body)
        }
        PanelContent::ModelConfig { models, current } => {
            let title = " Model ".to_string();
            let body: Vec<Line> = models
                .iter()
                .map(|m| {
                    let prefix = if m == current { "► " } else { "  " };
                    Line::from(format!("{}{}", prefix, m))
                })
                .collect();
            (title, body)
        }
        PanelContent::Logs { lines } => {
            let title = " Logs ".to_string();
            let body: Vec<Line> = lines.iter().map(|l| Line::from(l.as_str())).collect();
            (title, body)
        }
    };

    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title(title).border_style(border_style))
        .render(area, buf);
}

impl Widget for &NcView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.show_help {
            self.render_help(area, buf);
            return;
        }

        // Main layout: 4 panels + command line + status bar
        let chunks = Layout::vertical([
            Constraint::Percentage(40), // Top row (2 panels)
            Constraint::Percentage(40), // Bottom row (2 panels)
            Constraint::Length(3),      // Command line
            Constraint::Length(1),      // Status bar
            Constraint::Length(1),      // Function key bar
        ])
        .split(area);

        // Top row: 2 panels
        let top_chunks = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[0]);

        // Bottom row: 2 panels
        let bottom_chunks = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

        // Render 4 panels
        render_panel_static(&self.panels[0], top_chunks[0], buf, self.focused_panel == 0);
        render_panel_static(&self.panels[1], top_chunks[1], buf, self.focused_panel == 1);
        render_panel_static(&self.panels[2], bottom_chunks[0], buf, self.focused_panel == 2);
        render_panel_static(&self.panels[3], bottom_chunks[1], buf, self.focused_panel == 3);

        // Command line
        let cmd_area = chunks[2];
        let cmd_text = format!("cy > {}", self.cmd_buffer);
        Paragraph::new(cmd_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Command"))
            .render(cmd_area, buf);

        // Status bar
        let status_area = chunks[3];
        Paragraph::new(self.status.as_str())
            .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
            .render(status_area, buf);

        // Function key bar
        let fkey_area = chunks[4];
        let fkeys = Line::from(vec![
            "F1 Help".bold().cyan(),
            "  F2 Menu".dark_gray(),
            "  F3 View".dark_gray(),
            "  F4 Edit".dark_gray(),
            "  F5 Copy".dark_gray(),
            "  F6 Move".dark_gray(),
            "  F7 MkDir".dark_gray(),
            "  F8 Del".dark_gray(),
            "  F9 Menu".dark_gray(),
            "  F10 Quit".red(),
        ]);
        Paragraph::new(fkeys)
            .style(Style::default().bg(Color::Black))
            .render(fkey_area, buf);

        Paragraph::new(format!("chat: {} | {}", self.chat_id, self.status))
            .style(Style::default().bg(Color::DarkGray).fg(Color::LightCyan))
            .render(chunks[3], buf);
    }
}

/// Run the NC/VC/PIE Commander TUI
pub async fn run_ncview() -> anyhow::Result<()> {
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::Terminal;

    if !std::io::stdout().is_terminal() {
        anyhow::bail!("TUI requires a terminal (TTY). Use the command-line subcommands instead.");
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ncview = NcView::default();
    if let PanelContent::AgentOutput { lines } = &mut ncview.panels[1] {
        lines.insert(0, format!("chat: {}", ncview.chat_id));
    }
    let mut running = true;

    while running {
        ncview.sync_terminal_title();
        terminal.draw(|f| {
            f.render_widget(&ncview, f.area());
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                running = ncview.handle_key(key);
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
