
use std::time::Duration;
use crate::html_pre::{Frame, Drawable};
use crate::timer::Timer;


//--------------
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: u64) {
        self.timer.update(Duration::from_millis(delta));
        if self.timer.ready && !self.exploding {
            //bullet goes up
            if self.y > 0 {
                self.y -= 1;
            }
            
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame, idx: usize) {
        //let idx = idx+2;
        frame[idx] = if self.exploding { '*' } else { '|' };
        //frame[self.x][self.y] = if self.exploding { '*' } else { '|' };
    }
}

