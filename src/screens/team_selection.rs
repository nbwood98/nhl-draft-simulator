use crate::data::teams::NHL_TEAMS;
use crossterm::event::{KeyCode, KeyEvent};
use crate::app::Screen;
use crate::screens::screen::{ScreenAction, ScreenHandler};

#[derive(Debug, Clone)]
pub struct TeamSelectionState {
    pub teams: Vec<usize>,
    pub cursor: usize,
    pub grabbed: Option<usize>,
    pub offset: usize,
}

impl Default for TeamSelectionState {
    fn default() -> Self {
        Self {
            teams: (0..NHL_TEAMS.len()).collect(),
            cursor: 0,
            grabbed: None,
            offset: 0,
        }
    }
}

impl TeamSelectionState {
    pub fn move_up(&mut self) {
        if self.cursor == 0 {
            return;
        }
        if self.grabbed.is_some() {
            self.teams.swap(self.cursor, self.cursor - 1);
        }
        self.cursor -= 1;
        self.clamp_offset(20);
    }

    pub fn move_down(&mut self) {
        if self.cursor + 1 >= self.teams.len() {
            return;
        }
        if self.grabbed.is_some() {
            self.teams.swap(self.cursor, self.cursor + 1);
        }
        self.cursor += 1;
        self.clamp_offset(20);
    }

    pub fn toggle_grab(&mut self) {
        self.grabbed = if self.grabbed.is_some() {
            None
        } else {
            Some(self.cursor)
        };
    }

    pub fn release(&mut self) {
        self.grabbed = None;
    }

    pub fn is_grabbed(&self) -> bool {
        self.grabbed.is_some()
    }
    
    pub fn sync_offset(&mut self, visible_rows: usize) {
        self.clamp_offset(visible_rows);
    }

    fn clamp_offset(&mut self, visible: usize) {
        if visible == 0 {
            return;
        }
        if self.cursor < self.offset {
            self.offset = self.cursor;
        } else if self.cursor >= self.offset + visible {
            self.offset = self.cursor + 1 - visible;
        }
    }
}

impl ScreenHandler for TeamSelectionState {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction {
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
                self.toggle_grab();
                ScreenAction::None
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                if self.is_grabbed() {
                    self.release();
                    ScreenAction::None
                } else {
                    ScreenAction::GoTo(Screen::MainMenu)
                }
            }
            _ => ScreenAction::None,
        }
    }
}
