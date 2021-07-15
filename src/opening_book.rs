use std::io;
use std::io::prelude::*;

use super::position::*;
use super::transposition_table::*;

const TABLE_SIZE_24: usize = next_prime(1 << 24) as usize;

#[derive(Debug)]
pub struct OpeningBook {
    depth: u64,
    trans_table: TranspositionTable<u8, u8>,
}

impl OpeningBook {
    pub fn load<F: Read>(file: &mut F) -> io::Result<OpeningBook> {
        let mut meta = [0u8; 6];
        file.read_exact(&mut meta)?;
        let [width, height, depth, partial_key_bytes, value_bytes, log_size] = meta;

        if (width as u64) != WIDTH {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid width",
            ));
        }
        if (height as u64) != HEIGHT {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid height",
            ));
        }
        if depth > width * height {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid depth",
            ));
        }
        if partial_key_bytes != 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid internal key size",
            ));
        }
        if value_bytes != 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid value size",
            ));
        }
        if log_size > 40 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to load opening book: invalid log2(size)",
            ));
        }

        let mut trans_table: TranspositionTable<u8, u8> = match log_size {
            24 => TranspositionTable::new::<TABLE_SIZE_24>(),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unimplemented opening book size",
                ))
            }
        };

        file.read_exact(trans_table.keys.as_mut())?;
        file.read_exact(trans_table.values.as_mut())?;

        Ok(OpeningBook {
            depth: depth as u64,
            trans_table: trans_table,
        })
    }

    pub fn get(&self, p: &Position) -> u8 {
        if p.get_moves() > self.depth {
            0
        } else {
            let key3 = p.key3();
            self.trans_table.get(key3)
        }
    }
}
