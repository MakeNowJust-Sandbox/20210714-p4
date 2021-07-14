use super::position::WIDTH;

#[derive(Debug)]
pub struct MoveSorter {
    size: usize,
    entries: [Entry; WIDTH as usize],
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    mv: u64,
    score: u32,
}

impl MoveSorter {
    pub fn new() -> MoveSorter {
        MoveSorter {
            size: 0,
            entries: [Entry { mv: 0, score: 0 }; WIDTH as usize],
        }
    }

    pub fn add(&mut self, mv: u64, score: u32) {
        let mut pos = self.size;
        unsafe {
            while pos > 0 && self.entries.get_unchecked(pos - 1).score > score {
                *self.entries.get_unchecked_mut(pos) = *self.entries.get_unchecked(pos - 1);
                pos -= 1;
            }
            *self.entries.get_unchecked_mut(pos) = Entry {
                mv: mv,
                score: score,
            };
        }
        self.size += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &u64> {
        (&self.entries[0..self.size])
            .iter()
            .rev()
            .map(|entry| &entry.mv)
    }
}
