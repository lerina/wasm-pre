mod timer;
mod html_pre;
mod aliens;
mod player;
mod shot;

use std::fmt;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::aliens::{Alien, AlienType, ALIEN_WIDTH, ALIEN_HEIGHT};
use crate::html_pre::{NUM_COLS, NUM_ROWS, Drawable, Date};
use crate::player::*;

pub fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}



//---------------

#[derive(PartialEq, Debug)]
struct Wave {
    dir:i8,
    speed: i8
}

enum Gamestate {
    Playing,
    Gameover,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    player: Player,
    aliens: Vec<Vec<Alien>>,
    instant: u64,
    wave: Wave,
    gamestate: Gamestate,
    frames: Vec<char>, // should it be Frame?
}


#[wasm_bindgen] // Public methods are exported to JavaScript.
impl Universe {
    pub fn tick(&mut self) {
        //cls
        self.frames = (0..self.width * self.height).map(|_| ' ').collect();
        // 
        match self.gamestate  {
            Gamestate::Playing => {
                // Updates
                let current = Date::now() as u64;
                let delta = current - self.instant;
                self.instant = Date::now() as u64;
                self.player.update(delta);

                // render
                let (x,y) = self.player.get_pos();
                let idx = get_index(self.width, y, x);
                self.player.draw(&mut self.frames, idx);
            },
            Gamestate::Gameover => {
                //temp render just stay there    
                //self.draw_aliens();
            }
        }
        
    } //^--fn tick


    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;
        
        let instant = Date::now() as u64;
        let player = Player::new(NUM_COLS / 2, NUM_ROWS - PLAYER_OFFSET);
        //let (aliens, wave) = mk_alien_wave(-1, 2);
        let wave = Wave{dir: -1, speed: 2};
        let gamestate = Gamestate::Playing;
        let frames = (0..width * height).map(|_| ' ').collect();

        Universe {
            width,
            height,
            player,
            //aliens,
            instant,
            wave,
            gamestate,
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

