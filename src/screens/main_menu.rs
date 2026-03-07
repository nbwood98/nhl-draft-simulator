use std::time::{Instant};
use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{MenuItem, Screen};
use crate::screens::screen::{ScreenAction, ScreenHandler};

#[derive(Debug)]
pub struct MainMenuState {
    pub selected: usize,
    pub carousel: CarouselState,
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            selected: 0,
            carousel: CarouselState::default(),
        }
    }
}

impl MainMenuState {
    pub fn move_up(&mut self, max: usize) {
        if self.selected > 0 {
            self.selected -= 1;
        }
        let _ = max;
    }

    pub fn move_down(&mut self, max: usize) {
        if self.selected + 1 < max {
            self.selected += 1;
        }
    }
    
    pub fn tick(&mut self) {
        self.carousel.tick();
    }
    
}

#[derive(Debug)]
pub struct CarouselState {
    pub offset: f64,
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
    pub fn tick(&mut self) {
        self.offset += 0.35;
        self.last_tick = Instant::now();
    }
}

impl ScreenHandler for MainMenuState {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up(MenuItem::ALL.len());
                ScreenAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down(MenuItem::ALL.len());
                ScreenAction::None
            }
            KeyCode::Enter => match MenuItem::ALL[self.selected] {
                MenuItem::TeamSelection => ScreenAction::GoTo(Screen::TeamSelection),
                MenuItem::Quit => ScreenAction::Quit,
                _ => ScreenAction::None,
            },
            KeyCode::Char('q') | KeyCode::Esc => ScreenAction::Quit,
            _ => ScreenAction::None,
        }
    }
}
