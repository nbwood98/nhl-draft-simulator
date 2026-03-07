use std::time::{Duration, Instant};

/// How fast the carousel animates — ms per tick.
pub const TICK_MS: u64 = 60;

/// All state owned by the main menu screen, including the carousel animation.
/// The UI layer reads from this; `App` never needs to know the details.
#[derive(Debug)]
pub struct MainMenuState {
    pub selected: usize,
    pub carousel: CarouselState,
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            selected: 0,
            carousel: CarouselState::default(),
        }
    }
}

impl MainMenuState {
    pub fn move_up(&mut self, max: usize) {
        if self.selected > 0 {
            self.selected -= 1;
        }
        let _ = max;
    }

    pub fn move_down(&mut self, max: usize) {
        if self.selected + 1 < max {
            self.selected += 1;
        }
    }

    /// Advance the carousel animation by one frame if enough time has elapsed.
    pub fn tick(&mut self) {
        self.carousel.tick();
    }

    pub fn tick_duration() -> Duration {
        Duration::from_millis(TICK_MS)
    }
}

/// Encapsulates the scrolling offset for the team card carousel.
/// Completely invisible to `App`.
#[derive(Debug)]
pub struct CarouselState {
    /// Continuous horizontal scroll position in terminal columns.
    pub offset: f64,
    last_tick: Instant,
}

impl Default for CarouselState {
    fn default() -> Self {
        Self {
            offset: 0.0,
            last_tick: Instant::now(),
        }
    }
}

impl CarouselState {
    pub fn tick(&mut self) {
        self.offset += 0.35;
        self.last_tick = Instant::now();
    }
}

