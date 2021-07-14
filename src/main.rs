mod move_sorter;
mod opening_book;
mod position;
mod solver;
mod transposition_table;

use std::io;
use std::path::Path;

use opening_book::OpeningBook;
use position::Position;
use solver::Solver;

fn main() -> io::Result<()> {
    let book = OpeningBook::load(Path::new("./7x6.book"))?;
    let mut solver = Solver::new(Some(book));
    let mut p = Position::new();
    p.play_string("");
    println!("{}", p);
    println!("{:?}", solver.analyze(&p, false));
    println!("{}", solver.get_node_count());

    Ok(())
}
