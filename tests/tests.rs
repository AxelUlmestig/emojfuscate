#[cfg(test)]
mod tests {
    use emojfuscate::{ConstructFromEmoji, Demojfuscate, Emojfuscate};
    use proptest::prelude::*;
    use std::iter;
    use std::iter::{Chain, Once};
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
        #[derive(Emojfuscate, ConstructFromEmoji, Debug, PartialEq)]
        struct UnitStruct;

        let emojified = UnitStruct.emojfuscate();
        let roundtrip_message = emojified.clone().demojfuscate();
        assert_eq!(
            roundtrip_message,
            Ok(UnitStruct),
            "emojfuscated version: {}",
            emojified
        );
    }

    #[test]
    fn emojfuscate_infinite_streams() {
        let to_repeat: u8 = 0;
        let source = iter::repeat(to_repeat);

        let demojfuscated: Result<Vec<u8>, emojfuscate::FromEmojiError> = source
            .emojfuscate_stream()
            .demojfuscate_stream()
            .take(3)
            .collect();

        assert_eq!(demojfuscated, Ok(vec![0, 0, 0]));
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
        fn emojfuscate_single_tuple(string in "\\PC*") {
            let original_message = (string,);
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
        fn emojfuscate_tuple_4(b1 : bool, b2 : bool, b3 : bool, b4 : bool) {
            let original_message = (b1, b2, b3, b4);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_tuple_5(b1 : bool, b2 : bool, b3 : bool, b4 : bool, b5 : bool) {
            let original_message = (b1, b2, b3, b4, b5);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_tuple_6(b1 : bool, b2 : bool, b3 : bool, b4 : bool, b5 : bool, b6 : bool) {
            let original_message = (b1, b2, b3, b4, b5, b6);
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
        fn emojfuscate_u8_stream(original_message : Vec<u8>) {
            let emojified = original_message.clone().emojfuscate();

            let mut roundtrip_message: Vec<u8> = Vec::new();
            for result in emojified.clone().demojfuscate_stream() {
                match result {
                    Ok(u) => roundtrip_message.push(u),
                    Err(_err) => assert!(false, "error when parsing")
                }
            }
            assert_eq!(roundtrip_message, original_message, "emojfuscated version: {}", emojified);
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
        fn emojfuscate_derive_construct_from_emoji_named_fields(age : u8, name : String, is_cool : bool) {
            #[derive(ConstructFromEmoji, Emojfuscate, Debug, PartialEq, Clone)]
            struct Person {
                age: u8,
                name: String,
                is_cool: bool
            }

            let original_message = Person { age, name, is_cool };
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_construct_from_emoji_named_fields_with_generics(age : u8, name : String, luggage : bool) {
            #[derive(ConstructFromEmoji, Emojfuscate, Debug, PartialEq, Clone)]
            struct Person<A> {
                age: u8,
                name: String,
                luggage: A
            }

            let original_message : Person<bool> = Person { age, name, luggage };
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_construct_from_emoji_tuple_struct(age : u8, name : String) {
            #[derive(Emojfuscate, ConstructFromEmoji, Debug, PartialEq, Clone)]
            struct Person(u8, String);

            let original_message = Person(age, name);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_construct_from_emoji_tuple_struct_with_generics(age : u8, name : String, luggage : bool) {
            #[derive(Emojfuscate, ConstructFromEmoji, Debug, PartialEq, Clone)]
            struct Person<A>(u8, String, A);

            let original_message = Person(age, name, luggage);
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_enum(input : Result<Option<(bool, String)>, Result<u32, ((), i16)>>) {
            #[derive(Emojfuscate, ConstructFromEmoji, Debug, PartialEq, Clone)]
            // `name` and `likes_cuddles` are not alphabetically sorted here to make sure that the
            // machinery that emojfuscates the fields in alphabetical order works
            enum Animal {
                Cat{ name: String, likes_cuddles: bool },
                Lizard,
                Dog(u32),
                Donkey((), i16)
            }

            /*
            impl<IA, IB> Emojfuscate<Chain<Chain<Once<ByteOrBreak>, IA>, IB>> for Animal
            where
                Option<(bool, String)>: Emojfuscate<IA>,
                Option<u32>: Emojfuscate<IB>,
                IA: Iterator<Item = ByteOrBreak>,
                IB: Iterator<Item = ByteOrBreak>,
            {
                fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<Once<ByteOrBreak>, IA>, IB>> {
                    match self {
                        Animal::Cat(b,s) => {
                            0u8
                                .emojfuscate_stream()
                                .chain_emoji_bytes(Some((b,s)).emojfuscate_stream())
                                .chain_emoji_bytes(None.emojfuscate_stream())
                        },
                        Animal::Dog(i) => {
                            1u8
                                .emojfuscate_stream()
                                .chain_emoji_bytes(None.emojfuscate_stream())
                                .chain_emoji_bytes(Some(i).emojfuscate_stream())
                        }
                    }
                }
            }
            */

            /*
            impl<I> ConstructFromEmoji<Animal, I> for Animal
            where
                I: Iterator<Item = u8>,
            {
                fn construct_from_emoji(
                    mut byte_stream: DecodeEmojiToBytes<I>,
                ) -> Result<(Animal, DecodeEmojiToBytes<I>), FromEmojiError> {
                    let constructor_discriminator =
                        match u8::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((n, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                n
                            }
                        };

                    let constructor0 =
                        match Option::<(bool, String)>::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((x, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                x
                            }
                        };

                    let constructor1 =
                        match Option::<u32>::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((x, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                x
                            }
                        };

                    match (constructor_discriminator, constructor0, constructor1) {
                        (0, Some((likes_cuddles, name)), None) => Ok((Animal::Cat{likes_cuddles, name}, byte_stream)),
                        (1, None, Some(i)) => Ok((Animal::Dog(i), byte_stream)),
                        (2, None, None) => Ok((Animal::Lizard, byte_stream)),
                        _ => Err(FromEmojiError::UnexpectedInput("Constructor choice and data don't agree when demojfuscating Animal".to_string()))
                    }
                }
            }
            */

            let original_message = match input {
                Ok(Some((likes_cuddles, name))) => Animal::Cat{likes_cuddles, name},
                Ok(None) => Animal::Lizard,
                Err(Ok(i)) => Animal::Dog(i),
                Err(Err((u, i))) => Animal::Donkey(u, i),
            };
            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }

        #[test]
        fn emojfuscate_derive_enum_generic(input : Option<u8>) {
            #[derive(Emojfuscate, ConstructFromEmoji, Debug, PartialEq, Clone)]
            enum Maybe<A> {
                Just(A),
                Nothing
            }

            let original_message = match input {
                Some(x) => Maybe::Just(x),
                None => Maybe::Nothing
            };

            let emojified = original_message.clone().emojfuscate();
            let roundtrip_message = emojified.clone().demojfuscate();
            assert_eq!(roundtrip_message, Ok(original_message), "emojfuscated version: {}", emojified);
        }
    }
}
