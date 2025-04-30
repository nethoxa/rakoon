use colored::Colorize;
use command_handler::CommandHandler;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use engine::{Engine, errors::EngineError};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct App {
    engine: Engine,
    output: Arc<Mutex<String>>,
}

impl App {
    pub fn new(rpc: String) -> Self {
        App {
            engine: Engine::new(rpc),
            output: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let input = Arc::new(Mutex::new(String::new()));

        loop {
            terminal.draw(|f| self.ui(f, &input))?;

            if event::poll(Duration::from_millis(100))? {
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

                            // TODO
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame, input: &Arc<Mutex<String>>) {
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

        // Stats panel
        let stats_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if self.engine.is_running() { "Running" } else { "Stopped" },
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("Transactions: ", Style::default().fg(Color::Yellow)),
                Span::styled("0", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Errors: ", Style::default().fg(Color::Yellow)),
                Span::styled("0", Style::default().fg(Color::Red)),
            ]),
        ]);

        let stats_paragraph = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Fuzzer Stats"))
            .wrap(Wrap { trim: true });
        f.render_widget(stats_paragraph, main_chunks[0]);

        // Command panel
        let input_text = {
            let input_lock = input.lock().unwrap();
            let output_lock = self.output.lock().unwrap();
            Text::from(vec![
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(input_lock.clone()),
                ]),
                Line::from(output_lock.clone()),
            ])
        };

        let input_paragraph = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL).title("Command Input"))
            .wrap(Wrap { trim: true });
        f.render_widget(input_paragraph, main_chunks[1]);
    }
}
