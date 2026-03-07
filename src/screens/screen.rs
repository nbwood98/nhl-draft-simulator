use crossterm::event::KeyEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenAction {
    None,
    GoTo(crate::app::Screen),
    Quit,
}

pub trait ScreenHandler {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> ScreenAction;
}

