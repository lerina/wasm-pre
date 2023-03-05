use crate::{get_index, Duration};
use crate::shot::Shot;
use crate::html_pre::{NUM_COLS, Frame, Drawable}; //NUM_ROWS, OFFSET, new_frame};

pub const PLAYER_OFFSET: usize = 7;

//--------------

pub struct Player {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    shots: Vec<Shot>,
    shape: Vec<char>,
}


impl Player {

    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            width: 5,
            height: 3,
            shots: Vec::new(),
            shape: r#"  O  
 000 
OOOOO"#
                .chars()
                .collect(),
        }
    } //^-- new()

    pub fn set_pos(&mut self, x: usize, y: usize){
        self.x = x;
        self.y = y;        
    }

    pub fn get_pos(&self) -> (usize, usize) {
        
        (self.x, self.y)
    }
    //-------------------
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x + self.width/2, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
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
        }//^--for lines
        
        //---------
        for shot in self.shots.iter() {
            let idx = get_index(NUM_COLS, shot.y, shot.x);
            shot.draw(frame, idx);
        }
    }//^--fn draw
}//^--impl Drawable for Player

