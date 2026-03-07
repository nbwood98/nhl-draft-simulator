use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;
use crate::data::NhlData;
use crate::screens::{
    main_menu::MainMenuState,
    team_selection::TeamSelectionState,
    ScreenAction, ScreenHandler,
};

pub const TICK_MS: u64 = 60;

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub exit: bool,
    pub main_menu: MainMenuState,
    pub team_selection: TeamSelectionState,
    pub nhl_data: NhlData,
}

impl App {
    pub fn new(nhl_data: NhlData) -> Self {
        let team_count = nhl_data.len();
        Self {
            screen: Screen::MainMenu,
            exit: false,
            main_menu: MainMenuState::default(),
            team_selection: TeamSelectionState::new(team_count),
            nhl_data,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let nhl_data = NhlData::default();
        Self::new(nhl_data)
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick = Duration::from_millis(TICK_MS);
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

    fn active_handler(&mut self) -> &mut dyn ScreenHandler {
        match self.screen {
            Screen::MainMenu => &mut self.main_menu,
            Screen::TeamSelection => &mut self.team_selection,
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let action = self.active_handler().handle_key_event(key_event);
        match action {
            ScreenAction::GoTo(screen) => self.screen = screen,
            ScreenAction::Quit => self.exit = true,
            ScreenAction::None => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    TeamSelection,
}

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
