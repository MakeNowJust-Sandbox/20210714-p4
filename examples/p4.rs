extern crate p4;
extern crate rustyline;
extern crate structopt;

use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::*;

use p4::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "p4")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short, long)]
    weak: bool,

    #[structopt(short, long)]
    analyze: bool,

    #[structopt(short, long, parse(from_os_str))]
    book: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let book = match opt.book {
        Some(path) => {
            let mut file = File::open(path)?;
            Some(OpeningBook::load(&mut file)?)
        }
        None => None,
    };
    let mut solver = Solver::new(book);

    let mut rl = Editor::<()>::new();

    let mut prev = String::new();
    loop {
        let readline = rl.readline_with_initial(">> ", (prev.as_str(), ""));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut p = Position::new();
                p.play_string(&line);
                println!("{}", p);

                solver.reset_node_count();
                let start = Instant::now();

                if opt.analyze {
                    let scores = solver.analyze(&p, opt.weak);
                    for score in scores {
                        if score == INVALID_MOVE {
                            print!("  x ")
                        } else {
                            print!("{:3} ", score);
                        }
                    }
                    println!();
                } else {
                    let score = solver.solve(&p, opt.weak);
                    println!("score: {}", score);
                }

                if opt.verbose {
                    println!();
                    println!("elapsed time: {:.3}", start.elapsed().as_secs_f64());
                    println!("  node count: {}", solver.get_node_count());
                }
                println!();

                prev.clear();
                prev.push_str(line.as_str());
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                let message = format!("Error: {:?}", err);
                return Err(io::Error::new(io::ErrorKind::Other, message));
            }
        }
    }

    Ok(())
}
