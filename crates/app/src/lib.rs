use al::ALTransactionRunner;
use alloy::primitives::Address;
use blob::BlobTransactionRunner;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use eip1559::EIP1559TransactionRunner;
use eip7702::EIP7702TransactionRunner;
use legacy::LegacyTransactionRunner;
use random::RandomTransactionRunner;
use std::{collections::HashMap, time::Instant};
use tokio::task::JoinHandle;

use errors::AppStatus;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarState, Wrap,
    },
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};
pub mod config;
pub mod errors;
pub mod handler;
pub mod manager;

pub struct App {
    status: String,
    output: Arc<Mutex<String>>,
    command_history: Vec<String>,
    output_history: Vec<String>,
    error_history: Vec<bool>, // Track if each output is an error
    history_scroll_state: ScrollbarState,
    history_list_state: ListState,
    scroll_offset: usize,
    left_panel_width: u16, // Store the width of the left panel
    input_height: u16,     // Store the dynamic height of the input area
    last_refresh: Instant, // Track last UI refresh time
    tx_counts: Arc<Mutex<HashMap<String, u64>>>,
    handler: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,

    random_runner: Arc<Mutex<RandomTransactionRunner>>,
    legacy_runner: Arc<Mutex<LegacyTransactionRunner>>,
    al_runner: Arc<Mutex<ALTransactionRunner>>,
    blob_runner: Arc<Mutex<BlobTransactionRunner>>,
    eip1559_runner: Arc<Mutex<EIP1559TransactionRunner>>,
    eip7702_runner: Arc<Mutex<EIP7702TransactionRunner>>,

    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        App {
            status: "stopped".to_string(),
            output: Arc::new(Mutex::new(String::new())),
            command_history: Vec::new(),
            output_history: Vec::new(),
            error_history: Vec::new(), // Initialize error history
            history_scroll_state: ScrollbarState::default(),
            history_list_state: list_state,
            scroll_offset: 0,
            left_panel_width: 70, // Default width percentage
            input_height: 3,      // Default height (1 for content + 2 for borders)
            last_refresh: Instant::now(),
            tx_counts: Arc::new(Mutex::new(HashMap::new())),
            handler: Arc::new(Mutex::new(HashMap::new())),
            random_runner: Arc::new(Mutex::new(RandomTransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            legacy_runner: Arc::new(Mutex::new(LegacyTransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            al_runner: Arc::new(Mutex::new(ALTransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            blob_runner: Arc::new(Mutex::new(BlobTransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            eip1559_runner: Arc::new(Mutex::new(EIP1559TransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            eip7702_runner: Arc::new(Mutex::new(EIP7702TransactionRunner::new(
                config.rpc_url.clone(),
                config.sk.clone(),
                config.seed,
            ))),
            config,
        }
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
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

                            // Add command to history
                            if !command.trim().is_empty() {
                                self.command_history.push(command.clone());
                            }

                            // Store current output before handling command
                            let result = self.handle_command(command).await;

                            // Add output to history
                            let output = self.output.lock().unwrap().clone();
                            if !output.is_empty() {
                                self.output_history.push(output.clone());
                            }

                            // Reset input height to default
                            self.input_height = 3;

                            // Update list state to scroll to the bottom
                            let total_items = self.command_history.len()
                                + self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if total_items > 0 {
                                self.history_list_state.select(Some(total_items - 1));
                                // Ensure we're scrolled to the bottom
                                self.scroll_offset = total_items.saturating_sub(1);
                            }

                            if result.is_err() {
                                if result.unwrap_err() == AppStatus::Exit {
                                    break;
                                }

                                self.error_history.push(true);
                            } else {
                                self.error_history.push(false);
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Up => {
                            // Scroll history up
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                                if let Some(selected) = self.history_list_state.selected() {
                                    if selected > 0 {
                                        self.history_list_state.select(Some(selected - 1));
                                    }
                                }
                            }
                        }
                        KeyCode::Down => {
                            // Scroll history down
                            let total_items = self.command_history.len()
                                + self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if self.scroll_offset < total_items.saturating_sub(1) {
                                self.scroll_offset += 1;
                                if let Some(selected) = self.history_list_state.selected() {
                                    if selected < total_items.saturating_sub(1) {
                                        self.history_list_state.select(Some(selected + 1));
                                    }
                                }
                            }
                        }
                        KeyCode::Left => {
                            // Decrease left panel width (minimum 20%)
                            if self.left_panel_width > 20 {
                                self.left_panel_width -= 5;
                            }
                        }
                        KeyCode::Right => {
                            // Increase left panel width (maximum 70%)
                            if self.left_panel_width < 70 {
                                self.left_panel_width += 5;
                            }
                        }
                        KeyCode::PageUp => {
                            // Scroll history up by multiple lines
                            if self.scroll_offset > 0 {
                                self.scroll_offset = self.scroll_offset.saturating_sub(5);
                                self.history_list_state.select(Some(self.scroll_offset));
                            }
                        }
                        KeyCode::PageDown => {
                            // Scroll history down by multiple lines
                            let total_items = self.command_history.len()
                                + self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if self.scroll_offset < total_items.saturating_sub(1) {
                                self.scroll_offset =
                                    (self.scroll_offset + 5).min(total_items.saturating_sub(1));
                                self.history_list_state.select(Some(self.scroll_offset));
                            }
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
        // Check if we need to refresh the UI (every 10ms)
        if self.last_refresh.elapsed() >= Duration::from_millis(10) {
            self.last_refresh = Instant::now();
            // Update transaction counts from runners
            let mut tx_counts = self.tx_counts.lock().unwrap();
            for (runner, active) in &self.config.active_runners {
                if *active {
                    match runner.as_str() {
                        "random" => {
                            tx_counts.insert(
                                "random".to_string(),
                                self.random_runner.lock().unwrap().tx_sent,
                            );
                        }
                        "legacy" => {
                            tx_counts.insert(
                                "legacy".to_string(),
                                self.legacy_runner.lock().unwrap().tx_sent,
                            );
                        }
                        "al" => {
                            tx_counts
                                .insert("al".to_string(), self.al_runner.lock().unwrap().tx_sent);
                        }
                        "blob" => {
                            tx_counts.insert(
                                "blob".to_string(),
                                self.blob_runner.lock().unwrap().tx_sent,
                            );
                        }
                        "eip1559" => {
                            tx_counts.insert(
                                "eip1559".to_string(),
                                self.eip1559_runner.lock().unwrap().tx_sent,
                            );
                        }
                        "eip7702" => {
                            tx_counts.insert(
                                "eip7702".to_string(),
                                self.eip7702_runner.lock().unwrap().tx_sent,
                            );
                        }
                        _ => {}
                    }
                }
            }
        }

        // First split the screen into left and right panels using the stored width
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(self.left_panel_width),
                    Constraint::Percentage(100 - self.left_panel_width),
                ]
                .as_ref(),
            )
            .split(f.area());

        // Split the left panel into stats and command input
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(0),
                    Constraint::Length(self.input_height), // Dynamic height for input window
                ]
                .as_ref(),
            )
            .split(horizontal_chunks[0]);

        // Stats panel - use the entire available space
        let stats_area = left_chunks[0];

        // Set status color based on status value
        let status_color = if self.status == "running" { Color::Green } else { Color::Red };

        let stats_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(self.status.clone(), Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("Global Seed: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    self.config.global_seed.map_or("None".to_string(), |s| s.to_string()),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("Global Address: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    self.config
                        .global_sk
                        .as_ref()
                        .map_or("None".to_string(), |sk| Address::from_private_key(sk).to_string()),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ];

        // Add active runners information with transaction counts
        let mut active_runners = Vec::new();
        let tx_counts = self.tx_counts.lock().unwrap();
        for (runner, active) in &self.config.active_runners {
            if *active {
                let seed = self.config.get_runner_seed(runner);
                let address = Address::from_private_key(self.config.get_runner_sk(runner));
                let tx_count = tx_counts.get(runner).unwrap_or(&0);
                active_runners.push(Line::from(vec![
                    Span::styled(
                        format!("{} Runner: ", runner),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("seed={}, address={}, txs={}", seed, address, tx_count),
                        Style::default().fg(Color::Green),
                    ),
                ]));
            }
        }

        // Add available runners information
        let mut available_runners = Vec::new();
        for runner in [
            "random", "legacy", "al", "blob", "eip1559", "eip7702",
        ] {
            let seed = self.config.get_runner_seed(runner);
            let address = Address::from_private_key(self.config.get_runner_sk(runner));
            let is_active = self.config.is_runner_active(runner);
            let status_color = if is_active { Color::Green } else { Color::Gray };
            let tx_count = tx_counts.get(runner).unwrap_or(&0);

            available_runners.push(Line::from(vec![
                Span::styled(format!("{}: ", runner), Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("seed={}, address={}, txs={}", seed, address, tx_count),
                    Style::default().fg(status_color),
                ),
            ]));
        }

        let mut all_lines = stats_lines;
        all_lines.push(Line::from(""));
        all_lines.push(Line::from(vec![
            Span::styled("Active Runners:", Style::default().fg(Color::Yellow)),
        ]));
        all_lines.extend(active_runners);
        all_lines.push(Line::from(""));
        all_lines.push(Line::from(vec![
            Span::styled("Available Runners:", Style::default().fg(Color::Yellow)),
        ]));
        all_lines.extend(available_runners);

        let stats_text = Text::from(all_lines);

        // Calculate the height of the stats text to center it vertically
        let stats_height = stats_text.height() as u16;
        let vertical_padding = (stats_area.height.saturating_sub(2 + stats_height)) / 2; // -2 for borders

        // Create a block with empty lines before the content to center it vertically
        let mut centered_lines = Vec::new();
        for _ in 0..vertical_padding {
            centered_lines.push(Line::from(""));
        }
        centered_lines.extend(stats_text.lines.clone());

        let stats_paragraph = Paragraph::new(Text::from(centered_lines))
            .block(Block::default().borders(Borders::ALL).title("Fuzzer Stats"))
            .alignment(Alignment::Center) // Center horizontally
            .wrap(Wrap { trim: true });
        f.render_widget(stats_paragraph, stats_area);

        // Command panel
        let input_text = {
            let input_lock = input.lock().unwrap();
            Text::from(vec![
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(input_lock.clone()),
                ]),
            ])
        };

        let input_paragraph = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL).title("Command Input"));
        f.render_widget(&input_paragraph, left_chunks[1]);

        // History panel on the right
        let history_block = Block::default().borders(Borders::ALL).title("Command History");

        let history_area = horizontal_chunks[1];
        f.render_widget(history_block, history_area);

        // Create inner area for the history content
        let history_inner_area = Rect {
            x: history_area.x + 1,
            y: history_area.y + 1,
            width: history_area.width - 2,
            height: history_area.height - 2,
        };

        if !self.command_history.is_empty() {
            // Create list items for history
            let mut history_items = Vec::new();
            for (i, cmd) in self.command_history.iter().enumerate() {
                // Create a wrapped command line with proper indentation
                let available_width = history_inner_area.width.saturating_sub(3); // Subtract prefix width "> "
                let mut cmd_lines = Vec::new();
                let mut remaining = cmd.as_str();

                // First line with the command prefix
                let first_line_len = available_width.min(remaining.len() as u16);
                let (first_part, rest) = remaining.split_at(first_line_len as usize);
                cmd_lines.push(Line::from(vec![
                    Span::styled(format!("> {}", first_part), Style::default().fg(Color::Yellow)),
                ]));
                remaining = rest;

                // Subsequent lines with proper indentation if command is long
                while !remaining.is_empty() {
                    let line_len = available_width.min(remaining.len() as u16);
                    let (part, rest) = remaining.split_at(line_len as usize);
                    cmd_lines.push(Line::from(vec![
                        Span::styled(format!("  {}", part), Style::default().fg(Color::Yellow)),
                    ]));
                    remaining = rest;
                }

                // Add all command lines to history items
                for line in cmd_lines {
                    history_items.push(ListItem::new(line));
                }

                if i < self.output_history.len() && !self.output_history[i].is_empty() {
                    let output_text = &self.output_history[i];
                    // Use the error_history to determine if this is an error
                    let is_error = self.error_history[i];

                    // Create styled icon
                    let icon_span = if is_error {
                        Span::styled("-", Style::default().fg(Color::Red))
                    } else {
                        Span::styled("+", Style::default().fg(Color::Green))
                    };

                    // Wrap output text with proper indentation
                    let output_color = if is_error { Color::Red } else { Color::White };
                    let mut output_remaining = output_text.as_str();

                    // First line of output with icon
                    let first_output_len =
                        available_width.saturating_sub(5).min(output_remaining.len() as u16); // [+]  prefix
                    let (first_output, rest_output) =
                        output_remaining.split_at(first_output_len as usize);
                    history_items.push(ListItem::new(Line::from(vec![
                        Span::raw("  ["),
                        icon_span.clone(),
                        Span::raw("] "),
                        Span::styled(first_output, Style::default().fg(output_color)),
                    ])));
                    output_remaining = rest_output;

                    // Subsequent lines of output with proper indentation
                    while !output_remaining.is_empty() {
                        let line_len =
                            available_width.saturating_sub(5).min(output_remaining.len() as u16);
                        let (part, rest) = output_remaining.split_at(line_len as usize);
                        history_items.push(ListItem::new(Line::from(vec![
                            Span::raw("      "), // Align with text after icon
                            Span::styled(part, Style::default().fg(output_color)),
                        ])));
                        output_remaining = rest;
                    }
                }
            }

            // Create and render the list with auto-scroll
            let history_list =
                List::new(history_items).highlight_style(Style::default().bg(Color::DarkGray));

            // Update scrollbar state
            let total_items = self.command_history.len()
                + self.output_history.iter().filter(|o| !o.is_empty()).count();

            // Make sure we have a valid selection
            if self.history_list_state.selected().is_none() && total_items > 0 {
                self.history_list_state.select(Some(self.scroll_offset));
            }

            // Update scrollbar state with current position
            self.history_scroll_state =
                ScrollbarState::default().content_length(total_items).position(self.scroll_offset);

            f.render_stateful_widget(
                history_list,
                history_inner_area,
                &mut self.history_list_state,
            );

            // Render scrollbar
            let scrollbar = Scrollbar::default()
                .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            f.render_stateful_widget(scrollbar, history_inner_area, &mut self.history_scroll_state);
        }
    }
}
