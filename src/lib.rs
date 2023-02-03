use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;


pub type Frame = Vec<char>; //NUM_ROWS * NUM_COLS];

pub fn new_frame() -> Frame {
    vec![' '; NUM_ROWS * NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, idx: usize);
}

//--------------
#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    frames: Vec<char>, // should it be Frame?
}

#[wasm_bindgen] // Public methods are exported to JavaScript.
impl Universe {

    pub fn tick(&mut self) {
        //TODO
    } //^--fn tick


    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;

        let frames = (0..width * height).map(|_| ' ').collect();


        Universe {
            width,
            height,
            frames,
        }
    }//^-- new()

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
