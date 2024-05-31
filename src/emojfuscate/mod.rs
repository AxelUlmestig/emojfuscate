
pub mod to_emoji_stream;
pub mod from_emoji_stream;
pub mod hex_stream;
mod constants;


#[cfg(test)]
mod tests {
    use crate::to_emoji_stream::ToEmojiStream;
    use crate::from_emoji_stream::FromEmojiStream;

    #[test]
    fn emojfuscate_string() {
        // TODO: this breaks if I use Swedish characters
        let original_message : String = "Karin is the cutest <3".to_string();
        let emojified : String = original_message.clone().to_emoji_string();
        let roundtrip_message : String = emojified.bytes().from_emoji_stream().map(|c| c as char).collect();
        assert_eq!(roundtrip_message, original_message);
    }
}
