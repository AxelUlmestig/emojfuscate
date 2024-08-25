mod constants;
mod demojfuscate;
mod emojfuscate;
pub mod hex_stream;

pub use demojfuscate::{
    ConstructFromEmoji, DecodeEmojiToBytes, Demojfuscate, FromEmojiError, IsEmojiRepresentation,
};
pub use emojfuscate::{ByteOrBreak, Emojfuscate, EncodeBytesAsEmoji};
pub use emojfuscate_proc_macro::*;
