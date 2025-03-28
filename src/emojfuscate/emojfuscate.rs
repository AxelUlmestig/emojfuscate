use core::array::IntoIter;
use std::collections::VecDeque;
use std::iter::{empty, once, Chain, Empty, FlatMap, Flatten, Map, Once};
use std::vec::Vec;
use uuid::Uuid;

use super::constants::{
    usize_to_emoji, ByteInSequence, BITS_IN_A_BYTE, BITS_PER_EMOJI, MAX_EMOJI_VALUE,
    START_EMOJI_VALUE, STOP_EMOJI_VALUE,
};

pub trait Emojfuscate<I>
where
    I: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I>;

    fn emojfuscate(self) -> String
    where
        Self: Sized,
    {
        return self.emojfuscate_stream().collect();
    }
}

/// This is the same thing as Emojfuscate but it only applies to Iterators of bytes, it's an
/// optimization to avoid u8 iterators to wrap each u8 into Once<u8> which happens with the default
/// implementation of emojfuscate_stream
pub trait EmojfuscateByteStream<I>
where
    I: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_byte_stream(self) -> EncodeBytesAsEmoji<I>;
}

pub struct EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteInSequence>,
{
    iter: I,
    input_data: usize,
    defined_bits: u16,
    queued_emoji: VecDeque<char>,
}

impl<I> EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteInSequence>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            input_data: 0,
            defined_bits: 0,
            queued_emoji: VecDeque::with_capacity(3),
        }
    }

    pub fn add_start_emoji(self) -> EncodeBytesAsEmoji<Chain<Once<ByteInSequence>, I>> {
        EncodeBytesAsEmoji {
            iter: once(ByteInSequence::SequenceStart).chain(self.iter),
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji,
        }
    }

    pub fn add_stop_emoji(self) -> EncodeBytesAsEmoji<Chain<I, Once<ByteInSequence>>> {
        EncodeBytesAsEmoji {
            iter: self.iter.chain(once(ByteInSequence::SequenceEnd)),
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji,
        }
    }

    pub fn chain_emoji_bytes<I2>(
        self,
        other: EncodeBytesAsEmoji<I2>,
    ) -> EncodeBytesAsEmoji<Chain<I, I2>>
    where
        I2: Iterator<Item = ByteInSequence>,
    {
        EncodeBytesAsEmoji {
            iter: self.iter.chain(other.iter),
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji,
        }
    }

    /// Rust has a philosphy where adding a trait implementation is not supposed to be a breaking
    /// change. So to avoid breakage from crates adding trait implementations it considers concrete
    /// types from a crate to have a possible future implementation of all the traits from the same
    /// crate.
    ///
    /// This means that you will get a compiler error if you make a trait implementation for all
    /// generic values of `I` that implements some trait and then later on make an implementation
    /// for a concrete type from the same crate. Because if that crate decided to make an
    /// instance of that trait for that type in the future then there would be conflicting
    /// implementations of your trait.
    ///
    /// This happens in this crate because we want to say that all Iterators I implement
    /// Emojfuscate as long as the Item iterated over implements Emojfuscate. This should cause
    /// conflicts because the Iterator trait is from the standard library along with all the basic
    /// types such as u8, bool etc.
    ///
    /// For some reason this problem doesn't arise here. Except for tuples with two or three values
    /// (not four and up). But luckily we can make the compiler error go away by just wrapping the
    /// resuling iterator in useless IteratorWrapper layer 🧠
    pub fn bypass_future_trait_implementation_compiler_error(
        self,
    ) -> EncodeBytesAsEmoji<IteratorWrapper<I, ByteInSequence>> {
        EncodeBytesAsEmoji {
            iter: IteratorWrapper { iter: self.iter },
            input_data: self.input_data,
            defined_bits: self.defined_bits,
            queued_emoji: self.queued_emoji,
        }
    }
}

pub struct IteratorWrapper<I, A>
where
    I: Iterator<Item = A>,
{
    iter: I,
}

impl<I, A> Iterator for IteratorWrapper<I, A>
where
    I: Iterator<Item = A>,
{
    type Item = A;
    fn next(&mut self) -> Option<A> {
        self.iter.next()
    }
}

