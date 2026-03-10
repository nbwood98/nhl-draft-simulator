pub mod bulk_simulation;
pub mod carousel;
pub mod main_menu;
pub mod simulate_lottery;
pub mod team_selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenId {
    MainMenu,
    TeamSelection,
    SimulateLottery,
    BulkSimulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAction {
    None,
    GoTo(ScreenId),
    Quit,
}
