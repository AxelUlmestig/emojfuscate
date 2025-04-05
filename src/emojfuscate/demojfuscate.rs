use arrayvec::ArrayVec;
use paste::paste;
use std::str;
use uuid::Uuid;

use super::constants::{
    ByteInSequence, BITS_IN_A_BYTE, BITS_PER_EMOJI, EMOJI_VALUES, MAX_EMOJI_VALUE,
    START_EMOJI_VALUE, STOP_EMOJI_VALUE,
};

/// A trait representing some source of emoji data. This abstraction let's us use both Strings and
/// streams of bytes when demojfuscating data
pub trait IsEmojiRepresentation<I>
where
    I: Iterator<Item = u8>,
{
    fn demojfuscate_byte_stream(self) -> DecodeEmojiToBytes<I>;
}

impl<I: Iterator<Item = u8>> IsEmojiRepresentation<I> for I {
    fn demojfuscate_byte_stream(self) -> DecodeEmojiToBytes<I> {
        DecodeEmojiToBytes::new(self)
    }
}

impl IsEmojiRepresentation<std::vec::IntoIter<u8>> for String {
    fn demojfuscate_byte_stream(self) -> DecodeEmojiToBytes<std::vec::IntoIter<u8>> {
        self.into_bytes().into_iter().demojfuscate_byte_stream()
    }
}

impl<'a> IsEmojiRepresentation<core::str::Bytes<'a>> for &'a str {
    fn demojfuscate_byte_stream(self) -> DecodeEmojiToBytes<core::str::Bytes<'a>> {
        self.bytes().into_iter().demojfuscate_byte_stream()
    }
}

impl<I: Iterator<Item = char>>
    IsEmojiRepresentation<
        std::iter::FlatMap<I, std::vec::IntoIter<u8>, fn(char) -> std::vec::IntoIter<u8>>,
    > for I
{
    fn demojfuscate_byte_stream(
        self,
    ) -> DecodeEmojiToBytes<
        std::iter::FlatMap<I, std::vec::IntoIter<u8>, fn(char) -> std::vec::IntoIter<u8>>,
    > {
        // this feels awfully inefficient, is there a better way to do it?
        self.flat_map(
            (|c| c.to_string().into_bytes().into_iter()) as fn(char) -> std::vec::IntoIter<u8>,
        )
        .demojfuscate_byte_stream()
    }
}

/// A trait representing things that can be constructed from Emoji
pub trait ConstructFromEmoji<A, I>
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(byte_stream: &mut DecodeEmojiToBytes<I>) -> Result<A, FromEmojiError>
    where
        Self: Sized;
}

/// Once you have something that can be constructed from emoji and a source of emoji then you can
/// try to demojfuscate the data
pub trait Demojfuscate<A, I>
where
    Self: IsEmojiRepresentation<I>,
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>,
{
    fn demojfuscate_stream(self) -> DemojfuscateIterator<A, I>;

    fn demojfuscate(self) -> Result<A, FromEmojiError>
    where
        Self: Sized;
}

