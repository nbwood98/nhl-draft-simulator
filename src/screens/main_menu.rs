use std::time::{Instant};

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
    
    pub fn tick(&mut self) {
        self.carousel.tick();
    }
    
}

#[derive(Debug)]
pub struct CarouselState {
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