impl<I> Iterator for EncodeBytesAsEmoji<I>
where
    I: Iterator<Item = ByteInSequence>,
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
                Some(ByteInSequence::Byte(b)) => b,
                None => break,
                Some(ByteInSequence::SequenceStart) => {
                    let start_emoji = usize_to_emoji(usize::try_from(START_EMOJI_VALUE).unwrap());
                    self.queued_emoji.push_back(start_emoji);
                    break;
                }
                Some(ByteInSequence::SequenceEnd) => {
                    let stop_emoji = usize_to_emoji(usize::try_from(STOP_EMOJI_VALUE).unwrap());
                    self.queued_emoji.push_back(stop_emoji);
                    break;
                }
            };

            self.input_data = (self.input_data << BITS_IN_A_BYTE) | usize::from(b);
            self.defined_bits += BITS_IN_A_BYTE;

            if self.defined_bits < BITS_PER_EMOJI {
                continue;
            }

            let bits_used = self.defined_bits - BITS_PER_EMOJI;
            let emoji_index = self.input_data >> bits_used;

            // remove the used bits
            self.input_data = self.input_data ^ (emoji_index << bits_used);
            self.defined_bits -= BITS_PER_EMOJI;

            return Some(usize_to_emoji(emoji_index));
        }

        // If we don't have enough bytes for another emoji we encode the difference in a special
        // emoji and stash away the remaining information so it will be returned on the next next()
        if self.defined_bits > 0 {
            let padding = BITS_PER_EMOJI - self.defined_bits;
            let truncate_bits_emoji =
                usize_to_emoji(usize::try_from(MAX_EMOJI_VALUE + padding).unwrap());

            self.defined_bits = 0;
            let final_emoji = usize_to_emoji(self.input_data << padding);

            self.input_data = 0;

            // push to the front so they get in before the 'stop emoji' if it's set
            self.queued_emoji.push_front(final_emoji);
            self.queued_emoji.push_front(truncate_bits_emoji);
        }

        return self.queued_emoji.pop_front();
    }
}

fn wrap_byte(b: u8) -> ByteInSequence {
    ByteInSequence::Byte(b)
}

impl<I: Iterator<Item = u8>> EmojfuscateByteStream<Map<I, fn(u8) -> ByteInSequence>> for I {
    fn emojfuscate_byte_stream(self) -> EncodeBytesAsEmoji<Map<I, fn(u8) -> ByteInSequence>> {
        EncodeBytesAsEmoji::new(self.map(wrap_byte))
    }
}

// implementations
impl<I: Iterator<Item = ByteInSequence>> Emojfuscate<I> for I {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I> {
        EncodeBytesAsEmoji::new(self)
    }
}

impl Emojfuscate<Empty<ByteInSequence>> for () {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Empty<ByteInSequence>> {
        EncodeBytesAsEmoji::new(empty::<ByteInSequence>())
    }
}

impl Emojfuscate<Once<ByteInSequence>> for bool {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Once<ByteInSequence>> {
        <bool as Into<u8>>::into(self).emojfuscate_stream()
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 4>> for char {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 4>> {
        (self as u32).emojfuscate_stream()
    }
}

impl Emojfuscate<Once<ByteInSequence>> for u8 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Once<ByteInSequence>> {
        EncodeBytesAsEmoji::new(std::iter::once(ByteInSequence::Byte(self)))
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 2>> for u16 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 2>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 4>> for u32 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 4>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 8>> for u64 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 8>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 16>> for u128 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 16>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 1>> for i8 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 1>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 2>> for i16 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 2>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 4>> for i32 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 4>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 8>> for i64 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 8>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 16>> for i128 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 16>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 4>> for f32 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 4>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl Emojfuscate<IntoIter<ByteInSequence, 8>> for f64 {
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IntoIter<ByteInSequence, 8>> {
        EncodeBytesAsEmoji::new(
            self.to_be_bytes()
                .map(|b| ByteInSequence::Byte(b))
                .into_iter(),
        )
    }
}

impl<'a>
    Emojfuscate<
        Chain<
            Chain<Once<ByteInSequence>, Map<std::str::Bytes<'a>, fn(u8) -> ByteInSequence>>,
            Once<ByteInSequence>,
        >,
    > for &'a str
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<
        Chain<
            Chain<Once<ByteInSequence>, Map<std::str::Bytes<'a>, fn(u8) -> ByteInSequence>>,
            Once<ByteInSequence>,
        >,
    > {
        self.bytes()
            .into_iter()
            .emojfuscate_byte_stream()
            .add_start_emoji()
            .add_stop_emoji()
    }
}

impl
    Emojfuscate<
        Chain<
            Chain<Once<ByteInSequence>, Map<std::vec::IntoIter<u8>, fn(u8) -> ByteInSequence>>,
            Once<ByteInSequence>,
        >,
    > for String
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<
        Chain<
            Chain<Once<ByteInSequence>, Map<std::vec::IntoIter<u8>, fn(u8) -> ByteInSequence>>,
            Once<ByteInSequence>,
        >,
    > {
        self.into_bytes()
            .into_iter()
            .emojfuscate_byte_stream()
            .add_start_emoji()
            .add_stop_emoji()
    }
}

impl Emojfuscate<Map<std::array::IntoIter<u8, 16>, fn(u8) -> ByteInSequence>> for Uuid {
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<Map<std::array::IntoIter<u8, 16>, fn(u8) -> ByteInSequence>> {
        return self.into_bytes().into_iter().emojfuscate_byte_stream();
    }
}

impl<A, IA, const S: usize> Emojfuscate<FlatMap<std::array::IntoIter<A, S>, IA, fn(A) -> IA>>
    for [A; S]
where
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<FlatMap<std::array::IntoIter<A, S>, IA, fn(A) -> IA>> {
        self.into_iter()
            .flat_map(get_emojfuscate_iter as fn(A) -> IA)
            .emojfuscate_stream()
    }
}

