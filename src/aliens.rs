use crate::{get_index, Duration};
use crate::shot::Shot;
use crate::html_pre::{NUM_COLS, NUM_ROWS, OFFSET, Frame, Drawable, new_frame};

pub const ALIEN_WIDTH: usize = 6;
pub const ALIEN_HEIGHT: usize = 3;

#[derive(PartialEq)]
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

fn get_alien01(alien: AlienType) -> Vec<Vec<char>> {
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

pub struct Alien {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub speed: usize,
    animation: Animation,
    shape: Vec<Vec<char>>,
    //   shots: Vec<Shot>,
}


impl Alien {

    pub fn new(x: usize, y: usize, 
               alien: AlienType, 
               width: usize, height: usize, speed: usize) -> Self {
        Self {
            x, 
            y,                //50
            width,
            height,
            speed,
            animation: Animation::Up,
            shape: get_alien01(alien),
        }
    } //^-- new()

    pub fn shape_update(&mut self) {
        if self.animation == Animation::Down {
            self.animation = Animation::Up;
        } else {
            self.animation = Animation::Down;
        }
    }

    pub fn move_left(&mut self) {
        self.x -= self.speed;
    }
    pub fn move_right(&mut self) {
        self.x += self.speed;
    }
    pub fn move_up(&mut self) {
        if self.y >= OFFSET - self.height/2 {
            self.y -= self.speed;
        }
    }
    pub fn move_down(&mut self) {
        if self.y <= NUM_ROWS - OFFSET - self.height {
            self.y += self.height;
        }
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.speed = speed;
    }

    pub fn set_animation(&mut self, state: Animation) {
        self.animation = state;
    }
} //^-- Alien

impl Default for Alien {
    fn default() -> Self {
        Self::new(
                NUM_COLS / 2 - ALIEN_WIDTH / 2,
                OFFSET + ALIEN_HEIGHT,
                AlienType::Alien01,
                ALIEN_WIDTH,
                ALIEN_HEIGHT,
                1,
        )
    }
}

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
