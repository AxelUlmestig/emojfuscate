pub mod emojfuscate;
pub mod demojfuscate;
pub mod hex_stream;
mod constants;


#[cfg(test)]
mod tests {
    use crate::emojfuscate::Emojfuscate;
    use crate::demojfuscate::Demojfuscate;
    use uuid::uuid;
    use proptest::prelude::*;

    // there's no Arbitrary instance for Uuid :(
    #[test]
    fn emojfuscate_uuid() {
        let original_message = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        // let original_message : Uuid = Uuid::new_v4();
        let emojified = original_message.clone().emojfuscate();
        let roundtrip_message = emojified.demojfuscate();
        assert_eq!(roundtrip_message, Ok(original_message));
    }

    #[test]
    fn emojfuscate_tuple() {
        let original_message = (123, uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));
        let emojified = original_message.clone().emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
    }

    #[test]
    fn emojfuscate_string_triple_hardcoded() {
        let original_message = ("foo".to_string(), "bar".to_string(), "baz".to_string());
        let emojified = original_message.clone().emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
    }

    proptest! {
        // "\\PC*" generating arbitrary strings composed of arbitrary non-control characters
        #[test]
        fn emojfuscate_string(original_message in "\\PC*") {
            // let original_message = "Karin är söt <3".to_string();
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message));
        }

        #[test]
        fn emojfuscate_u8(original_message : u8) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message));
        }

        #[test]
        fn emojfuscate_string_tuple(string1 in "\\PC*", string2 in "\\PC*") {
            let original_message = (string1, string2);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_string_triple(string1 in "\\PC*", string2 in "\\PC*", string3 in "\\PC*") {
            let original_message = (string1, string2, string3);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }
    }
}
