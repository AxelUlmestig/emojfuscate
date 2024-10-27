use arrayvec::ArrayVec;
use paste::paste;
use std::str;
use uuid::Uuid;

use super::constants::{
    ByteInSequence, BITS_IN_A_BYTE, BITS_PER_EMOJI, EMOJI_VALUES, MAX_EMOJI_VALUE,
    START_EMOJI_VALUE, STOP_EMOJI_VALUE,
};

// fundamental traits
pub trait Demojfuscate<A, I>
where
    Self: IsEmojiRepresentation<I>,
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>,
{
    fn demojfuscate(self) -> Result<A, FromEmojiError>
    where
        Self: Sized;
}

pub trait IsEmojiRepresentation<I>
where
    I: Iterator<Item = u8>,
{
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<I>;
}

pub trait ConstructFromEmoji<A, I>
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(A, DecodeEmojiToBytes<I>), FromEmojiError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum FromEmojiError {
    InvalidUtf8,
    InputIsNotAnEmoji(String),
    NotEnoughEmoji,
    UnexpectedInput(String),
    MissingSequenceStart,
    UnexpectedSequenceStart(String),
    UnexpectedSequenceEnd,
}

pub struct DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>,
{
    iter: I,
    accumulated_data: u16,
    defined_bits: u16,
    bits_to_truncate: u16,
    peeked_at: Option<Option<Result<ByteInSequence, FromEmojiError>>>,
}

impl<I> DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            accumulated_data: 0,
            defined_bits: 0,
            bits_to_truncate: 0,
            peeked_at: None,
        }
    }

    // If I were a better rustacean I would have written an implementation of `peek`, Alas...
    pub fn reached_end_of_sequence(&mut self) -> bool {
        match &self.peeked_at {
            Some(Some(Ok(ByteInSequence::SequenceEnd))) => {
                return true;
            }
            Some(_) => {
                return false;
            }
            None => {
                let peeked_at = self.next();

                let reached_end = match peeked_at {
                    Some(Ok(ByteInSequence::SequenceEnd)) => true,
                    _ => false,
                };

                self.peeked_at = Some(peeked_at);

                return reached_end;
            }
        };
    }
}

// implementations
impl<I> Iterator for DecodeEmojiToBytes<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Result<ByteInSequence, FromEmojiError>;
    fn next(&mut self) -> Option<Result<ByteInSequence, FromEmojiError>> {
        match self.peeked_at.take() {
            Some(peeked_at) => {
                return peeked_at;
            }
            None => (),
        };

        loop {
            if self.defined_bits >= BITS_IN_A_BYTE {
                let u16_byte_to_output =
                    self.accumulated_data >> (self.defined_bits - BITS_IN_A_BYTE);
                self.accumulated_data = self.accumulated_data
                    ^ (u16_byte_to_output << (self.defined_bits - BITS_IN_A_BYTE));
                let [byte_to_output, _] = u16_byte_to_output.to_ne_bytes();
                self.defined_bits -= BITS_IN_A_BYTE;

                return Some(Ok(ByteInSequence::Byte(byte_to_output)));
            }

            let emoji = {
                let Some(b) = self.iter.next() else {
                    return None;
                };

                // The first bits of the first byte signify how long (in bytes) the UTF-8 character is.
                // 0b10XXXXXX means a one byte character. 0b110XXXXX means a two byte character and so
                // on up to four bytes.
                let remaining_bytes_in_char = match b & 0b11110000 {
                    0b10000000 => 0,
                    0b11000000 => 1,
                    0b11100000 => 2,
                    0b11110000 => 3,
                    _ => return Some(Err(FromEmojiError::InvalidUtf8)),
                };

                let mut input_bytes = vec![b];
                let mut bytes_after_first: Vec<u8> =
                    self.iter.by_ref().take(remaining_bytes_in_char).collect();

                if bytes_after_first.len() != remaining_bytes_in_char {
                    return Some(Err(FromEmojiError::InvalidUtf8));
                }

                input_bytes.append(&mut bytes_after_first);

                match str::from_utf8(&input_bytes) {
                    Ok(s) => s.chars().nth(0).unwrap(),
                    Err(_) => return Some(Err(FromEmojiError::InvalidUtf8)),
                }
            };

            let emoji_value = match EMOJI_VALUES.get(&emoji) {
                Some(x) => x,
                None => {
                    return Some(Err(FromEmojiError::InputIsNotAnEmoji(format!(
                        "Unexpected input character: {}",
                        emoji
                    ))))
                }
            };

            // the start/stop emoji are used by types whose type is unknown at compile time (e.g.
            // strings) to indicate beginning and end of data with dynamic length
            if *emoji_value == START_EMOJI_VALUE {
                return Some(Ok(ByteInSequence::SequenceStart));
            }

            if *emoji_value == STOP_EMOJI_VALUE {
                return Some(Ok(ByteInSequence::SequenceEnd));
            }

            // emoji beyond 2047 are used to indicate that the next emoji produces too many bits. This
            // happens at the end of the encoded message
            if *emoji_value >= MAX_EMOJI_VALUE {
                self.bits_to_truncate = *emoji_value - MAX_EMOJI_VALUE;
                continue;
            }

            self.accumulated_data = (self.accumulated_data << BITS_PER_EMOJI) | emoji_value;
            self.defined_bits += BITS_PER_EMOJI;

            // TODO: combine this with the above statement
            self.accumulated_data = self.accumulated_data >> self.bits_to_truncate;
            self.defined_bits -= self.bits_to_truncate;
            self.bits_to_truncate = 0;
        }
    }
}

