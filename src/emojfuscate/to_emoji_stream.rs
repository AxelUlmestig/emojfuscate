
use uuid::Uuid;
use core::array::IntoIter;
// use std::vec;
// use std::str::Bytes;

use super::constants;

pub trait ToEmojiStream<I>
where
    I: Iterator<Item = u8>
{
    fn to_emoji_stream(self) -> EncodeBytesAsEmoji<I>;

    fn to_emoji_string(self) -> String where Self: Sized {
        return self.to_emoji_stream().collect();
    }
}

impl<I : Iterator<Item = u8>> ToEmojiStream<I> for I
{
    fn to_emoji_stream(self) -> EncodeBytesAsEmoji<I> { EncodeBytesAsEmoji::new(self) }
}

impl ToEmojiStream<std::vec::IntoIter<u8>> for String
{
    fn to_emoji_stream(self) -> EncodeBytesAsEmoji<std::vec::IntoIter<u8>> { self.into_bytes().into_iter().to_emoji_stream() }
}

impl ToEmojiStream<IntoIter<u8, 16>> for Uuid {
    fn to_emoji_stream(self) -> EncodeBytesAsEmoji<IntoIter<u8, 16>> { EncodeBytesAsEmoji::new(self.into_bytes().into_iter()) }
}

impl ToEmojiStream<std::vec::IntoIter<u8>> for u8 {
    fn to_emoji_stream(self) -> EncodeBytesAsEmoji<std::vec::IntoIter<u8>> {
        EncodeBytesAsEmoji::new(vec![self, 1].into_iter())
    }
}

pub struct EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    iter: I,
    input_data : usize,
    defined_bits : u32,
    final_emoji : Option<char>
}

impl<I> EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    pub fn new(iter : I) -> Self {
        Self { iter, input_data: 0, defined_bits: 0, final_emoji: None }
    }
}

impl<I> Iterator for EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    type Item = char;
    fn next(&mut self) -> Option<char> {
        loop {
            let mb = self.iter.next();
            let Some(b) = mb else { break };

            self.input_data = (self.input_data << constants::BITS_IN_A_BYTE) | usize::from(b);
            self.defined_bits += constants::BITS_IN_A_BYTE;

            if self.defined_bits < constants::BITS_PER_EMOJI {
                continue;
            }

            let bits_used = self.defined_bits - constants::BITS_PER_EMOJI;
            let emoji_index = self.input_data >> bits_used;

            // remove the used bits
            self.input_data = self.input_data ^ (emoji_index << bits_used);
            self.defined_bits -= constants::BITS_PER_EMOJI;

            return Some(constants::usize_to_emoji(emoji_index));
        }

        // If we don't have enough bytes for another emoji we encode the difference in a special
        // emoji and stash away the remaining information so it will be returned on the next next()
        if self.defined_bits > 0 {
            let padding = constants::BITS_PER_EMOJI - self.defined_bits;
            let truncate_bits_emoji = constants::usize_to_emoji(usize::try_from(constants::MAX_EMOJI_VALUE + padding).unwrap());

            self.defined_bits = 0;
            self.final_emoji = Some(constants::usize_to_emoji(self.input_data << padding));

            return Some(truncate_bits_emoji);
        }

        // If we have a stashed final emoji we delete it and return it
        match self.final_emoji {
            None => return None,
            Some(emoji) => {
                self.final_emoji = None;
                return Some(emoji);
            }
        }
    }
}
