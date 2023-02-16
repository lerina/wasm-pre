mod html_pre;
mod player;
mod shot;
mod timer;

use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::html_pre::{NUM_COLS, NUM_ROWS, OFFSET, Frame, Drawable, new_frame};
use crate::player::*;


//--------------
#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    player: Player,
    frames: Vec<char>, // should it be Frame?
}

pub fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

#[wasm_bindgen] // Public methods are exported to JavaScript.
impl Universe {
    pub fn tick(&mut self) {
        //cls
        self.frames = (0..self.width * self.height).map(|_| ' ').collect();

        let (x,y) = self.player.get_pos();
        let idx = get_index(self.width, y, x);
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
        let (x,y) = self.player.get_pos();
        if x - 1 >= PLAYER_OFFSET {
           self.player.set_pos(x-1, y);
        }
    }

    pub fn move_player_right(&mut self) {
        let (x,y) = self.player.get_pos();
        if x + 1 <= NUM_COLS - PLAYER_OFFSET - PLAYER_OFFSET/2{ //self.player.width/2{
         self.player.set_pos(x+1, y);
        }
    }
    pub fn move_player_up(&mut self) {
        let (x,y) = self.player.get_pos();
        if y - 1 >= PLAYER_OFFSET {
         self.player.set_pos(x, y-1);
        }
    }

    pub fn move_player_down(&mut self) {
        let (x,y) = self.player.get_pos();
        if y + 1 <= NUM_ROWS - PLAYER_OFFSET {
            self.player.set_pos(x, y+1);
        }
    }
    //
    pub fn player_shoot(&mut self){
        let (x,y) = self.player.get_pos();
        self.player.shoot(x, y);   
    }

    // DEBUG STUFF
    pub fn player_x(&self) -> usize {
        let (x,_) = self.player.get_pos();
        x
    }

    pub fn player_y(&self) -> usize {
        let (_,y) = self.player.get_pos();
        y
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
