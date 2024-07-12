use std::collections::HashMap;
use std::str;
use uuid::Uuid;
use uuid::Builder;

#[path = "constants.rs"]
mod constants;

// fundamental traits
pub trait Demojfuscate<A, I>
where
    Self: ConstructFromEmojiStream<I>,
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>
{
    fn demojfuscate(self) -> Result<A, FromEmojiError> where Self : Sized;
}

pub trait ConstructFromEmojiStream<I>
where
    I: Iterator<Item = u8>
{
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<I>;
}

pub trait ConstructFromEmoji<A, I>
where
    I: Iterator<Item = u8>
{
    fn construct_from_emoji(byte_stream : DecodeEmojiToBytes<I>) ->
        Result<(A, DecodeEmojiToBytes<I>), FromEmojiError> where Self : Sized;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FromEmojiError {
    NotEnoughEmoji
}

// implementations
impl<A, B, I> Demojfuscate<B, I> for A
where
    Self: ConstructFromEmojiStream<I>,
    B: ConstructFromEmoji<B, I>,
    I: Iterator<Item = u8>
{
    fn demojfuscate(self) -> Result<B, FromEmojiError> where Self : Sized {
        match B::construct_from_emoji(self.demojfuscate_stream()) {
            Ok((value, _)) => Ok(value),
            Err(err) => Err(err)
        }
    }
}

impl<I : Iterator<Item = u8>> ConstructFromEmojiStream<I> for I
{
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<I> {
        DecodeEmojiToBytes::new(self)
    }
}

impl ConstructFromEmojiStream<std::vec::IntoIter<u8>> for String
{
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<std::vec::IntoIter<u8>> {
        self.into_bytes().into_iter().demojfuscate_stream()
    }
}

/*
impl<I> ConstructFromEmoji<DecodeEmojiToBytes<I>, I> for DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>
{
    fn construct_from_emoji(byte_stream : DecodeEmojiToBytes<I>) -> Result<(DecodeEmojiToBytes<I>, DecodeEmojiToBytes<I>), FromEmojiError> {
        Ok(byte_stream)
    }
}
*/

impl<I> ConstructFromEmoji<Uuid, I> for Uuid
where
    I: Iterator<Item = u8>
{
    fn construct_from_emoji(mut byte_stream : DecodeEmojiToBytes<I>) -> Result<(Uuid, DecodeEmojiToBytes<I>), FromEmojiError> {
        match Builder::from_slice(byte_stream.by_ref().take(16).collect::<Vec<u8>>().as_slice()) {
            Ok(builder) => Ok((builder.into_uuid(), byte_stream)),
            Err(_err) => Err(FromEmojiError::NotEnoughEmoji)
        }
    }
}

impl<I> ConstructFromEmoji<String, I> for String
where
    I: Iterator<Item = u8>
{
    fn construct_from_emoji(mut byte_stream : DecodeEmojiToBytes<I>) -> Result<(String, DecodeEmojiToBytes<I>), FromEmojiError> {
        Ok((String::from_utf8(byte_stream.by_ref().collect::<Vec<u8>>()).unwrap(), byte_stream))
    }
}

impl<I> ConstructFromEmoji<u8, I> for u8
where
    I: Iterator<Item = u8>
{
    fn construct_from_emoji(mut byte_stream : DecodeEmojiToBytes<I>) -> Result<(u8, DecodeEmojiToBytes<I>), FromEmojiError> {
        match byte_stream.next() {
            Some(byte) => Ok((byte, byte_stream)),
            None => Err(FromEmojiError::NotEnoughEmoji)
        }
    }
}

impl<I, A, B> ConstructFromEmoji<(A, B), I> for (A, B)
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
    B: ConstructFromEmoji<B, I>,
{
    fn construct_from_emoji(byte_stream : DecodeEmojiToBytes<I>) -> Result<((A, B), DecodeEmojiToBytes<I>), FromEmojiError> {
        let (first, byte_stream_after_1) =
            match A::construct_from_emoji(byte_stream) {
                Err(err) => return Err(err),
                Ok(result) => result
            };

        let (second, byte_stream_after_2) =
            match B::construct_from_emoji(byte_stream_after_1) {
                Err(err) => return Err(err),
                Ok(result) => result
            };

        return Ok(((first, second), byte_stream_after_2));
    }
}

impl<I, A, B, C> ConstructFromEmoji<(A, B, C), I> for (A, B, C)
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
    B: ConstructFromEmoji<B, I>,
    C: ConstructFromEmoji<C, I>,
{
    fn construct_from_emoji(byte_stream : DecodeEmojiToBytes<I>) -> Result<((A, B, C), DecodeEmojiToBytes<I>), FromEmojiError> {
        let (first, byte_stream_after_1) =
            match A::construct_from_emoji(byte_stream) {
                Err(err) => return Err(err),
                Ok(result) => result
            };

        let (second, byte_stream_after_2) =
            match B::construct_from_emoji(byte_stream_after_1) {
                Err(err) => return Err(err),
                Ok(result) => result
            };

        let (third, byte_stream_after_3) =
            match C::construct_from_emoji(byte_stream_after_2) {
                Err(err) => return Err(err),
                Ok(result) => result
            };

        return Ok(((first, second, third), byte_stream_after_3));
    }
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

            // the stop emoji is used by types whose type is unknown at compile time (e.g. strings
            // or arrays) to indicate that they're done
            if *emoji_value == constants::STOP_EMOJI_VALUE {
                return None
            }

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
