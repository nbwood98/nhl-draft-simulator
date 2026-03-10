use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::data::NhlData;
use crate::screen::{ScreenAction, ScreenId};
use crate::lottery::simulator::Simulator;
use crate::widget::footer::Footer;
use crate::widget::ranked_table::RankedTable;

#[derive(Debug, Clone, Default)]
pub struct SimulateLotteryState {
    result_teams: Vec<usize>,
    simulated: bool,
}

impl SimulateLotteryState {
    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        team_order: &[usize],
    ) -> ScreenAction {
        match key_event.code {
            KeyCode::Enter => {
                self.simulate(team_order);
                ScreenAction::None
            }
            KeyCode::Char('r' | 'R') => {
                self.reset();
                ScreenAction::None
            }
            KeyCode::Esc | KeyCode::Char('q') => ScreenAction::GoTo(ScreenId::MainMenu),
            _ => ScreenAction::None,
        }
    }

    fn simulate(&mut self, team_order: &[usize]) {
        self.result_teams = Simulator::quick_simulate(team_order.to_vec());
        self.simulated = true;
    }

    fn reset(&mut self) {
        self.result_teams.clear();
        self.simulated = false;
    }

    pub fn draw(&self, frame: &mut Frame, nhl_data: &NhlData, team_order: &[usize]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        frame.render_widget(
            RankedTable {
                title: "Initial Draft Order",
                rank_header: "Finish",
                teams: team_order,
                nhl_data,
                offset: 0,
                cursor: None,
                initial_order: None,
            },
            columns[0],
        );

        self.draw_result_panel(frame, columns[1], nhl_data, team_order);

        frame.render_widget(
            Footer {
                bindings: &[
                    ("Enter", " quick simulate   "),
                    ("R", " reset   "),
                    ("Esc/q", " back"),
                ],
            },
            chunks[1],
        );
    }

    fn draw_result_panel(&self, frame: &mut Frame, area: Rect, nhl_data: &NhlData, team_order: &[usize]) {
        if self.simulated {
            frame.render_widget(
                RankedTable {
                    title: "Lottery Results",
                    rank_header: "Pick",
                    teams: &self.result_teams,
                    nhl_data,
                    offset: 0,
                    cursor: None,
                    initial_order: Some(team_order),
                },
                area,
            );
        } else {
            let block = Block::bordered()
                .border_style(Style::default().fg(Color::Cyan))
                .title(
                    Line::from(Span::styled(
                        " Lottery Results ",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .centered(),
                );

            let inner = block.inner(area);
            frame.render_widget(block, area);

            let v_center = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Length(1),
                    Constraint::Percentage(45),
                ])
                .split(inner);

            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    "Press [Enter] to run the simulation",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )))
                .alignment(Alignment::Center),
                v_center[1],
            );
        }
    }
}
