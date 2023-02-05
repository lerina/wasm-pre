use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub const NUM_COLS: usize = 106;
pub const NUM_ROWS: usize = 60;
pub const OFFSET: usize = 5;
pub const PLAYER_OFFSET: usize = 6;

pub type Frame = Vec<char>; //NUM_ROWS * NUM_COLS];

pub fn new_frame() -> Frame {
    vec![' '; NUM_ROWS * NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, idx: usize);
}

//--------------
#[wasm_bindgen]
pub struct Player {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    shape: Vec<char>,
    //   shots: Vec<Shot>,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,             //90 - width/2
            y: NUM_ROWS - PLAYER_OFFSET, //40 - offset - height/2
            width: 3,
            height: 2,
            shape: r#" O 
000"#
                .chars()
                .collect(),
        }
    } //^-- new()
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame, idx: usize) {
        let mut idx = idx;
        let new_line = NUM_COLS; //- self.width ;
        for lines in self.shape.as_slice().chunks(self.width + 1) {
            //console::log_1(&format!("{:?}",lines).into());
            for (i, s) in lines.iter().enumerate() {
                if *s == '\n' {
                    idx += new_line;
                } else if *s == ' ' {
                    frame[idx + i] = '.';
                } else {
                    frame[idx + i] = *s;
                }
            }
        }

        //        for shot in self.shots.iter() {
        //            shot.draw(frame);
        //        }
    }
}

//--------------
#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    player: Player,
    frames: Vec<char>, // should it be Frame?
}

fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

#[wasm_bindgen] // Public methods are exported to JavaScript.
impl Universe {
    pub fn tick(&mut self) {
        //cls
        self.frames = (0..self.width * self.height).map(|_| ' ').collect();

        let idx = get_index(self.width, self.player.y, self.player.x);
        self.player.draw(&mut self.frames, idx);
    } //^--fn tick

    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;
        let player = Player::new();
        let frames = (0..width * height).map(|_| ' ').collect();

        Universe {
            width,
            height,
            player,
            frames,
        }
    } //^-- new()

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn frames(&self) -> *const char {
        self.frames.as_ptr()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
} //^-- impl Universe

/// used by render()
///
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.frames.as_slice().chunks(self.width) {
            // as usize) {
            for &cell in line {
                // ⬜ U+2B1C □ U+25A1   ⬛ U+2B1B ■U+25A0
                //let symbol = if cell == Cell::Dead { '⬜' } else { '⬛' };
                write!(f, "{}", cell)?; //symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

//---------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
