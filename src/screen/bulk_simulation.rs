use std::cmp;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Gauge, Paragraph},
    Frame,
};

use crate::data::NhlData;
use crate::lottery::simulator::Simulator;
use crate::screen::{ScreenAction, ScreenId};
use crate::widget::footer::Footer;
use crate::widget::probability_table::ProbabilityTable;

const BATCH_SIZE: u32 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
    Configuring,
    Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfigField {
    SimCount,
    SleepMs,
}

#[derive(Debug, Clone)]
pub struct BulkSimulationState {
    phase: Phase,
    sim_count_input: String,
    sleep_ms_input: String,
    selected_field: ConfigField,
    pick_counts: [[u32; 16]; 16],
    completed: u32,
    total: u32,
    sleep_ms: u64,
    last_sim_time: Option<Instant>,
}

impl Default for BulkSimulationState {
    fn default() -> Self {
        Self {
            phase: Phase::Configuring,
            sim_count_input: String::from("1000"),
            sleep_ms_input: String::from("0"),
            selected_field: ConfigField::SimCount,
            pick_counts: [[0; 16]; 16],
            completed: 0,
            total: 0,
            sleep_ms: 0,
            last_sim_time: None,
        }
    }
}

impl BulkSimulationState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn tick(&mut self, team_order: &[usize]) {
        if self.phase != Phase::Running || self.completed >= self.total {
            return;
        }

        let now = Instant::now();
        let sleep = Duration::from_millis(self.sleep_ms);

        if sleep.is_zero() {
            let batch = cmp::min(BATCH_SIZE, self.total - self.completed);
            for _ in 0..batch {
                self.run_one_sim(team_order);
            }
        } else {
            let should_run = match self.last_sim_time {
                Some(last) => now.duration_since(last) >= sleep,
                None => true,
            };
            if should_run {
                self.run_one_sim(team_order);
                self.last_sim_time = Some(now);
            }
        }
    }

    fn run_one_sim(&mut self, team_order: &[usize]) {
        let result = Simulator::quick_simulate(team_order.to_vec());

        for (seed, &team_idx) in team_order.iter().enumerate().take(16) {
            if let Some(pick) = result.iter().position(|&t| t == team_idx)
                && pick < 16
            {
                self.pick_counts[seed][pick] += 1;
            }
        }
        self.completed += 1;
    }

    fn start_simulation(&mut self) {
        let total: u32 = self.sim_count_input.parse().unwrap_or(0);
        let sleep_ms: u64 = self.sleep_ms_input.parse().unwrap_or(0);

        if total == 0 {
            return;
        }

        self.total = total;
        self.sleep_ms = sleep_ms;
        self.pick_counts = [[0; 16]; 16];
        self.completed = 0;
        self.last_sim_time = None;
        self.phase = Phase::Running;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction {
        match self.phase {
            Phase::Configuring => self.handle_config_key(key_event),
            Phase::Running => self.handle_running_key(key_event),
        }
    }

    fn handle_config_key(&mut self, key_event: KeyEvent) -> ScreenAction {
        match key_event.code {
            KeyCode::Up | KeyCode::Down | KeyCode::Tab | KeyCode::BackTab => {
                self.selected_field = match self.selected_field {
                    ConfigField::SimCount => ConfigField::SleepMs,
                    ConfigField::SleepMs => ConfigField::SimCount,
                };
                ScreenAction::None
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.active_input_mut().push(c);
                ScreenAction::None
            }
            KeyCode::Backspace => {
                self.active_input_mut().pop();
                ScreenAction::None
            }
            KeyCode::Enter => {
                self.start_simulation();
                ScreenAction::None
            }
            KeyCode::Esc | KeyCode::Char('q') => ScreenAction::GoTo(ScreenId::MainMenu),
            _ => ScreenAction::None,
        }
    }

    fn handle_running_key(&mut self, key_event: KeyEvent) -> ScreenAction {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => ScreenAction::GoTo(ScreenId::MainMenu),
            _ => ScreenAction::None,
        }
    }

    fn active_input_mut(&mut self) -> &mut String {
        match self.selected_field {
            ConfigField::SimCount => &mut self.sim_count_input,
            ConfigField::SleepMs => &mut self.sleep_ms_input,
        }
    }

    pub fn draw(&self, frame: &mut Frame, nhl_data: &NhlData, team_order: &[usize]) {
        match self.phase {
            Phase::Configuring => self.draw_config(frame),
            Phase::Running => self.draw_running(frame, nhl_data, team_order),
        }
    }

    fn draw_config(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    " Bulk Simulation - Configuration ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let inner = block.inner(chunks[0]);
        frame.render_widget(block, chunks[0]);

        let v_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Percentage(35),
            ])
            .split(inner);

        let form_width = 50;

        self.draw_input_field(
            frame,
            center_horizontal(v_layout[1], form_width),
            "Number of Simulations",
            &self.sim_count_input,
            self.selected_field == ConfigField::SimCount,
        );

        self.draw_input_field(
            frame,
            center_horizontal(v_layout[3], form_width),
            "Sleep Between Sims (ms)",
            &self.sleep_ms_input,
            self.selected_field == ConfigField::SleepMs,
        );

        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Press [Enter] to start",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )))
            .alignment(Alignment::Center),
            v_layout[5],
        );

        frame.render_widget(
            Footer {
                bindings: &[
                    ("↑/↓/Tab", " switch field   "),
                    ("0-9", " input   "),
                    ("Backspace", " delete   "),
                    ("Enter", " start   "),
                    ("Esc/q", " back"),
                ],
            },
            chunks[1],
        );
    }

    fn draw_input_field(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        value: &str,
        selected: bool,
    ) {
        let border_color = if selected { Color::Yellow } else { Color::DarkGray };
        let label_style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let block = Block::bordered()
            .border_style(Style::default().fg(border_color))
            .title(Line::from(Span::styled(
                format!(" {label} "),
                label_style,
            )));

        let display = if selected {
            format!("{value}▏")
        } else {
            value.to_string()
        };

        let paragraph = Paragraph::new(Line::from(Span::styled(
            display,
            Style::default().fg(Color::White),
        )))
        .block(block);

        frame.render_widget(paragraph, area);
    }

    fn draw_running(&self, frame: &mut Frame, nhl_data: &NhlData, team_order: &[usize]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Progress bar
        let ratio = if self.total > 0 {
            self.completed as f64 / self.total as f64
        } else {
            0.0
        };
        let percent = (ratio * 100.0) as u16;
        let label = format!("{}/{} ({}%)", self.completed, self.total, percent);

        let gauge = Gauge::default()
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(Line::from(Span::styled(
                        " Progress ",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ))),
            )
            .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
            .ratio(ratio)
            .label(Span::styled(
                label,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));

        frame.render_widget(gauge, chunks[0]);

        // Probability table
        frame.render_widget(
            ProbabilityTable {
                pick_counts: &self.pick_counts,
                completed: self.completed,
                team_order,
                nhl_data,
            },
            chunks[1],
        );

        // Footer
        let footer_bindings: &[(&str, &str)] = if self.completed >= self.total {
            &[("Esc/q", " back")]
        } else {
            &[("Esc/q", " cancel")]
        };

        frame.render_widget(
            Footer {
                bindings: footer_bindings,
            },
            chunks[2],
        );
    }
}

fn center_horizontal(area: Rect, width: u16) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(area);
    chunks[1]
}
