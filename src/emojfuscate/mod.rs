mod constants;
mod demojfuscate;
mod emojfuscate;

pub use constants::ByteInSequence;
pub use demojfuscate::{
    ConstructFromEmoji, DecodeEmojiToBytes, Demojfuscate, FromEmojiError, IsEmojiRepresentation,
};
pub use emojfuscate::{Emojfuscate, EmojfuscateByteStream, EncodeBytesAsEmoji};
pub use emojfuscate_proc_macro::*;
