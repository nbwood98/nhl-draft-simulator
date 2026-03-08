#[derive(Debug, Clone, Default)]
pub struct ScrollState {
    pub cursor: usize,
    pub offset: usize,
    viewport_height: usize,
}

impl ScrollState {
    pub fn move_up(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.clamp_offset();
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self, item_count: usize) -> bool {
        if self.cursor + 1 < item_count {
            self.cursor += 1;
            self.clamp_offset();
            true
        } else {
            false
        }
    }

    /// Updates the cached viewport height and re-clamps the offset.
    /// Call this during rendering when the actual area size is known.
    pub fn update_viewport(&mut self, height: usize) {
        if self.viewport_height != height {
            self.viewport_height = height;
            self.clamp_offset();
        }
    }

    fn clamp_offset(&mut self) {
        if self.viewport_height == 0 {
            return;
        }
        if self.cursor < self.offset {
            self.offset = self.cursor;
        } else if self.cursor >= self.offset + self.viewport_height {
            self.offset = self.cursor + 1 - self.viewport_height;
        }
    }
}
