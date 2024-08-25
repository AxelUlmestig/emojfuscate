pub struct HexStream<I>
where
    I: Iterator<Item = u8>,
{
    iter: I,
}

impl<I> HexStream<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for HexStream<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let m_nybble1 = self.iter.next();
        let m_nybble2 = self.iter.next();

        match (m_nybble1, m_nybble2) {
            (Some(nybble1), Some(nybble2)) => {
                let mut output = 0;

                if nybble1 >= 48 && nybble1 <= 57 {
                    output += 16 * (nybble1 - 48); // 0-9
                } else if nybble1 >= 65 && nybble1 <= 70 {
                    output += 16 * (nybble1 - 65 + 10); // A-F
                } else if nybble1 >= 97 && nybble1 <= 102 {
                    output += 16 * (nybble1 - 97 + 10); // a-f
                } else {
                    panic!("unexpected character: '{}', expected 0-9 or A-F", nybble1);
                }

                if nybble2 >= 48 && nybble2 <= 57 {
                    output += nybble2 - 48; // 0-9
                } else if nybble2 >= 65 && nybble2 <= 70 {
                    output += nybble2 - 65 + 10; // A-F
                } else if nybble2 >= 97 && nybble2 <= 102 {
                    output += nybble2 - 97 + 10; // A-F
                } else {
                    panic!("unexpected character: '{}', expected 0-9 or A-F", nybble2);
                }

                return Some(output);
            }
            (None, None) => return None,
            _ => {
                panic!("expected hex characters to come in pairs of two")
            }
        }
    }
}
