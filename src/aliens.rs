use crate::Duration;
//use crate::shot::Shot;
use crate::html_pre::{NUM_COLS, NUM_ROWS, OFFSET, Frame, Drawable,};

use crate::timer::Timer;

pub const ALIEN_WIDTH: usize = 6;
pub const ALIEN_HEIGHT: usize = 3;

#[derive(Debug, PartialEq)]
pub enum Animation {
    Up,
    Down,
}

pub enum AlienType {
    Alien01,
    Alien02,
    Alien03,
    Alien04,
    Alien05,
}


fn get_alien(alien: AlienType) -> Vec<Vec<char>> {
    match alien {
        AlienType::Alien01 => {
vec![r#"
 xOOx 
xxxxxx
 /\/\ "#
                    .chars()
                    .collect(),
r#"
 xOOx 
xxxxxx
 \  / "#
                    .chars()
                    .collect(),
            ]
        },
        AlienType::Alien02 => {
vec![r#"
⢀⡴⣿⢦⡀ 
⢈⢝⠭⡫⡁ 
"#
                    .chars()
                    .collect(),
r#"
⢀⡴⣿⢦⡀ 
⠨⡋⠛⢙⠅ 
"#
                    .chars()
                    .collect(),
            ]
        },
        AlienType::Alien03 => {
vec![r#"
⢀⡵⣤⡴⣅ 
⠏⢟⡛⣛⠏⠇
"#
                    .chars()
                    .collect(),
r#"
⣆⡵⣤⡴⣅⡆
⢘⠟⠛⠛⢟ 
"#
                    .chars()
                    .collect(),
            ]
        },
        AlienType::Alien04 => {
vec![r#"
⣴⡶⢿⡿⢶⣦
⠩⣟⠫⠝⣻⠍
      "#
                    .chars()
                    .collect(),
r#"
⣴⡶⢿⡿⢶⣦
⣉⠽⠫⠝⠯⣉
      "#
                    .chars()
                    .collect(),
            ]
        },
        AlienType::Alien05 => {
vec![r#"
⢀⡴⣾⢿⢦⡀
⠉⠻⠋⠙⠟⠉ 
      "#
                    .chars()
                    .collect(),
r#"
      
⢀⡴⡿⣷⢦⡀
⠉⠻⠋⠙⠟⠉
"#
                    .chars()
                    .collect(),
            ]
        },
    }//^--match alien
}//^--fn get_alien()


#[derive(Debug, PartialEq)]
pub struct Alien {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub speed: usize,
    animation: Animation,
    shape: Vec<Vec<char>>,
    //   shots: Vec<Shot>,
    timer: Timer,
}

impl Alien {

    pub fn new(x: usize, y: usize, 
               alien: AlienType, 
               width: usize, height: usize, 
               speed: usize) -> Self {

        Self {  x, y,                //50
                width, height,
                speed,
                animation: Animation::Up,
                shape: get_alien(alien),
                timer: Timer::from_millis(160),
        }
    } //^-- new()

    pub fn shape_update(&mut self, delta: u64) { 
        
        self.timer.update(Duration::from_millis(delta));
        if self.timer.ready {
            match self.animation {
                Animation::Down => self.animation = Animation::Up,
                Animation::Up => self.animation = Animation::Down,
            }
            self.timer.reset();
        }
    }

}//^--impl Alien
//-----------------------------------

impl Drawable for Alien {
    fn draw(&self, frame: &mut Frame, idx: usize) {
        let mut idx = idx;
        let new_line = NUM_COLS; //- self.width ;

        let current: usize = match self.animation {
            Animation::Up => 0,
            Animation::Down => 1,
        };

        for lines in self.shape[current].as_slice().chunks(self.width + 1) {
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
        
    } //^-- Draw

} //^-- impl Drawable for Alien      

