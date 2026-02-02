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
}

#[cfg(feature = "tui")]
impl Runtime {
    /// Create a runtime with a target frames-per-second value.
    pub fn new(fps: u64) -> Self {
        Self {
            fps,
            quit_on_q: true,
        }
    }

    /// Control whether pressing `q` exits the runtime loop.
    pub fn quit_on_q(mut self, enabled: bool) -> Self {
        self.quit_on_q = enabled;
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

        loop {
            let frame_start = Instant::now();

            if self.quit_on_q && poll_quit()? {
                break;
            }

            store.drain();
            let snapshot = store.snapshot();

            terminal.draw(|f| render_frame(f, &snapshot))?;

            let elapsed = frame_start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
}

#[cfg(feature = "tui")]
fn poll_quit() -> io::Result<bool> {
    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            return Ok(matches!(key.code, KeyCode::Char('q')));
        }
    }
    Ok(false)
}

#[cfg(feature = "tui")]
fn render_frame(frame: &mut Frame<'_>, snapshot: &[crate::ClientState]) {
    let size = frame.size();

    let blocks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(size);

    let items: Vec<ListItem> = snapshot
        .iter()
        .map(|state| ListItem::new(format_task_line(state)))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Clients"));

    let info = Paragraph::new("Press 'q' to quit")
        .block(Block::default().borders(Borders::ALL).title("Controls"));

    frame.render_widget(list, blocks[0]);

    frame.render_widget(info, blocks[1]);
}

#[cfg(feature = "tui")]
fn format_task_line(state: &crate::ClientState) -> String {
    let label = state.label.as_deref().unwrap_or("unnamed");
    let status = state
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

    format!("{label} | {status} | {current}/{total_str} | {bar} {pct_str}")
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