fn get_emojfuscate_iter<A, I>(a: A) -> I
where
    A: Emojfuscate<I>,
    I: Iterator<Item = ByteInSequence>,
{
    a.emojfuscate_stream().iter
}

impl<A, IA>
    Emojfuscate<
        Chain<
            Chain<Once<ByteInSequence>, FlatMap<std::vec::IntoIter<A>, IA, fn(A) -> IA>>,
            Once<ByteInSequence>,
        >,
    > for Vec<A>
where
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<
        Chain<
            Chain<Once<ByteInSequence>, FlatMap<std::vec::IntoIter<A>, IA, fn(A) -> IA>>,
            Once<ByteInSequence>,
        >,
    > {
        self.into_iter().emojfuscate_stream()
    }
}

impl<I, A, IA>
    Emojfuscate<
        Chain<Chain<Once<ByteInSequence>, FlatMap<I, IA, fn(A) -> IA>>, Once<ByteInSequence>>,
    > for I
where
    I: Iterator<Item = A>,
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<
        Chain<Chain<Once<ByteInSequence>, FlatMap<I, IA, fn(A) -> IA>>, Once<ByteInSequence>>,
    > {
        self.flat_map(get_emojfuscate_iter as fn(A) -> IA)
            .emojfuscate_stream()
            .add_start_emoji()
            .add_stop_emoji()
    }
}

impl<A, IA> Emojfuscate<Chain<Once<ByteInSequence>, Flatten<std::option::IntoIter<IA>>>>
    for Option<A>
where
    A: Emojfuscate<IA>,
    IA: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<Chain<Once<ByteInSequence>, Flatten<std::option::IntoIter<IA>>>> {
        let constructor_discriminator = match self {
            None => 0u8,
            Some(_) => 1,
        };

        let iter = self
            .map(get_emojfuscate_iter as fn(A) -> IA)
            .into_iter()
            .flatten();

        return constructor_discriminator
            .emojfuscate_stream()
            .chain_emoji_bytes(EncodeBytesAsEmoji::new(iter));
    }
}

impl<A, B, IA, IB> Emojfuscate<Chain<Chain<Once<ByteInSequence>, IA>, IB>> for Result<A, B>
where
    Option<A>: Emojfuscate<IA>,
    Option<B>: Emojfuscate<IB>,
    IA: Iterator<Item = ByteInSequence>,
    IB: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<Once<ByteInSequence>, IA>, IB>> {
        match self {
            Ok(x) => 0u8
                .emojfuscate_stream()
                .chain_emoji_bytes(Some(x).emojfuscate_stream())
                .chain_emoji_bytes(None::<B>.emojfuscate_stream()),
            Err(x) => 1u8
                .emojfuscate_stream()
                .chain_emoji_bytes(None::<A>.emojfuscate_stream())
                .chain_emoji_bytes(Some(x).emojfuscate_stream()),
        }
    }
}

impl<A, I> Emojfuscate<IteratorWrapper<I, ByteInSequence>> for (A,)
where
    A: Emojfuscate<I>,
    I: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<IteratorWrapper<I, ByteInSequence>> {
        let (a,) = self;
        return a
            .emojfuscate_stream()
            .bypass_future_trait_implementation_compiler_error();
    }
}

impl<A, B, I1, I2> Emojfuscate<IteratorWrapper<Chain<I1, I2>, ByteInSequence>> for (A, B)
where
    A: Emojfuscate<I1>,
    B: Emojfuscate<I2>,
    I1: Iterator<Item = ByteInSequence>,
    I2: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<IteratorWrapper<Chain<I1, I2>, ByteInSequence>> {
        let (a, b) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream())
            .bypass_future_trait_implementation_compiler_error();
    }
}

