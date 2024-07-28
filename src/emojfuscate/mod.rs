mod constants;
pub mod demojfuscate;
pub mod emojfuscate;
pub mod hex_stream;

#[cfg(test)]
mod tests {
    use crate::demojfuscate::{
        ConstructFromEmoji, DecodeEmojiToBytes, Demojfuscate, FromEmojiError,
    };
    use crate::emojfuscate::{ByteOrBreak, Emojfuscate, EncodeBytesAsEmoji};
    use emojfuscate_proc_macro::*;
    use proptest::prelude::*;
    use std::iter::Once;
    use uuid::uuid;

    // there's no Arbitrary instance for Uuid :(
    #[test]
    fn emojfuscate_uuid() {
        let original_message = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        // let original_message : Uuid = Uuid::new_v4();
        let emojified = original_message.clone().emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(
            roundtrip_message,
            Ok(original_message),
            "emojfuscated version: {}",
            emojified
        );
    }

    #[test]
    fn emojfuscate_tuple() {
        let original_message = (123u8, uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));
        let emojified = original_message.clone().emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(
            roundtrip_message,
            Ok(original_message),
            "emojfuscated version: {}",
            emojified
        );
    }

    #[test]
    fn emojfuscate_derive_construct_from_emoji_unit_struct() {
        #[derive(ConstructFromEmoji, Debug, PartialEq)]
        struct UnitStruct;

        let emojified = ().emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(
            roundtrip_message,
            Ok(UnitStruct),
            "emojfuscated version: {}",
            emojified
        );
    }

    proptest! {
        #[test]
        fn emojfuscate_bool(original_message : bool) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        fn emojfuscate_unit(original_message : ()) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        fn emojfuscate_char(original_message : char) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        // "\\PC*" generating arbitrary strings composed of arbitrary non-control characters
        #[test]
        fn emojfuscate_string(original_message in "\\PC*") {
            // let original_message = "Karin är söt <3".to_string();
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u8(original_message : u8) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u16(original_message : u16) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u32(original_message : u32) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u64(original_message : u64) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u128(original_message : u128) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_i8(original_message : i8) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_i16(original_message : i16) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_i32(original_message : i32) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_i64(original_message : i64) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_i128(original_message : i128) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_f32(original_message : f32) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_f64(original_message : f64) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
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

        #[test]
        fn emojfuscate_u8_vec(original_message : Vec<u8>) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_string_vec(string1 in "\\PC*", string2 in "\\PC*", string3 in "\\PC*") {
            let original_message = vec![string1, string2, string3];
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_u8_array(b1 : u8, b2 : u8, b3 : u8, b4 : u8, b5 : u8) {
            let original_message = [b1, b2, b3, b4, b5];
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message : Result<[u8; 5], _> = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_option_u8_array(b1 : Option<u8>, b2 : Option<u8>, b3 : Option<u8>) {
            let original_message = [b1, b2, b3];
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message : Result<[Option<u8>; 3], _> = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_result_u8_string_vec(original_message : Vec<Result<u8, String>>) {
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_construct_from_emoji_named_fields(age : u8, name : String) {
            #[derive(ConstructFromEmoji, Debug, PartialEq)]
            struct Person {
                age: u8,
                name: String
            }

            let emojified = (age, name.clone()).clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(Person { age, name }), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_construct_from_emoji_tuple_struct(age : u8, name : String) {
            #[derive(ConstructFromEmoji, Debug, PartialEq)]
            struct Person(u8, String);

            let emojified = (age, name.clone()).clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(Person(age, name)), "emojfuscated version: {}", emojified);
        }
    }
}
