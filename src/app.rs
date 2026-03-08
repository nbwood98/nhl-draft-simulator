use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;

use crate::data::NhlData;
use crate::screen::main_menu::MainMenuState;
use crate::screen::simulate_lottery::SimulateLotteryState;
use crate::screen::team_selection::TeamSelectionState;
use crate::screen::{ScreenAction, ScreenId};

const TICK_MS: u64 = 60;

pub struct App {
    active_screen: ScreenId,
    exit: bool,
    nhl_data: NhlData,
    main_menu: MainMenuState,
    team_selection: TeamSelectionState,
    simulate_lottery: SimulateLotteryState,
}

impl App {
    pub fn new(nhl_data: NhlData) -> Self {
        let draft_order = nhl_data.draft_order().to_vec();
        Self {
            active_screen: ScreenId::MainMenu,
            exit: false,
            main_menu: MainMenuState::default(),
            team_selection: TeamSelectionState::new(draft_order),
            simulate_lottery: SimulateLotteryState::default(),
            nhl_data,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick = Duration::from_millis(TICK_MS);
        loop {
            self.tick();
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(tick)?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.handle_key_event(key);
            }

            if self.exit {
                break;
            }
        }
        Ok(())
    }

    fn tick(&mut self) {
        if self.active_screen == ScreenId::MainMenu {
            self.main_menu.tick();
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        let action = match self.active_screen {
            ScreenId::MainMenu => self.main_menu.handle_key_event(key),
            ScreenId::TeamSelection => self.team_selection.handle_key_event(key),
            ScreenId::SimulateLottery => self
                .simulate_lottery
                .handle_key_event(key, &self.team_selection.team_order),
        };
        match action {
            ScreenAction::GoTo(id) => self.active_screen = id,
            ScreenAction::Quit => self.exit = true,
            ScreenAction::None => {}
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        match self.active_screen {
            ScreenId::MainMenu => {
                self.main_menu
                    .draw(frame, &self.nhl_data, &self.team_selection.team_order);
            }
            ScreenId::TeamSelection => {
                self.team_selection.draw(frame, &self.nhl_data);
            }
            ScreenId::SimulateLottery => {
                self.simulate_lottery
                    .draw(frame, &self.nhl_data, &self.team_selection.team_order);
            }
        }
    }
}
