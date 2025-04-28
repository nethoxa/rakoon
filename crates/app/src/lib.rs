use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use engine::{Engine, errors::EngineError};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct App {
    engine: Engine,
}

impl App {
    pub fn new() -> Self {
        Self { engine: Engine::default() }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        // First, configure the terminal in raw mode. This is because we want to turn off all
        // the things the terminal does automatically, so that we can handle it manually.
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Some shared state
        let info_text = Arc::new(Mutex::new(Vec::<String>::new()));
        let input = Arc::new(Mutex::new(String::new()));
        let command_history = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
        let scroll_position = Arc::new(Mutex::new(0));

        // Main loop
        loop {
            terminal
                .draw(|f| self.ui(f, &info_text, &input, &command_history, &scroll_position))?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                // [nethoxa] check this number
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            let mut current_input = input.lock().unwrap();
                            current_input.push(c);
                        }
                        KeyCode::Backspace => {
                            let mut current_input = input.lock().unwrap();
                            current_input.pop();
                        }
                        KeyCode::Enter => {
                            let mut current_input = input.lock().unwrap();
                            let command = current_input.clone();
                            current_input.clear();

                            // Add the command to the history
                            let mut history = command_history.lock().unwrap();

                            // Check if the engine is running, so that we don't execute anything if
                            // it's not running
                            if !self.engine.is_running() {
                                match command.trim() {
                                    "start" => match self.engine.start() {
                                        Ok(output) => {
                                            history.push((
                                                command.clone(),
                                                format!("[{}] {}", "+".bright_green(), output),
                                            ));
                                        }
                                        Err(e) => {
                                            history.push((
                                                command.clone(),
                                                format!("[{}] {}", "-".bright_red(), e),
                                            ));
                                        }
                                    }, // [nethoxa] add fields here so that they are setters
                                    "exit" => {
                                        break;
                                    }
                                    _ => {
                                        history.push((
                                            command.clone(),
                                            format!(
                                                "[{}] {}",
                                                "-".bright_red(),
                                                EngineError::EngineNotRunning
                                            ),
                                        ));
                                    }
                                }
                            } else {
                                match command.trim() {
                                    "start" => {
                                        history.push((
                                            command.clone(),
                                            format!(
                                                "[{}] {}",
                                                "-".bright_red(),
                                                "Engine already running"
                                            ),
                                        ));
                                    }
                                    "stop" => match self.engine.stop() {
                                        Ok(output) => {
                                            history.push((
                                                command.clone(),
                                                format!("[{}] {}", "+".bright_green(), output),
                                            ));
                                        }
                                        Err(e) => {
                                            history.push((
                                                command.clone(),
                                                format!("[{}] {}", "-".bright_red(), e),
                                            ));
                                        }
                                    },
                                    "exit" => {
                                        break;
                                    }
                                    _ => {
                                        history.push((
                                            command.clone(),
                                            format!(
                                                "[{}] {}",
                                                "-".bright_red(),
                                                EngineError::InvalidCommand(command.clone())
                                            ),
                                        ));
                                    }
                                }
                            }

                            // Reset the scroll position to the end, so that it keeps showing the
                            // latest commands
                            let mut scroll = scroll_position.lock().unwrap();
                            *scroll = 0;
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Down => {
                            let mut scroll = scroll_position.lock().unwrap();
                            if *scroll > 0 {
                                *scroll -= 1;
                            }
                        }
                        KeyCode::Up => {
                            let mut scroll = scroll_position.lock().unwrap();
                            let history = command_history.lock().unwrap();
                            if *scroll < history.len() {
                                *scroll += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn ui(
        &mut self,
        f: &mut Frame,
        info: &Arc<Mutex<Vec<String>>>,
        input: &Arc<Mutex<String>>,
        command_history: &Arc<Mutex<Vec<(String, String)>>>,
        scroll_position: &Arc<Mutex<usize>>,
    ) {
        // Divide the screen in two parts (upper and lower)
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(f.area());

        // Divide the upper part in two horizontal parts
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(main_chunks[0]);

        // Left panel (fuzzer info)
        let info_lock = info.lock().unwrap();
        let info_text =
            Text::from(info_lock.iter().map(|s| Line::from(s.clone())).collect::<Vec<Line>>());
        let info_paragraph = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Fuzzer info"))
            .wrap(Wrap { trim: true });
        f.render_widget(info_paragraph, top_chunks[0]);

        // Right panel (command history)
        let history_lock = command_history.lock().unwrap();
        let scroll = *scroll_position.lock().unwrap();

        let mut history_lines = Vec::new();

        // Show commands from oldest (up) to newest (down)
        for (cmd, result) in history_lock.iter() {
            history_lines.push(Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Yellow)),
                Span::styled(cmd, Style::default().fg(Color::Green)),
            ]));
            history_lines.push(Line::from(vec![Span::raw(result)]));
            history_lines.push(Line::from(vec![Span::raw("")]));
        }

        // Calculate the automatic scroll to keep the latest commands visible
        let history_area = top_chunks[1].inner(Margin::new(1, 1));
        let visible_lines = history_area.height as usize;
        let total_lines = history_lines.len();

        // Calculate the automatic scroll to keep the latest commands visible
        let auto_scroll = total_lines.saturating_sub(visible_lines);

        // Apply the manual scroll of the user (inverted)
        let scroll_amount = if auto_scroll > scroll { auto_scroll - scroll } else { 0 };

        let history_text = Text::from(history_lines);

        let history_paragraph = Paragraph::new(history_text)
            .block(Block::default().borders(Borders::ALL).title("Command history"))
            .wrap(Wrap { trim: true })
            .scroll((scroll_amount as u16, 0));

        f.render_widget(history_paragraph, top_chunks[1]);

        // Add the scrollbar
        if total_lines > visible_lines {
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            f.render_stateful_widget(
                scrollbar,
                top_chunks[1].inner(Margin::new(1, 1)),
                &mut ratatui::widgets::ScrollbarState::new(auto_scroll).position(scroll_amount),
            );
        }

        // Bottom panel (command input)
        let input_lock = input.lock().unwrap();
        let input_text = Text::from(vec![
            Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Yellow)),
                Span::raw(&*input_lock),
            ]),
        ]);

        let input_paragraph = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL).title("Command input"));
        f.render_widget(input_paragraph, main_chunks[1]);
    }
}
