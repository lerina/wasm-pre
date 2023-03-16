use wasm_bindgen::prelude::*;

pub const NUM_COLS: usize = 106;
pub const NUM_ROWS: usize = 60;
pub const OFFSET: usize = 5;

pub type Frame = Vec<char>; //NUM_ROWS * NUM_COLS];

pub fn new_frame() -> Frame {
    vec![' '; NUM_ROWS * NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, idx: usize);
}



#[wasm_bindgen]
extern "C" {
    pub type Date;

    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;
}

