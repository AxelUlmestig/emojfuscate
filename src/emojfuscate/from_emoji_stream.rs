use std::collections::HashMap;
use std::str;

#[path = "constants.rs"]
mod constants;

pub trait FromEmojiStream<I>
where
    I: Iterator<Item = u8>
{
    fn from_emoji_stream(self) -> DecodeEmojiToBytes<I>;
}

impl<I : Iterator<Item = u8>> FromEmojiStream<I> for I
{
    fn from_emoji_stream(self) -> DecodeEmojiToBytes<I> { DecodeEmojiToBytes::new(self) }
}

pub struct DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>
{
    iter: I,
    accumulated_data : u32,
    defined_bits : u32,
    accumulated_input_bytes : Vec<u8>,
    bits_to_truncate : u32,
    emoji_values : HashMap <char, u32>
}

impl<I> DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>
{
    pub fn new(iter : I) -> Self {
        Self
            { iter
            , accumulated_data : 0
            , defined_bits : 0
            , accumulated_input_bytes : Vec::new()
            , bits_to_truncate : 0
            , emoji_values :
                HashMap::from_iter(
                    constants::EMOJI
                        .iter()
                        .enumerate()
                        .map(|(i, unicode)| (char::from_u32(*unicode).unwrap(), u32::try_from(i).unwrap()))
                )
            }
    }
}


impl<I> Iterator for DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        loop {
            if self.defined_bits >= constants::BITS_IN_A_BYTE {
                let u32_byte_to_output = self.accumulated_data >> (self.defined_bits - constants::BITS_IN_A_BYTE);
                self.accumulated_data = self.accumulated_data ^ (u32_byte_to_output << (self.defined_bits - constants::BITS_IN_A_BYTE));
                let [byte_to_output, _, _, _] = u32_byte_to_output.to_ne_bytes();
                self.defined_bits -= constants::BITS_IN_A_BYTE;

                return Some(byte_to_output);
            }

            let mb = self.iter.next();
            let Some(b) = mb else { return None };

            self.accumulated_input_bytes.push(b);

            if self.accumulated_input_bytes.len() < 3 {
                continue;
            }

            if self.accumulated_input_bytes.len() > 5 {
                panic!("accumulated_input_bytes.len() > 5");
            }

            let emoji =
                match str::from_utf8(&self.accumulated_input_bytes) {
                    Ok(s) => s.chars().nth(0).unwrap(),
                    Err(_) => continue
                };

            // delete the accumulated bytes
            self.accumulated_input_bytes.truncate(0);


            let emoji_value =
                match self.emoji_values.get(&emoji) {
                    Some(x) => x,
                    None => panic!("Unexpected input character: {}", emoji)
                };

            // emoji beyond 2047 are used to indicate that the next emoji produces too many bits. This
            // happens at the end of the encoded message
            if *emoji_value >= constants::MAX_EMOJI_VALUE {
                self.bits_to_truncate = *emoji_value - constants::MAX_EMOJI_VALUE;
                //println!("emoji: {}, bits_to_truncate: {}", emoji, bits_to_truncate);
                continue;
            }

            self.accumulated_data = (self.accumulated_data << constants::BITS_PER_EMOJI) | emoji_value;
            self.defined_bits += constants::BITS_PER_EMOJI;

            // TODO: combine this with the above statement
            self.accumulated_data = self.accumulated_data >> self.bits_to_truncate;
            self.defined_bits -= self.bits_to_truncate;
            self.bits_to_truncate = 0;
        }
    }
}
