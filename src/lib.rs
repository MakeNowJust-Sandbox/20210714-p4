mod move_sorter;
mod opening_book;
mod position;
mod solver;
mod transposition_table;

pub use opening_book::*;
pub use position::*;
pub use solver::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    extern crate once_cell;
    extern crate wasm_bindgen;

    use once_cell::sync::Lazy;
    use wasm_bindgen::prelude::*;

    use std::io::Cursor;
    use std::sync::Mutex;

    use super::opening_book::*;
    use super::position::*;
    use super::solver::*;

    static SOLVER: Lazy<Mutex<Solver>> = Lazy::new(|| {
        let book_bytes = include_bytes!("../data/7x6.book");
        let mut cursor = Cursor::new(book_bytes);
        let book = OpeningBook::load(&mut cursor).unwrap();
        Mutex::new(Solver::new(Some(book)))
    });

    #[wasm_bindgen]
    pub fn analyze(s: &str) -> Vec<i32> {
        let mut p = Position::new();
        p.play_string(s);
        SOLVER.lock().unwrap().analyze(&p, false)
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
