pub mod carousel;
pub mod main_menu;
pub mod simulate_lottery;
pub mod team_selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenId {
    MainMenu,
    TeamSelection,
    SimulateLottery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAction {
    None,
    GoTo(ScreenId),
    Quit,
}