impl<A, B, I> Demojfuscate<B, I> for A
where
    Self: IsEmojiRepresentation<I>,
    B: ConstructFromEmoji<B, I>,
    I: Iterator<Item = u8>,
{
    fn demojfuscate(self) -> Result<B, FromEmojiError>
    where
        Self: Sized,
    {
        match B::construct_from_emoji(self.demojfuscate_stream()) {
            Ok((value, _)) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

impl<I: Iterator<Item = u8>> IsEmojiRepresentation<I> for I {
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<I> {
        DecodeEmojiToBytes::new(self)
    }
}

impl IsEmojiRepresentation<std::vec::IntoIter<u8>> for String {
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<std::vec::IntoIter<u8>> {
        self.into_bytes().into_iter().demojfuscate_stream()
    }
}

impl<'a> IsEmojiRepresentation<core::str::Bytes<'a>> for &'a str {
    fn demojfuscate_stream(self) -> DecodeEmojiToBytes<core::str::Bytes<'a>> {
        self.bytes().into_iter().demojfuscate_stream()
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

// implementations
impl<I> ConstructFromEmoji<(), I> for ()
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<((), DecodeEmojiToBytes<I>), FromEmojiError> {
        Ok(((), byte_stream))
    }
}

impl<I> ConstructFromEmoji<bool, I> for bool
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(bool, DecodeEmojiToBytes<I>), FromEmojiError> {
        match byte_stream.next() {
            Some(Ok(ByteInSequence::Byte(0))) => Ok((false, byte_stream)),
            Some(Ok(ByteInSequence::Byte(1))) => Ok((true, byte_stream)),
            Some(Ok(ByteInSequence::Byte(x))) => Err(FromEmojiError::UnexpectedInput(format!("Received unexpected byte when trying to demojfuscate bool, expected 0 or 1 but received {}", x))),
            Some(Ok(ByteInSequence::SequenceStart)) => Err(FromEmojiError::UnexpectedSequenceStart("When demojfuscating bool".to_string())),
            Some(Ok(ByteInSequence::SequenceEnd)) => Err(FromEmojiError::UnexpectedSequenceEnd),
            Some(Err(err)) => Err(err),
            None => Err(FromEmojiError::NotEnoughEmoji)
        }
    }
}

impl<I> ConstructFromEmoji<char, I> for char
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(char, DecodeEmojiToBytes<I>), FromEmojiError> {
        match <[u8; 4]>::construct_from_emoji(byte_stream) {
            Err(err) => Err(err),
            Ok((bytes, new_byte_stream)) => match char::from_u32(u32::from_be_bytes(bytes)) {
                Some(char) => Ok((char, new_byte_stream)),
                None => Err(FromEmojiError::UnexpectedInput(format!(
                    "Can't create char from u32: {}",
                    u32::from_be_bytes(bytes)
                ))),
            },
        }
    }
}

impl<I> ConstructFromEmoji<u8, I> for u8
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(u8, DecodeEmojiToBytes<I>), FromEmojiError> {
        match byte_stream.next() {
            Some(Ok(ByteInSequence::Byte(byte))) => Ok((byte, byte_stream)),
            Some(Ok(ByteInSequence::SequenceStart)) => Err(
                FromEmojiError::UnexpectedSequenceStart("When demojfuscating u8".to_string()),
            ),
            Some(Ok(ByteInSequence::SequenceEnd)) => Err(FromEmojiError::UnexpectedSequenceEnd),
            Some(Err(err)) => Err(err),
            None => Err(FromEmojiError::NotEnoughEmoji),
        }
    }
}

