use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::data::NhlData;
use crate::screen::{ScreenAction, ScreenId};
use crate::widget::footer::Footer;
use crate::widget::ranked_table::{CursorInfo, RankedTable};
use crate::widget::scroll::ScrollState;

#[derive(Debug, Clone)]
pub struct TeamSelectionState {
    pub team_order: Vec<usize>,
    scroll: ScrollState,
    grabbed: bool,
}

impl TeamSelectionState {
    pub fn new(team_count: usize) -> Self {
        Self {
            team_order: (0..team_count).collect(),
            scroll: ScrollState::default(),
            grabbed: false,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
                ScreenAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
                ScreenAction::None
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.grabbed = !self.grabbed;
                ScreenAction::None
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                if self.grabbed {
                    self.grabbed = false;
                    ScreenAction::None
                } else {
                    ScreenAction::GoTo(ScreenId::MainMenu)
                }
            }
            _ => ScreenAction::None,
        }
    }

    fn move_up(&mut self) {
        let prev = self.scroll.cursor;
        if self.scroll.move_up() && self.grabbed {
            self.team_order.swap(prev, self.scroll.cursor);
        }
    }

    fn move_down(&mut self) {
        let prev = self.scroll.cursor;
        if self.scroll.move_down(self.team_order.len()) && self.grabbed {
            self.team_order.swap(prev, self.scroll.cursor);
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, nhl_data: &NhlData) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let table_area = chunks[0];
        // bordered block (-2) + table header row (-1)
        let visible_rows = table_area.height.saturating_sub(3) as usize;
        self.scroll.update_viewport(visible_rows);

        frame.render_widget(
            RankedTable {
                title: "Draft Order (Reverse Standings)",
                rank_header: "Finish",
                teams: &self.team_order,
                nhl_data,
                offset: self.scroll.offset,
                cursor: Some(CursorInfo {
                    position: self.scroll.cursor,
                    grabbed: self.grabbed,
                }),
            },
            table_area,
        );

        frame.render_widget(
            Footer {
                bindings: &[
                    ("↑/↓ j/k", " navigate   "),
                    ("Space/Enter", " grab/drop   "),
                    ("Esc/q", " release/back"),
                ],
            },
            chunks[1],
        );
    }
}
