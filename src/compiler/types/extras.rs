use super::*;

#[derive(Default, Debug)]
pub struct LexerExtras {
    prev_offset: Offset,
    offset: Offset,
}

impl LexerExtras {
    #[inline]
    pub fn load(&mut self) {
        self.offset = self.prev_offset;
    }
    #[inline]
    pub fn save(&mut self) {
        self.prev_offset = self.offset;
    }
}

impl Extras<char> for LexerExtras {
    #[inline]
    fn new() -> Self {
        LexerExtras::default()
    }
    #[inline]
    fn clone(&self) -> Self {
        LexerExtras {
            prev_offset: self.prev_offset,
            offset: self.offset,
        }
    }
    #[inline]
    fn reset(&mut self) {
        let def = Offset::default();
        self.prev_offset = def;
        self.offset = def;
    }
    #[inline]
    fn change(&mut self, input: &char, pos: usize) {
        if pos < self.offset.pos {
            self.load(); // == undo
        } else {
            self.save();
            self.offset.pos = pos;
            match *input {
                '\n' => {
                    self.offset.line += 1;
                    self.offset.column = 0;
                }
                _ => self.offset.column += 1,
            }
        }
    }
}
