use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::screens::{
    main_menu::MainMenuState,
    team_selection::TeamSelectionState,
};

// ---------------------------------------------------------------------------
// Screen routing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    TeamSelection,
}

// ---------------------------------------------------------------------------
// Menu items
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    TeamSelection,
    Item2,
    Item3,
    Quit,
}

impl MenuItem {
    pub const ALL: &'static [MenuItem] = &[
        MenuItem::TeamSelection,
        MenuItem::Item2,
        MenuItem::Item3,
        MenuItem::Quit,
    ];

    pub fn label(self) -> &'static str {
        match self {
            MenuItem::TeamSelection => "Team Selection",
            MenuItem::Item2 => "Item 2",
            MenuItem::Item3 => "Item 3",
            MenuItem::Quit => "Quit",
        }
    }
}

// ---------------------------------------------------------------------------
// App — owns screen state, handles routing
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub exit: bool,
    pub main_menu: MainMenuState,
    pub team_selection: TeamSelectionState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::MainMenu,
            exit: false,
            main_menu: MainMenuState::default(),
            team_selection: TeamSelectionState::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick = MainMenuState::tick_duration();
        loop {
            terminal.draw(|frame| crate::ui::draw(frame, self))?;

            if event::poll(tick)? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event);
                    }
                }
            }

            if self.exit {
                break;
            }

            if self.screen == Screen::MainMenu {
                self.main_menu.tick();
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.screen {
            Screen::MainMenu => self.handle_main_menu_key(key_event),
            Screen::TeamSelection => self.handle_team_selection_key(key_event),
        }
    }

    fn handle_main_menu_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.main_menu.move_up(MenuItem::ALL.len());
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.main_menu.move_down(MenuItem::ALL.len());
            }
            KeyCode::Enter => self.main_menu_select(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }

    fn main_menu_select(&mut self) {
        match MenuItem::ALL[self.main_menu.selected] {
            MenuItem::TeamSelection => self.screen = Screen::TeamSelection,
            MenuItem::Quit => self.exit = true,
            _ => {}
        }
    }

    fn handle_team_selection_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => self.team_selection.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.team_selection.move_down(),
            KeyCode::Char(' ') | KeyCode::Enter => self.team_selection.toggle_grab(),
            KeyCode::Esc | KeyCode::Char('q') => {
                if self.team_selection.is_grabbed() {
                    self.team_selection.release();
                } else {
                    self.screen = Screen::MainMenu;
                }
            }
            _ => {}
        }
    }
}
