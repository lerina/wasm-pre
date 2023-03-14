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
use crate::html_pre::{NUM_COLS, NUM_ROWS, Drawable};
use crate::player::*;

pub fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}



#[wasm_bindgen]
extern "C" {
    type Date;

    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;
}

//---------------

//-----------------
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

