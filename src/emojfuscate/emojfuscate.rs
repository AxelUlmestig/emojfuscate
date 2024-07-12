
use uuid::Uuid;
use core::array::IntoIter;
use std::collections::VecDeque;
// use std::vec;
// use std::str::Bytes;

use super::constants;

pub trait Emojfuscate<I>
where
    I: Iterator<Item = u8>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I>;

    fn emojfuscate(self) -> String where Self: Sized {
        return self.emojfuscate_stream().collect();
    }
}

impl<I : Iterator<Item = u8>> Emojfuscate<I> for I
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I> { EncodeBytesAsEmoji::new(self) }
}

impl Emojfuscate<std::vec::IntoIter<u8>> for String
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<std::vec::IntoIter<u8>> {
        self.into_bytes().into_iter().emojfuscate_stream().add_stop_emoji()
    }
}

impl Emojfuscate<IntoIter<u8, 16>> for Uuid {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<u8, 16>> { EncodeBytesAsEmoji::new(self.into_bytes().into_iter()) }
}

impl Emojfuscate<std::iter::Once<u8>> for u8 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<std::iter::Once<u8>> {
        EncodeBytesAsEmoji::new(std::iter::once(self))
    }
}

impl<A, B, I1, I2> Emojfuscate<EmojiByteChain<I1, I2>> for (A, B)
where
    A: Emojfuscate<I1>,
    B: Emojfuscate<I2>,
    I1: Iterator<Item = u8>,
    I2: Iterator<Item = u8>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<EmojiByteChain<I1, I2>> {
        let (a, b) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream());
    }
}

impl<A, B, C, IA, IB, IC> Emojfuscate<EmojiByteChain<EmojiByteChain<IA, IB>, IC>> for (A, B, C)
where
    A: Emojfuscate<IA>,
    B: Emojfuscate<IB>,
    C: Emojfuscate<IC>,
    IA: Iterator<Item = u8>,
    IB: Iterator<Item = u8>,
    IC: Iterator<Item = u8>
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<EmojiByteChain<EmojiByteChain<IA, IB>, IC>> {
        let (a, b, c) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream())
            .chain_emoji_bytes(c.emojfuscate_stream());
    }
}

pub struct EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    iter: I,
    input_data : usize,
    defined_bits : u32,
    queued_emoji : VecDeque<char>,
    // final_emoji : Option<char>,
    // TODO: make an enum that describes the possible stop emoji configurations
    stop_emoji_set : bool,
    stop_emoji_armed : bool
}

impl<I> EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    pub fn new(iter : I) -> Self {
        Self { iter, input_data: 0, defined_bits: 0, /*final_emoji: None,*/ stop_emoji_set: false, stop_emoji_armed: false, queued_emoji: VecDeque::with_capacity(3) }
    }

    pub fn add_stop_emoji(mut self) -> Self {
        self.stop_emoji_set = true;
        self.stop_emoji_armed = true;
        return self;
    }

    pub fn chain_emoji_bytes<I2>(self, other : EncodeBytesAsEmoji<I2>) -> EncodeBytesAsEmoji<EmojiByteChain<I, I2>>
    where
        I2: Iterator<Item = u8>
    {
        if self.stop_emoji_set {
            return EncodeBytesAsEmoji::new(EmojiByteChain::new(self, other)).add_stop_emoji();
        } else {
            return EncodeBytesAsEmoji::new(EmojiByteChain::new(self, other));
        }
    }
}

impl<I> Iterator for EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = u8>
{
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match self.queued_emoji.pop_front() {
            Some(emoji) => return Some(emoji),
            None => {}
        }

        /*
        // If we have a stashed final emoji we delete it and return it
        match self.final_emoji {
            None => (),
            Some(emoji) => {
                self.final_emoji = None;
                self.stop_emoji_armed = self.stop_emoji_set;
                return Some(emoji);
            }
        }

        // if the iterator temporarily emitted None then we don't want to ask it for next() again
        // until we've emitted all of the end-of-the-message formatting information, that's why we
        // have to push this all the way up here
        if self.stop_emoji_armed {
            self.stop_emoji_armed = false;
            self.stop_emoji_set = false;
            let stop_emoji = constants::usize_to_emoji(usize::try_from(constants::STOP_EMOJI_VALUE).unwrap());
            return Some(stop_emoji);
        }
        */

        loop {
            let mb = self.iter.next();
            let Some(b) = mb else { break };

            self.stop_emoji_armed = self.stop_emoji_set;

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
            // self.final_emoji = Some(final_emoji);

            self.input_data = 0;

            self.queued_emoji.push_back(truncate_bits_emoji);
            self.queued_emoji.push_back(final_emoji);

            // return Some(truncate_bits_emoji);
        }

        // emit a special emoji if `add_stop_emoji` is set. It's useful if the value's size is not
        // known in compile time
        if self.stop_emoji_armed {
            self.stop_emoji_armed = false;
            // self.stop_emoji_set = false;
            let stop_emoji = constants::usize_to_emoji(usize::try_from(constants::STOP_EMOJI_VALUE).unwrap());
            self.queued_emoji.push_back(stop_emoji);
            // return Some(stop_emoji);
        }

        return self.queued_emoji.pop_front()
        // return None;
    }
}

pub struct EmojiByteChain<I1, I2>
where
    I1: Iterator<Item = u8>,
    I2: Iterator<Item = u8>
{
    iter1: I1,
    iter2: I2,
    break_between_streams: bool,
    break_between_streams_armed: bool
}


impl<I1, I2> EmojiByteChain<I1, I2>
where
    I1: Iterator<Item = u8>,
    I2: Iterator<Item = u8>
{
    pub fn new(emoji_stream_1 : EncodeBytesAsEmoji<I1>, emoji_stream_2 : EncodeBytesAsEmoji<I2>) -> Self {
        Self { iter1 : emoji_stream_1.iter, iter2 : emoji_stream_2.iter, break_between_streams : emoji_stream_1.stop_emoji_set, break_between_streams_armed : emoji_stream_1.stop_emoji_set  }
    }
}

impl<I1, I2> Iterator for EmojiByteChain<I1, I2>
where
    I1: Iterator<Item = u8>,
    I2: Iterator<Item = u8>
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        // TODO: this is probably horribly inefficient, look up stream fusion and copy from the
        // existing Chain code https://doc.rust-lang.org/src/core/iter/adapters/chain.rs.html
        match self.iter1.next() {
            Some(x) => {
                self.break_between_streams_armed = self.break_between_streams;
                return Some(x);
            },
            None => if self.break_between_streams_armed {
                self.break_between_streams_armed = false;
                return None;
            } else {
                return self.iter2.next();
            }
        }
    }
}