impl<A, I, X> Demojfuscate<A, I> for X
where
    Self: IsEmojiRepresentation<I>,
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>,
{
    /// Produce a lazy stream `Iterator<Item = Result<A, FromEmojiError>` from which you can keep
    /// taking values until the stream ends or you are satisfied.
    fn demojfuscate_stream(self) -> DemojfuscateIterator<A, I> {
        DemojfuscateIterator {
            iter: self.demojfuscate_byte_stream(),
            past_sequence_start: false,
            reached_sequence_end: false,
            encountered_error: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Try to construct a value from a source of emoji
    fn demojfuscate(self) -> Result<A, FromEmojiError>
    where
        Self: Sized,
    {
        A::construct_from_emoji(&mut self.demojfuscate_byte_stream())
    }
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

/// This holds an iterator that produces bytes that are interpreted as UTF-8 encoded emoji
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

    /// If I were a more competent rustacean I would have written an implementation of `peek`,
    /// Alas...
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

/// A struct that is used to lazily produce more values of A from a source of emoji.
pub struct DemojfuscateIterator<A, I>
where
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>,
{
    iter: DecodeEmojiToBytes<I>,
    past_sequence_start: bool,
    reached_sequence_end: bool,
    encountered_error: bool,
    _phantom: std::marker::PhantomData<A>,
}

impl<A, I> Iterator for DemojfuscateIterator<A, I>
where
    A: ConstructFromEmoji<A, I>,
    I: Iterator<Item = u8>,
{
    type Item = Result<A, FromEmojiError>;
    fn next(&mut self) -> Option<Result<A, FromEmojiError>> {
        if self.reached_sequence_end || self.encountered_error {
            return None;
        }

        if !self.past_sequence_start {
            match self.iter.next() {
                Some(Ok(ByteInSequence::SequenceStart)) => {
                    self.past_sequence_start = true;
                }
                Some(Ok(_)) => {
                    self.encountered_error = true;
                    return Some(Err(FromEmojiError::MissingSequenceStart));
                }
                Some(Err(err)) => {
                    self.encountered_error = true;
                    return Some(Err(err));
                }
                None => {
                    self.encountered_error = true;
                    return Some(Err(FromEmojiError::NotEnoughEmoji));
                }
            }
        }

        if self.iter.reached_end_of_sequence() {
            self.iter.next(); // pop off the SequenceEnd value
            self.reached_sequence_end = true;
            return None;
        }

        match A::construct_from_emoji(&mut self.iter) {
            Ok(value) => return Some(Ok(value)),
            Err(err) => {
                self.encountered_error = true;
                return Some(Err(err));
            }
        };
    }
}

// ConstructFromEmoji implementations
impl<I> ConstructFromEmoji<(), I> for ()
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(_: &mut DecodeEmojiToBytes<I>) -> Result<(), FromEmojiError> {
        Ok(())
    }
}

impl<I> ConstructFromEmoji<bool, I> for bool
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<bool, FromEmojiError> {
        match byte_stream.next() {
            Some(Ok(ByteInSequence::Byte(0))) => Ok(false),
            Some(Ok(ByteInSequence::Byte(1))) => Ok(true),
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<char, FromEmojiError> {
        match <[u8; 4]>::construct_from_emoji(byte_stream) {
            Err(err) => Err(err),
            Ok(bytes) => match char::from_u32(u32::from_be_bytes(bytes)) {
                Some(char) => Ok(char),
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
    fn construct_from_emoji(byte_stream: &mut DecodeEmojiToBytes<I>) -> Result<u8, FromEmojiError> {
        match byte_stream.next() {
            Some(Ok(ByteInSequence::Byte(byte))) => Ok(byte),
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<u16, FromEmojiError> {
        <[u8; 2]>::construct_from_emoji(byte_stream).map(u16::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<u32, I> for u32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<u32, FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream).map(u32::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<u64, I> for u64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<u64, FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream).map(u64::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<u128, I> for u128
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<u128, FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream).map(u128::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<i8, I> for i8
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(byte_stream: &mut DecodeEmojiToBytes<I>) -> Result<i8, FromEmojiError> {
        <[u8; 1]>::construct_from_emoji(byte_stream).map(i8::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<i16, I> for i16
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<i16, FromEmojiError> {
        <[u8; 2]>::construct_from_emoji(byte_stream).map(i16::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<i32, I> for i32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<i32, FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream).map(i32::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<i64, I> for i64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<i64, FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream).map(i64::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<i128, I> for i128
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<i128, FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream).map(i128::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<f32, I> for f32
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<f32, FromEmojiError> {
        <[u8; 4]>::construct_from_emoji(byte_stream).map(f32::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<f64, I> for f64
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<f64, FromEmojiError> {
        <[u8; 8]>::construct_from_emoji(byte_stream).map(f64::from_be_bytes)
    }
}

impl<I> ConstructFromEmoji<Uuid, I> for Uuid
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<Uuid, FromEmojiError> {
        <[u8; 16]>::construct_from_emoji(byte_stream).map(Uuid::from_bytes)
    }
}

impl<I> ConstructFromEmoji<String, I> for String
where
    I: Iterator<Item = u8>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<String, FromEmojiError> {
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

        return Ok(string);
    }
}

impl<I, A, const S: usize> ConstructFromEmoji<[A; S], I> for [A; S]
where
    I: Iterator<Item = u8>,
    A: ConstructFromEmoji<A, I>,
{
    fn construct_from_emoji(
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<[A; S], FromEmojiError> {
        let mut array_vec = ArrayVec::<A, S>::new();

        for element_index in 0..array_vec.capacity() {
            match A::construct_from_emoji(byte_stream) {
                Err(err) => return Err(err),
                Ok(x) => {
                    array_vec.insert(element_index, x);
                }
            };
        }

        return match array_vec.into_inner() {
            Ok(array) => Ok(array),
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<Vec<A>, FromEmojiError> {
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
                return Ok(vec);
            }

            match A::construct_from_emoji(byte_stream) {
                Ok(element) => {
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<Option<A>, FromEmojiError> {
        match u8::construct_from_emoji(byte_stream) {
            Err(err) => Err(err),
            Ok(0) => Ok(None),
            Ok(1) => A::construct_from_emoji(byte_stream).map(Some),
            Ok(n) => Err(FromEmojiError::UnexpectedInput(format!("Error parsing Option, expected first byte to be 0 for None or 1 for Some, instead got: {}", n)))
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<Result<A, B>, FromEmojiError> {
        let constructor_discriminator = match u8::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(n) => n,
        };

        let m_ok_data = match Option::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(x) => x,
        };

        let m_err_data = match Option::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(x) => x,
        };

        return match (constructor_discriminator, m_ok_data, m_err_data) {
            // Ok
            (0, None, None) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, no data found when parsing Ok branch".to_string())),
            (0, None, Some(_)) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, only Err data found when parsing Ok branch".to_string())),
            (0, Some(ok_data), None) => Ok(Ok(ok_data)),
            (0, Some(_), Some(_)) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, both Ok and Err data found when parsing Ok branch".to_string())),
            // Err
            (1, None, None) => Err(FromEmojiError::UnexpectedInput("Error parsing Result, no data found when parsing Err branch".to_string())),
            (1, None, Some(err_data)) => Ok(Err(err_data)),
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
        byte_stream: &mut DecodeEmojiToBytes<I>,
    ) -> Result<(A,), FromEmojiError> {
        return match A::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(result) => Ok((result,)),
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
                    byte_stream: &mut DecodeEmojiToBytes<Iter>,
                ) -> Result<($($name),+), FromEmojiError> {
                    $(
                    let [<$name:lower>] = match $name::construct_from_emoji(byte_stream) {
                        Err(err) => return Err(err),
                        Ok(result) => {
                            result
                        }
                    };

                    )+

                    return Ok(($([<$name:lower>]),+));
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
        byte_stream: &mut DecodeEmojiToBytes<Iter>,
    ) -> Result<(A, B, C), FromEmojiError> {
        let a = match A::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(result) => {
                result
            }
        };

        let b = match B::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(result) => {
                result
            }
        };

        let c = match C::construct_from_emoji(byte_stream) {
            Err(err) => return Err(err),
            Ok(result) => {
                result
            }
        };

        return Ok((a, b, c));
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
