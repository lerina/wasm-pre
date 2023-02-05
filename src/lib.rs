use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub const NUM_COLS: usize = 106;
pub const NUM_ROWS: usize = 60;
pub const OFFSET: usize = 5;
pub const PLAYER_OFFSET: usize = 7;

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
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            width: 5,
            height: 3,
            shape: r#"  O  
 000 
OOOOO"#
                .chars()
                .collect(),
        }
    } //^-- new()

    fn set_pos(&mut self, x: usize, y: usize){
        self.x = x;
        self.y = y;        
    }

    fn get_pos(&self) -> (usize, usize) {
        
        (self.x, self.y)
    }
}//^--impl Player

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
                    //see: https://github.com/yewstack/yew/issues/405
                    frame[idx + i] = '\u{00a0}';
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
        //degug
        console::log_1(&format!("{:?}", self.player.get_pos()).into());
    } //^--fn tick

    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;
        let player = Player::new(NUM_COLS / 2, NUM_ROWS - PLAYER_OFFSET);
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
//--------------
    // kb handled in js so we expose player here
    pub fn move_player_left(&mut self) {
        if self.player.x - 1 >= PLAYER_OFFSET {
           self.player.set_pos(self.player.x-1,self.player.y);
        }
    }

    pub fn move_player_right(&mut self) {
        if self.player.x + 1 <= NUM_COLS - PLAYER_OFFSET - self.player.width/2{
         self.player.set_pos(self.player.x+1,self.player.y);
        }
    }
    pub fn move_player_up(&mut self) {
        if self.player.y - 1 >= PLAYER_OFFSET {
         self.player.set_pos(self.player.x,self.player.y-1);
        }
    }

    pub fn move_player_down(&mut self) {
        if self.player.y + 1 <= NUM_ROWS - PLAYER_OFFSET {
            self.player.set_pos(self.player.x,self.player.y+1);
        }
    }

    // DEBUG STUFF
    pub fn player_x(&self) -> usize {
        self.player.x
    }

    pub fn player_y(&self) -> usize {
        self.player.y
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
    fn create_player_with_pos() {
        let player = Player::new(20, 20);
        assert_eq!(player.x, 20);
    }

    #[test]
    fn set_player_pos() {
        let mut player = Player::new(0, 0);
        //player.x = 20;
        //player.y =  25;
        player.set_pos(20, 25);
        assert_eq!(player.x, 20);
        assert_eq!(player.y, 25);
    }

    #[test]
    fn get_player_pos() {
        let player = Player::new(20, 40);
        let (x, y) = player.get_pos();
        assert_eq!(x, 20);
        assert_eq!(y, 40);
    }

}
