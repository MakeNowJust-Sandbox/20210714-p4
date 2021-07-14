use std::fmt;

pub const WIDTH: u64 = 7;
pub const HEIGHT: u64 = 6;

pub const MIN_SCORE: i32 = -((WIDTH * HEIGHT) as i32) / 2 + 3;
pub const MAX_SCORE: i32 = (((WIDTH * HEIGHT) as i32) + 1) / 2 - 3;

#[derive(Clone, Debug)]
pub struct Position {
    current_position: u64, // bitmap of the current_player stones
    mask: u64,             // bitmap of all the already palyed spots
    moves: u64,            // number of moves played since the beinning of the game
}

impl Position {
    pub fn new() -> Position {
        Position {
            current_position: 0,
            mask: 0,
            moves: 0,
        }
    }

    pub fn play_string<S: Into<String>>(&mut self, s: S) {
        for c in s.into().chars() {
            let column = (c as u64) - ('1' as u64);
            self.play_column(column);
        }
    }

    pub fn get_moves(&self) -> u64 {
        self.moves
    }

    pub fn key(&self) -> u64 {
        self.current_position + self.mask
    }

    pub fn key3(&self) -> u64 {
        let mut key_forward = 0u64;
        for i in 0..WIDTH {
            self.partial_key3(&mut key_forward, i);
        }

        let mut key_reverse = 0u64;
        for i in 1..=WIDTH {
            self.partial_key3(&mut key_reverse, (WIDTH as u64) - i);
        }

        if key_forward < key_reverse {
            key_forward / 3
        } else {
            key_reverse / 3
        }
    }

    fn partial_key3(&self, key: &mut u64, column: u64) {
        let mut pos = 1u64 << (column * ((HEIGHT as u64) + 1));
        while pos & self.mask != 0 {
            *key *= 3;
            if pos & self.current_position != 0 {
                *key += 1
            } else {
                *key += 2
            }
            pos <<= 1;
        }
        *key *= 3;
    }

    pub fn play(&mut self, mv: u64) {
        self.current_position ^= self.mask;
        self.mask |= mv;
        self.moves += 1;
    }

    pub fn can_play_column(&self, column: u64) -> bool {
        self.mask & sentinel_column_mask(column) == 0
    }

    pub fn play_column(&mut self, column: u64) {
        let mv = (self.mask + bottom_column_mask(column)) & column_mask(column);
        self.play(mv);
    }

    pub fn can_win_next(&self) -> bool {
        self.winning_position() & self.possible() != 0
    }

    pub fn is_winning_column(&self, column: u64) -> bool {
        self.winning_position() & self.possible() & column_mask(column) != 0
    }

    pub fn possible_non_losing_moves(&self) -> u64 {
        let mut possible_mask = self.possible();
        let opponent_win = self.opponent_winning_position();
        let forced_moves = possible_mask & opponent_win;
        if forced_moves != 0 {
            // check if there is more than one forced move
            // the opponnent has two winning moves and you cannot stop him
            // enforce to play the single forced move
            if forced_moves & (forced_moves - 1) != 0 {
                return 0;
            }
            possible_mask = forced_moves;
        }
        possible_mask & !(opponent_win >> 1)
    }

    pub fn move_score(&self, mv: u64) -> u32 {
        compute_winning_position(self.current_position | mv, self.mask).count_ones()
    }

    fn winning_position(&self) -> u64 {
        compute_winning_position(self.current_position, self.mask)
    }

    fn opponent_winning_position(&self) -> u64 {
        compute_winning_position(self.current_position ^ self.mask, self.mask)
    }

    fn possible(&self) -> u64 {
        (self.mask + BOTTOM_MASK) & BOARD_MASK
    }
}

fn compute_winning_position(position: u64, mask: u64) -> u64 {
    // vertical;
    let mut r = (position << 1) & (position << 2) & (position << 3);

    //horizontal
    let mut p = (position << (HEIGHT + 1)) & (position << 2 * (HEIGHT + 1));
    r |= p & (position << 3 * (HEIGHT + 1));
    r |= p & (position >> (HEIGHT + 1));
    p = (position >> (HEIGHT + 1)) & (position >> 2 * (HEIGHT + 1));
    r |= p & (position << (HEIGHT + 1));
    r |= p & (position >> 3 * (HEIGHT + 1));

    //diagonal 1
    p = (position << HEIGHT) & (position << 2 * HEIGHT);
    r |= p & (position << 3 * HEIGHT);
    r |= p & (position >> HEIGHT);
    p = (position >> HEIGHT) & (position >> 2 * HEIGHT);
    r |= p & (position << HEIGHT);
    r |= p & (position >> 3 * HEIGHT);

    //diagonal 2
    p = (position << (HEIGHT + 2)) & (position << 2 * (HEIGHT + 2));
    r |= p & (position << 3 * (HEIGHT + 2));
    r |= p & (position >> (HEIGHT + 2));
    p = (position >> (HEIGHT + 2)) & (position >> 2 * (HEIGHT + 2));
    r |= p & (position << (HEIGHT + 2));
    r |= p & (position >> 3 * (HEIGHT + 2));

    r & (BOARD_MASK ^ mask)
}

const fn sentinel_column_mask(column: u64) -> u64 {
    1 << ((HEIGHT - 1) + column * (HEIGHT + 1))
}

const fn bottom_column_mask(column: u64) -> u64 {
    1 << column * (HEIGHT + 1)
}

pub const fn column_mask(column: u64) -> u64 {
    ((1 << HEIGHT) - 1) << column * (HEIGHT + 1)
}

const BOTTOM_MASK: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;
const BOARD_MASK: u64 = 0b0111111_0111111_0111111_0111111_0111111_0111111_0111111;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (current_player, opponent) = if self.moves % 2 == 0 {
            ("@", "O")
        } else {
            ("O", "@")
        };
        for i in 1..=HEIGHT {
            for j in 0..WIDTH {
                let position = 1 << j * (HEIGHT + 1) + (HEIGHT - i);
                if self.mask & position != 0 {
                    if self.current_position & position != 0 {
                        write!(f, " {} ", current_player)?;
                    } else {
                        write!(f, " {} ", opponent)?;
                    }
                } else {
                    write!(f, " . ")?;
                }
            }
            write!(f, "\n")?;
        }
        Result::Ok(())
    }
}
