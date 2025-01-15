#[test]
fn ext() {
    let s = "€1𝄞123";

    panic!("{:X} {:X} {}", char_to_utf8(s.chars().next().unwrap()), next_char(s, 0).0, s.as_bytes().iter().map(|x| format!("{x:X}")).collect::<String>());

}

#[inline(always)]
pub const fn find_next_code_point(s: &str, start: usize) -> usize {
    let marker = s.as_bytes()[0];
    start + code_point_len(marker)
}

#[inline(always)]
const fn code_point_len(marker: u8) -> usize {
    if marker >> 7 == 0 {
        1
    }
    else if marker >> 5 == 0b110 {
        2
    }
    else if marker >> 4 == 0b1110 {
        3
    }
    else if marker >> 3 == 0b11110 {
        4
    }
    else {
        panic!("Malformed UTF-8 codepoint");
    }
}

#[inline(always)]
pub const fn char_to_utf8(c: char) -> u32 {
    let mut tmp = [0; 4];
    c.encode_utf8(&mut tmp);
    u32::from_be_bytes(tmp)
}

#[inline(always)]
pub const fn next_char(s: &str, pos: usize) -> (u32, usize) {
    let next = find_next_code_point(s, pos);
    let bs = s.as_bytes();
    let ret = u32::from_be_bytes(
        match next - pos {
            1 => [bs[0], 0, 0, 0],
            2 => [bs[0], bs[1], 0, 0],
            3 => [bs[0], bs[1], bs[2], 0],
            4 => [bs[0], bs[1], bs[2], bs[3]],
            _ => unreachable!()
        }
    );
    unsafe { (ret, next) }
}

pub struct CharSlice<'a> {
    offset: usize,
    // next_offset: Option<usize>,
    base: &'a str,
}
impl<'a> CharSlice<'a> {
    #[inline(always)]
    pub const fn new(s: &'a str) -> Self {
        Self { offset: 0, base: s }
    }
    #[inline(always)]
    pub const fn is_empty(&self) -> bool { self.offset == self.base.len() }
    // #[inline(always)]
    // pub const fn get_next_offset(&mut self) -> usize {
    //     if let Some(x) = self.next_offset {
    //         x
    //     }
    //     else {
    //         let x = find_next_code_point(self.base, self.offset);
    //         self.next_offset = Some(x);
    //         x
    //     }
    // }

    #[inline(always)]
    pub const fn get_advance(&self) -> ([u8; 4], CharSlice<'a>) {
        let next_offset = find_next_code_point(self.base, self.offset);

        let first = {
            let mut ret = [0; 4];
            let mut j = 0;
            let mut i = self.offset;
            let len = next_offset - self.offset;
            while j < len {
                ret[j] = self.base.as_bytes()[i];
                i += 1;
                j += 1;
            }
            ret
        };

        (first, CharSlice { offset: next_offset, base: self.base })
    }

    // pub const fn first(&mut self) -> [u8; 4] {
    //     let mut ret = [0; 4];
    //     let mut j = 0;
    //     let mut i = self.offset;
    //     let len = self.get_next_offset() - self.offset;
    //     while j < len {
    //         ret[j] = self.base.as_bytes()[i];
    //         i += 1;
    //         j += 1;
    //     }
    //     ret
    // }
    // #[inline(always)]
    // pub const fn new_advance(&mut self) -> CharSlice<'a> {
    //     let next_offset = self.get_next_offset();
    //     CharSlice { offset: next_offset, next_offset: None, base: self.base }
    // }
}