impl<A, B, C, IA, IB, IC> Emojfuscate<IteratorWrapper<Chain<Chain<IA, IB>, IC>, ByteInSequence>>
    for (A, B, C)
where
    A: Emojfuscate<IA>,
    B: Emojfuscate<IB>,
    C: Emojfuscate<IC>,
    IA: Iterator<Item = ByteInSequence>,
    IB: Iterator<Item = ByteInSequence>,
    IC: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<IteratorWrapper<Chain<Chain<IA, IB>, IC>, ByteInSequence>> {
        let (a, b, c) = self;
        return a
            .emojfuscate_stream()
            .chain_emoji_bytes(b.emojfuscate_stream())
            .chain_emoji_bytes(c.emojfuscate_stream())
            .bypass_future_trait_implementation_compiler_error();
    }
}

impl<A1, A2, A3, A4, I1, I2, I3, I4> Emojfuscate<Chain<Chain<Chain<I1, I2>, I3>, I4>>
    for (A1, A2, A3, A4)
where
    A1: Emojfuscate<I1>,
    A2: Emojfuscate<I2>,
    A3: Emojfuscate<I3>,
    A4: Emojfuscate<I4>,
    I1: Iterator<Item = ByteInSequence>,
    I2: Iterator<Item = ByteInSequence>,
    I3: Iterator<Item = ByteInSequence>,
    I4: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<Chain<I1, I2>, I3>, I4>> {
        let (a1, a2, a3, a4) = self;
        return a1
            .emojfuscate_stream()
            .chain_emoji_bytes(a2.emojfuscate_stream())
            .chain_emoji_bytes(a3.emojfuscate_stream())
            .chain_emoji_bytes(a4.emojfuscate_stream());
    }
}

impl<A1, A2, A3, A4, A5, I1, I2, I3, I4, I5>
    Emojfuscate<Chain<Chain<Chain<Chain<I1, I2>, I3>, I4>, I5>> for (A1, A2, A3, A4, A5)
where
    A1: Emojfuscate<I1>,
    A2: Emojfuscate<I2>,
    A3: Emojfuscate<I3>,
    A4: Emojfuscate<I4>,
    A5: Emojfuscate<I5>,
    I1: Iterator<Item = ByteInSequence>,
    I2: Iterator<Item = ByteInSequence>,
    I3: Iterator<Item = ByteInSequence>,
    I4: Iterator<Item = ByteInSequence>,
    I5: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<Chain<Chain<Chain<Chain<I1, I2>, I3>, I4>, I5>> {
        let (a1, a2, a3, a4, a5) = self;
        return a1
            .emojfuscate_stream()
            .chain_emoji_bytes(a2.emojfuscate_stream())
            .chain_emoji_bytes(a3.emojfuscate_stream())
            .chain_emoji_bytes(a4.emojfuscate_stream())
            .chain_emoji_bytes(a5.emojfuscate_stream());
    }
}

impl<A1, A2, A3, A4, A5, A6, I1, I2, I3, I4, I5, I6>
    Emojfuscate<Chain<Chain<Chain<Chain<Chain<I1, I2>, I3>, I4>, I5>, I6>>
    for (A1, A2, A3, A4, A5, A6)
where
    A1: Emojfuscate<I1>,
    A2: Emojfuscate<I2>,
    A3: Emojfuscate<I3>,
    A4: Emojfuscate<I4>,
    A5: Emojfuscate<I5>,
    A6: Emojfuscate<I6>,
    I1: Iterator<Item = ByteInSequence>,
    I2: Iterator<Item = ByteInSequence>,
    I3: Iterator<Item = ByteInSequence>,
    I4: Iterator<Item = ByteInSequence>,
    I5: Iterator<Item = ByteInSequence>,
    I6: Iterator<Item = ByteInSequence>,
{
    fn emojfuscate_stream(
        self,
    ) -> EncodeBytesAsEmoji<Chain<Chain<Chain<Chain<Chain<I1, I2>, I3>, I4>, I5>, I6>> {
        let (a1, a2, a3, a4, a5, a6) = self;
        return a1
            .emojfuscate_stream()
            .chain_emoji_bytes(a2.emojfuscate_stream())
            .chain_emoji_bytes(a3.emojfuscate_stream())
            .chain_emoji_bytes(a4.emojfuscate_stream())
            .chain_emoji_bytes(a5.emojfuscate_stream())
            .chain_emoji_bytes(a6.emojfuscate_stream());
    }
}