impl<I> ConstructFromEmoji<u16, I> for u16
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(u16, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 2]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (u16::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<u32, I> for u32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(u32, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (u32::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<u64, I> for u64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(u64, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (u64::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<u128, I> for u128
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(u128, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (u128::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<i8, I> for i8
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(i8, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 1]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (i8::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<i16, I> for i16
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(i16, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 2]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (i16::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<i32, I> for i32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(i32, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (i32::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<i64, I> for i64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(i64, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (i64::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<i128, I> for i128
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(i128, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (i128::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<f32, I> for f32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(f32, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (f32::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<f64, I> for f64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(f64, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (f64::from_be_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<Uuid, I> for Uuid
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(Uuid, DecodeEmojiToBytes<I>), FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream)
            .map(|(bytes, new_byte_stream)| (Uuid::from_bytes(bytes), new_byte_stream))
    }
}

impl<I> ConstructFromEmoji<String, I> for String
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(String, DecodeEmojiToBytes<I>), FromEmojiError> {
        // verify that first item is SequenceStart
        match byte_stream.next() {
            Some(Ok(ByteInSequence::SequenceStart)) => {}
            Some(Err(err)) => return Err(err),
            _ => return Err(FromEmojiError::MissingSequenceStart),
        };

        // collect bytes until we hit SequenceEnd. The SequenceEnd element is dropped by map_while
        let byte_vec_or_err: Result<Vec<u8>, FromEmojiError> = byte_stream
            .by_ref()
            .map_while(|pb| match pb {
                Ok(ByteInSequence::Byte(b)) => Some(Ok(b)),
                Ok(ByteInSequence::SequenceEnd) => None,
                Ok(ByteInSequence::SequenceStart) => {
                    Some(Err(FromEmojiError::UnexpectedSequenceStart(
                        "When demojfuscating String".to_string(),
                    )))
                }
                Err(err) => Some(Err(err)),
            })
            .collect();

        let byte_vec = match byte_vec_or_err {
            Err(err) => return Err(err),
            Ok(byte_vec) => byte_vec,
        };

        let string = match String::from_utf8(byte_vec) {
            Err(_) => {
                return Err(FromEmojiError::UnexpectedInput(
                    "Failed to deserialize to UTF-8 string".to_string(),
                ))
            }
            Ok(string) => string,
        };

        return Ok((string, byte_stream));
    }
}

impl<I, A, const S: usize> ConstructFromEmoji<[A; S], I> for [A; S]
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<([A; S], DecodeEmojiToBytes<I>), FromEmojiError> {
        let mut array_vec = ArrayVec::<A, S>::new();

        for element_index in 0..array_vec.capacity() {
            match A::construct_from_emoji(byte_stream) {
                Err(err) => return Err(err),
                Ok((x, new_byte_stream)) => {
                    byte_stream = new_byte_stream;
                    array_vec.insert(element_index, x);
                }
            };
        }

        return match array_vec.into_inner() {
            Ok(array) => Ok((array, byte_stream)),
            Err(_) => Err(FromEmojiError::NotEnoughEmoji), // this should be impossible
        };
    }
}

impl<I, A> ConstructFromEmoji<Vec<A>, I> for Vec<A>
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(Vec<A>, DecodeEmojiToBytes<I>), FromEmojiError> {
        match byte_stream.next() {
            Some(Ok(ByteInSequence::SequenceStart)) => {}
            Some(Ok(_)) => return Err(FromEmojiError::MissingSequenceStart),
            Some(Err(err)) => return Err(err),
            None => return Err(FromEmojiError::NotEnoughEmoji),
        }

        let mut vec = Vec::new();

        loop {
            if byte_stream.reached_end_of_sequence() {
                byte_stream.next(); // pop off the SequenceEnd value
                return Ok((vec, byte_stream));
            }

            match A::construct_from_emoji(byte_stream) {
                Ok((element, new_byte_stream)) => {
                    byte_stream = new_byte_stream;
                    vec.push(element);
                }
                Err(err) => return Err(err),
            };
        }
    }
}

impl<I, A> ConstructFromEmoji<Option<A>, I> for Option<A>
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<(Option<A>, DecodeEmojiToBytes<I>), FromEmojiError> {
        match u8::construct_from_emoji(byte_stream) {
            Err(err) => Err(err),
            Ok((0, new_byte_stream)) => Ok((None, new_byte_stream)),
            Ok((1, new_byte_stream)) => A::construct_from_emoji(new_byte_stream).map(|(x, bs)| (Some(x), bs)),
            Ok((n, _)) => Err(FromEmojiError::UnexpectedInput(format!("Error parsing Option, expected first byte to be 0 for None or 1 for Some, instead got: {}", n)))
        }
    }
}

impl<I, A, B> ConstructFromEmoji<Result<A, B>, I> for Result<A, B>
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
    B: ConstructFromEmoji<B, I>,
{
    fn construct_from_emoji(
        byte_stream_0: DecodeEmojiToBytes<I>,
    ) -> Result<(Result<A, B>, DecodeEmojiToBytes<I>), FromEmojiError> {
        let (constructor_discriminator, byte_stream_1) =
            match u8::construct_from_emoji(byte_stream_0) {
                Err(err) => return Err(err),
                Ok((n, byte_stream_1)) => (n, byte_stream_1),
            };

        let (m_ok_data, byte_stream_2) = match Option::construct_from_emoji(byte_stream_1) {
            Err(err) => return Err(err),
            Ok((x, byte_stream_2)) => (x, byte_stream_2),
        };

        let (m_err_data, byte_stream_3) = match Option::construct_from_emoji(byte_stream_2) {
            Err(err) => return Err(err),
            Ok((x, byte_stream_3)) => (x, byte_stream_3),
        };

        return match (constructor_discriminator, m_ok_data, m_err_data) {
            // Ok
            (0, None, None) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, no data found when parsing Ok branch".to_string())),
            (0, None, Some(_)) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, only Err data found when parsing Ok branch".to_string())),
            (0, Some(ok_data), None) => Ok((Ok(ok_data), byte_stream_3)),
            (0, Some(_), Some(_)) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, both Ok and Err data found when parsing Ok branch".to_string())),
            // Err
            (1, None, None) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, no data found when parsing Err branch".to_string())),
            (1, None, Some(err_data)) => Ok((Err(err_data), byte_stream_3)),
            (1, Some(_), None) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, only Ok data found when parsing Err branch".to_string())),
            (1, Some(_), Some(_)) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, both Ok and Err data found when parsing Err branch".to_string())),
            // None of the above
            (n, _, _) => Err(FromEmojiError::UnexpectedInput(format!("Error parsing Result, expected first byte to be 0 for Ok or 1 for Err, instead got: {}", n)))
        };
    }
}

impl<I, A> ConstructFromEmoji<(A,), I> for (A,)
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
{
    fn construct_from_emoji(
        byte_stream: DecodeEmojiToBytes<I>,
    ) -> Result<((A,), DecodeEmojiToBytes<I>), FromEmojiError> {
        return match A::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok((result, new_byte_stream)) => Ok(((result,), new_byte_stream)),
        };
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        paste! {
            impl<Iter, $($name),+> ConstructFromEmoji<($($name),+), Iter> for ($($name),+)
            where
                Iter: Iterator<Item = u8>,
                $($name: ConstructFromEmoji<$name, Iter>
                ),+
            {
                fn construct_from_emoji(
                    mut byte_stream: DecodeEmojiToBytes<Iter>,
                ) -> Result<(($($name),+), DecodeEmojiToBytes<Iter>), FromEmojiError> {
                    $(
                    let [<$name:lower>] = match $name::construct_from_emoji(byte_stream) {
                        Err(err) => return Err(err),
                        Ok((result, new_byte_stream)) => {
                            byte_stream = new_byte_stream;
                            result
                        }
                    };

                    )+

                    return Ok((($([<$name:lower>]),+), byte_stream));
                }
            }
        }
    }
}

/*
tuples of 24 elements should be plenty, I can't be bothered to go beyond the letters of the
alphabet as variable names.

The generated code should look something like this:

impl<Iter, A, B, C> ConstructFromEmoji<(A, B, C), Iter> for (A, B, C)
where
    Iter: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, Iter>,
    B: ConstructFromEmoji<B, Iter>,
    C: ConstructFromEmoji<C, Iter>,
{
    fn construct_from_emoji(
        mut byte_stream: DecodeEmojiToBytes<Iter>,
    ) -> Result<((A, B, C), DecodeEmojiToBytes<Iter>), FromEmojiError> {
        let a = match A::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok((result, new_byte_stream)) => {
                byte_stream = new_byte_stream;
                result
            }
        };

        let b = match B::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok((result, new_byte_stream)) => {
                byte_stream = new_byte_stream;
                result
            }
        };

        let c = match C::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok((result, new_byte_stream)) => {
                byte_stream = new_byte_stream;
                result
            }
        };

        return Ok(((a, b, c), byte_stream));
    }
}
*/
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O }
tuple_impls! { A B C D E F G H I J K L M N O P }
tuple_impls! { A B C D E F G H I J K L M N O P Q }
tuple_impls! { A B C D E F G H I J K L M N O P Q R }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
