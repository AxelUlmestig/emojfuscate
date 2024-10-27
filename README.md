# Emojfuscate: The ultimate human readable serialization format

Emojfuscate is a library that turns data into emoji. Other popular
serialization formats, like JSON and XML, serialize data using scary symbols
like `{`, `[` and `<`. This is friendly to robots, not humans. Emojfuscate will
encode your data using emoji, i.e. pure human emotion.

As a library Emojfuscate promises to bring concrete business value to your
project by virtue of it being written in Rust and being _smolderingly quick_
(trademark pending).

Let's see an example.

```rust
use emojfuscate::{ConstructFromEmojiStream, Demojfuscate, Emojfuscate, FromEmojiError};

#[derive(Emojfuscate, ConstructFromEmoji, Clone, Debug, PartialEq)]
struct Person {
    age: u8,
    name: String,
    is_cool: bool,
}

let original_person = Person {
    age: 33,
    name: "Axel".to_string(),
    is_cool: true,
};

// ❣😋🥫🐭💴📜😆
let emojified = original_person.clone().emojfuscate();
let deserialized_person = emojified.clone().demojfuscate();

assert_eq!(deserialized_person, Ok(original_person));
```

### Laziness

To further embrace human properties, Emojfuscate is as lazy as
possible. Any iterator of `u8` can be turned into a lazy iterator of emoji.

```rust
use emojfuscate::{ConstructFromEmoji, Demojfuscate, Emojfuscate, ConstructFromEmojiStream};

for emoji in "hunter2".bytes().emojfuscate_stream() {
    println!("{}", emoji);
}
```

Similarly you can demojfuscate an iterator of (emoji) bytes into an iterator of
demojfuscated bytes:
```rust
use emojfuscate::{ConstructFromEmoji, Demojfuscate, Emojfuscate, ConstructFromEmojiStream};

for byte in "🐠🥏👛👿🌴📰🌨".bytes().demojfuscate_stream() {
    println!("{}", byte as char);
}
```

Note that the `.bytes()` method call isn't necessary in any of the two above
examples, it was added to demonstrate that you can convert iterators to other
iterators.

### How it works

Let's say we have a tuple of `u8`
```rust
let tuple = (1u8, 2u8, 3u8, 4u8, 5u8);
```

The raw bits will look like this:
```
    1        2        3        4        5
/---^--\ /---^--\ /---^--\ /---^--\ /---^--\
00000001 00000010 00000011 00000100 00000101
```

Every 10 bits will be mapped to one emoji
```
    4=😆      32=🫣     193=👏       5=😅
/----^----\/----^----\/----^----\/----^----\
00000001 00000010 00000011 00000100 00000101
```

_But what if the number bits are not divisible by 10?_

If we have a tuple with only two `u8`
```rust
let tuple = (1u8, 2u8);
```

Then the bits will be padded with zeros until we get an even multiple of 10
```
    4=😆      32=🫣
/----^----\/----^----\
00000001 00000010 0000
                  \--/
                  four padded zeros
```

The emoji with the padded zeros will be prefixed with 📰 to signify four padded zeros.
📃, 📜, 📄, 📰, 🗞, 📑, 🔖, 🏷 and 💰 signify that the next emoji has 1, 2, 3,
4, 5, 6, 7, 8 or 9 padded zeros respectively (though in practice it's
impossible to be offset by an odd number since we're only dealing with an
integer number of bytes).

So `(1u8, 2u8)` will be emojfuscated into 😆📰🫣.
