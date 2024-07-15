
use uuid::Uuid;
use core::array::IntoIter;
use std::collections::VecDeque;
use std::vec::Vec;
use std::iter::{Map, FlatMap, Once, once, Chain};

use super::constants;

pub trait Emojfuscate<I>
where
    I: Iterator<Item = ByteOrBreak>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I>;

    fn emojfuscate(self) -> String where Self: Sized {
        return self.emojfuscate_stream().collect();
    }
}

// implementations
impl<I : Iterator<Item = ByteOrBreak>> Emojfuscate<I> for I
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I> {
        EncodeBytesAsEmoji::new(self)
    }
}

impl<I : Iterator<Item = u8>> Emojfuscate<Map<I, fn(u8) -> ByteOrBreak>> for I
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Map<I, fn(u8) -> ByteOrBreak>> {
        EncodeBytesAsEmoji::new(self.map(wrap_byte))
    }
}

impl Emojfuscate<Chain<Map<std::vec::IntoIter<u8>, fn(u8) -> ByteOrBreak>, Once<ByteOrBreak>>> for String
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Map<std::vec::IntoIter<u8>, fn(u8) -> ByteOrBreak>, Once<ByteOrBreak>>> {
        self.into_bytes().into_iter().emojfuscate_stream().add_stop_emoji()
    }
}

impl Emojfuscate<IntoIter<ByteOrBreak, 16>> for Uuid {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteOrBreak, 16>> {
        EncodeBytesAsEmoji::new(self.into_bytes().map(|b| ByteOrBreak::Byte(b)).into_iter())
    }
}

impl Emojfuscate<Once<ByteOrBreak>> for u8 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Once<ByteOrBreak>> {
        EncodeBytesAsEmoji::new(std::iter::once(ByteOrBreak::Byte(self)))
    }
}

impl Emojfuscate<IntoIter<ByteOrBreak, 2>> for u16 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteOrBreak, 2>> {
        EncodeBytesAsEmoji::new(self.to_be_bytes().map(|b| ByteOrBreak::Byte(b)).into_iter())
    }
}

/*
impl<A, IA> Emojfuscate<IA> for [A; 3]
where
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteOrBreak>
{
    fn emojfuscate_stream(&self) -> EncodeBytesAsEmoji<IA>
    /*where Self: Sized*/ {
        self
            .into_iter()
            .flat_map(get_emojfuscate_iter)
            .emojfuscate_stream()
    }
}
*/

fn get_emojfuscate_iter<A, I>(a : A) -> I
where
    A: Emojfuscate<I>,
    I: Iterator<Item = ByteOrBreak>
{
    a.emojfuscate_stream().iter
}


impl<A, IA> Emojfuscate<Chain<IntoIter<ByteOrBreak, 2>, FlatMap<std::vec::IntoIter<A>, IA, fn(A) -> IA>>> for Vec<A>
where
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteOrBreak>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<IntoIter<ByteOrBreak, 2>, FlatMap<std::vec::IntoIter<A>, IA, fn(A) -> IA>>> {
        let element_count =
            match u16::try_from(self.len()) {
                Ok(count) => count,
                // TODO: is there a more graceful way to handle this?
                Err(err) => panic!("can't emojfuscate lists that are longer than 2^16 elements, {}", err)
            };

        let content_data =
            self
                .into_iter()
                .flat_map(get_emojfuscate_iter as fn(A) -> IA)
                .emojfuscate_stream();

        return element_count
            .emojfuscate_stream()
            .chain_emoji_bytes(content_data);
    }
}

impl<A, B, I1, I2> Emojfuscate<Chain<I1, I2>> for (A, B)
where
    A: Emojfuscate<I1>,
    B: Emojfuscate<I2>,
    I1: Iterator<Item = ByteOrBreak>,
    I2: Iterator<Item = ByteOrBreak>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<I1, I2>> {
        let (a, b) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream());
    }
}

impl<A, B, C, IA, IB, IC> Emojfuscate<Chain<Chain<IA, IB>, IC>> for (A, B, C)
where
    A: Emojfuscate<IA>,
    B: Emojfuscate<IB>,
    C: Emojfuscate<IC>,
    IA: Iterator<Item = ByteOrBreak>,
    IB: Iterator<Item = ByteOrBreak>,
    IC: Iterator<Item = ByteOrBreak>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<IA, IB>, IC>> {
        let (a, b, c) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream())
            .chain_emoji_bytes(c.emojfuscate_stream());
    }
}

pub struct EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteOrBreak>
{
    iter: I,
    input_data : usize,
    defined_bits : u32,
    queued_emoji : VecDeque<char>
}

impl<I> EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteOrBreak>
{
    pub fn new(iter : I) -> Self {
        Self { iter, input_data: 0, defined_bits: 0, queued_emoji: VecDeque::with_capacity(3) }
    }

    pub fn add_stop_emoji(self) -> EncodeBytesAsEmoji<Chain<I, Once<ByteOrBreak>>> {
        EncodeBytesAsEmoji {
            iter: self.iter.chain(once(ByteOrBreak::TemporaryBreak)),
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji
        }
    }

    pub fn chain_emoji_bytes<I2>(self, other : EncodeBytesAsEmoji<I2>) -> EncodeBytesAsEmoji<Chain<I, I2>>
    where
        I2: Iterator<Item = ByteOrBreak>
    {
        EncodeBytesAsEmoji {
            iter: self.iter.chain(other.iter),
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji
        }
    }
}

impl<I> Iterator for EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteOrBreak>
{
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match self.queued_emoji.pop_front() {
            Some(emoji) => return Some(emoji),
            None => {}
        }

        loop {
            let mb = self.iter.next();
            let b = match mb {
                Some(ByteOrBreak::Byte(b)) => b,
                None => break,
                Some(ByteOrBreak::TemporaryBreak) => {
                    let stop_emoji = constants::usize_to_emoji(usize::try_from(constants::STOP_EMOJI_VALUE).unwrap());
                    self.queued_emoji.push_back(stop_emoji);
                    break
                }
            };

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
            let final_emoji = constants::usize_to_emoji(self.input_data << padding);

            self.input_data = 0;

            // push to the front so they get in before the 'stop emoji' if it's set
            self.queued_emoji.push_front(final_emoji);
            self.queued_emoji.push_front(truncate_bits_emoji);
        }

        return self.queued_emoji.pop_front()
    }
}

pub enum ByteOrBreak {
    Byte(u8),
    TemporaryBreak
}

fn wrap_byte(b : u8) -> ByteOrBreak {
    ByteOrBreak::Byte(b)
}

