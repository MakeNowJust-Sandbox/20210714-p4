use super::move_sorter::*;
use super::opening_book::*;
use super::position::*;
use super::transposition_table::*;

const TABLE_BIT_SIZE: u32 = 24;
const TABLE_SIZE: usize = next_prime(1 << TABLE_BIT_SIZE) as usize;

pub const INVALID_MOVE: i32 = -1000;

#[derive(Debug)]
pub struct Solver {
    trans_table: TranspositionTable<u64, u8>,
    column_order: Vec<u64>,
    node_count: u64,
    book: Option<OpeningBook>,
}

impl Solver {
    pub fn new(book: Option<OpeningBook>) -> Solver {
        let trans_table = TranspositionTable::new::<TABLE_SIZE>();
        let mut column_order: Vec<u64> = Vec::new();
        // initialize the column exploration order, starting with center columns
        // example for WIDTH=7: columnOrder = {3, 2, 4, 1, 5, 0, 6}
        for i in 0..(WIDTH as i32) {
            column_order.push(((WIDTH as i32) / 2 + (1i32 - 2 * (i % 2)) * (i + 1) / 2) as u64)
        }

        Solver {
            trans_table: trans_table,
            column_order: column_order,
            node_count: 0,
            book: book,
        }
    }

    pub fn reset_node_count(&mut self) {
        self.node_count = 0;
    }

    pub fn get_node_count(&self) -> u64 {
        self.node_count
    }

    pub fn analyze(&mut self, p: &Position, weak: bool) -> Vec<i32> {
        let mut scores = vec![INVALID_MOVE; WIDTH as usize];
        for column in 0..WIDTH {
            if p.can_play_column(column) {
                if p.is_winning_column(column) {
                    scores[column as usize] =
                        ((WIDTH as i32) * (HEIGHT as i32) + 1 - (p.get_moves() as i32)) / 2;
                } else {
                    let mut p2 = p.clone();
                    p2.play_column(column);
                    scores[column as usize] = -self.solve(&p2, weak);
                }
            }
        }
        scores
    }

    pub fn solve(&mut self, p: &Position, weak: bool) -> i32 {
        let (mut min, mut max) = if weak {
            (-1, 1)
        } else {
            let min = -((WIDTH as i32) * (HEIGHT as i32) - (p.get_moves() as i32)) / 2;
            let max = ((WIDTH as i32) * (HEIGHT as i32) + 1 - (p.get_moves() as i32)) / 2;
            (min, max)
        };

        // check if win in one move as the Negamax function does not support this case.
        if p.can_win_next() {
            return max;
        }

        // iteratively narrow the min-max exploration window
        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2;
            } else if med >= 0 && max / 2 > med {
                med = max / 2;
            }
            // use a null depth window to know if the actual score is greater or smaller than med
            let score = self.negamax(p, med, med + 1);
            if score <= med {
                max = score;
            } else {
                min = score;
            }
        }

        min
    }

    fn negamax(&mut self, p: &Position, alpha: i32, beta: i32) -> i32 {
        let (mut alpha, mut beta) = (alpha, beta);

        // increment counter of explored nodes
        self.node_count += 1;

        // if no possible non losing move, opponent wins next move
        let possible = p.possible_non_losing_moves();
        if possible == 0 {
            return -((WIDTH as i32) * (HEIGHT as i32) - (p.get_moves() as i32)) / 2;
        }

        // check for draw game
        if p.get_moves() >= WIDTH * HEIGHT {
            return 0;
        }

        // lower bound of score as opponent cannot win next move
        let min = -((WIDTH as i32) * (HEIGHT as i32) - 2 - (p.get_moves() as i32)) / 2;
        if alpha < min {
            // there is no need to keep alpha below our max possible score.
            alpha = min;
            // prune the exploration if the [alpha;beta] window is empty.
            if alpha >= beta {
                return alpha;
            }
        }

        // upper bound of our score as we cannot win immediately
        let max = ((WIDTH as i32) * (HEIGHT as i32) - 1 - (p.get_moves() as i32)) / 2;
        if beta > max {
            // there is no need to keep beta above our max possible score.
            beta = max;
            // prune the exploration if the [alpha;beta] window is empty.
            if alpha >= beta {
                return beta;
            }
        }

        let key = p.key();
        let v = self.trans_table.get(key);
        if v != 0 {
            let value = v as i32;
            if value > MAX_SCORE - MIN_SCORE + 1 {
                // we have an lower bound
                let min = value + 2 * MIN_SCORE - MAX_SCORE - 2;
                if alpha < min {
                    // there is no need to keep beta above our max possible score.
                    alpha = min;
                    // prune the exploration if the [alpha;beta] window is empty.
                    if alpha >= beta {
                        return alpha;
                    }
                }
            } else {
                // we have an upper bound
                let max = value + MIN_SCORE - 1;
                if beta > max {
                    // there is no need to keep beta above our max possible score.
                    beta = max;
                    // prune the exploration if the [alpha;beta] window is empty.
                    if alpha >= beta {
                        return beta;
                    }
                }
            }
        }

        let v3 = self
            .book
            .as_ref()
            .map(|book| book.get(p))
            .unwrap_or_default();
        if v3 != 0 {
            return (v3 as i32) + MIN_SCORE - 1;
        }

        let mut moves = MoveSorter::new();
        for column in (&self.column_order).iter().rev() {
            let mv = possible & column_mask(*column);
            if mv != 0 {
                let score = p.move_score(mv);
                moves.add(mv, score);
            }
        }

        for mv in (&moves).iter() {
            let mut p2 = p.clone();
            // It's opponent turn in P2 position after current player plays x column.
            p2.play(*mv);
            // explore opponent's score within [-beta;-alpha] windows:
            let score = -self.negamax(&p2, -beta, -alpha);
            // no need to have good precision for score better than beta (opponent's score worse than -beta)
            // no need to check for score worse than alpha (opponent's score worse better than -alpha)

            if score >= beta {
                // save the lower bound of the position
                self.trans_table
                    .insert(key, (score + MAX_SCORE - 2 * MIN_SCORE + 2) as u8);
                // prune the exploration if we find a possible move better than what we were looking for.
                return score;
            }
            if score > alpha {
                // reduce the [alpha;beta] window for next exploration, as we only
                // need to search for a position that is better than the best so far.
                alpha = score;
            }
        }

        self.trans_table.insert(key, (alpha - MIN_SCORE + 1) as u8);
        return alpha;
    }
}
