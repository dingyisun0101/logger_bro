/// ==============================================================================
/// src/runtime.rs
/// Runtime thread for draining the store and rendering a TUI.
/// ==============================================================================

#[cfg(feature = "tui")]
use std::io;
#[cfg(feature = "tui")]
use std::time::{Duration, Instant};

#[cfg(feature = "tui")]
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "tui")]
use ratatui::{prelude::*, widgets::{Block, Borders, List, ListItem, Paragraph}};

#[cfg(feature = "tui")]
use crate::ClientStore;

/// Runtime that owns the main loop for draining updates and rendering.
///
/// This is intended to run on the "runtime thread" and should not be
/// invoked from simulation threads.
#[cfg(feature = "tui")]
pub struct Runtime {
    /// Target frames per second for rendering.
    fps: u64,
    /// Whether to exit the loop when the user presses `q`.
    quit_on_q: bool,
    /// Optional project label shown at the top of the UI.
    project_label: Option<String>,
    /// Monotonic start time for the runtime session.
    start_time: Option<Instant>,
    /// Whether a quit confirmation is currently active.
    confirm_quit: bool,
}

#[cfg(feature = "tui")]
impl Runtime {
    /// Create a runtime with a target frames-per-second value.
    pub fn new(fps: u64) -> Self {
        Self {
            fps,
            quit_on_q: true,
            project_label: None,
            start_time: None,
            confirm_quit: false,
        }
    }

    /// Control whether pressing `q` exits the runtime loop.
    pub fn quit_on_q(mut self, enabled: bool) -> Self {
        self.quit_on_q = enabled;
        self
    }

    /// Set the project label displayed at the top of the UI.
    pub fn project_label(mut self, label: impl Into<String>) -> Self {
        self.project_label = Some(label.into());
        self
    }

    /// Run the runtime loop, draining the store and drawing to the terminal.
    ///
    /// This method blocks until the loop exits (for example, when `q` is pressed).
    pub fn run(&mut self, store: &mut ClientStore) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let frame_time = Duration::from_millis(1_000 / self.fps.max(1));
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        loop {
            let frame_start = Instant::now();

            if self.quit_on_q {
                if let Some(quit) = handle_quit_input(self.confirm_quit)? {
                    match quit {
                        QuitAction::RequestConfirm => self.confirm_quit = true,
                        QuitAction::Cancel => self.confirm_quit = false,
                        QuitAction::Confirm => break,
                    }
                }
            }

            store.drain();
            let snapshot = store.snapshot();
            let elapsed = self
                .start_time
                .map(|start| start.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            let header = format_project_header(self.project_label.as_deref(), elapsed);

            terminal.draw(|f| render_frame(f, &snapshot, &header, self.confirm_quit))?;

            let elapsed = frame_start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        std::process::exit(0);
    }
}

#[cfg(feature = "tui")]
enum QuitAction {
    RequestConfirm,
    Confirm,
    Cancel,
}

fn handle_quit_input(confirming: bool) -> io::Result<Option<QuitAction>> {
    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            return Ok(match key.code {
                KeyCode::Char('q') if !confirming => Some(QuitAction::RequestConfirm),
                KeyCode::Char('y') if confirming => Some(QuitAction::Confirm),
                KeyCode::Char('n') if confirming => Some(QuitAction::Cancel),
                KeyCode::Esc if confirming => Some(QuitAction::Cancel),
                _ => None,
            });
        }
    }
    Ok(None)
}

#[cfg(feature = "tui")]
fn render_frame(
    frame: &mut Frame<'_>,
    snapshot: &[crate::ClientState],
    header: &str,
    confirm_quit: bool,
) {
    let size = frame.area();

    let blocks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(67),
            Constraint::Percentage(30),
        ])
        .split(size);

    let items: Vec<ListItem> = snapshot
        .iter()
        .map(|state| format_task_item(state))
        .collect();

    let header = Paragraph::new(header.to_string())
        .block(Block::default().borders(Borders::ALL).title("Project"))
        .alignment(Alignment::Center);

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Clients"));

    let info_text = if confirm_quit {
        "Quit? (y/n)"
    } else {
        "Press 'q' to quit"
    };
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"));

    frame.render_widget(header, blocks[0]);

    frame.render_widget(list, blocks[1]);

    frame.render_widget(info, blocks[2]);
}

#[cfg(feature = "tui")]
fn format_task_item(state: &crate::ClientState) -> ListItem<'_> {
    let label = state.label.as_deref().unwrap_or("unnamed");
    let status_str = state
        .status
        .map(|s| format!("{s:?}"))
        .unwrap_or_else(|| "Unknown".to_string());
    let current = state.current.unwrap_or(0);
    let total = state.total;
    let percent = total.map(|t| {
        if t == 0 {
            0
        } else {
            ((current.saturating_mul(100)) / t) as u16
        }
    });

    let bar = render_bar(percent, 20);
    let total_str = total
        .map(|t| t.to_string())
        .unwrap_or_else(|| "?".to_string());
    let pct_str = percent
        .map(|p| format!("{p:3}%"))
        .unwrap_or_else(|| " ??%".to_string());
    let last_update = format_duration(state.last_update.elapsed());

    let status_style = match state.status {
        Some(crate::TaskStatus::Completed) => Style::default().fg(Color::Green),
        Some(crate::TaskStatus::Failed) | Some(crate::TaskStatus::Canceled) => {
            Style::default().fg(Color::Red)
        }
        _ => Style::default(),
    };

    let line = Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(Color::Blue)),
        Span::raw(" | "),
        Span::styled(status_str, status_style),
        Span::raw(format!(
            " | {current}/{total_str} | {bar} {pct_str} | last {last_update}"
        )),
    ]);

    ListItem::new(line)
}

#[cfg(feature = "tui")]
fn render_bar(percent: Option<u16>, width: usize) -> String {
    let filled = percent
        .map(|p| ((p as usize * width) / 100).min(width))
        .unwrap_or(0);
    let mut bar = String::with_capacity(width + 2);
    bar.push('[');
    for i in 0..width {
        if i < filled {
            bar.push('#');
        } else {
            bar.push('-');
        }
    }
    bar.push(']');
    bar
}

#[cfg(feature = "tui")]
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 1 {
        return format!("{}ms", duration.as_millis());
    }
    if secs < 60 {
        let millis = duration.subsec_millis();
        return format!("{:.1}s", secs as f64 + (millis as f64 / 1000.0));
    }
    if secs < 3_600 {
        return format!("{}m{:02}s", secs / 60, secs % 60);
    }
    format!("{}h{:02}m", secs / 3_600, (secs % 3_600) / 60)
}

#[cfg(feature = "tui")]
fn format_project_header(label: Option<&str>, elapsed: Duration) -> String {
    let label = label.unwrap_or("Project");
    format!("{label} | elapsed {}", format_duration(elapsed))
}
