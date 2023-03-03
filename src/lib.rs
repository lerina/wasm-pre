mod timer;
mod html_pre;
mod aliens;

use std::fmt;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::aliens::{Alien, AlienType, ALIEN_WIDTH, ALIEN_HEIGHT};
use crate::html_pre::{NUM_COLS, NUM_ROWS, Drawable};


pub fn get_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

fn mk_alien_wave(dir: i8, speed: i8) -> (Vec<Vec<Alien>>, Wave) {
    (vec![ 
       vec![/*Alien::new(21, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(29, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(37, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(45, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1),*/
            Alien::new(53, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 3),
            /*Alien::new(61, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(69, 6, AlienType::Alien05, ALIEN_WIDTH, ALIEN_HEIGHT, 1),*/
            ],
       vec![Alien::new(21, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(29, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(37, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(45, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(53, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(61, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(69, 11, AlienType::Alien04, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            ],
       vec![Alien::new(21, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(29, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(37, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(45, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(53, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(61, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(69, 14, AlienType::Alien03, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            ],
       vec![Alien::new(21, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(29, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(37, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(45, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(53, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(61, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(69, 17, AlienType::Alien02, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            ],
       vec![Alien::new(21, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1), 
            Alien::new(31, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(40, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(49, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(59, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(69, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            Alien::new(78, 20, AlienType::Alien01, ALIEN_WIDTH, ALIEN_HEIGHT, 1),
            ],
    ],
     Wave { dir,
            left_most: 21,
            right_most: 78+ALIEN_WIDTH,
            speed
          }
    )

}//^-- mk_aliens()

#[wasm_bindgen]
extern "C" {
    type Date;

    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;
}


struct Wave {
    dir:i8,
    left_most: usize,
    right_most: usize,
    speed: i8
}

enum Gamestate {
    playing,
    gameover,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
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
            Gamestate::playing => {
                // Updates
                let current = Date::now() as u64;
                let delta = current - self.instant;
                self.instant = Date::now() as u64;

                self.update_aliens(delta);

                //render    
                self.draw_aliens();
            },
            Gamestate::gameover => {
                //temp render just stay there    
                //self.draw_aliens();
            }
        }
        
    } //^--fn tick

    pub fn new() -> Universe {
        let width = NUM_COLS;
        let height = NUM_ROWS;
        
        let instant = Date::now() as u64;
        let (aliens, wave) = mk_alien_wave(-1, 2);
        let gamestate = Gamestate::playing;
        let frames = (0..width * height).map(|_| ' ').collect();

        Universe {
            width,
            height,
            aliens,
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

    fn most_left_right_bottom(&self) -> (usize, usize, usize) {
        let mut left_most = self.width/2; 
        let mut right_most = self.width/2;
        let mut bottom_most = 0;
        for row in &self.aliens {
            for alien in row {
                if alien.x < left_most {
                    left_most = alien.x;                                    
                }
                if alien.x > right_most {
                    right_most = alien.x;                                    
                }
                if alien.y > bottom_most {
                    bottom_most = alien.y;                                    
                }
            }
        }
        
        (left_most, right_most, bottom_most)
    }

    fn mv_wave_down(&mut self) { 
        for row in &mut self.aliens {
            for alien in row {
                alien.y += 1;
            }
         }
    }

    fn update_aliens(&mut self, delta: u64)  {
        let (left_most, right_most, bottom_most) = self.most_left_right_bottom();
        
        // check left bound            check right bound 
        if (left_most <= 6 && self.wave.dir == -1)
        || (right_most >= self.width - 12 && self.wave.dir == 1){
            // change direction
            self.wave.dir *= -1;
        }

        // mv down
        if bottom_most <= self.height -17 {
            self.mv_wave_down();

            for row in &mut self.aliens {
                for alien in row {
                    if alien.shape_update(delta){
                        alien.x += (self.wave.speed * self.wave.dir) as usize; 

              console::log_1(&format!("x: {} y: {}",alien.x, alien.y).into());
                    }
                }
            }
        } else {
          //GAMEOVER
          self.gamestate = Gamestate::gameover;                
        }


    }

    fn alien_within_bound(&self) -> bool {
        for row in &self.aliens {
            for alien in row {
                if alien.x > 0 && alien.x < self.width &&
                   alien.y > 0 && alien.y < self.height - alien.height {
                    return true
                }
            }
        }

        false
    }

    pub fn draw_aliens(&mut self)  {
        for row in &mut self.aliens {
            for alien in row {
                let idx = get_index(self.width, alien.y, alien.x);
                alien.draw(&mut self.frames, idx);
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


