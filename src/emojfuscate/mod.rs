
pub mod to_emoji_stream;
pub mod from_emoji_stream;
pub mod hex_stream;
mod constants;


#[cfg(test)]
mod tests {
    use crate::to_emoji_stream::ToEmojiStream;
    use crate::from_emoji_stream::FromEmoji;
    use uuid::uuid;
    // use uuid::Builder;

    #[test]
    fn emojfuscate_string() {
        let original_message = "Karin är söt <3".to_string();
        let emojified = original_message.clone().to_emoji_string();
        let roundtrip_message : String = emojified.from_emoji();
        assert_eq!(roundtrip_message, original_message);
    }

    #[test]
    fn emojfuscate_uuid() {
        let original_message = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        // let original_message : Uuid = Uuid::new_v4();
        let emojified = original_message.clone().to_emoji_string();
        let roundtrip_message : uuid::Uuid = emojified.from_emoji();
        assert_eq!(roundtrip_message, original_message);
    }

    #[test]
    fn emojfuscate_u8() {
        let original_message : u8 = 255;
        // let original_message : Uuid = Uuid::new_v4();
        let emojified = original_message.clone().to_emoji_string();
        let roundtrip_message : u8 = emojified.from_emoji();
        assert_eq!(roundtrip_message, original_message);
    }
}
