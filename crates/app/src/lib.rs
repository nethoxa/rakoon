use alloy::{
    hex, primitives::Address, signers::k256::ecdsa::SigningKey, transports::http::reqwest::Url,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
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
use runners::{
    Runner, Runner::*, al::ALTransactionRunner, blob::BlobTransactionRunner,
    eip1559::EIP1559TransactionRunner, eip7702::EIP7702TransactionRunner,
    legacy::LegacyTransactionRunner, random::RandomTransactionRunner,
};
use std::{
    collections::HashMap,
    io,
    time::Duration,
};
use tokio::task::JoinHandle;
pub mod errors;
pub mod handler;
pub mod manager;

pub struct App {
    // Whether the app is running or not. It is displayed in the
    // UI to know if there are any `Runners` running.
    running: bool,

    // The global seed all runners will use. If per-runner seeds [nethoxa] TODO this func
    // are used, this won't be used and will dissapear from the UI.
    seed: u64,

    // The private key of the account that will be sending the
    // transactions. The same as with `seed`, if per-runner sk
    // are used, this won't be used and will dissapear from the UI.
    sk: SigningKey,

    // The RPC URL. The same as with `seed` and `sk`, if per-runner
    // RPC URLs are used, this won't be used and will dissapear from
    // the UI.
    rpc_url: Url,

    // The maximum number of operations per mutation.
    max_operations_per_mutation: u64,

    // The output buffer. This is used to store the output of the
    // command that is being executed.
    output: String,

    // The history of commands that have been executed.
    command_history: Vec<String>,

    // The history of outputs that have been produced.
    output_history: Vec<String>,

    // The history of errors that have been produced. This is used
    // to determine the symbol and color to display in the output
    // window.
    error_history: Vec<bool>,

    // The scrollbar widget state.
    history_scroll_state: ScrollbarState,

    // The state of the history list.
    history_list_state: ListState,

    // The scroll offset of the history list. This is used to keep
    // the bottom of the output window always where the last command
    // was executed.
    scroll_offset: usize,

    // The width of the left panel. This is used to widden the output
    // window to show larger information in a more readable way.
    left_panel_width: u16,

    // The height of the input area. This is used as a constant to
    // keep the input area at a fixed height.
    input_height: u16,

    // The handler for each runner. This is used to abort the
    // runner when the user wants to stop the fuzzing process of
    // either a specific runner or all runners.
    handler: HashMap<Runner, JoinHandle<()>>,

    // The random runner.
    random_runner: RandomTransactionRunner,

    // The legacy runner.
    legacy_runner: LegacyTransactionRunner,

    // The AL runner.
    al_runner: ALTransactionRunner,

    // The blob runner.
    blob_runner: BlobTransactionRunner,

    // The EIP-1559 runner.
    eip1559_runner: EIP1559TransactionRunner,

    // The EIP-7702 runner.
    eip7702_runner: EIP7702TransactionRunner,

    // The active runners. This is used to know which runners are
    // currently running and update the information in the UI
    // accordingly.
    active_runners: HashMap<Runner, bool>,

    // The seeds for each runner. This is to have more granular control
    // over the runners.
    runner_seeds: HashMap<Runner, u64>,

    // The private keys for each runner. The same as with `runner_seeds`.
    runner_sks: HashMap<Runner, SigningKey>,

    // The RPC URLs for each runner. The same as with `runner_seeds`.
    runner_rpcs: HashMap<Runner, Url>,
}

impl App {
    /// Creates a new `App` instance.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The URL of the RPC endpoint.
    /// * `sk` - The private key of the account that will be sending the transactions.
    /// * `seed` - The seed to use for the runners.
    pub fn new(rpc_url: Url, sk: SigningKey, seed: u64, max_operations_per_mutation: u64) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        App {
            running: false,
            seed,
            sk: sk.clone(),
            rpc_url: rpc_url.clone(),
            max_operations_per_mutation,
            output: String::new(),
            command_history: Vec::new(),
            output_history: Vec::new(),
            error_history: Vec::new(),
            history_scroll_state: ScrollbarState::default(),
            history_list_state: list_state,
            scroll_offset: 0,
            left_panel_width: 70,
            input_height: 3,
            handler: HashMap::new(),
            random_runner: RandomTransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            legacy_runner: LegacyTransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            al_runner: ALTransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            blob_runner: BlobTransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            eip1559_runner: EIP1559TransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            eip7702_runner: EIP7702TransactionRunner::new(rpc_url.clone(), sk.clone(), seed, max_operations_per_mutation),
            active_runners: HashMap::new(),
            runner_seeds: HashMap::new(),
            runner_sks: HashMap::new(),
            runner_rpcs: HashMap::new(),
        }
    }

    /// This is the entry point for the app. It will start the UI and
    /// handle the input from the user.
    pub async fn run(&mut self) -> Result<(), io::Error> {
        // This is done to make it possible for the TUI to disable by-default
        // behaviors of the terminal. That way, we can build ours from the
        // ground up.
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut input = String::new();
        loop {
            terminal.draw(|f| self.ui(f, &input))?;

            // We check if there is an event in the queue. If there is,
            // we read it and handle it.
            if event::poll(Duration::from_millis(100))? {
                // Pressed key event
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            input.push(c);
                        }

                        // Backspace key event, we remove the last character from the input.
                        KeyCode::Backspace => {
                            input.pop();
                        }

                        // Enter key event, we handle all the stuff the user has typed in.
                        KeyCode::Enter => {
                            let command = input.clone();
                            input.clear();

                            // Add command to history
                            if !command.trim().is_empty() {
                                self.command_history.push(command.clone());
                            }

                            // Handle the command
                            let result = self.handle_command(command).await;

                            // Add output to history
                            if !self.output.is_empty() {
                                self.output_history.push(self.output.clone());
                            }

                            // Update list state to scroll to the bottom
                            let total_items = self.command_history.len()
                                + self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if total_items > 0 {
                                self.history_list_state.select(Some(total_items - 1));
                                // Ensure we're scrolled to the bottom
                                self.scroll_offset = total_items.saturating_sub(1);
                            }

                            if result.is_err() {
                                // This is a hack to exit the app if the command is `exit`.
                                if result.unwrap_err() == AppStatus::Exit {
                                    break;
                                }

                                // Add the error to the history, so that it is displayed as [-]
                                self.error_history.push(true);
                            } else {
                                // Add the success to the history, so that it is displayed as [+]
                                self.error_history.push(false);
                            }
                        }

                        // Escape key event, we exit the app.
                        KeyCode::Esc => {
                            break;
                        }

                        // Up key event, we scroll up the history by one line.
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

                        // Down key event, we scroll down the history by one line.
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

                        // Left key event, we decrease the left panel width by 5.
                        KeyCode::Left => {
                            // Decrease left panel width (minimum 20%)
                            if self.left_panel_width > 20 {
                                self.left_panel_width -= 5;
                            }
                        }

                        // Right key event, we increase the left panel width by 5.
                        KeyCode::Right => {
                            // Increase left panel width (maximum 70%)
                            if self.left_panel_width < 70 {
                                self.left_panel_width += 5;
                            }
                        }

                        // Page up key event, or scroll up from the touchpad, we scroll up the
                        // history by 5 lines. [nethoxa] check this
                        KeyCode::PageUp => {
                            // Scroll history up by multiple lines
                            if self.scroll_offset > 0 {
                                self.scroll_offset = self.scroll_offset.saturating_sub(5);
                                self.history_list_state.select(Some(self.scroll_offset));
                            }
                        }

                        // Page down key event, or scroll down from the touchpad, we scroll down the
                        // history by 5 lines.
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

    /// This is the function that renders the UI.
    ///
    /// # Arguments
    ///
    /// * `f` - The frame to render the UI on.
    /// * `input` - The input from the user.
    fn ui(&mut self, f: &mut Frame, input: &String) {
        // First split the screen into left and right panels using the stored width.
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
        let status_color = if self.running { Color::Green } else { Color::Red };

        // Pretty print status
        let status = if self.running { "Running" } else { "Stopped" };

        // This is the info that will be displayed in the stats panel.
        let stats_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(status, Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("Seed: ", Style::default().fg(Color::Yellow)),
                Span::styled(self.seed.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("RPC: ", Style::default().fg(Color::Yellow)),
                Span::styled(self.rpc_url.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Signer: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("0x{}", hex::encode(self.sk.to_bytes())),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ];

        let mut runners = vec![
            AL, Blob, EIP1559, EIP7702, Legacy, Random,
        ];

        // Add active runners information
        let mut active_runners = Vec::new();
        for (runner, active) in &self.active_runners {
            if *active {
                // Here, if there is no per-runner `seed` or `sk`, we use the `global` seed and
                // `sk`.
                let seed = self.runner_seeds.get(runner).unwrap_or(&self.seed);
                let address =
                    Address::from_private_key(self.runner_sks.get(runner).unwrap_or(&self.sk));
                let rpc = self.runner_rpcs.get(runner).unwrap_or(&self.rpc_url);

                active_runners.push(Line::from(vec![
                    Span::styled(format!("{}: ", runner), Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("seed={}, signer={}, rpc={}, ops={}", seed, address, rpc, self.max_operations_per_mutation),
                        Style::default().fg(Color::Green),
                    ),
                ]));

                runners.remove(runners.iter().position(|r| r == runner).unwrap());
            }
        }

        let mut available_runners = String::new();
        for runner in runners {
            let line = Line::from(vec![
                Span::styled(format!("{}", runner), Style::default().fg(Color::Yellow)),
            ]);
            if !active_runners.contains(&line) {
                available_runners.push_str(&format!("{} ", runner));
            }
        }

        // Build all the lines to be displayed in the stats panel.
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
        all_lines.push(Line::from(vec![
            Span::styled(available_runners, Style::default().fg(Color::DarkGray)),
        ]));

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

        // Create a paragraph with the centered lines.
        let stats_paragraph = Paragraph::new(Text::from(centered_lines))
            .block(Block::default().borders(Borders::ALL).title("Fuzzer Stats"))
            .alignment(Alignment::Center) // Center horizontally
            .wrap(Wrap { trim: true });
        f.render_widget(stats_paragraph, stats_area);

        // Command panel
        let input_text = {
            Text::from(vec![
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(input.clone()),
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

    fn print(&mut self, output: &str) {
        self.output = output.to_string();
    }
}
