use crate::data::teams::NHL_TEAMS;

/// State for the Team Selection screen.
///
/// Teams are ordered by final standing: index 0 = position 1 (best/champion),
/// index 31 = position 32 (worst/last place).
/// Draft lottery odds are derived later by reversing this order.
#[derive(Debug, Clone)]
pub struct TeamSelectionState {
    /// Ordered list of team indices into `NHL_TEAMS`.
    pub teams: Vec<usize>,
    /// Cursor row (0-based).
    pub cursor: usize,
    /// `Some` when the user has grabbed a team for dragging.
    pub grabbed: Option<usize>,
    /// Scroll offset — kept in sync with the visible viewport.
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

    /// Called by the widget once it knows the real viewport height.
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

