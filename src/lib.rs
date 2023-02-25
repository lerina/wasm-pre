mod timer;
mod html_pre;
mod aliens;

use std::fmt;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::console;

use instant::Instant;

use crate::aliens::{Alien, AlienType, ALIEN_WIDTH, ALIEN_HEIGHT};
use crate::html_pre::{NUM_COLS, NUM_ROWS, Drawable};


pub fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

fn mk_aliens() -> Vec<Vec<Alien>> {
    vec![ 
       vec![Alien::new(6, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(14, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(22, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(30, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(38, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
           ],
    ]
}//^-- mk_aliens()



#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    aliens: Vec<Vec<Alien>>,
    instant: Instant,
    frames: Vec<char>, // should it be Frame?
}

#[wasm_bindgen] // Public methods are exported to JavaScript.
impl Universe {
    pub fn tick(&mut self) {
        //cls
        self.frames = (0..self.width * self.height).map(|_| ' ').collect();
        // 
        // Updates
        let delta = self.instant.elapsed();
        self.instant = Instant::now();

        // alien shape_update

        //render
    
        self.draw_aliens();
    } //^--fn tick

    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;
        
        let mut instant = Instant::now();
        let aliens = mk_aliens();  
        let frames = (0..width * height).map(|_| ' ').collect();

        Universe {
            width,
            height,
            aliens,
            instant,
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

    pub fn draw_aliens(&mut self) {
        for row in &mut self.aliens {
            for alien in row {
                let idx = get_index(self.width, alien.y, alien.x);
                alien.draw(&mut self.frames, idx);
                alien.shape_update(self.instant.elapsed());
            }
        }
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


//---TESTS ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_aliens() {
        let expected =     vec![ 
       vec![Alien::new(6, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(14, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(22, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(30, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(38, 6, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
           ]
        ];

        assert_eq!(mk_aliens(), expected);
    }

}


