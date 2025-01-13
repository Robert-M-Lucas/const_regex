#[inline(always)]
pub const fn find_next_code_point(s: &str, start: usize) -> usize {
    let marker = s.as_bytes()[0];
    if marker >> 7 == 0 {
        start + 1
    }
    else if marker >> 5 == 0b110 {
        start + 2
    }
    else if marker >> 4 == 0b1110 {
        start + 3
    }
    else if marker >> 3 == 0b11110 {
        start + 4
    }
    else if marker >> 2 == 0b111110 {
        start + 5
    }
    else if marker >> 1 == 0b1111110 {
        start + 6
    }
    else {
        panic!("Malformed UTF-8 codepoint");
    }
}

#[inline(always)]
pub const fn extend_char(c: char) -> [u8; 4] {
    let mut ret = [0; 4];
    c.encode_utf8(&mut ret);
    ret
}

pub struct CharSlice<'a> {
    offset: usize,
    next_offset: Option<usize>,
    base: &'a str,
}
impl<'a> CharSlice<'a> {
    #[inline(always)]
    pub const fn new(s: &'a str) -> Self {
        Self { offset: 0, next_offset: None, base: s }
    }
    #[inline(always)]
    pub const fn is_empty(&self) -> bool { self.offset == self.base.len() }
    #[inline(always)]
    pub const fn get_next_offset(&mut self) -> usize {
        if let Some(x) = self.next_offset {
            x
        }
        else {
            let x = find_next_code_point(self.base, self.offset);
            self.next_offset = Some(x);
            x
        }
    }

    pub const fn first(&mut self) -> [u8; 4] {
        let mut ret = [0; 4];
        let mut j = 0;
        let mut i = self.offset;
        let len = self.get_next_offset() - self.offset;
        while j < len {
            ret[j] = self.base.as_bytes()[i];
            i += 1;
            j += 1;
        }
        ret
    }
    #[inline(always)]
    pub const fn new_advance(&mut self) -> CharSlice<'a> {
        let next_offset = self.get_next_offset();
        CharSlice { offset: next_offset, next_offset: None, base: self.base }
    }
}