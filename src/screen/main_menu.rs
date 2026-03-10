use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
    Frame,
};

use crate::data::NhlData;
use crate::screen::carousel::Carousel;
use crate::screen::{ScreenAction, ScreenId};
use crate::widget::header_banner::{HeaderBanner, BANNER_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuItem {
    TeamSelection,
    SimulateLottery,
    BulkSimulation,
    Quit,
}

impl MenuItem {
    const ALL: &[MenuItem] = &[
        MenuItem::TeamSelection,
        MenuItem::SimulateLottery,
        MenuItem::BulkSimulation,
        MenuItem::Quit,
    ];

    fn label(self) -> &'static str {
        match self {
            MenuItem::TeamSelection => "Team Selection",
            MenuItem::SimulateLottery => "Simulate Draft Lottery",
            MenuItem::BulkSimulation => "Bulk Simulation",
            MenuItem::Quit => "Quit",
        }
    }
}

#[derive(Debug, Default)]
pub struct MainMenuState {
    selected: usize,
    carousel: CarouselState,
}

impl MainMenuState {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                ScreenAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < MenuItem::ALL.len() {
                    self.selected += 1;
                }
                ScreenAction::None
            }
            KeyCode::Enter => match MenuItem::ALL[self.selected] {
                MenuItem::TeamSelection => ScreenAction::GoTo(ScreenId::TeamSelection),
                MenuItem::SimulateLottery => ScreenAction::GoTo(ScreenId::SimulateLottery),
                MenuItem::BulkSimulation => ScreenAction::GoTo(ScreenId::BulkSimulation),
                MenuItem::Quit => ScreenAction::Quit,
            },
            KeyCode::Char('q') | KeyCode::Esc => ScreenAction::Quit,
            _ => ScreenAction::None,
        }
    }

    pub fn tick(&mut self) {
        self.carousel.tick();
    }

    pub fn draw(&self, frame: &mut Frame, nhl_data: &NhlData, team_order: &[usize]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(frame.area());

        self.draw_header(frame, chunks[0], nhl_data, team_order);
        self.draw_menu(frame, chunks[1]);
    }

    fn draw_header(
        &self,
        frame: &mut Frame,
        area: Rect,
        nhl_data: &NhlData,
        team_order: &[usize],
    ) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(vec![
                    Span::styled("🏒 ", Style::default()),
                    Span::styled(
                        " NHL DRAFT LOTTERY ",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" 🏒", Style::default()),
                ])
                .centered(),
            );

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(BANNER_HEIGHT), Constraint::Min(0)])
            .split(inner);

        frame.render_widget(HeaderBanner, header_chunks[0]);
        frame.render_widget(
            Carousel {
                offset: self.carousel.offset,
                teams: team_order,
                nhl_data,
            },
            header_chunks[1],
        );
    }

    fn draw_menu(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    " Menu ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let items: Vec<ListItem> = MenuItem::ALL
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_quit = *item == MenuItem::Quit;
                let label = if i == self.selected {
                    format!("  ▶  {}  ", item.label())
                } else {
                    format!("     {}  ", item.label())
                };

                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if is_quit {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(Line::from(Span::styled(label, style)).centered())
            })
            .collect();

        let list = List::new(items).block(block);
        let mut state = ListState::default();
        state.select(Some(self.selected));
        frame.render_stateful_widget(list, area, &mut state);
    }
}

#[derive(Debug)]
pub struct CarouselState {
    offset: f64,
    last_tick: Instant,
}

impl Default for CarouselState {
    fn default() -> Self {
        Self {
            offset: 0.0,
            last_tick: Instant::now(),
        }
    }
}

impl CarouselState {
    fn tick(&mut self) {
        self.offset += 0.35;
        self.last_tick = Instant::now();
    }
}
