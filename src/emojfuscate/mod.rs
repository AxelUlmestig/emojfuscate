
pub mod to_emoji_stream;
pub mod from_emoji_stream;
pub mod hex_stream;
mod constants;


#[cfg(test)]
mod tests {
    use crate::to_emoji_stream::ToEmojiStream;
    use crate::from_emoji_stream::FromEmojiStream;
    use uuid::uuid;
    use uuid::Builder;

    #[test]
    fn emojfuscate_string() {
        // TODO: this breaks if I use Swedish characters
        let original_message = "Karin is the cutest <3".to_string();
        let emojified = original_message.clone().to_emoji_string();
        let roundtrip_message : String = emojified.bytes().from_emoji_stream().map(|c| c as char).collect();
        assert_eq!(roundtrip_message, original_message);
    }

    #[test]
    fn emojfuscate_uuid() {
        let original_message = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        // let original_message : Uuid = Uuid::new_v4();
        let emojified = original_message.clone().to_emoji_string();
        let roundtrip_message = Builder::from_slice(emojified.bytes().from_emoji_stream().collect::<Vec<u8>>().as_slice()).unwrap().into_uuid();
        assert_eq!(roundtrip_message, original_message);
    }
}
