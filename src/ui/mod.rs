pub mod carousel;
pub mod header;
pub mod menu;
pub mod team_selection;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::{App, MenuItem, Screen};
use header::Header;
use menu::Menu;
use team_selection::TeamSelectionWidget;

/// Top-level draw function — routes to the active screen.
pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::MainMenu => draw_main_menu(frame, app),
        Screen::TeamSelection => draw_team_selection(frame, app),
    }
}

fn draw_main_menu(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(65), // header
            Constraint::Percentage(35), // menu
        ])
        .split(frame.area());

    frame.render_widget(
        Header {
            carousel_offset: app.main_menu.carousel.offset,
            team_order: &app.team_selection.teams,
        },
        chunks[0],
    );
    frame.render_widget(
        Menu {
            items: MenuItem::ALL,
            selected: app.main_menu.selected,
        },
        chunks[1],
    );
}

fn draw_team_selection(frame: &mut Frame, app: &mut App) {
    frame.render_widget(
        TeamSelectionWidget {
            state: &mut app.team_selection,
        },
        frame.area(),
    );
}
