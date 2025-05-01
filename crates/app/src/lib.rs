use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState, ScrollbarState, Scrollbar},
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};
pub mod handler;

pub struct App {
    status: String,
    output: Arc<Mutex<String>>,
    command_history: Vec<String>,
    output_history: Vec<String>,
    history_scroll_state: ScrollbarState,
    history_list_state: ListState,
    history_index: Option<usize>,
    scroll_offset: usize,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        App {
            status: "stopped".to_string(),
            output: Arc::new(Mutex::new(String::new())),
            command_history: Vec::new(),
            output_history: Vec::new(),
            history_scroll_state: ScrollbarState::default(),
            history_list_state: list_state,
            history_index: None,
            scroll_offset: 0,
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
                            // Reset history navigation when typing
                            self.history_index = None;
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
                            let result = self.handle_command(command);
                            
                            // Add output to history
                            let output = self.output.lock().unwrap().clone();
                            if !output.is_empty() {
                                self.output_history.push(output);
                            }

                            // Reset history navigation
                            self.history_index = None;
                            
                            // Update list state to scroll to the bottom
                            let total_items = self.command_history.len() + 
                                self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if total_items > 0 {
                                self.history_list_state.select(Some(total_items - 1));
                                // Ensure we're scrolled to the bottom
                                self.scroll_offset = total_items.saturating_sub(1);
                            }

                            if result.is_err() {
                                break;
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Up => {
                            // Handle command history navigation
                            if !self.command_history.is_empty() {
                                let new_index = match self.history_index {
                                    Some(idx) if idx > 0 => Some(idx - 1),
                                    None => Some(self.command_history.len() - 1),
                                    _ => self.history_index,
                                };
                                
                                if let Some(idx) = new_index {
                                    let mut current_input = input.lock().unwrap();
                                    *current_input = self.command_history[idx].clone();
                                    self.history_index = new_index;
                                }
                            }
                            
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
                            // Handle command history navigation
                            if !self.command_history.is_empty() {
                                if let Some(idx) = self.history_index {
                                    let new_index = if idx < self.command_history.len() - 1 {
                                        Some(idx + 1)
                                    } else {
                                        None
                                    };
                                    
                                    if let Some(idx) = new_index {
                                        let mut current_input = input.lock().unwrap();
                                        *current_input = self.command_history[idx].clone();
                                    } else {
                                        let mut current_input = input.lock().unwrap();
                                        current_input.clear();
                                    }
                                    
                                    self.history_index = new_index;
                                }
                            }
                            
                            // Scroll history down
                            let total_items = self.command_history.len() + 
                                self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if self.scroll_offset < total_items.saturating_sub(1) {
                                self.scroll_offset += 1;
                                if let Some(selected) = self.history_list_state.selected() {
                                    if selected < total_items.saturating_sub(1) {
                                        self.history_list_state.select(Some(selected + 1));
                                    }
                                }
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
                            let total_items = self.command_history.len() + 
                                self.output_history.iter().filter(|o| !o.is_empty()).count();
                            if self.scroll_offset < total_items.saturating_sub(1) {
                                self.scroll_offset = (self.scroll_offset + 5).min(total_items.saturating_sub(1));
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
        // First split the screen into left and right panels
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
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
                    Constraint::Length(3), // Fixed height for input window (1 for content + 2 for borders)
                ]
                .as_ref(),
            )
            .split(horizontal_chunks[0]);

        // Stats panel - use the entire available space
        let stats_area = left_chunks[0];

        // Set status color based on status value
        let status_color = if self.status == "running" {
            Color::Green
        } else {
            Color::Red
        };

        let mut stats_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(self.status.clone(), Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("Transactions: ", Style::default().fg(Color::Yellow)),
                Span::styled("0", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Errors: ", Style::default().fg(Color::Yellow)),
                Span::styled("0", Style::default().fg(Color::Red)),
            ]),
        ];

        let stats_text = Text::from(stats_lines);

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
            .block(Block::default().borders(Borders::ALL).title("Command Input"))
            .wrap(Wrap { trim: true });
        f.render_widget(&input_paragraph, left_chunks[1]);

        // History panel on the right
        let history_block = Block::default()
            .borders(Borders::ALL)
            .title("Command History");
        
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
                history_items.push(ListItem::new(Line::from(vec![
                    Span::styled(format!("> {}", cmd), Style::default().fg(Color::Yellow)),
                ])));
                
                if i < self.output_history.len() && !self.output_history[i].is_empty() {
                    let output_text = &self.output_history[i];
                    let is_error = output_text == "invalid command";
                    
                    // Create styled output text
                    let output_span = if is_error {
                        Span::styled(output_text, Style::default().fg(Color::Red))
                    } else {
                        Span::styled(output_text, Style::default().fg(Color::White))
                    };
                    
                    // Create styled icon
                    let icon_span = if is_error {
                        Span::styled("-", Style::default().fg(Color::Red))
                    } else {
                        Span::styled("+", Style::default().fg(Color::Green))
                    };
                    
                    history_items.push(ListItem::new(Line::from(vec![
                        Span::raw("  ["),
                        icon_span,
                        Span::raw("] "),
                        output_span
                    ])));
                }
            }

            // Create and render the list with auto-scroll
            let history_list = List::new(history_items)
                .highlight_style(Style::default().bg(Color::DarkGray));
            
            // Update scrollbar state
            let total_items = self.command_history.len() + 
                self.output_history.iter().filter(|o| !o.is_empty()).count();
            
            // Make sure we have a valid selection
            if self.history_list_state.selected().is_none() && total_items > 0 {
                self.history_list_state.select(Some(self.scroll_offset));
            }
            
            // Update scrollbar state with current position
            self.history_scroll_state = ScrollbarState::default()
                .content_length(total_items)
                .position(self.scroll_offset);
            
            f.render_stateful_widget(history_list, history_inner_area, &mut self.history_list_state);
            
            // Render scrollbar
            let scrollbar = Scrollbar::default()
                .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
                
            f.render_stateful_widget(
                scrollbar,
                history_inner_area,
                &mut self.history_scroll_state,
            );
        }
    }
}
