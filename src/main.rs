use clap::{Parser, Subcommand};
use std::io;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::convert::TryFrom;
use std::str;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    line_break: bool
}

#[derive(Subcommand)]
enum Commands {
    Encode,
    Decode,
}

fn main() {
    let cli = Cli::parse();

    let unwrapped_std_in = io::stdin().bytes().map(|b| b.unwrap());

    match &cli.command {
        Commands::Encode => encode_bytes_as_emoji(unwrapped_std_in),
        Commands::Decode => decode_emoji_to_bytes(unwrapped_std_in)
    };

    if cli.line_break {
        print!("\n");
    }
}

fn usize_to_emoji(u : usize) -> char {
    let emoji_unicode = EMOJI[u];
    return char::from_u32(emoji_unicode).unwrap();
}

// TODO: is there some way to return an iterator of bytes that can be lazily consumed?
fn encode_bytes_as_emoji<I>(bs : I)
    where I : Iterator<Item = u8>
{
// fn encode_bytes_as_emoji(bs : io::Bytes<io::Stdin>) {
    let mut input_data : usize = 0;
    let mut defined_bits : u32 = 0;

    for b in bs {
        // let b = i.unwrap();

        input_data = (input_data << BITS_IN_A_BYTE) | usize::from(b);
        defined_bits += BITS_IN_A_BYTE;

        if defined_bits < BITS_PER_EMOJI {
            continue;
        }

        let bits_used = defined_bits - BITS_PER_EMOJI;
        let emoji_index = input_data >> bits_used;

        // remove the used bits
        input_data = input_data ^ (emoji_index << bits_used);
        defined_bits -= BITS_PER_EMOJI;

        print!("{}", usize_to_emoji(emoji_index));
    }

    if defined_bits > 0 {
        let padding = BITS_PER_EMOJI - defined_bits;
        let truncate_bits_emoji = usize_to_emoji(usize::try_from(MAX_EMOJI_VALUE + padding).unwrap());
        print!(
            "{}{}",
            truncate_bits_emoji,
            usize_to_emoji(input_data << padding)
        );
    }
}

fn decode_emoji_to_bytes<I>(bs : I)
    where I : Iterator<Item = u8>
{
    // TODO: Try to move this into a const value so it can be computed during compile time
    let emoji_values : HashMap <char, u32> = 
        HashMap::from_iter(
            EMOJI
                .iter()
                .enumerate()
                .map(|(i, unicode)| (char::from_u32(*unicode).unwrap(), u32::try_from(i).unwrap()))
        );

    let mut accumulated_data : u32 = 0;
    let mut defined_bits : u32 = 0;
    let mut accumulated_input_bytes = Vec::new();
    let mut bits_to_truncate : u32 = 0;

    for b in bs {
        accumulated_input_bytes.push(b);

        if accumulated_input_bytes.len() < 3 {
            continue;
        }

        if accumulated_input_bytes.len() > 5 {
            panic!("accumulated_input_bytes.len() > 5");
        }

        // println!("accumulated_input_bytes: {:?}", accumulated_input_bytes);

        let emoji =
            match str::from_utf8(&accumulated_input_bytes) {
                Ok(s) => s.chars().nth(0).unwrap(),
                Err(_) => continue
            };

        // delete the accumulated bytes
        accumulated_input_bytes.truncate(0);

        let emoji_value =
            match emoji_values.get(&emoji) {
                Some(x) => x,
                None => panic!("Unexpected input character: {}", emoji)
            };


        // println!("emoji: {}, emoji_value: {}", emoji, *emoji_value);

        // emoji beyond 2047 are used to indicate that the next emoji produces too many bits. This
        // happens at the end of the encoded message
        if *emoji_value >= MAX_EMOJI_VALUE {
            bits_to_truncate = *emoji_value - MAX_EMOJI_VALUE;
            //println!("emoji: {}, bits_to_truncate: {}", emoji, bits_to_truncate);
            continue;
        }

        accumulated_data = (accumulated_data << BITS_PER_EMOJI) | emoji_value;
        defined_bits += BITS_PER_EMOJI;

        // TODO: combine this with the above statement
        accumulated_data = accumulated_data >> bits_to_truncate;
        bits_to_truncate = 0;

        while defined_bits >= BITS_IN_A_BYTE {
            let u32_byte_to_output = accumulated_data >> (defined_bits - BITS_IN_A_BYTE);
            accumulated_data = accumulated_data ^ (u32_byte_to_output << (defined_bits - BITS_IN_A_BYTE));
            let [byte_to_output, _, _, _] = u32_byte_to_output.to_ne_bytes();
            defined_bits -= BITS_IN_A_BYTE;

            // println!("stdout byte: {}", byte_to_output);

            // TODO: is there a less hacky way than wrapping a single byte in an array?
            io::stdout().write(&[byte_to_output]).unwrap();
        }
    }

    io::stdout().flush().unwrap();
}

const BITS_IN_A_BYTE : u32 = 8;
const BITS_PER_EMOJI : u32 = 10;
const MAX_EMOJI_VALUE : u32 = u32::pow(2, BITS_PER_EMOJI);

/*
static EMOJI_HASHMAP : HashMap <char, u32> =
    HashMap::from_iter(
        EMOJI
            .iter()
            .enumerate()
            .map(|(i, unicode)| (char::from_u32(*unicode).unwrap(), u32::try_from(i).unwrap()))
    );
*/

/*
const EMOJI_HASHMAP : HashMap <char, u32> = loop {
    
};
*/

const EMOJI : [u32; 1385] =
    [
      0x1F600, //  😀  😀  😀  —   —   —   grinning face
      0x1F603, //  😃  😃  😃  😃  😃  😃  grinning face with big eyes
      0x1F604, //  😄  😄  😄  😄  —   —   grinning face with smiling eyes
      0x1F601, //  😁  😁  😁  😁  😁  😁  beaming face with smiling eyes
      0x1F606, //  😆  😆  😆  —   😆  —   grinning squinting face
      0x1F605, //  😅  😅  😅  —   😅  —   grinning face with sweat
      0x1F923, //  🤣  🤣  —   —   —   —   rolling on the floor laughing
      0x1F602, //  😂  😂  😂  😂  —   😂  face with tears of joy
      0x1F642, //  🙂  🙂  🙂  —   —   —   slightly smiling face
      0x1F643, //  🙃  🙃  —   —   —   —   upside-down face
      0x1FAE0, //  🫠  🫠  —   —   —   —   melting face
      0x1F609, //  😉  😉  😉  😉  😉  😉  winking face
      0x1F60A, //  😊  😊  😊  😊  —   😊  smiling face with smiling eyes
      0x1F607, //  😇  😇  —   —   —   —   smiling face with halo
      0x1F970, //  🥰  🥰  —   —   —   —   smiling face with hearts
      0x1F60D, //  😍  😍  😍  😍  😍  😍  smiling face with heart-eyes
      0x1F929, //  🤩  🤩  —   —   —   —   star-struck
      0x1F618, //  😘  😘  😘  😘  —   😘  face blowing a kiss
      0x1F617, //  😗  😗  —   —   —   —   kissing face
      0x263A, // ☺   ☺   ☺   ☺   —   ☺   smiling face
      0x1F61A, //  😚  😚  😚  😚  —   😚  kissing face with closed eyes
      0x1F619, //  😙  😙  —   —   —   —   kissing face with smiling eyes
      0x1F972, //  🥲  🥲  —   —   —   —   smiling face with tear
      0x1F60B, //  😋  😋  😋  —   😋  —   face savoring food
      0x1F61B, //  😛  😛  —   —   —   —   face with tongue
      0x1F61C, //  😜  😜  😜  😜  😜  😜  winking face with tongue
      0x1F92A, //  🤪  🤪  —   —   —   —   zany face
      0x1F61D, //  😝  😝  😝  😝  —   —   squinting face with tongue
      0x1F911, //  🤑  🤑  —   —   —   —   money-mouth face
      0x1F917, //  🤗  🤗  —   —   —   —   smiling face with open hands
      0x1F92D, //  🤭  🤭  —   —   —   —   face with hand over mouth
      0x1FAE2, //  🫢  🫢  —   —   —   —   face with open eyes and hand over mouth
      0x1FAE3, //  🫣  🫣  —   —   —   —   face with peeking eye
      0x1F92B, //  🤫  🤫  —   —   —   —   shushing face
      0x1F914, //  🤔  🤔  —   —   —   —   thinking face
      0x1FAE1, //  🫡  🫡  —   —   —   —   saluting face
      0x1F910, //  🤐  🤐  —   —   —   —   zipper-mouth face
      0x1F928, //  🤨  🤨  —   —   —   —   face with raised eyebrow
      0x1F610, //  😐  😐  —   —   —   —   neutral face
      0x1F611, //  😑  😑  —   —   —   —   expressionless face
      0x1F636, //  😶  😶  —   —   —   —   face without mouth
      0x1FAE5, //  🫥  🫥  —   —   —   —   dotted line face
      0x1F60F, //  😏  😏  😏  😏  😏  😏  smirking face
      0x1F612, //  😒  😒  😒  😒  😒  😒  unamused face
      0x1F644, //  🙄  🙄  —   —   —   —   face with rolling eyes
      0x1F62C, //  😬  😬  —   —   —   —   grimacing face
      0x1F925, //  🤥  🤥  —   —   —   —   lying face
      0x1FAE8, //  🫨  🫨  —   —   —   —   shaking face
      0x1F60C, //  😌  😌  😌  😌  😌  😌  relieved face
      0x1F614, //  😔  😔  😔  😔  😔  😔  pensive face
      0x1F62A, //  😪  😪  😪  😪  —   😪  sleepy face
      0x1F924, //  🤤  🤤  —   —   —   —   drooling face
      0x1F634, //  😴  😴  —   —   —   —   sleeping face
      0x1F637, //  😷  😷  😷  😷  —   😷  face with medical mask
      0x1F912, //  🤒  🤒  —   —   —   —   face with thermometer
      0x1F915, //  🤕  🤕  —   —   —   —   face with head-bandage
      0x1F922, //  🤢  🤢  —   —   —   —   nauseated face
      0x1F92E, //  🤮  🤮  —   —   —   —   face vomiting
      0x1F927, //  🤧  🤧  —   —   —   —   sneezing face
      0x1F975, //  🥵  🥵  —   —   —   —   hot face
      0x1F976, //  🥶  🥶  —   —   —   —   cold face
      0x1F974, //  🥴  🥴  —   —   —   —   woozy face
      0x1F635, //  😵  😵  😵  —   😵  😵  face with crossed-out eyes
      0x1F92F, //  🤯  🤯  —   —   —   —   exploding head
      0x1F920, //  🤠  🤠  —   —   —   —   cowboy hat face
      0x1F973, //  🥳  🥳  —   —   —   —   partying face
      0x1F978, //  🥸  🥸  —   —   —   —   disguised face
      0x1F60E, //  😎  😎  😎  —   —   —   smiling face with sunglasses
      0x1F913, //  🤓  🤓  —   —   —   —   nerd face
      0x1F9D0, //  🧐  🧐  —   —   —   —   face with monocle
      0x1F615, //  😕  😕  😕  —   —   —   confused face
      0x1FAE4, //  🫤  🫤  —   —   —   —   face with diagonal mouth
      0x1F61F, //  😟  😟  😟  —   —   —   worried face
      0x1F641, //  🙁  🙁  —   —   —   —   slightly frowning face
      0x2639, // ☹   ☹   —   —   —   —   frowning face
      0x1F62E, //  😮  😮  —   —   —   —   face with open mouth
      0x1F62F, //  😯  😯  —   —   —   —   hushed face
      0x1F632, //  😲  😲  😲  😲  —   😲  astonished face
      0x1F633, //  😳  😳  😳  😳  —   😳  flushed face
      0x1F97A, //  🥺  🥺  —   —   —   —   pleading face
      0x1F979, //  🥹  🥹  —   —   —   —   face holding back tears
      0x1F626, //  😦  😦  —   —   —   —   frowning face with open mouth
      0x1F627, //  😧  😧  —   —   —   —   anguished face
      0x1F628, //  😨  😨  😨  😨  —   😨  fearful face
      0x1F630, //  😰  😰  😰  😰  —   😰  anxious face with sweat
      0x1F625, //  😥  😥  😥  😥  —   —   sad but relieved face
      0x1F622, //  😢  😢  😢  😢  😢  😢  crying face
      0x1F62D, //  😭  😭  😭  😭  😭  😭  loudly crying face
      0x1F631, //  😱  😱  😱  😱  😱  😱  face screaming in fear
      0x1F616, //  😖  😖  😖  😖  😖  😖  confounded face
      0x1F623, //  😣  😣  😣  😣  😣  😣  persevering face
      0x1F61E, //  😞  😞  😞  😞  😞  —   disappointed face
      0x1F613, //  😓  😓  😓  😓  😓  😓  downcast face with sweat
      0x1F629, //  😩  😩  😩  —   —   😩  weary face
      0x1F62B, //  😫  😫  😫  —   —   😫  tired face
      0x1F971, //  🥱  🥱  —   —   —   —   yawning face
      0x1F624, //  😤  😤  😤  —   —   😤  face with steam from nose
      0x1F621, //  😡  😡  😡  😡  😡  😡  enraged face
      0x1F620, //  😠  😠  😠  😠  😠  😠  angry face
      0x1F92C, //  🤬  🤬  —   —   —   —   face with symbols on mouth
      0x1F608, //  😈  😈  —   —   —   —   smiling face with horns
      0x1F47F, //  👿  👿  👿  👿  —   👿  angry face with horns
      0x1F480, //  💀  💀  💀  💀  —   💀  skull
      0x2620, // ☠   ☠   —   —   —   —   skull and crossbones
      0x1F4A9, //  💩  💩  💩  💩  —   💩  pile of poo
      0x1F921, //  🤡  🤡  —   —   —   —   clown face
      0x1F479, //  👹  👹  👹  —   —   👹  ogre
      0x1F47A, //  👺  👺  👺  —   —   👺  goblin
      0x1F47B, //  👻  👻  👻  👻  —   👻  ghost
      0x1F47D, //  👽  👽  👽  👽  —   👽  alien
      0x1F47E, //  👾  👾  👾  👾  —   👾  alien monster
      0x1F916, //  🤖  🤖  —   —   —   —   robot
      0x1F63A, //  😺  😺  😺  —   —   😺  grinning cat
      0x1F638, //  😸  😸  😸  —   —   😸  grinning cat with smiling eyes
      0x1F639, //  😹  😹  😹  —   —   😹  cat with tears of joy
      0x1F63B, //  😻  😻  😻  —   —   😻  smiling cat with heart-eyes
      0x1F63C, //  😼  😼  😼  —   —   😼  cat with wry smile
      0x1F63D, //  😽  😽  😽  —   —   😽  kissing cat
      0x1F640, //  🙀  🙀  🙀  —   —   🙀  weary cat
      0x1F63F, //  😿  😿  😿  —   —   😿  crying cat
      0x1F63E, //  😾  😾  😾  —   —   😾  pouting cat
      0x1F648, //  🙈  🙈  🙈  —   —   🙈  see-no-evil monkey
      0x1F649, //  🙉  🙉  🙉  —   —   🙉  hear-no-evil monkey
      0x1F64A, //  🙊  🙊  🙊  —   —   🙊  speak-no-evil monkey
      0x1F48C, //  💌  💌  💌  —   💌  💌  love letter
      0x1F498, //  💘  💘  💘  💘  —   💘  heart with arrow
      0x1F49D, //  💝  💝  💝  💝  —   💝  heart with ribbon
      0x1F496, //  💖  💖  💖  —   —   💖  sparkling heart
      0x1F497, //  💗  💗  💗  💗  —   —   growing heart
      0x1F493, //  💓  💓  💓  💓  💓  💓  beating heart
      0x1F49E, //  💞  💞  💞  —   —   💞  revolving hearts
      0x1F495, //  💕  💕  💕  —   💕  💕  two hearts
      0x1F49F, //  💟  💟  💟  💟  —   —   heart decoration
      0x2763, // ❣   ❣   —   —   —   —   heart exclamation
      0x1F494, //  💔  💔  💔  💔  💔  💔  broken heart
      0x2764, // ❤   ❤   ❤   ❤   ❤   ❤   red heart
      0x1FA77, //  🩷  🩷  —   —   —   —   pink heart
      0x1F9E1, //  🧡  🧡  —   —   —   —   orange heart
      0x1F49B, //  💛  💛  💛  💛  —   💛  yellow heart
      0x1F49A, //  💚  💚  💚  💚  —   💚  green heart
      0x1F499, //  💙  💙  💙  💙  —   💙  blue heart
      0x1FA75, //  🩵  🩵  —   —   —   —   light blue heart
      0x1F49C, //  💜  💜  💜  💜  —   💜  purple heart
      0x1F90E, //  🤎  🤎  —   —   —   —   brown heart
      0x1F5A4, //  🖤  🖤  —   —   —   —   black heart
      0x1FA76, //  🩶  🩶  —   —   —   —   grey heart
      0x1F90D, //  🤍  🤍  —   —   —   —   white heart
      0x1F48B, //  💋  💋  💋  💋  💋  💋  kiss mark
      0x1F4AF, //  💯  💯  💯  —   —   💯  hundred points
      0x1F4A2, //  💢  💢  💢  💢  💢  💢  anger symbol
      0x1F4A5, //  💥  💥  💥  —   💥  💥  collision
      0x1F4AB, //  💫  💫  💫  —   —   💫  dizzy
      0x1F4A6, //  💦  💦  💦  💦  💦  💦  sweat droplets
      0x1F4A8, //  💨  💨  💨  💨  💨  💨  dashing away
      0x1F573, //  🕳  🕳  —   —   —   —   hole
      0x1F4AC, //  💬  💬  💬  —   —   💬  speech balloon
      0x1F5E8, //  🗨  🗨  —   —   —   —   left speech bubble
      0x1F5EF, //  🗯  🗯  —   —   —   —   right anger bubble
      0x1F4AD, //  💭  💭  —   —   —   —   thought balloon
      0x1F4A4, //  💤  💤  💤  💤  💤  💤  ZZZ
      0x1F44B, //  👋  👋  👋  👋  —   👋  waving hand
      0x1F91A, //  🤚  🤚  —   —   —   —   raised back of hand
      0x1F590, //  🖐  🖐  —   —   —   —   hand with fingers splayed
      0x270B, // ✋  ✋  ✋  ✋  ✋  ✋  raised hand
      0x1F596, //  🖖  🖖  —   —   —   —   vulcan salute
      0x1FAF1, //  🫱  🫱  —   —   —   —   rightwards hand
      0x1FAF2, //  🫲  🫲  —   —   —   —   leftwards hand
      0x1FAF3, //  🫳  🫳  —   —   —   —   palm down hand
      0x1FAF4, //  🫴  🫴  —   —   —   —   palm up hand
      0x1FAF7, //  🫷  🫷  —   —   —   —   leftwards pushing hand
      0x1FAF8, //  🫸  🫸  —   —   —   —   rightwards pushing hand
      0x1F44C, //  👌  👌  👌  👌  —   👌  OK hand
      0x1F90C, //  🤌  🤌  —   —   —   —   pinched fingers
      0x1F90F, //  🤏  🤏  —   —   —   —   pinching hand
      0x270C, // ✌   ✌   ✌   ✌   ✌   ✌   victory hand
      0x1F91E, //  🤞  🤞  —   —   —   —   crossed fingers
      0x1FAF0, //  🫰  🫰  —   —   —   —   hand with index finger and thumb crossed
      0x1F91F, //  🤟  🤟  —   —   —   —   love-you gesture
      0x1F918, //  🤘  🤘  —   —   —   —   sign of the horns
      0x1F919, //  🤙  🤙  —   —   —   —   call me hand
      0x1F448, //  👈  👈  👈  👈  —   👈  backhand index pointing left
      0x1F449, //  👉  👉  👉  👉  —   👉  backhand index pointing right
      0x1F446, //  👆  👆  👆  👆  —   👆  backhand index pointing up
      0x1F595, //  🖕  🖕  —   —   —   —   middle finger
      0x1F447, //  👇  👇  👇  👇  —   👇  backhand index pointing down
      0x261D, // ☝   ☝   ☝   ☝   —   ☝   index pointing up
      0x1FAF5, //  🫵  🫵  —   —   —   —   index pointing at the viewer
      0x1F44D, //  👍  👍  👍  👍  👍  👍  thumbs up
      0x1F44E, //  👎  👎  👎  👎  —   👎  thumbs down
      0x270A, // ✊  ✊  ✊  ✊  ✊  ✊  raised fist
      0x1F44A, //  👊  👊  👊  👊  👊  👊  oncoming fist
      0x1F91B, //  🤛  🤛  —   —   —   —   left-facing fist
      0x1F91C, //  🤜  🤜  —   —   —   —   right-facing fist
      0x1F44F, //  👏  👏  👏  👏  —   👏  clapping hands
      0x1F64C, //  🙌  🙌  🙌  🙌  —   🙌  raising hands
      0x1FAF6, //  🫶  🫶  —   —   —   —   heart hands
      0x1F450, //  👐  👐  👐  👐  —   —   open hands
      0x1F932, //  🤲  🤲  —   —   —   —   palms up together
      0x1F91D, //  🤝  🤝  —   —   —   —   handshake
      0x1F64F, //  🙏  🙏  🙏  🙏  —   🙏  folded hands
      0x270D, // ✍   ✍   —   —   —   —   writing hand
      0x1F485, //  💅  💅  💅  💅  —   💅  nail polish
      0x1F933, //  🤳  🤳  —   —   —   —   selfie
      0x1F4AA, //  💪  💪  💪  💪  —   💪  flexed biceps
      0x1F9BE, //  🦾  🦾  —   —   —   —   mechanical arm
      0x1F9BF, //  🦿  🦿  —   —   —   —   mechanical leg
      0x1F9B5, //  🦵  🦵  —   —   —   —   leg
      0x1F9B6, //  🦶  🦶  —   —   —   —   foot
      0x1F442, //  👂  👂  👂  👂  👂  👂  ear
      0x1F9BB, //  🦻  🦻  —   —   —   —   ear with hearing aid
      0x1F443, //  👃  👃  👃  👃  —   👃  nose
      0x1F9E0, //  🧠  🧠  —   —   —   —   brain
      0x1FAC0, //  🫀  🫀  —   —   —   —   anatomical heart
      0x1FAC1, //  🫁  🫁  —   —   —   —   lungs
      0x1F9B7, //  🦷  🦷  —   —   —   —   tooth
      0x1F9B4, //  🦴  🦴  —   —   —   —   bone
      0x1F440, //  👀  👀  👀  👀  👀  👀  eyes
      0x1F441, //  👁  👁  —   —   —   —   eye
      0x1F445, //  👅  👅  👅  —   —   👅  tongue
      0x1F444, //  👄  👄  👄  👄  —   👄  mouth
      0x1FAE6, //  🫦  🫦  —   —   —   —   biting lip
      0x1F476, //  👶  👶  👶  👶  —   👶  baby
      0x1F9D2, //  🧒  🧒  —   —   —   —   child
      0x1F466, //  👦  👦  👦  👦  —   —   boy
      0x1F467, //  👧  👧  👧  👧  —   —   girl
      0x1F9D1, //  🧑  🧑  —   —   —   —   person
      0x1F471, //  👱  👱  👱  👱  —   👱  person: blond hair
      0x1F468, //  👨  👨  👨  👨  —   👨  man
      0x1F9D4, //  🧔  🧔  —   —   —   —   person: beard
      0x1F469, //  👩  👩  👩  👩  —   👩  woman
      0x1F9D3, //  🧓  🧓  —   —   —   —   older person
      0x1F474, //  👴  👴  👴  👴  —   👴  old man
      0x1F475, //  👵  👵  👵  👵  —   👵  old woman
      0x1F64D, //  🙍  🙍  🙍  —   —   🙍  person frowning
      0x1F64E, //  🙎  🙎  🙎  —   —   🙎  person pouting
      0x1F645, //  🙅  🙅  🙅  🙅  —   🙅  person gesturing NO
      0x1F646, //  🙆  🙆  🙆  🙆  —   🙆  person gesturing OK
      0x1F481, //  💁  💁  💁  💁  —   —   person tipping hand
      0x1F64B, //  🙋  🙋  🙋  —   —   🙋  person raising hand
      0x1F9CF, //  🧏  🧏  —   —   —   —   deaf person
      0x1F647, //  🙇  🙇  🙇  🙇  —   🙇  person bowing
      0x1F926, //  🤦  🤦  —   —   —   —   person facepalming
      0x1F937, //  🤷  🤷  —   —   —   —   person shrugging
      0x1F46E, //  👮  👮  👮  👮  —   👮  police officer
      0x1F575, //  🕵  🕵  —   —   —   —   detective
      0x1F482, //  💂  💂  💂  💂  —   —   guard
      0x1F977, //  🥷  🥷  —   —   —   —   ninja
      0x1F477, //  👷  👷  👷  👷  —   👷  construction worker
      0x1FAC5, //  🫅  🫅  —   —   —   —   person with crown
      0x1F934, //  🤴  🤴  —   —   —   —   prince
      0x1F478, //  👸  👸  👸  👸  —   👸  princess
      0x1F473, //  👳  👳  👳  👳  —   👳  person wearing turban
      0x1F472, //  👲  👲  👲  👲  —   👲  person with skullcap
      0x1F9D5, //  🧕  🧕  —   —   —   —   woman with headscarf
      0x1F935, //  🤵  🤵  —   —   —   —   person in tuxedo
      0x1F470, //  👰  👰  👰  —   —   👰  person with veil
      0x1F930, //  🤰  🤰  —   —   —   —   pregnant woman
      0x1FAC3, //  🫃  🫃  —   —   —   —   pregnant man
      0x1FAC4, //  🫄  🫄  —   —   —   —   pregnant person
      0x1F931, //  🤱  🤱  —   —   —   —   breast-feeding
      0x1F47C, //  👼  👼  👼  👼  —   👼  baby angel
      0x1F385, //  🎅  🎅  🎅  🎅  —   🎅  Santa Claus
      0x1F936, //  🤶  🤶  —   —   —   —   Mrs. Claus
      0x1F9B8, //  🦸  🦸  —   —   —   —   superhero
      0x1F9B9, //  🦹  🦹  —   —   —   —   supervillain
      0x1F9D9, //  🧙  🧙  —   —   —   —   mage
      0x1F9DA, //  🧚  🧚  —   —   —   —   fairy
      0x1F9DB, //  🧛  🧛  —   —   —   —   vampire
      0x1F9DC, //  🧜  🧜  —   —   —   —   merperson
      0x1F9DD, //  🧝  🧝  —   —   —   —   elf
      0x1F9DE, //  🧞  🧞  —   —   —   —   genie
      0x1F9DF, //  🧟  🧟  —   —   —   —   zombie
      0x1F9CC, //  🧌  🧌  —   —   —   —   troll
      0x1F486, //  💆  💆  💆  💆  —   💆  person getting massage
      0x1F487, //  💇  💇  💇  💇  —   💇  person getting haircut
      0x1F6B6, //  🚶  🚶  🚶  🚶  —   🚶  person walking
      0x1F9CD, //  🧍  🧍  —   —   —   —   person standing
      0x1F9CE, //  🧎  🧎  —   —   —   —   person kneeling
      0x1F3C3, //  🏃  🏃  🏃  🏃  🏃  🏃  person running
      0x1F483, //  💃  💃  💃  💃  —   💃  woman dancing
      0x1F57A, //  🕺  🕺  —   —   —   —   man dancing
      0x1F574, //  🕴  🕴  —   —   —   —   person in suit levitating
      0x1F46F, //  👯  👯  👯  👯  —   👯  people with bunny ears
      0x1F9D6, //  🧖  🧖  —   —   —   —   person in steamy room
      0x1F9D7, //  🧗  🧗  —   —   —   —   person climbing
      0x1F93A, //  🤺  🤺  —   —   —   —   person fencing
      0x1F3C7, //  🏇  🏇  —   —   —   —   horse racing
      0x26F7, // ⛷   ⛷   —   —   —   —   skier
      0x1F3C2, //  🏂  🏂  🏂  —   🏂  🏂  snowboarder
      0x1F3CC, //  🏌  🏌  —   —   —   —   person golfing
      0x1F3C4, //  🏄  🏄  🏄  🏄  —   🏄  person surfing
      0x1F6A3, //  🚣  🚣  —   —   —   —   person rowing boat
      0x1F3CA, //  🏊  🏊  🏊  🏊  —   🏊  person swimming
      0x26F9, // ⛹   ⛹   —   —   —   —   person bouncing ball
      0x1F3CB, //  🏋  🏋  —   —   —   —   person lifting weights
      0x1F6B4, //  🚴  🚴  —   —   —   —   person biking
      0x1F6B5, //  🚵  🚵  —   —   —   —   person mountain biking
      0x1F938, //  🤸  🤸  —   —   —   —   person cartwheeling
      0x1F93C, //  🤼  🤼  —   —   —   —   people wrestling
      0x1F93D, //  🤽  🤽  —   —   —   —   person playing water polo
      0x1F93E, //  🤾  🤾  —   —   —   —   person playing handball
      0x1F939, //  🤹  🤹  —   —   —   —   person juggling
      0x1F9D8, //  🧘  🧘  —   —   —   —   person in lotus position
      0x1F6C0, //  🛀  🛀  🛀  🛀  —   🛀  person taking bath
      0x1F6CC, //  🛌  🛌  —   —   —   —   person in bed
      0x1F46D, //  👭  👭  —   —   —   —   women holding hands
      0x1F46B, //  👫  👫  👫  👫  —   —   woman and man holding hands
      0x1F46C, //  👬  👬  —   —   —   —   men holding hands
      0x1F48F, //  💏  💏  💏  💏  —   💏  kiss
      0x1F491, //  💑  💑  💑  💑  —   💑  couple with heart
      0x1F5E3, //  🗣  🗣  —   —   —   —   speaking head
      0x1F464, //  👤  👤  👤  —   👤  —   bust in silhouette
      0x1F465, //  👥  👥  —   —   —   —   busts in silhouette
      0x1FAC2, //  🫂  🫂  —   —   —   —   people hugging
      0x1F46A, //  👪  👪  👪  —   —   👪  family
      0x1F463, //  👣  👣  👣  👣  👣  👣  footprints
      0x1F9B0, //  🦰  🦰  —   —   —   —   red hair
      0x1F9B1, //  🦱  🦱  —   —   —   —   curly hair
      0x1F9B3, //  🦳  🦳  —   —   —   —   white hair
      0x1F9B2, //  🦲  🦲  —   —   —   —   bald
      0x1F435, //  🐵  🐵  🐵  🐵  —   🐵  monkey face
      0x1F412, //  🐒  🐒  🐒  🐒  —   —   monkey
      0x1F98D, //  🦍  🦍  —   —   —   —   gorilla
      0x1F9A7, //  🦧  🦧  —   —   —   —   orangutan
      0x1F436, //  🐶  🐶  🐶  🐶  🐶  🐶  dog face
      0x1F415, //  🐕  🐕  —   —   —   —   dog
      0x1F9AE, //  🦮  🦮  —   —   —   —   guide dog
      0x1F429, //  🐩  🐩  🐩  —   —   🐩  poodle
      0x1F43A, //  🐺  🐺  🐺  🐺  —   —   wolf
      0x1F98A, //  🦊  🦊  —   —   —   —   fox
      0x1F99D, //  🦝  🦝  —   —   —   —   raccoon
      0x1F431, //  🐱  🐱  🐱  🐱  🐱  🐱  cat face
      0x1F408, //  🐈  🐈  —   —   —   —   cat
      0x1F981, //  🦁  🦁  —   —   —   —   lion
      0x1F42F, //  🐯  🐯  🐯  🐯  —   🐯  tiger face
      0x1F405, //  🐅  🐅  —   —   —   —   tiger
      0x1F406, //  🐆  🐆  —   —   —   —   leopard
      0x1F434, //  🐴  🐴  🐴  🐴  🐴  🐴  horse face
      0x1FACE, //  🫎  🫎  —   —   —   —   moose
      0x1FACF, //  🫏  🫏  —   —   —   —   donkey
      0x1F40E, //  🐎  🐎  🐎  🐎  —   —   horse
      0x1F984, //  🦄  🦄  —   —   —   —   unicorn
      0x1F993, //  🦓  🦓  —   —   —   —   zebra
      0x1F98C, //  🦌  🦌  —   —   —   —   deer
      0x1F9AC, //  🦬  🦬  —   —   —   —   bison
      0x1F42E, //  🐮  🐮  🐮  🐮  —   🐮  cow face
      0x1F402, //  🐂  🐂  —   —   —   —   ox
      0x1F403, //  🐃  🐃  —   —   —   —   water buffalo
      0x1F404, //  🐄  🐄  —   —   —   —   cow
      0x1F437, //  🐷  🐷  🐷  🐷  🐷  🐷  pig face
      0x1F416, //  🐖  🐖  —   —   —   —   pig
      0x1F417, //  🐗  🐗  🐗  🐗  —   🐗  boar
      0x1F43D, //  🐽  🐽  🐽  —   —   🐽  pig nose
      0x1F40F, //  🐏  🐏  —   —   —   —   ram
      0x1F411, //  🐑  🐑  🐑  🐑  —   —   ewe
      0x1F410, //  🐐  🐐  —   —   —   —   goat
      0x1F42A, //  🐪  🐪  —   —   —   —   camel
      0x1F42B, //  🐫  🐫  🐫  🐫  —   🐫  two-hump camel
      0x1F999, //  🦙  🦙  —   —   —   —   llama
      0x1F992, //  🦒  🦒  —   —   —   —   giraffe
      0x1F418, //  🐘  🐘  🐘  🐘  —   🐘  elephant
      0x1F9A3, //  🦣  🦣  —   —   —   —   mammoth
      0x1F98F, //  🦏  🦏  —   —   —   —   rhinoceros
      0x1F99B, //  🦛  🦛  —   —   —   —   hippopotamus
      0x1F42D, //  🐭  🐭  🐭  🐭  —   🐭  mouse face
      0x1F401, //  🐁  🐁  —   —   —   —   mouse
      0x1F400, //  🐀  🐀  —   —   —   —   rat
      0x1F439, //  🐹  🐹  🐹  🐹  —   —   hamster
      0x1F430, //  🐰  🐰  🐰  🐰  —   🐰  rabbit face
      0x1F407, //  🐇  🐇  —   —   —   —   rabbit
      0x1F43F, //  🐿  🐿  —   —   —   —   chipmunk
      0x1F9AB, //  🦫  🦫  —   —   —   —   beaver
      0x1F994, //  🦔  🦔  —   —   —   —   hedgehog
      0x1F987, //  🦇  🦇  —   —   —   —   bat
      0x1F43B, //  🐻  🐻  🐻  🐻  —   🐻  bear
      0x1F428, //  🐨  🐨  🐨  🐨  —   🐨  koala
      0x1F43C, //  🐼  🐼  🐼  —   —   🐼  panda
      0x1F9A5, //  🦥  🦥  —   —   —   —   sloth
      0x1F9A6, //  🦦  🦦  —   —   —   —   otter
      0x1F9A8, //  🦨  🦨  —   —   —   —   skunk
      0x1F998, //  🦘  🦘  —   —   —   —   kangaroo
      0x1F9A1, //  🦡  🦡  —   —   —   —   badger
      0x1F43E, //  🐾  🐾  🐾  —   —   🐾  paw prints
      0x1F983, //  🦃  🦃  —   —   —   —   turkey
      0x1F414, //  🐔  🐔  🐔  🐔  —   🐔  chicken
      0x1F413, //  🐓  🐓  —   —   —   —   rooster
      0x1F423, //  🐣  🐣  🐣  —   —   🐣  hatching chick
      0x1F424, //  🐤  🐤  🐤  🐤  🐤  🐤  baby chick
      0x1F425, //  🐥  🐥  🐥  —   —   🐥  front-facing baby chick
      0x1F426, //  🐦  🐦  🐦  🐦  —   —   bird
      0x1F427, //  🐧  🐧  🐧  🐧  🐧  🐧  penguin
      0x1F54A, //  🕊  🕊  —   —   —   —   dove
      0x1F985, //  🦅  🦅  —   —   —   —   eagle
      0x1F986, //  🦆  🦆  —   —   —   —   duck
      0x1F9A2, //  🦢  🦢  —   —   —   —   swan
      0x1F989, //  🦉  🦉  —   —   —   —   owl
      0x1F9A4, //  🦤  🦤  —   —   —   —   dodo
      0x1FAB6, //  🪶  🪶  —   —   —   —   feather
      0x1F9A9, //  🦩  🦩  —   —   —   —   flamingo
      0x1F99A, //  🦚  🦚  —   —   —   —   peacock
      0x1F99C, //  🦜  🦜  —   —   —   —   parrot
      0x1FABD, //  🪽  🪽  —   —   —   —   wing
      0x1FABF, //  🪿  🪿  —   —   —   —   goose
      0x1F438, //  🐸  🐸  🐸  🐸  —   🐸  frog
      0x1F40A, //  🐊  🐊  —   —   —   —   crocodile
      0x1F422, //  🐢  🐢  🐢  —   —   🐢  turtle
      0x1F98E, //  🦎  🦎  —   —   —   —   lizard
      0x1F40D, //  🐍  🐍  🐍  🐍  —   🐍  snake
      0x1F432, //  🐲  🐲  🐲  —   —   🐲  dragon face
      0x1F409, //  🐉  🐉  —   —   —   —   dragon
      0x1F995, //  🦕  🦕  —   —   —   —   sauropod
      0x1F996, //  🦖  🦖  —   —   —   —   T-Rex
      0x1F433, //  🐳  🐳  🐳  🐳  —   🐳  spouting whale
      0x1F40B, //  🐋  🐋  —   —   —   —   whale
      0x1F42C, //  🐬  🐬  🐬  🐬  —   🐬  dolphin
      0x1F9AD, //  🦭  🦭  —   —   —   —   seal
      0x1F41F, //  🐟  🐟  🐟  🐟  🐟  —   fish
      0x1F420, //  🐠  🐠  🐠  🐠  —   🐠  tropical fish
      0x1F421, //  🐡  🐡  🐡  —   —   🐡  blowfish
      0x1F988, //  🦈  🦈  —   —   —   —   shark
      0x1F419, //  🐙  🐙  🐙  🐙  —   🐙  octopus
      0x1F41A, //  🐚  🐚  🐚  🐚  —   🐚  spiral shell
      0x1FAB8, //  🪸  🪸  —   —   —   —   coral
      0x1FABC, //  🪼  🪼  —   —   —   —   jellyfish
      0x1F40C, //  🐌  🐌  🐌  —   🐌  🐌  snail
      0x1F98B, //  🦋  🦋  —   —   —   —   butterfly
      0x1F41B, //  🐛  🐛  🐛  🐛  —   🐛  bug
      0x1F41C, //  🐜  🐜  🐜  —   —   🐜  ant
      0x1F41D, //  🐝  🐝  🐝  —   —   🐝  honeybee
      0x1FAB2, //  🪲  🪲  —   —   —   —   beetle
      0x1F41E, //  🐞  🐞  🐞  —   —   🐞  lady beetle
      0x1F997, //  🦗  🦗  —   —   —   —   cricket
      0x1FAB3, //  🪳  🪳  —   —   —   —   cockroach
      0x1F577, //  🕷  🕷  —   —   —   —   spider
      0x1F578, //  🕸  🕸  —   —   —   —   spider web
      0x1F982, //  🦂  🦂  —   —   —   —   scorpion
      0x1F99F, //  🦟  🦟  —   —   —   —   mosquito
      0x1FAB0, //  🪰  🪰  —   —   —   —   fly
      0x1FAB1, //  🪱  🪱  —   —   —   —   worm
      0x1F9A0, //  🦠  🦠  —   —   —   —   microbe
      0x1F490, //  💐  💐  💐  💐  —   💐  bouquet
      0x1F338, //  🌸  🌸  🌸  🌸  🌸  🌸  cherry blossom
      0x1F4AE, //  💮  💮  💮  —   —   💮  white flower
      0x1FAB7, //  🪷  🪷  —   —   —   —   lotus
      0x1F3F5, //  🏵  🏵  —   —   —   —   rosette
      0x1F339, //  🌹  🌹  🌹  🌹  —   🌹  rose
      0x1F940, //  🥀  🥀  —   —   —   —   wilted flower
      0x1F33A, //  🌺  🌺  🌺  🌺  —   🌺  hibiscus
      0x1F33B, //  🌻  🌻  🌻  🌻  —   🌻  sunflower
      0x1F33C, //  🌼  🌼  🌼  —   —   🌼  blossom
      0x1F337, //  🌷  🌷  🌷  🌷  🌷  🌷  tulip
      0x1FABB, //  🪻  🪻  —   —   —   —   hyacinth
      0x1F331, //  🌱  🌱  🌱  —   🌱  🌱  seedling
      0x1FAB4, //  🪴  🪴  —   —   —   —   potted plant
      0x1F332, //  🌲  🌲  —   —   —   —   evergreen tree
      0x1F333, //  🌳  🌳  —   —   —   —   deciduous tree
      0x1F334, //  🌴  🌴  🌴  🌴  —   🌴  palm tree
      0x1F335, //  🌵  🌵  🌵  🌵  —   🌵  cactus
      0x1F33E, //  🌾  🌾  🌾  🌾  —   —   sheaf of rice
      0x1F33F, //  🌿  🌿  🌿  —   —   🌿  herb
      0x2618, // ☘   ☘   —   —   —   —   shamrock
      0x1F340, //  🍀  🍀  🍀  🍀  🍀  🍀  four leaf clover
      0x1F341, //  🍁  🍁  🍁  🍁  🍁  🍁  maple leaf
      0x1F342, //  🍂  🍂  🍂  🍂  —   🍂  fallen leaf
      0x1F343, //  🍃  🍃  🍃  🍃  —   —   leaf fluttering in wind
      0x1FAB9, //  🪹  🪹  —   —   —   —   empty nest
      0x1FABA, //  🪺  🪺  —   —   —   —   nest with eggs
      0x1F344, //  🍄  🍄  🍄  —   —   🍄  mushroom
      0x1F347, //  🍇  🍇  🍇  —   —   🍇  grapes
      0x1F348, //  🍈  🍈  🍈  —   —   🍈  melon
      0x1F349, //  🍉  🍉  🍉  🍉  —   🍉  watermelon
      0x1F34A, //  🍊  🍊  🍊  🍊  —   🍊  tangerine
      0x1F34B, //  🍋  🍋  —   —   —   —   lemon
      0x1F34C, //  🍌  🍌  🍌  —   🍌  🍌  banana
      0x1F34D, //  🍍  🍍  🍍  —   —   🍍  pineapple
      0x1F96D, //  🥭  🥭  —   —   —   —   mango
      0x1F34E, //  🍎  🍎  🍎  🍎  🍎  🍎  red apple
      0x1F34F, //  🍏  🍏  🍏  —   —   🍏  green apple
      0x1F350, //  🍐  🍐  —   —   —   —   pear
      0x1F351, //  🍑  🍑  🍑  —   —   🍑  peach
      0x1F352, //  🍒  🍒  🍒  —   🍒  🍒  cherries
      0x1F353, //  🍓  🍓  🍓  🍓  —   🍓  strawberry
      0x1FAD0, //  🫐  🫐  —   —   —   —   blueberries
      0x1F95D, //  🥝  🥝  —   —   —   —   kiwi fruit
      0x1F345, //  🍅  🍅  🍅  🍅  —   🍅  tomato
      0x1FAD2, //  🫒  🫒  —   —   —   —   olive
      0x1F965, //  🥥  🥥  —   —   —   —   coconut
      0x1F951, //  🥑  🥑  —   —   —   —   avocado
      0x1F346, //  🍆  🍆  🍆  🍆  —   🍆  eggplant
      0x1F954, //  🥔  🥔  —   —   —   —   potato
      0x1F955, //  🥕  🥕  —   —   —   —   carrot
      0x1F33D, //  🌽  🌽  🌽  —   —   🌽  ear of corn
      0x1F336, //  🌶  🌶  —   —   —   —   hot pepper
      0x1FAD1, //  🫑  🫑  —   —   —   —   bell pepper
      0x1F952, //  🥒  🥒  —   —   —   —   cucumber
      0x1F96C, //  🥬  🥬  —   —   —   —   leafy green
      0x1F966, //  🥦  🥦  —   —   —   —   broccoli
      0x1F9C4, //  🧄  🧄  —   —   —   —   garlic
      0x1F9C5, //  🧅  🧅  —   —   —   —   onion
      0x1F95C, //  🥜  🥜  —   —   —   —   peanuts
      0x1FAD8, //  🫘  🫘  —   —   —   —   beans
      0x1F330, //  🌰  🌰  🌰  —   —   🌰  chestnut
      0x1FADA, //  🫚  🫚  —   —   —   —   ginger root
      0x1FADB, //  🫛  🫛  —   —   —   —   pea pod
      0x1F35E, //  🍞  🍞  🍞  🍞  🍞  🍞  bread
      0x1F950, //  🥐  🥐  —   —   —   —   croissant
      0x1F956, //  🥖  🥖  —   —   —   —   baguette bread
      0x1FAD3, //  🫓  🫓  —   —   —   —   flatbread
      0x1F968, //  🥨  🥨  —   —   —   —   pretzel
      0x1F96F, //  🥯  🥯  —   —   —   —   bagel
      0x1F95E, //  🥞  🥞  —   —   —   —   pancakes
      0x1F9C7, //  🧇  🧇  —   —   —   —   waffle
      0x1F9C0, //  🧀  🧀  —   —   —   —   cheese wedge
      0x1F356, //  🍖  🍖  🍖  —   —   🍖  meat on bone
      0x1F357, //  🍗  🍗  🍗  —   —   🍗  poultry leg
      0x1F969, //  🥩  🥩  —   —   —   —   cut of meat
      0x1F953, //  🥓  🥓  —   —   —   —   bacon
      0x1F354, //  🍔  🍔  🍔  🍔  🍔  🍔  hamburger
      0x1F35F, //  🍟  🍟  🍟  🍟  —   🍟  french fries
      0x1F355, //  🍕  🍕  🍕  —   —   🍕  pizza
      0x1F32D, //  🌭  🌭  —   —   —   —   hot dog
      0x1F96A, //  🥪  🥪  —   —   —   —   sandwich
      0x1F32E, //  🌮  🌮  —   —   —   —   taco
      0x1F32F, //  🌯  🌯  —   —   —   —   burrito
      0x1FAD4, //  🫔  🫔  —   —   —   —   tamale
      0x1F959, //  🥙  🥙  —   —   —   —   stuffed flatbread
      0x1F9C6, //  🧆  🧆  —   —   —   —   falafel
      0x1F95A, //  🥚  🥚  —   —   —   —   egg
      0x1F373, //  🍳  🍳  🍳  🍳  —   🍳  cooking
      0x1F958, //  🥘  🥘  —   —   —   —   shallow pan of food
      0x1F372, //  🍲  🍲  🍲  🍲  —   🍲  pot of food
      0x1FAD5, //  🫕  🫕  —   —   —   —   fondue
      0x1F963, //  🥣  🥣  —   —   —   —   bowl with spoon
      0x1F957, //  🥗  🥗  —   —   —   —   green salad
      0x1F37F, //  🍿  🍿  —   —   —   —   popcorn
      0x1F9C8, //  🧈  🧈  —   —   —   —   butter
      0x1F9C2, //  🧂  🧂  —   —   —   —   salt
      0x1F96B, //  🥫  🥫  —   —   —   —   canned food
      0x1F371, //  🍱  🍱  🍱  🍱  —   🍱  bento box
      0x1F358, //  🍘  🍘  🍘  🍘  —   🍘  rice cracker
      0x1F359, //  🍙  🍙  🍙  🍙  🍙  🍙  rice ball
      0x1F35A, //  🍚  🍚  🍚  🍚  —   🍚  cooked rice
      0x1F35B, //  🍛  🍛  🍛  🍛  —   🍛  curry rice
      0x1F35C, //  🍜  🍜  🍜  🍜  🍜  🍜  steaming bowl
      0x1F35D, //  🍝  🍝  🍝  🍝  —   🍝  spaghetti
      0x1F360, //  🍠  🍠  🍠  —   —   🍠  roasted sweet potato
      0x1F362, //  🍢  🍢  🍢  🍢  —   🍢  oden
      0x1F363, //  🍣  🍣  🍣  🍣  —   🍣  sushi
      0x1F364, //  🍤  🍤  🍤  —   —   🍤  fried shrimp
      0x1F365, //  🍥  🍥  🍥  —   —   🍥  fish cake with swirl
      0x1F96E, //  🥮  🥮  —   —   —   —   moon cake
      0x1F361, //  🍡  🍡  🍡  🍡  —   🍡  dango
      0x1F95F, //  🥟  🥟  —   —   —   —   dumpling
      0x1F960, //  🥠  🥠  —   —   —   —   fortune cookie
      0x1F961, //  🥡  🥡  —   —   —   —   takeout box
      0x1F980, //  🦀  🦀  —   —   —   —   crab
      0x1F99E, //  🦞  🦞  —   —   —   —   lobster
      0x1F990, //  🦐  🦐  —   —   —   —   shrimp
      0x1F991, //  🦑  🦑  —   —   —   —   squid
      0x1F9AA, //  🦪  🦪  —   —   —   —   oyster
      0x1F366, //  🍦  🍦  🍦  🍦  —   🍦  soft ice cream
      0x1F367, //  🍧  🍧  🍧  🍧  —   🍧  shaved ice
      0x1F368, //  🍨  🍨  🍨  —   —   🍨  ice cream
      0x1F369, //  🍩  🍩  🍩  —   —   🍩  doughnut
      0x1F36A, //  🍪  🍪  🍪  —   —   🍪  cookie
      0x1F382, //  🎂  🎂  🎂  🎂  🎂  🎂  birthday cake
      0x1F370, //  🍰  🍰  🍰  🍰  🍰  🍰  shortcake
      0x1F9C1, //  🧁  🧁  —   —   —   —   cupcake
      0x1F967, //  🥧  🥧  —   —   —   —   pie
      0x1F36B, //  🍫  🍫  🍫  —   —   🍫  chocolate bar
      0x1F36C, //  🍬  🍬  🍬  —   —   🍬  candy
      0x1F36D, //  🍭  🍭  🍭  —   —   🍭  lollipop
      0x1F36E, //  🍮  🍮  🍮  —   —   🍮  custard
      0x1F36F, //  🍯  🍯  🍯  —   —   🍯  honey pot
      0x1F37C, //  🍼  🍼  —   —   —   —   baby bottle
      0x1F95B, //  🥛  🥛  —   —   —   —   glass of milk
      0x2615, // ☕  ☕  ☕  ☕  ☕  ☕  hot beverage
      0x1FAD6, //  🫖  🫖  —   —   —   —   teapot
      0x1F375, //  🍵  🍵  🍵  🍵  🍵  🍵  teacup without handle
      0x1F376, //  🍶  🍶  🍶  🍶  🍶  🍶  sake
      0x1F37E, //  🍾  🍾  —   —   —   —   bottle with popping cork
      0x1F377, //  🍷  🍷  🍷  —   🍷  🍷  wine glass
      0x1F378, //  🍸  🍸  🍸  🍸  🍸  🍸  cocktail glass
      0x1F379, //  🍹  🍹  🍹  —   —   🍹  tropical drink
      0x1F37A, //  🍺  🍺  🍺  🍺  🍺  🍺  beer mug
      0x1F37B, //  🍻  🍻  🍻  🍻  —   🍻  clinking beer mugs
      0x1F942, //  🥂  🥂  —   —   —   —   clinking glasses
      0x1F943, //  🥃  🥃  —   —   —   —   tumbler glass
      0x1FAD7, //  🫗  🫗  —   —   —   —   pouring liquid
      0x1F964, //  🥤  🥤  —   —   —   —   cup with straw
      0x1F9CB, //  🧋  🧋  —   —   —   —   bubble tea
      0x1F9C3, //  🧃  🧃  —   —   —   —   beverage box
      0x1F9C9, //  🧉  🧉  —   —   —   —   mate
      0x1F9CA, //  🧊  🧊  —   —   —   —   ice
      0x1F962, //  🥢  🥢  —   —   —   —   chopsticks
      0x1F37D, //  🍽  🍽  —   —   —   —   fork and knife with plate
      0x1F374, //  🍴  🍴  🍴  🍴  🍴  🍴  fork and knife
      0x1F944, //  🥄  🥄  —   —   —   —   spoon
      0x1F52A, //  🔪  🔪  🔪  —   —   🔪  kitchen knife
      0x1FAD9, //  🫙  🫙  —   —   —   —   jar
      0x1F3FA, //  🏺  🏺  —   —   —   —   amphora
      0x1F30D, //  🌍  🌍  —   —   —   —   globe showing Europe-Africa
      0x1F30E, //  🌎  🌎  —   —   —   —   globe showing Americas
      0x1F30F, //  🌏  🌏  🌏  —   —   🌏  globe showing Asia-Australia
      0x1F310, //  🌐  🌐  —   —   —   —   globe with meridians
      0x1F5FA, //  🗺  🗺  —   —   —   —   world map
      0x1F5FE, //  🗾  🗾  🗾  —   —   🗾  map of Japan
      0x1F9ED, //  🧭  🧭  —   —   —   —   compass
      0x1F3D4, //  🏔  🏔  —   —   —   —   snow-capped mountain
      0x26F0, // ⛰   ⛰   —   —   —   —   mountain
      0x1F30B, //  🌋  🌋  🌋  —   —   🌋  volcano
      0x1F5FB, //  🗻  🗻  🗻  🗻  🗻  🗻  mount fuji
      0x1F3D5, //  🏕  🏕  —   —   —   —   camping
      0x1F3D6, //  🏖  🏖  —   —   —   —   beach with umbrella
      0x1F3DC, //  🏜  🏜  —   —   —   —   desert
      0x1F3DD, //  🏝  🏝  —   —   —   —   desert island
      0x1F3DE, //  🏞  🏞  —   —   —   —   national park
      0x1F3DF, //  🏟  🏟  —   —   —   —   stadium
      0x1F3DB, //  🏛  🏛  —   —   —   —   classical building
      0x1F3D7, //  🏗  🏗  —   —   —   —   building construction
      0x1F9F1, //  🧱  🧱  —   —   —   —   brick
      0x1FAA8, //  🪨  🪨  —   —   —   —   rock
      0x1FAB5, //  🪵  🪵  —   —   —   —   wood
      0x1F6D6, //  🛖  🛖  —   —   —   —   hut
      0x1F3D8, //  🏘  🏘  —   —   —   —   houses
      0x1F3DA, //  🏚  🏚  —   —   —   —   derelict house
      0x1F3E0, //  🏠  🏠  🏠  🏠  🏠  🏠  house
      0x1F3E1, //  🏡  🏡  🏡  —   —   🏡  house with garden
      0x1F3E2, //  🏢  🏢  🏢  🏢  🏢  🏢  office building
      0x1F3E3, //  🏣  🏣  🏣  🏣  🏣  🏣  Japanese post office
      0x1F3E4, //  🏤  🏤  —   —   —   —   post office
      0x1F3E5, //  🏥  🏥  🏥  🏥  🏥  🏥  hospital
      0x1F3E6, //  🏦  🏦  🏦  🏦  🏦  🏦  bank
      0x1F3E8, //  🏨  🏨  🏨  🏨  🏨  🏨  hotel
      0x1F3E9, //  🏩  🏩  🏩  🏩  —   🏩  love hotel
      0x1F3EA, //  🏪  🏪  🏪  🏪  🏪  🏪  convenience store
      0x1F3EB, //  🏫  🏫  🏫  🏫  🏫  🏫  school
      0x1F3EC, //  🏬  🏬  🏬  🏬  —   🏬  department store
      0x1F3ED, //  🏭  🏭  🏭  🏭  —   🏭  factory
      0x1F3EF, //  🏯  🏯  🏯  🏯  —   🏯  Japanese castle
      0x1F3F0, //  🏰  🏰  🏰  🏰  —   🏰  castle
      0x1F492, //  💒  💒  💒  💒  —   —   wedding
      0x1F5FC, //  🗼  🗼  🗼  🗼  —   🗼  Tokyo tower
      0x1F5FD, //  🗽  🗽  🗽  🗽  —   —   Statue of Liberty
      0x26EA, // ⛪  ⛪  ⛪  ⛪  —   ⛪  church
      0x1F54C, //  🕌  🕌  —   —   —   —   mosque
      0x1F6D5, //  🛕  🛕  —   —   —   —   hindu temple
      0x1F54D, //  🕍  🕍  —   —   —   —   synagogue
      0x26E9, // ⛩   ⛩   —   —   —   —   shinto shrine
      0x1F54B, //  🕋  🕋  —   —   —   —   kaaba
      0x26F2, // ⛲  ⛲  ⛲  ⛲  —   ⛲  fountain
      0x26FA, // ⛺  ⛺  ⛺  ⛺  —   ⛺  tent
      0x1F301, //  🌁  🌁  🌁  —   🌁  🌁  foggy
      0x1F303, //  🌃  🌃  🌃  🌃  🌃  🌃  night with stars
      0x1F3D9, //  🏙  🏙  —   —   —   —   cityscape
      0x1F304, //  🌄  🌄  🌄  🌄  —   —   sunrise over mountains
      0x1F305, //  🌅  🌅  🌅  🌅  —   🌅  sunrise
      0x1F306, //  🌆  🌆  🌆  🌆  —   🌆  cityscape at dusk
      0x1F307, //  🌇  🌇  🌇  🌇  —   —   sunset
      0x1F309, //  🌉  🌉  🌉  —   —   🌉  bridge at night
      0x2668, // ♨   ♨   ♨   ♨   ♨   ♨   hot springs
      0x1F3A0, //  🎠  🎠  🎠  —   🎠  —   carousel horse
      0x1F6DD, //  🛝  🛝  —   —   —   —   playground slide
      0x1F3A1, //  🎡  🎡  🎡  🎡  —   🎡  ferris wheel
      0x1F3A2, //  🎢  🎢  🎢  🎢  —   🎢  roller coaster
      0x1F488, //  💈  💈  💈  💈  —   💈  barber pole
      0x1F3AA, //  🎪  🎪  🎪  —   🎪  🎪  circus tent
      0x1F682, //  🚂  🚂  —   —   —   —   locomotive
      0x1F683, //  🚃  🚃  🚃  🚃  🚃  🚃  railway car
      0x1F684, //  🚄  🚄  🚄  🚄  🚄  —   high-speed train
      0x1F685, //  🚅  🚅  🚅  🚅  —   🚅  bullet train
      0x1F686, //  🚆  🚆  —   —   —   —   train
      0x1F687, //  🚇  🚇  🚇  🚇  —   🚇  metro
      0x1F688, //  🚈  🚈  —   —   —   —   light rail
      0x1F689, //  🚉  🚉  🚉  🚉  —   🚉  station
      0x1F68A, //  🚊  🚊  —   —   —   —   tram
      0x1F69D, //  🚝  🚝  —   —   —   —   monorail
      0x1F69E, //  🚞  🚞  —   —   —   —   mountain railway
      0x1F68B, //  🚋  🚋  —   —   —   —   tram car
      0x1F68C, //  🚌  🚌  🚌  🚌  🚌  🚌  bus
      0x1F68D, //  🚍  🚍  —   —   —   —   oncoming bus
      0x1F68E, //  🚎  🚎  —   —   —   —   trolleybus
      0x1F690, //  🚐  🚐  —   —   —   —   minibus
      0x1F691, //  🚑  🚑  🚑  🚑  —   🚑  ambulance
      0x1F692, //  🚒  🚒  🚒  🚒  —   🚒  fire engine
      0x1F693, //  🚓  🚓  🚓  🚓  —   🚓  police car
      0x1F694, //  🚔  🚔  —   —   —   —   oncoming police car
      0x1F695, //  🚕  🚕  🚕  🚕  —   —   taxi
      0x1F696, //  🚖  🚖  —   —   —   —   oncoming taxi
      0x1F697, //  🚗  🚗  🚗  🚗  🚗  🚗  automobile
      0x1F698, //  🚘  🚘  —   —   —   —   oncoming automobile
      0x1F699, //  🚙  🚙  🚙  🚙  🚙  —   sport utility vehicle
      0x1F6FB, //  🛻  🛻  —   —   —   —   pickup truck
      0x1F69A, //  🚚  🚚  🚚  🚚  —   🚚  delivery truck
      0x1F69B, //  🚛  🚛  —   —   —   —   articulated lorry
      0x1F69C, //  🚜  🚜  —   —   —   —   tractor
      0x1F3CE, //  🏎  🏎  —   —   —   —   racing car
      0x1F3CD, //  🏍  🏍  —   —   —   —   motorcycle
      0x1F6F5, //  🛵  🛵  —   —   —   —   motor scooter
      0x1F9BD, //  🦽  🦽  —   —   —   —   manual wheelchair
      0x1F9BC, //  🦼  🦼  —   —   —   —   motorized wheelchair
      0x1F6FA, //  🛺  🛺  —   —   —   —   auto rickshaw
      0x1F6B2, //  🚲  🚲  🚲  🚲  🚲  🚲  bicycle
      0x1F6F4, //  🛴  🛴  —   —   —   —   kick scooter
      0x1F6F9, //  🛹  🛹  —   —   —   —   skateboard
      0x1F6FC, //  🛼  🛼  —   —   —   —   roller skate
      0x1F68F, //  🚏  🚏  🚏  🚏  —   🚏  bus stop
      0x1F6E3, //  🛣  🛣  —   —   —   —   motorway
      0x1F6E4, //  🛤  🛤  —   —   —   —   railway track
      0x1F6E2, //  🛢  🛢  —   —   —   —   oil drum
      0x26FD, // ⛽  ⛽  ⛽  ⛽  ⛽  ⛽  fuel pump
      0x1F6DE, //  🛞  🛞  —   —   —   —   wheel
      0x1F6A8, //  🚨  🚨  🚨  —   —   🚨  police car light
      0x1F6A5, //  🚥  🚥  🚥  🚥  🚥  🚥  horizontal traffic light
      0x1F6A6, //  🚦  🚦  —   —   —   —   vertical traffic light
      0x1F6D1, //  🛑  🛑  —   —   —   —   stop sign
      0x1F6A7, //  🚧  🚧  🚧  🚧  —   🚧  construction
      0x2693, // ⚓  ⚓  ⚓  —   —   ⚓  anchor
      0x1F6DF, //  🛟  🛟  —   —   —   —   ring buoy
      0x26F5, // ⛵  ⛵  ⛵  ⛵  ⛵  ⛵  sailboat
      0x1F6F6, //  🛶  🛶  —   —   —   —   canoe
      0x1F6A4, //  🚤  🚤  🚤  🚤  —   —   speedboat
      0x1F6F3, //  🛳  🛳  —   —   —   —   passenger ship
      0x26F4, // ⛴   ⛴   —   —   —   —   ferry
      0x1F6E5, //  🛥  🛥  —   —   —   —   motor boat
      0x1F6A2, //  🚢  🚢  🚢  🚢  🚢  🚢  ship
      0x2708, // ✈   ✈   ✈   ✈   ✈   ✈   airplane
      0x1F6E9, //  🛩  🛩  —   —   —   —   small airplane
      0x1F6EB, //  🛫  🛫  —   —   —   —   airplane departure
      0x1F6EC, //  🛬  🛬  —   —   —   —   airplane arrival
      0x1FA82, //  🪂  🪂  —   —   —   —   parachute
      0x1F4BA, //  💺  💺  💺  💺  💺  —   seat
      0x1F681, //  🚁  🚁  —   —   —   —   helicopter
      0x1F69F, //  🚟  🚟  —   —   —   —   suspension railway
      0x1F6A0, //  🚠  🚠  —   —   —   —   mountain cableway
      0x1F6A1, //  🚡  🚡  —   —   —   —   aerial tramway
      0x1F6F0, //  🛰  🛰  —   —   —   —   satellite
      0x1F680, //  🚀  🚀  🚀  🚀  —   🚀  rocket
      0x1F6F8, //  🛸  🛸  —   —   —   —   flying saucer
      0x1F6CE, //  🛎  🛎  —   —   —   —   bellhop bell
      0x1F9F3, //  🧳  🧳  —   —   —   —   luggage
      0x231B, // ⌛  ⌛  ⌛  —   —   ⌛  hourglass done
      0x23F3, // ⏳  ⏳  ⏳  —   ⏳  ⏳  hourglass not done
      0x231A, // ⌚  ⌚  ⌚  —   ⌚  ⌚  watch
      0x23F0, // ⏰  ⏰  ⏰  —   ⏰  ⏰  alarm clock
      0x23F1, // ⏱   ⏱   —   —   —   —   stopwatch
      0x23F2, // ⏲   ⏲   —   —   —   —   timer clock
      0x1F570, //  🕰  🕰  —   —   —   —   mantelpiece clock
      0x1F55B, //  🕛  🕛  🕛  🕛  —   —   twelve o’clock
      0x1F567, //  🕧  🕧  —   —   —   —   twelve-thirty
      0x1F550, //  🕐  🕐  🕐  🕐  —   —   one o’clock
      0x1F55C, //  🕜  🕜  —   —   —   —   one-thirty
      0x1F551, //  🕑  🕑  🕑  🕑  —   —   two o’clock
      0x1F55D, //  🕝  🕝  —   —   —   —   two-thirty
      0x1F552, //  🕒  🕒  🕒  🕒  —   —   three o’clock
      0x1F55E, //  🕞  🕞  —   —   —   —   three-thirty
      0x1F553, //  🕓  🕓  🕓  🕓  —   —   four o’clock
      0x1F55F, //  🕟  🕟  —   —   —   —   four-thirty
      0x1F554, //  🕔  🕔  🕔  🕔  —   —   five o’clock
      0x1F560, //  🕠  🕠  —   —   —   —   five-thirty
      0x1F555, //  🕕  🕕  🕕  🕕  —   —   six o’clock
      0x1F561, //  🕡  🕡  —   —   —   —   six-thirty
      0x1F556, //  🕖  🕖  🕖  🕖  —   —   seven o’clock
      0x1F562, //  🕢  🕢  —   —   —   —   seven-thirty
      0x1F557, //  🕗  🕗  🕗  🕗  —   —   eight o’clock
      0x1F563, //  🕣  🕣  —   —   —   —   eight-thirty
      0x1F558, //  🕘  🕘  🕘  🕘  —   —   nine o’clock
      0x1F564, //  🕤  🕤  —   —   —   —   nine-thirty
      0x1F559, //  🕙  🕙  🕙  🕙  —   —   ten o’clock
      0x1F565, //  🕥  🕥  —   —   —   —   ten-thirty
      0x1F55A, //  🕚  🕚  🕚  🕚  —   —   eleven o’clock
      0x1F566, //  🕦  🕦  —   —   —   —   eleven-thirty
      0x1F311, //  🌑  🌑  🌑  —   🌑  🌑  new moon
      0x1F312, //  🌒  🌒  —   —   —   —   waxing crescent moon
      0x1F313, //  🌓  🌓  🌓  —   🌓  🌓  first quarter moon
      0x1F314, //  🌔  🌔  🌔  —   🌔  🌔  waxing gibbous moon
      0x1F315, //  🌕  🌕  🌕  —   🌕  —   full moon
      0x1F316, //  🌖  🌖  —   —   —   —   waning gibbous moon
      0x1F317, //  🌗  🌗  —   —   —   —   last quarter moon
      0x1F318, //  🌘  🌘  —   —   —   —   waning crescent moon
      0x1F319, //  🌙  🌙  🌙  🌙  🌙  🌙  crescent moon
      0x1F31A, //  🌚  🌚  —   —   —   —   new moon face
      0x1F31B, //  🌛  🌛  🌛  —   —   🌛  first quarter moon face
      0x1F31C, //  🌜  🌜  —   —   —   —   last quarter moon face
      0x1F321, //  🌡  🌡  —   —   —   —   thermometer
      0x2600, // ☀   ☀   ☀   ☀   ☀   ☀   sun
      0x1F31D, //  🌝  🌝  —   —   —   —   full moon face
      0x1F31E, //  🌞  🌞  —   —   —   —   sun with face
      0x1FA90, //  🪐  🪐  —   —   —   —   ringed planet
      0x2B50, // ⭐  ⭐  —   ⭐  —   ⭐  star
      0x1F31F, //  🌟  🌟  🌟  🌟  —   —   glowing star
      0x1F320, //  🌠  🌠  🌠  —   —   🌠  shooting star
      0x1F30C, //  🌌  🌌  🌌  —   —   🌌  milky way
      0x2601, // ☁   ☁   ☁   ☁   ☁   ☁   cloud
      0x26C5, // ⛅  ⛅  ⛅  —   —   ⛅  sun behind cloud
      0x26C8, // ⛈   ⛈   —   —   —   —   cloud with lightning and rain
      0x1F324, //  🌤  🌤  —   —   —   —   sun behind small cloud
      0x1F325, //  🌥  🌥  —   —   —   —   sun behind large cloud
      0x1F326, //  🌦  🌦  —   —   —   —   sun behind rain cloud
      0x1F327, //  🌧  🌧  —   —   —   —   cloud with rain
      0x1F328, //  🌨  🌨  —   —   —   —   cloud with snow
      0x1F329, //  🌩  🌩  —   —   —   —   cloud with lightning
      0x1F32A, //  🌪  🌪  —   —   —   —   tornado
      0x1F32B, //  🌫  🌫  —   —   —   —   fog
      0x1F32C, //  🌬  🌬  —   —   —   —   wind face
      0x1F300, //  🌀  🌀  🌀  🌀  🌀  🌀  cyclone
      0x1F308, //  🌈  🌈  🌈  🌈  —   🌈  rainbow
      0x1F302, //  🌂  🌂  🌂  🌂  🌂  🌂  closed umbrella
      0x2602, // ☂   ☂   —   —   —   —   umbrella
      0x2614, // ☔  ☔  ☔  ☔  ☔  ☔  umbrella with rain drops
      0x26F1, // ⛱   ⛱   —   —   —   —   umbrella on ground
      0x26A1, // ⚡  ⚡  ⚡  ⚡  ⚡  ⚡  high voltage
      0x2744, // ❄   ❄   ❄   —   —   ❄   snowflake
      0x2603, // ☃   ☃   —   —   —   —   snowman
      0x26C4, // ⛄  ⛄  ⛄  ⛄  ⛄  ⛄  snowman without snow
      0x2604, // ☄   ☄   —   —   —   —   comet
      0x1F525, //  🔥  🔥  🔥  🔥  —   🔥  fire
      0x1F4A7, //  💧  💧  💧  —   💧  💧  droplet
      0x1F30A, //  🌊  🌊  🌊  🌊  🌊  🌊  water wave
      0x1F383, //  🎃  🎃  🎃  🎃  —   🎃  jack-o-lantern
      0x1F384, //  🎄  🎄  🎄  🎄  🎄  🎄  Christmas tree
      0x1F386, //  🎆  🎆  🎆  🎆  —   🎆  fireworks
      0x1F387, //  🎇  🎇  🎇  🎇  —   🎇  sparkler
      0x1F9E8, //  🧨  🧨  —   —   —   —   firecracker
      0x2728, // ✨  ✨  ✨  ✨  ✨  ✨  sparkles
      0x1F388, //  🎈  🎈  🎈  🎈  —   🎈  balloon
      0x1F389, //  🎉  🎉  🎉  🎉  —   🎉  party popper
      0x1F38A, //  🎊  🎊  🎊  —   —   🎊  confetti ball
      0x1F38B, //  🎋  🎋  🎋  —   —   🎋  tanabata tree
      0x1F38D, //  🎍  🎍  🎍  🎍  —   🎍  pine decoration
      0x1F38E, //  🎎  🎎  🎎  🎎  —   🎎  Japanese dolls
      0x1F38F, //  🎏  🎏  🎏  🎏  —   🎏  carp streamer
      0x1F390, //  🎐  🎐  🎐  🎐  —   🎐  wind chime
      0x1F391, //  🎑  🎑  🎑  🎑  —   🎑  moon viewing ceremony
      0x1F9E7, //  🧧  🧧  —   —   —   —   red envelope
      0x1F380, //  🎀  🎀  🎀  🎀  🎀  🎀  ribbon
      0x1F381, //  🎁  🎁  🎁  🎁  🎁  🎁  wrapped gift
      0x1F397, //  🎗  🎗  —   —   —   —   reminder ribbon
      0x1F39F, //  🎟  🎟  —   —   —   —   admission tickets
      0x1F3AB, //  🎫  🎫  🎫  🎫  🎫  🎫  ticket
      0x1F396, //  🎖  🎖  —   —   —   —   military medal
      0x1F3C6, //  🏆  🏆  🏆  🏆  —   🏆  trophy
      0x1F3C5, //  🏅  🏅  —   —   —   —   sports medal
      0x1F947, //  🥇  🥇  —   —   —   —   1st place medal
      0x1F948, //  🥈  🥈  —   —   —   —   2nd place medal
      0x1F949, //  🥉  🥉  —   —   —   —   3rd place medal
      0x26BD, // ⚽  ⚽  ⚽  ⚽  ⚽  ⚽  soccer ball
      0x26BE, // ⚾  ⚾  ⚾  ⚾  ⚾  ⚾  baseball
      0x1F94E, //  🥎  🥎  —   —   —   —   softball
      0x1F3C0, //  🏀  🏀  🏀  🏀  🏀  🏀  basketball
      0x1F3D0, //  🏐  🏐  —   —   —   —   volleyball
      0x1F3C8, //  🏈  🏈  🏈  🏈  —   🏈  american football
      0x1F3C9, //  🏉  🏉  —   —   —   —   rugby football
      0x1F3BE, //  🎾  🎾  🎾  🎾  🎾  🎾  tennis
      0x1F94F, //  🥏  🥏  —   —   —   —   flying disc
      0x1F3B3, //  🎳  🎳  🎳  —   —   🎳  bowling
      0x1F3CF, //  🏏  🏏  —   —   —   —   cricket game
      0x1F3D1, //  🏑  🏑  —   —   —   —   field hockey
      0x1F3D2, //  🏒  🏒  —   —   —   —   ice hockey
      0x1F94D, //  🥍  🥍  —   —   —   —   lacrosse
      0x1F3D3, //  🏓  🏓  —   —   —   —   ping pong
      0x1F3F8, //  🏸  🏸  —   —   —   —   badminton
      0x1F94A, //  🥊  🥊  —   —   —   —   boxing glove
      0x1F94B, //  🥋  🥋  —   —   —   —   martial arts uniform
      0x1F945, //  🥅  🥅  —   —   —   —   goal net
      0x26F3, // ⛳  ⛳  ⛳  ⛳  ⛳  ⛳  flag in hole
      0x26F8, // ⛸   ⛸   —   —   —   —   ice skate
      0x1F3A3, //  🎣  🎣  🎣  —   —   🎣  fishing pole
      0x1F93F, //  🤿  🤿  —   —   —   —   diving mask
      0x1F3BD, //  🎽  🎽  🎽  —   🎽  —   running shirt
      0x1F3BF, //  🎿  🎿  🎿  🎿  🎿  🎿  skis
      0x1F6F7, //  🛷  🛷  —   —   —   —   sled
      0x1F94C, //  🥌  🥌  —   —   —   —   curling stone
      0x1F3AF, //  🎯  🎯  🎯  🎯  —   🎯  bullseye
      0x1FA80, //  🪀  🪀  —   —   —   —   yo-yo
      0x1FA81, //  🪁  🪁  —   —   —   —   kite
      0x1F52B, //  🔫  🔫  🔫  🔫  —   🔫  water pistol
      0x1F3B1, //  🎱  🎱  🎱  🎱  —   🎱  pool 8 ball
      0x1F52E, //  🔮  🔮  🔮  —   —   🔮  crystal ball
      0x1FA84, //  🪄  🪄  —   —   —   —   magic wand
      0x1F3AE, //  🎮  🎮  🎮  —   🎮  🎮  video game
      0x1F579, //  🕹  🕹  —   —   —   —   joystick
      0x1F3B0, //  🎰  🎰  🎰  🎰  —   🎰  slot machine
      0x1F3B2, //  🎲  🎲  🎲  —   —   🎲  game die
      0x1F9E9, //  🧩  🧩  —   —   —   —   puzzle piece
      0x1F9F8, //  🧸  🧸  —   —   —   —   teddy bear
      0x1FA85, //  🪅  🪅  —   —   —   —   piñata
      0x1FAA9, //  🪩  🪩  —   —   —   —   mirror ball
      0x1FA86, //  🪆  🪆  —   —   —   —   nesting dolls
      0x2660, // ♠   ♠   ♠   ♠   ♠   ♠   spade suit
      0x2665, // ♥   ♥   ♥   ♥   ♥   ♥   heart suit
      0x2666, // ♦   ♦   ♦   ♦   ♦   ♦   diamond suit
      0x2663, // ♣   ♣   ♣   ♣   ♣   ♣   club suit
      0x265F, // ♟   ♟   —   —   —   —   chess pawn
      0x1F0CF, //  🃏  🃏  🃏  —   —   🃏  joker
      0x1F004, //  🀄  🀄  🀄  🀄  —   🀄  mahjong red dragon
      0x1F3B4, //  🎴  🎴  🎴  —   —   🎴  flower playing cards
      0x1F3AD, //  🎭  🎭  🎭  —   —   🎭  performing arts
      0x1F5BC, //  🖼  🖼  —   —   —   —   framed picture
      0x1F3A8, //  🎨  🎨  🎨  🎨  🎨  🎨  artist palette
      0x1F9F5, //  🧵  🧵  —   —   —   —   thread
      0x1FAA1, //  🪡  🪡  —   —   —   —   sewing needle
      0x1F9F6, //  🧶  🧶  —   —   —   —   yarn
      0x1FAA2, //  🪢  🪢  —   —   —   —   knot
      0x1F453, //  👓  👓  👓  —   👓  👓  glasses
      0x1F576, //  🕶  🕶  —   —   —   —   sunglasses
      0x1F97D, //  🥽  🥽  —   —   —   —   goggles
      0x1F97C, //  🥼  🥼  —   —   —   —   lab coat
      0x1F9BA, //  🦺  🦺  —   —   —   —   safety vest
      0x1F454, //  👔  👔  👔  👔  —   👔  necktie
      0x1F455, //  👕  👕  👕  👕  👕  👕  t-shirt
      0x1F456, //  👖  👖  👖  —   👖  👖  jeans
      0x1F9E3, //  🧣  🧣  —   —   —   —   scarf
      0x1F9E4, //  🧤  🧤  —   —   —   —   gloves
      0x1F9E5, //  🧥  🧥  —   —   —   —   coat
      0x1F9E6, //  🧦  🧦  —   —   —   —   socks
      0x1F457, //  👗  👗  👗  👗  —   👗  dress
      0x1F458, //  👘  👘  👘  👘  —   👘  kimono
      0x1F97B, //  🥻  🥻  —   —   —   —   sari
      0x1FA71, //  🩱  🩱  —   —   —   —   one-piece swimsuit
      0x1FA72, //  🩲  🩲  —   —   —   —   briefs
      0x1FA73, //  🩳  🩳  —   —   —   —   shorts
      0x1F459, //  👙  👙  👙  👙  —   👙  bikini
      0x1F45A, //  👚  👚  👚  —   —   👚  woman’s clothes
      0x1FAAD, //  🪭  🪭  —   —   —   —   folding hand fan
      0x1F45B, //  👛  👛  👛  —   👛  👛  purse
      0x1F45C, //  👜  👜  👜  👜  👜  👜  handbag
      0x1F45D, //  👝  👝  👝  —   👝  —   clutch bag
      0x1F6CD, //  🛍  🛍  —   —   —   —   shopping bags
      0x1F392, //  🎒  🎒  🎒  🎒  —   🎒  backpack
      0x1FA74, //  🩴  🩴  —   —   —   —   thong sandal
      0x1F45E, //  👞  👞  👞  —   —   👞  man’s shoe
      0x1F45F, //  👟  👟  👟  👟  👟  👟  running shoe
      0x1F97E, //  🥾  🥾  —   —   —   —   hiking boot
      0x1F97F, //  🥿  🥿  —   —   —   —   flat shoe
      0x1F460, //  👠  👠  👠  👠  👠  👠  high-heeled shoe
      0x1F461, //  👡  👡  👡  👡  —   —   woman’s sandal
      0x1FA70, //  🩰  🩰  —   —   —   —   ballet shoes
      0x1F462, //  👢  👢  👢  👢  —   👢  woman’s boot
      0x1FAAE, //  🪮  🪮  —   —   —   —   hair pick
      0x1F451, //  👑  👑  👑  👑  👑  👑  crown
      0x1F452, //  👒  👒  👒  👒  —   👒  woman’s hat
      0x1F3A9, //  🎩  🎩  🎩  🎩  🎩  🎩  top hat
      0x1F393, //  🎓  🎓  🎓  🎓  —   🎓  graduation cap
      0x1F9E2, //  🧢  🧢  —   —   —   —   billed cap
      0x1FA96, //  🪖  🪖  —   —   —   —   military helmet
      0x26D1, // ⛑   ⛑   —   —   —   —   rescue worker’s helmet
      0x1F4FF, //  📿  📿  —   —   —   —   prayer beads
      0x1F484, //  💄  💄  💄  💄  💄  💄  lipstick
      0x1F48D, //  💍  💍  💍  💍  💍  💍  ring
      0x1F48E, //  💎  💎  💎  💎  —   —   gem stone
      0x1F507, //  🔇  🔇  —   —   —   —   muted speaker
      0x1F508, //  🔈  🔈  —   —   —   —   speaker low volume
      0x1F509, //  🔉  🔉  —   —   —   —   speaker medium volume
      0x1F50A, //  🔊  🔊  🔊  🔊  —   🔊  speaker high volume
      0x1F4E2, //  📢  📢  📢  📢  —   —   loudspeaker
      0x1F4E3, //  📣  📣  📣  📣  —   —   megaphone
      0x1F4EF, //  📯  📯  —   —   —   —   postal horn
      0x1F514, //  🔔  🔔  🔔  🔔  🔔  🔔  bell
      0x1F515, //  🔕  🔕  —   —   —   —   bell with slash
      0x1F3BC, //  🎼  🎼  🎼  —   —   🎼  musical score
      0x1F3B5, //  🎵  🎵  🎵  🎵  🎵  🎵  musical note
      0x1F3B6, //  🎶  🎶  🎶  🎶  🎶  🎶  musical notes
      0x1F399, //  🎙  🎙  —   —   —   —   studio microphone
      0x1F39A, //  🎚  🎚  —   —   —   —   level slider
      0x1F39B, //  🎛  🎛  —   —   —   —   control knobs
      0x1F3A4, //  🎤  🎤  🎤  🎤  🎤  🎤  microphone
      0x1F3A7, //  🎧  🎧  🎧  🎧  🎧  🎧  headphone
      0x1F4FB, //  📻  📻  📻  📻  —   📻  radio
      0x1F3B7, //  🎷  🎷  🎷  🎷  —   —   saxophone
      0x1FA97, //  🪗  🪗  —   —   —   —   accordion
      0x1F3B8, //  🎸  🎸  🎸  🎸  —   🎸  guitar
      0x1F3B9, //  🎹  🎹  🎹  —   —   🎹  musical keyboard
      0x1F3BA, //  🎺  🎺  🎺  🎺  —   🎺  trumpet
      0x1F3BB, //  🎻  🎻  🎻  —   —   🎻  violin
      0x1FA95, //  🪕  🪕  —   —   —   —   banjo
      0x1F941, //  🥁  🥁  —   —   —   —   drum
      0x1FA98, //  🪘  🪘  —   —   —   —   long drum
      0x1FA87, //  🪇  🪇  —   —   —   —   maracas
      0x1FA88, //  🪈  🪈  —   —   —   —   flute
      0x1F4F1, //  📱  📱  📱  📱  📱  📱  mobile phone
      0x1F4F2, //  📲  📲  📲  📲  📲  📲  mobile phone with arrow
      0x260E, // ☎   ☎   ☎   ☎   ☎   ☎   telephone
      0x1F4DE, //  📞  📞  📞  —   —   📞  telephone receiver
      0x1F4DF, //  📟  📟  📟  —   📟  📟  pager
      0x1F4E0, //  📠  📠  📠  📠  📠  📠  fax machine
      0x1F50B, //  🔋  🔋  🔋  —   —   🔋  battery
      0x1FAAB, //  🪫  🪫  —   —   —   —   low battery
      0x1F50C, //  🔌  🔌  🔌  —   —   🔌  electric plug
      0x1F4BB, //  💻  💻  💻  💻  💻  💻  laptop
      0x1F5A5, //  🖥  🖥  —   —   —   —   desktop computer
      0x1F5A8, //  🖨  🖨  —   —   —   —   printer
      0x2328, // ⌨   ⌨   —   —   —   —   keyboard
      0x1F5B1, //  🖱  🖱  —   —   —   —   computer mouse
      0x1F5B2, //  🖲  🖲  —   —   —   —   trackball
      0x1F4BD, //  💽  💽  💽  💽  —   💽  computer disk
      0x1F4BE, //  💾  💾  💾  —   —   💾  floppy disk
      0x1F4BF, //  💿  💿  💿  💿  💿  💿  optical disk
      0x1F4C0, //  📀  📀  📀  📀  —   —   dvd
      0x1F9EE, //  🧮  🧮  —   —   —   —   abacus
      0x1F3A5, //  🎥  🎥  🎥  🎥  🎥  🎥  movie camera
      0x1F39E, //  🎞  🎞  —   —   —   —   film frames
      0x1F4FD, //  📽  📽  —   —   —   —   film projector
      0x1F3AC, //  🎬  🎬  🎬  🎬  🎬  🎬  clapper board
      0x1F4FA, //  📺  📺  📺  📺  📺  📺  television
      0x1F4F7, //  📷  📷  📷  📷  📷  📷  camera
      0x1F4F8, //  📸  📸  —   —   —   —   camera with flash
      0x1F4F9, //  📹  📹  📹  —   —   📹  video camera
      0x1F4FC, //  📼  📼  📼  📼  —   📼  videocassette
      0x1F50D, //  🔍  🔍  🔍  🔍  🔍  🔍  magnifying glass tilted left
      0x1F50E, //  🔎  🔎  🔎  —   —   🔎  magnifying glass tilted right
      0x1F56F, //  🕯  🕯  —   —   —   —   candle
      0x1F4A1, //  💡  💡  💡  💡  💡  💡  light bulb
      0x1F526, //  🔦  🔦  🔦  —   —   🔦  flashlight
      0x1F3EE, //  🏮  🏮  🏮  —   —   🏮  red paper lantern
      0x1FA94, //  🪔  🪔  —   —   —   —   diya lamp
      0x1F4D4, //  📔  📔  📔  —   —   📔  notebook with decorative cover
      0x1F4D5, //  📕  📕  📕  —   —   📕  closed book
      0x1F4D6, //  📖  📖  📖  📖  📖  📖  open book
      0x1F4D7, //  📗  📗  📗  —   —   📗  green book
      0x1F4D8, //  📘  📘  📘  —   —   📘  blue book
      0x1F4D9, //  📙  📙  📙  —   —   📙  orange book
      0x1F4DA, //  📚  📚  📚  —   —   📚  books
      0x1F4D3, //  📓  📓  📓  —   —   📓  notebook
      0x1F4D2, //  📒  📒  📒  —   —   📒  ledger
      0x1F4C3, //  📃  📃  📃  —   —   📃  page with curl
      0x1F4DC, //  📜  📜  📜  —   —   📜  scroll
      0x1F4C4, //  📄  📄  📄  —   —   📄  page facing up
      0x1F4F0, //  📰  📰  📰  —   —   📰  newspaper
      0x1F5DE, //  🗞  🗞  —   —   —   —   rolled-up newspaper
      0x1F4D1, //  📑  📑  📑  —   —   📑  bookmark tabs
      0x1F516, //  🔖  🔖  🔖  —   —   🔖  bookmark
      0x1F3F7, //  🏷  🏷  —   —   —   —   label
      0x1F4B0, //  💰  💰  💰  💰  💰  💰  money bag
      0x1FA99, //  🪙  🪙  —   —   —   —   coin
      0x1F4B4, //  💴  💴  💴  —   💴  💴  yen banknote
      0x1F4B5, //  💵  💵  💵  —   —   💵  dollar banknote
      0x1F4B6, //  💶  💶  —   —   —   —   euro banknote
      0x1F4B7, //  💷  💷  —   —   —   —   pound banknote
      0x1F4B8, //  💸  💸  💸  —   —   💸  money with wings
      0x1F4B3, //  💳  💳  💳  —   —   💳  credit card
      0x1F9FE, //  🧾  🧾  —   —   —   —   receipt
      0x1F4B9, //  💹  💹  💹  💹  —   💹  chart increasing with yen
      0x2709, // ✉   ✉   ✉   —   ✉   ✉   envelope
      0x1F4E7, //  📧  📧  📧  —   —   📧  e-mail
      0x1F4E8, //  📨  📨  📨  —   —   📨  incoming envelope
      0x1F4E9, //  📩  📩  📩  📩  📩  📩  envelope with arrow
      0x1F4E4, //  📤  📤  📤  —   —   📤  outbox tray
      0x1F4E5, //  📥  📥  📥  —   —   📥  inbox tray
      0x1F4E6, //  📦  📦  📦  —   —   📦  package
      0x1F4EB, //  📫  📫  📫  📫  —   📫  closed mailbox with raised flag
      0x1F4EA, //  📪  📪  📪  —   —   📪  closed mailbox with lowered flag
      0x1F4EC, //  📬  📬  —   —   —   —   open mailbox with raised flag
      0x1F4ED, //  📭  📭  —   —   —   —   open mailbox with lowered flag
      0x1F4EE, //  📮  📮  📮  📮  —   —   postbox
      0x1F5F3, //  🗳  🗳  —   —   —   —   ballot box with ballot
      0x270F, // ✏   ✏   ✏   —   ✏   ✏   pencil
      0x2712, // ✒   ✒   ✒   —   ✒   ✒   black nib
      0x1F58B, //  🖋  🖋  —   —   —   —   fountain pen
      0x1F58A, //  🖊  🖊  —   —   —   —   pen
      0x1F58C, //  🖌  🖌  —   —   —   —   paintbrush
      0x1F58D, //  🖍  🖍  —   —   —   —   crayon
      0x1F4DD, //  📝  📝  📝  📝  📝  📝  memo
      0x1F4BC, //  💼  💼  💼  💼  —   💼  briefcase
      0x1F4C1, //  📁  📁  📁  —   —   📁  file folder
      0x1F4C2, //  📂  📂  📂  —   —   📂  open file folder
      0x1F5C2, //  🗂  🗂  —   —   —   —   card index dividers
      0x1F4C5, //  📅  📅  📅  —   —   📅  calendar
      0x1F4C6, //  📆  📆  📆  —   —   📆  tear-off calendar
      0x1F5D2, //  🗒  🗒  —   —   —   —   spiral notepad
      0x1F5D3, //  🗓  🗓  —   —   —   —   spiral calendar
      0x1F4C7, //  📇  📇  📇  —   —   📇  card index
      0x1F4C8, //  📈  📈  📈  —   —   📈  chart increasing
      0x1F4C9, //  📉  📉  📉  —   —   📉  chart decreasing
      0x1F4CA, //  📊  📊  📊  —   —   📊  bar chart
      0x1F4CB, //  📋  📋  📋  —   —   📋  clipboard
      0x1F4CC, //  📌  📌  📌  —   —   📌  pushpin
      0x1F4CD, //  📍  📍  📍  —   —   📍  round pushpin
      0x1F4CE, //  📎  📎  📎  —   📎  📎  paperclip
      0x1F587, //  🖇  🖇  —   —   —   —   linked paperclips
      0x1F4CF, //  📏  📏  📏  —   —   📏  straight ruler
      0x1F4D0, //  📐  📐  📐  —   —   📐  triangular ruler
      0x2702, // ✂   ✂   ✂   ✂   ✂   ✂   scissors
      0x1F5C3, //  🗃  🗃  —   —   —   —   card file box
      0x1F5C4, //  🗄  🗄  —   —   —   —   file cabinet
      0x1F5D1, //  🗑  🗑  —   —   —   —   wastebasket
      0x1F512, //  🔒  🔒  🔒  🔒  —   🔒  locked
      0x1F513, //  🔓  🔓  🔓  🔓  —   —   unlocked
      0x1F50F, //  🔏  🔏  🔏  —   —   🔏  locked with pen
      0x1F510, //  🔐  🔐  🔐  —   —   🔐  locked with key
      0x1F511, //  🔑  🔑  🔑  🔑  🔑  🔑  key
      0x1F5DD, //  🗝  🗝  —   —   —   —   old key
      0x1F528, //  🔨  🔨  🔨  🔨  —   🔨  hammer
      0x1FA93, //  🪓  🪓  —   —   —   —   axe
      0x26CF, // ⛏   ⛏   —   —   —   —   pick
      0x2692, // ⚒   ⚒   —   —   —   —   hammer and pick
      0x1F6E0, //  🛠  🛠  —   —   —   —   hammer and wrench
      0x1F5E1, //  🗡  🗡  —   —   —   —   dagger
      0x2694, // ⚔   ⚔   —   —   —   —   crossed swords
      0x1F4A3, //  💣  💣  💣  💣  💣  💣  bomb
      0x1FA83, //  🪃  🪃  —   —   —   —   boomerang
      0x1F3F9, //  🏹  🏹  —   —   —   —   bow and arrow
      0x1F6E1, //  🛡  🛡  —   —   —   —   shield
      0x1FA9A, //  🪚  🪚  —   —   —   —   carpentry saw
      0x1F527, //  🔧  🔧  🔧  —   🔧  🔧  wrench
      0x1FA9B, //  🪛  🪛  —   —   —   —   screwdriver
      0x1F529, //  🔩  🔩  🔩  —   —   🔩  nut and bolt
      0x2699, // ⚙   ⚙   —   —   —   —   gear
      0x1F5DC, //  🗜  🗜  —   —   —   —   clamp
      0x2696, // ⚖   ⚖   —   —   —   —   balance scale
      0x1F9AF, //  🦯  🦯  —   —   —   —   white cane
      0x1F517, //  🔗  🔗  🔗  —   —   🔗  link
      0x26D3, // ⛓   ⛓   —   —   —   —   chains
      0x1FA9D, //  🪝  🪝  —   —   —   —   hook
      0x1F9F0, //  🧰  🧰  —   —   —   —   toolbox
      0x1F9F2, //  🧲  🧲  —   —   —   —   magnet
      0x1FA9C, //  🪜  🪜  —   —   —   —   ladder
      0x2697, // ⚗   ⚗   —   —   —   —   alembic
      0x1F9EA, //  🧪  🧪  —   —   —   —   test tube
      0x1F9EB, //  🧫  🧫  —   —   —   —   petri dish
      0x1F9EC, //  🧬  🧬  —   —   —   —   dna
      0x1F52C, //  🔬  🔬  —   —   —   —   microscope
      0x1F52D, //  🔭  🔭  —   —   —   —   telescope
      0x1F4E1, //  📡  📡  📡  📡  —   📡  satellite antenna
      0x1F489, //  💉  💉  💉  💉  —   💉  syringe
      0x1FA78, //  🩸  🩸  —   —   —   —   drop of blood
      0x1F48A, //  💊  💊  💊  💊  —   💊  pill
      0x1FA79, //  🩹  🩹  —   —   —   —   adhesive bandage
      0x1FA7C, //  🩼  🩼  —   —   —   —   crutch
      0x1FA7A, //  🩺  🩺  —   —   —   —   stethoscope
      0x1FA7B, //  🩻  🩻  —   —   —   —   x-ray
      0x1F6AA, //  🚪  🚪  🚪  —   🚪  —   door
      0x1F6D7, //  🛗  🛗  —   —   —   —   elevator
      0x1FA9E, //  🪞  🪞  —   —   —   —   mirror
      0x1FA9F, //  🪟  🪟  —   —   —   —   window
      0x1F6CF, //  🛏  🛏  —   —   —   —   bed
      0x1F6CB, //  🛋  🛋  —   —   —   —   couch and lamp
      0x1FA91, //  🪑  🪑  —   —   —   —   chair
      0x1F6BD, //  🚽  🚽  🚽  🚽  —   —   toilet
      0x1FAA0, //  🪠  🪠  —   —   —   —   plunger
      0x1F6BF, //  🚿  🚿  —   —   —   —   shower
      0x1F6C1, //  🛁  🛁  —   —   —   —   bathtub
      0x1FAA4, //  🪤  🪤  —   —   —   —   mouse trap
      0x1FA92, //  🪒  🪒  —   —   —   —   razor
      0x1F9F4, //  🧴  🧴  —   —   —   —   lotion bottle
      0x1F9F7, //  🧷  🧷  —   —   —   —   safety pin
      0x1F9F9, //  🧹  🧹  —   —   —   —   broom
      0x1F9FA, //  🧺  🧺  —   —   —   —   basket
      0x1F9FB, //  🧻  🧻  —   —   —   —   roll of paper
      0x1FAA3, //  🪣  🪣  —   —   —   —   bucket
      0x1F9FC, //  🧼  🧼  —   —   —   —   soap
      0x1FAE7, //  🫧  🫧  —   —   —   —   bubbles
      0x1FAA5, //  🪥  🪥  —   —   —   —   toothbrush
      0x1F9FD, //  🧽  🧽  —   —   —   —   sponge
      0x1F9EF, //  🧯  🧯  —   —   —   —   fire extinguisher
      0x1F6D2, //  🛒  🛒  —   —   —   —   shopping cart
      0x1F6AC, //  🚬  🚬  🚬  🚬  🚬  🚬  cigarette
      0x26B0, // ⚰   ⚰   —   —   —   —   coffin
      0x1FAA6, //  🪦  🪦  —   —   —   —   headstone
      0x26B1, // ⚱   ⚱   —   —   —   —   funeral urn
      0x1F9FF, //  🧿  🧿  —   —   —   —   nazar amulet
      0x1FAAC, //  🪬  🪬  —   —   —   —   hamsa
      0x1F5FF, //  🗿  🗿  🗿  —   —   🗿  moai
      0x1FAA7, //  🪧  🪧  —   —   —   —   placard
      0x1FAAA, //  🪪  🪪  —   —   —   —   identification card
      0x1F3E7, //  🏧  🏧  🏧  🏧  🏧  🏧  ATM sign
      0x1F6AE, //  🚮  🚮  —   —   —   —   litter in bin sign
      0x1F6B0, //  🚰  🚰  —   —   —   —   potable water
      0x267F, // ♿  ♿  ♿  ♿  ♿  ♿  wheelchair symbol
      0x1F6B9, //  🚹  🚹  🚹  🚹  —   —   men’s room
      0x1F6BA, //  🚺  🚺  🚺  🚺  —   —   women’s room
      0x1F6BB, //  🚻  🚻  🚻  🚻  🚻  🚻  restroom
      0x1F6BC, //  🚼  🚼  🚼  🚼  —   —   baby symbol
      0x1F6BE, //  🚾  🚾  🚾  🚾  —   —   water closet
      0x1F6C2, //  🛂  🛂  —   —   —   —   passport control
      0x1F6C3, //  🛃  🛃  —   —   —   —   customs
      0x1F6C4, //  🛄  🛄  —   —   —   —   baggage claim
      0x1F6C5, //  🛅  🛅  —   —   —   —   left luggage
      0x26A0, // ⚠   ⚠   ⚠   ⚠   ⚠   ⚠   warning
      0x1F6B8, //  🚸  🚸  —   —   —   —   children crossing
      0x26D4, // ⛔  ⛔  ⛔  —   —   ⛔  no entry
      0x1F6AB, //  🚫  🚫  🚫  —   —   🚫  prohibited
      0x1F6B3, //  🚳  🚳  —   —   —   —   no bicycles
      0x1F6AD, //  🚭  🚭  🚭  🚭  🚭  🚭  no smoking
      0x1F6AF, //  🚯  🚯  —   —   —   —   no littering
      0x1F6B1, //  🚱  🚱  —   —   —   —   non-potable water
      0x1F6B7, //  🚷  🚷  —   —   —   —   no pedestrians
      0x1F4F5, //  📵  📵  —   —   —   —   no mobile phones
      0x1F51E, //  🔞  🔞  🔞  🔞  —   🔞  no one under eighteen
      0x2622, // ☢   ☢   —   —   —   —   radioactive
      0x2623, // ☣   ☣   —   —   —   —   biohazard
      0x2B06, // ⬆   ⬆   —   ⬆   —   ⬆   up arrow
      0x2197, // ↗   ↗   ↗   ↗   ↗   ↗   up-right arrow
      0x27A1, // ➡   ➡   —   ➡   —   ➡   right arrow
      0x2198, // ↘   ↘   ↘   ↘   ↘   ↘   down-right arrow
      0x2B07, // ⬇   ⬇   —   ⬇   —   ⬇   down arrow
      0x2199, // ↙   ↙   ↙   ↙   ↙   ↙   down-left arrow
      0x2B05, // ⬅   ⬅   —   ⬅   —   ⬅   left arrow
      0x2196, // ↖   ↖   ↖   ↖   ↖   ↖   up-left arrow
      0x2195, // ↕   ↕   ↕   —   ↕   ↕   up-down arrow
      0x2194, // ↔   ↔   ↔   —   ↔   ↔   left-right arrow
      0x21A9, // ↩   ↩   —   —   ↩   ↩   right arrow curving left
      0x21AA, // ↪   ↪   ↪   —   —   ↪   left arrow curving right
      0x2934, // ⤴   ⤴   ⤴   —   ⤴   ⤴   right arrow curving up
      0x2935, // ⤵   ⤵   ⤵   —   ⤵   ⤵   right arrow curving down
      0x1F503, //  🔃  🔃  🔃  —   —   🔃  clockwise vertical arrows
      0x1F504, //  🔄  🔄  —   —   —   —   counterclockwise arrows button
      0x1F519, //  🔙  🔙  🔙  —   —   🔙  BACK arrow
      0x1F51A, //  🔚  🔚  🔚  —   🔚  —   END arrow
      0x1F51B, //  🔛  🔛  🔛  —   🔛  —   ON! arrow
      0x1F51C, //  🔜  🔜  🔜  —   🔜  —   SOON arrow
      0x1F51D, //  🔝  🔝  🔝  🔝  —   —   TOP arrow
      0x1F6D0, //  🛐  🛐  —   —   —   —   place of worship
      0x269B, // ⚛   ⚛   —   —   —   —   atom symbol
      0x1F549, //  🕉  🕉  —   —   —   —   om
      0x2721, // ✡   ✡   —   —   —   —   star of David
      0x2638, // ☸   ☸   —   —   —   —   wheel of dharma
      0x262F, // ☯   ☯   —   —   —   —   yin yang
      0x271D, // ✝   ✝   —   —   —   —   latin cross
      0x2626, // ☦   ☦   —   —   —   —   orthodox cross
      0x262A, // ☪   ☪   —   —   —   —   star and crescent
      0x262E, // ☮   ☮   —   —   —   —   peace symbol
      0x1F54E, //  🕎  🕎  —   —   —   —   menorah
      0x1F52F, //  🔯  🔯  🔯  🔯  —   —   dotted six-pointed star
      0x1FAAF, //  🪯  🪯  —   —   —   —   khanda
      0x2648, // ♈  ♈  ♈  ♈  ♈  ♈  Aries
      0x2649, // ♉  ♉  ♉  ♉  ♉  ♉  Taurus
      0x264A, // ♊  ♊  ♊  ♊  ♊  ♊  Gemini
      0x264B, // ♋  ♋  ♋  ♋  ♋  ♋  Cancer
      0x264C, // ♌  ♌  ♌  ♌  ♌  ♌  Leo
      0x264D, // ♍  ♍  ♍  ♍  ♍  ♍  Virgo
      0x264E, // ♎  ♎  ♎  ♎  ♎  ♎  Libra
      0x264F, // ♏  ♏  ♏  ♏  ♏  ♏  Scorpio
      0x2650, // ♐  ♐  ♐  ♐  ♐  ♐  Sagittarius
      0x2651, // ♑  ♑  ♑  ♑  ♑  ♑  Capricorn
      0x2652, // ♒  ♒  ♒  ♒  ♒  ♒  Aquarius
      0x2653, // ♓  ♓  ♓  ♓  ♓  ♓  Pisces
      0x26CE, // ⛎  ⛎  ⛎  ⛎  —   ⛎  Ophiuchus
      0x1F500, //  🔀  🔀  —   —   —   —   shuffle tracks button
      0x1F501, //  🔁  🔁  —   —   —   —   repeat button
      0x1F502, //  🔂  🔂  —   —   —   —   repeat single button
      0x25B6, // ▶   ▶   ▶   ▶   —   ▶   play button
      0x23E9, // ⏩  ⏩  ⏩  ⏩  —   ⏩  fast-forward button
      0x23ED, // ⏭   ⏭   —   —   —   —   next track button
      0x23EF, // ⏯   ⏯   —   —   —   —   play or pause button
      0x25C0, // ◀   ◀   ◀   ◀   —   ◀   reverse button
      0x23EA, // ⏪  ⏪  ⏪  ⏪  —   ⏪  fast reverse button
      0x23EE, // ⏮   ⏮   —   —   —   —   last track button
      0x1F53C, //  🔼  🔼  🔼  —   —   🔼  upwards button
      0x23EB, // ⏫  ⏫  ⏫  —   —   ⏫  fast up button
      0x1F53D, //  🔽  🔽  🔽  —   —   🔽  downwards button
      0x23EC, // ⏬  ⏬  ⏬  —   —   ⏬  fast down button
      0x23F8, // ⏸   ⏸   —   —   —   —   pause button
      0x23F9, // ⏹   ⏹   —   —   —   —   stop button
      0x23FA, // ⏺   ⏺   —   —   —   —   record button
      0x23CF, // ⏏   ⏏   —   —   —   —   eject button
      0x1F3A6, //  🎦  🎦  🎦  🎦  —   —   cinema
      0x1F505, //  🔅  🔅  —   —   —   —   dim button
      0x1F506, //  🔆  🔆  —   —   —   —   bright button
      0x1F4F6, //  📶  📶  📶  📶  —   📶  antenna bars
      0x1F6DC, //  🛜  🛜  —   —   —   —   wireless
      0x1F4F3, //  📳  📳  📳  📳  —   📳  vibration mode
      0x1F4F4, //  📴  📴  📴  📴  —   📴  mobile phone off
      0x2640, // ♀   ♀   —   —   —   —   female sign
      0x2642, // ♂   ♂   —   —   —   —   male sign
      0x26A7, // ⚧   ⚧   —   —   —   —   transgender symbol
      0x2716, // ✖   ✖   —   —   —   ✖   multiply
      0x2795, // ➕  ➕  ➕  —   —   ➕  plus
      0x2796, // ➖  ➖  ➖  —   —   ➖  minus
      0x2797, // ➗  ➗  ➗  —   —   ➗  divide
      0x1F7F0, //  🟰  🟰  —   —   —   —   heavy equals sign
      0x267E, // ♾   ♾   —   —   —   —   infinity
      0x203C, // ‼   ‼   ‼   —   ‼   ‼   double exclamation mark
      0x2049, // ⁉   ⁉   ⁉   —   ⁉   ⁉   exclamation question mark
      0x2753, // ❓  ❓  ❓  ❓  —   ❓  red question mark
      0x2754, // ❔  ❔  ❔  ❔  —   —   white question mark
      0x2755, // ❕  ❕  ❕  ❕  —   —   white exclamation mark
      0x2757, // ❗  ❗  ❗  ❗  ❗  ❗  red exclamation mark
      0x3030, // 〰  〰  〰  —   〰  —   wavy dash
      0x1F4B1, //  💱  💱  💱  💱  —   —   currency exchange
      0x1F4B2, //  💲  💲  💲  —   —   💲  heavy dollar sign
      0x2695, // ⚕   ⚕   —   —   —   —   medical symbol
      0x267B, // ♻   ♻   ♻   —   ♻   ♻   recycling symbol
      0x269C, // ⚜   ⚜   —   —   —   —   fleur-de-lis
      0x1F531, //  🔱  🔱  🔱  🔱  —   —   trident emblem
      0x1F4DB, //  📛  📛  📛  —   —   📛  name badge
      0x1F530, //  🔰  🔰  🔰  🔰  —   🔰  Japanese symbol for beginner
      0x2B55, // ⭕  ⭕  ⭕  ⭕  —   ⭕  hollow red circle
      0x2705, // ✅  ✅  ✅  —   —   ✅  check mark button
      0x2611, // ☑   ☑   ☑   —   —   ☑   check box with check
      0x2714, // ✔   ✔   —   —   —   ✔   check mark
      0x274C, // ❌  ❌  ❌  ❌  —   ❌  cross mark
      0x274E, // ❎  ❎  ❎  —   —   ❎  cross mark button
      0x27B0, // ➰  ➰  ➰  —   ➰  ➰  curly loop
      0x27BF, // ➿  ➿  ➿  —   —   —   double curly loop
      0x303D, // 〽  〽  〽  〽  —   —   part alternation mark
      0x2733, // ✳   ✳   ✳   ✳   —   ✳   eight-spoked asterisk
      0x2734, // ✴   ✴   —   ✴   —   ✴   eight-pointed star
      0x2747, // ❇   ❇   ❇   —   —   ❇   sparkle
      0x00A9, // ©   ©   ©   ©   ©   ©   copyright
      0x00AE, // ®   ®   ®   ®   ®   ®   registered
      0x2122, // ™   ™   ™   ™   ™   ™   trade mark
      0x1F51F, //  🔟  🔟  🔟  —   —   🔟  keycap: 10
      0x1F520, //  🔠  🔠  🔠  —   —   🔠  input latin uppercase
      0x1F521, //  🔡  🔡  🔡  —   —   🔡  input latin lowercase
      0x1F522, //  🔢  🔢  🔢  —   —   🔢  input numbers
      0x1F523, //  🔣  🔣  🔣  —   —   🔣  input symbols
      0x1F524, //  🔤  🔤  🔤  —   —   🔤  input latin letters
      0x1F170, //  🅰   🅰   🅰   🅰   —   🅰   A button (blood type)
      0x1F18E, //  🆎  🆎  🆎  🆎  —   🆎  AB button (blood type)
      0x1F171, //  🅱   🅱   🅱   🅱   —   🅱   B button (blood type)
      0x1F191, //  🆑  🆑  🆑  —   🆑  🆑  CL button
      0x1F192, //  🆒  🆒  🆒  🆒  —   🆒  COOL button
      0x1F193, //  🆓  🆓  🆓  —   🆓  🆓  FREE button
      0x2139, // ℹ   ℹ   ℹ   —   —   ℹ   information
      0x1F194, //  🆔  🆔  🆔  🆔  🆔  🆔  ID button
      0x24C2, // Ⓜ   Ⓜ   Ⓜ   —   Ⓜ   —   circled M
      0x1F195, //  🆕  🆕  🆕  🆕  🆕  🆕  NEW button
      0x1F196, //  🆖  🆖  🆖  —   🆖  —   NG button
      0x1F17E, //  🅾   🅾   🅾   🅾   —   🅾   O button (blood type)
      0x1F197, //  🆗  🆗  🆗  🆗  🆗  🆗  OK button
      0x1F17F, //  🅿   🅿   🅿   🅿   🅿   🅿   P button
      0x1F198, //  🆘  🆘  🆘  —   —   🆘  SOS button
      0x1F19A, //  🆚  🆚  🆚  🆚  —   🆚  VS button
      0x1F201, //  🈁  🈁  🈁  🈁  —   —   Japanese “here” button
      0x1F202, //  🈂  🈂  🈂  🈂  —   🈂  Japanese “service charge” button
      0x1F237, //  🈷  🈷  🈷  🈷  —   —   Japanese “monthly amount” button
      0x1F236, //  🈶  🈶  🈶  🈶  —   —   Japanese “not free of charge” button
      0x1F22F, //  🈯  🈯  🈯  🈯  —   🈯  Japanese “reserved” button
      0x1F250, //  🉐  🉐  🉐  🉐  —   🉐  Japanese “bargain” button
      0x1F239, //  🈹  🈹  🈹  🈹  —   🈹  Japanese “discount” button
      0x1F21A, //  🈚  🈚  🈚  🈚  —   —   Japanese “free of charge” button
      0x1F232, //  🈲  🈲  🈲  —   🈲  —   Japanese “prohibited” button
      0x1F251, //  🉑  🉑  🉑  —   —   🉑  Japanese “acceptable” button
      0x1F238, //  🈸  🈸  🈸  🈸  —   —   Japanese “application” button
      0x1F234, //  🈴  🈴  🈴  —   🈴  —   Japanese “passing grade” button
      0x1F233, //  🈳  🈳  🈳  🈳  🈳  🈳  Japanese “vacancy” button
      0x3297, // ㊗  ㊗  ㊗  ㊗  —   ㊗  Japanese “congratulations” button
      0x3299, // ㊙  ㊙  ㊙  ㊙  ㊙  ㊙  Japanese “secret” button
      0x1F23A, //  🈺  🈺  🈺  🈺  —   🈺  Japanese “open for business” button
      0x1F235, //  🈵  🈵  🈵  🈵  🈵  🈵  Japanese “no vacancy” button
      0x1F534, //  🔴  🔴  🔴  🔴  —   🔴  red circle
      0x1F7E0, //  🟠  🟠  —   —   —   —   orange circle
      0x1F7E1, //  🟡  🟡  —   —   —   —   yellow circle
      0x1F7E2, //  🟢  🟢  —   —   —   —   green circle
      0x1F535, //  🔵  🔵  🔵  —   —   🔵  blue circle
      0x1F7E3, //  🟣  🟣  —   —   —   —   purple circle
      0x1F7E4, //  🟤  🟤  —   —   —   —   brown circle
      0x26AB, // ⚫  ⚫  ⚫  —   —   ⚫  black circle
      0x26AA, // ⚪  ⚪  ⚪  —   —   ⚪  white circle
      0x1F7E5, //  🟥  🟥  —   —   —   —   red square
      0x1F7E7, //  🟧  🟧  —   —   —   —   orange square
      0x1F7E8, //  🟨  🟨  —   —   —   —   yellow square
      0x1F7E9, //  🟩  🟩  —   —   —   —   green square
      0x1F7E6, //  🟦  🟦  —   —   —   —   blue square
      0x1F7EA, //  🟪  🟪  —   —   —   —   purple square
      0x1F7EB, //  🟫  🟫  —   —   —   —   brown square
      0x2B1B, // ⬛  ⬛  —   —   —   ⬛  black large square
      0x2B1C, // ⬜  ⬜  —   —   —   ⬜  white large square
      0x25FC, // ◼   ◼   ◼   —   —   ◼   black medium square
      0x25FB, // ◻   ◻   ◻   —   —   ◻   white medium square
      0x25FE, // ◾  ◾  ◾  —   —   ◾  black medium-small square
      0x25FD, // ◽  ◽  ◽  —   —   ◽  white medium-small square
      0x25AA, // ▪   ▪   ▪   —   —   ▪   black small square
      0x25AB, // ▫   ▫   ▫   —   —   ▫   white small square
      0x1F536, //  🔶  🔶  🔶  —   —   🔶  large orange diamond
      0x1F537, //  🔷  🔷  🔷  —   —   🔷  large blue diamond
      0x1F538, //  🔸  🔸  🔸  —   —   🔸  small orange diamond
      0x1F539, //  🔹  🔹  🔹  —   —   🔹  small blue diamond
      0x1F53A, //  🔺  🔺  🔺  —   —   🔺  red triangle pointed up
      0x1F53B, //  🔻  🔻  🔻  —   —   🔻  red triangle pointed down
      0x1F4A0, //  💠  💠  💠  —   💠  —   diamond with a dot
      0x1F518, //  🔘  🔘  🔘  —   —   🔘  radio button
      0x1F533, //  🔳  🔳  🔳  🔳  —   —   white square button
      0x1F532, //  🔲  🔲  —   🔲  —   —   black square button
      0x1F3C1, //  🏁  🏁  🏁  🏁  🏁  🏁  chequered flag
      0x1F6A9, //  🚩  🚩  🚩  —   🚩  🚩  triangular flag
      0x1F38C, //  🎌  🎌  🎌  🎌  —   🎌  crossed flags
      0x1F3F4, //  🏴  🏴  —   —   —   —   black flag
      0x1F3F3, //  🏳  🏳  —   —   —   —   white flag
      0x1F3FB, //  🏻  🏻  light skin tone
      0x1F3FC, //  🏼  🏼  medium-light skin tone
      0x1F3FD, //  🏽  🏽  medium skin tone
      0x1F3FE, //  🏾  🏾  medium-dark skin tone
      0x1F3FF, //  🏿  🏿  dark skin tone
    ];


/*
const EMOJI : [u32; 3782] =
    [
        0x1F600, // 😀
        0x1F603, // 😃
        0x1F604, // 😄
        0x1F601, // 😁
        0x1F606, // 😆
        0x1F605, // 😅
        0x1F923, // 🤣
        0x1F602, // 😂
        0x1F642, // 🙂
        0x1F643, // 🙃
        0x1FAE0, // 🫠
        0x1F609, // 😉
        0x1F60A, // 😊
        0x1F607, // 😇
        0x1F970, // 🥰
        0x1F60D, // 😍
        0x1F929, // 🤩
        0x1F618, // 😘
        0x1F617, // 😗
        0x263A, // ☺
        0x1F61A, // 😚
        0x1F619, // 😙
        0x1F972, // 🥲
        0x1F60B, // 😋
        0x1F61B, // 😛
        0x1F61C, // 😜
        0x1F92A, // 🤪
        0x1F61D, // 😝
        0x1F911, // 🤑
        0x1F917, // 🤗
        0x1F92D, // 🤭
        0x1FAE2, // 🫢
        0x1FAE3, // 🫣
        0x1F92B, // 🤫
        0x1F914, // 🤔
        0x1FAE1, // 🫡
        0x1F910, // 🤐
        0x1F928, // 🤨
        0x1F610, // 😐
        0x1F611, // 😑
        0x1F636, // 😶
        0x1FAE5, // 🫥
        0x1F636, // U
        0x1F60F, // 😏
        0x1F612, // 😒
        0x1F644, // 🙄
        0x1F62C, // 😬
        0x1F62E, // U
        0x1F925, // 🤥
        0x1FAE8, // 🫨
        0x1F642, // U
        0x1F642, // U
        0x1F60C, // 😌
        0x1F614, // 😔
        0x1F62A, // 😪
        0x1F924, // 🤤
        0x1F634, // 😴
        0x1F637, // 😷
        0x1F912, // 🤒
        0x1F915, // 🤕
        0x1F922, // 🤢
        0x1F92E, // 🤮
        0x1F927, // 🤧
        0x1F975, // 🥵
        0x1F976, // 🥶
        0x1F974, // 🥴
        0x1F635, // 😵
        0x1F635, // U
        0x1F92F, // 🤯
        0x1F920, // 🤠
        0x1F973, // 🥳
        0x1F978, // 🥸
        0x1F60E, // 😎
        0x1F913, // 🤓
        0x1F9D0, // 🧐
        0x1F615, // 😕
        0x1FAE4, // 🫤
        0x1F61F, // 😟
        0x1F641, // 🙁
        0x2639, // ☹
        0x1F62E, // 😮
        0x1F62F, // 😯
        0x1F632, // 😲
        0x1F633, // 😳
        0x1F97A, // 🥺
        0x1F979, // 🥹
        0x1F626, // 😦
        0x1F627, // 😧
        0x1F628, // 😨
        0x1F630, // 😰
        0x1F625, // 😥
        0x1F622, // 😢
        0x1F62D, // 😭
        0x1F631, // 😱
        0x1F616, // 😖
        0x1F623, // 😣
        0x1F61E, // 😞
        0x1F613, // 😓
        0x1F629, // 😩
        0x1F62B, // 😫
        0x1F971, // 🥱
        0x1F624, // 😤
        0x1F621, // 😡
        0x1F620, // 😠
        0x1F92C, // 🤬
        0x1F608, // 😈
        0x1F47F, // 👿
        0x1F480, // 💀
        0x2620, // ☠
        0x1F4A9, // 💩
        0x1F921, // 🤡
        0x1F479, // 👹
        0x1F47A, // 👺
        0x1F47B, // 👻
        0x1F47D, // 👽
        0x1F47E, // 👾
        0x1F916, // 🤖
        0x1F63A, // 😺
        0x1F638, // 😸
        0x1F639, // 😹
        0x1F63B, // 😻
        0x1F63C, // 😼
        0x1F63D, // 😽
        0x1F640, // 🙀
        0x1F63F, // 😿
        0x1F63E, // 😾
        0x1F648, // 🙈
        0x1F649, // 🙉
        0x1F64A, // 🙊
        0x1F48C, // 💌
        0x1F498, // 💘
        0x1F49D, // 💝
        0x1F496, // 💖
        0x1F497, // 💗
        0x1F493, // 💓
        0x1F49E, // 💞
        0x1F495, // 💕
        0x1F49F, // 💟
        0x2763, // ❣
        0x1F494, // 💔
        0x2764, // U
        0x2764, // U
        0x2764, // ❤
        0x1FA77, // 🩷
        0x1F9E1, // 🧡
        0x1F49B, // 💛
        0x1F49A, // 💚
        0x1F499, // 💙
        0x1FA75, // 🩵
        0x1F49C, // 💜
        0x1F90E, // 🤎
        0x1F5A4, // 🖤
        0x1FA76, // 🩶
        0x1F90D, // 🤍
        0x1F48B, // 💋
        0x1F4AF, // 💯
        0x1F4A2, // 💢
        0x1F4A5, // 💥
        0x1F4AB, // 💫
        0x1F4A6, // 💦
        0x1F4A8, // 💨
        0x1F573, // 🕳
        0x1F4AC, // 💬
        0x1F441, // U
        0x1F5E8, // 🗨
        0x1F5EF, // 🗯
        0x1F4AD, // 💭
        0x1F4A4, // 💤
        0x1F44B, // 👋
        0x1F91A, // 🤚
        0x1F590, // 🖐
        0x270B, // ✋
        0x1F596, // 🖖
        0x1FAF1, // 🫱
        0x1FAF2, // 🫲
        0x1FAF3, // 🫳
        0x1FAF4, // 🫴
        0x1FAF7, // 🫷
        0x1FAF8, // 🫸
        0x1F44C, // 👌
        0x1F90C, // 🤌
        0x1F90F, // 🤏
        0x270C, // ✌
        0x1F91E, // 🤞
        0x1FAF0, // 🫰
        0x1F91F, // 🤟
        0x1F918, // 🤘
        0x1F919, // 🤙
        0x1F448, // 👈
        0x1F449, // 👉
        0x1F446, // 👆
        0x1F595, // 🖕
        0x1F447, // 👇
        0x261D, // ☝
        0x1FAF5, // 🫵
        0x1F44D, // 👍
        0x1F44E, // 👎
        0x270A, // ✊
        0x1F44A, // 👊
        0x1F91B, // 🤛
        0x1F91C, // 🤜
        0x1F44F, // 👏
        0x1F64C, // 🙌
        0x1FAF6, // 🫶
        0x1F450, // 👐
        0x1F932, // 🤲
        0x1F91D, // 🤝
        0x1F64F, // 🙏
        0x270D, // ✍
        0x1F485, // 💅
        0x1F933, // 🤳
        0x1F4AA, // 💪
        0x1F9BE, // 🦾
        0x1F9BF, // 🦿
        0x1F9B5, // 🦵
        0x1F9B6, // 🦶
        0x1F442, // 👂
        0x1F9BB, // 🦻
        0x1F443, // 👃
        0x1F9E0, // 🧠
        0x1FAC0, // 🫀
        0x1FAC1, // 🫁
        0x1F9B7, // 🦷
        0x1F9B4, // 🦴
        0x1F440, // 👀
        0x1F441, // 👁
        0x1F445, // 👅
        0x1F444, // 👄
        0x1FAE6, // 🫦
        0x1F476, // 👶
        0x1F9D2, // 🧒
        0x1F466, // 👦
        0x1F467, // 👧
        0x1F9D1, // 🧑
        0x1F471, // 👱
        0x1F468, // 👨
        0x1F9D4, // 🧔
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // 👩
        0x1F469, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F471, // U
        0x1F471, // U
        0x1F9D3, // 🧓
        0x1F474, // 👴
        0x1F475, // 👵
        0x1F64D, // 🙍
        0x1F64D, // U
        0x1F64D, // U
        0x1F64E, // 🙎
        0x1F64E, // U
        0x1F64E, // U
        0x1F645, // 🙅
        0x1F645, // U
        0x1F645, // U
        0x1F646, // 🙆
        0x1F646, // U
        0x1F646, // U
        0x1F481, // 💁
        0x1F481, // U
        0x1F481, // U
        0x1F64B, // 🙋
        0x1F64B, // U
        0x1F64B, // U
        0x1F9CF, // 🧏
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F647, // 🙇
        0x1F647, // U
        0x1F647, // U
        0x1F926, // 🤦
        0x1F926, // U
        0x1F926, // U
        0x1F937, // 🤷
        0x1F937, // U
        0x1F937, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F469, // U
        0x1F46E, // 👮
        0x1F46E, // U
        0x1F46E, // U
        0x1F575, // 🕵
        0x1F575, // U
        0x1F575, // U
        0x1F482, // 💂
        0x1F482, // U
        0x1F482, // U
        0x1F977, // 🥷
        0x1F477, // 👷
        0x1F477, // U
        0x1F477, // U
        0x1FAC5, // 🫅
        0x1F934, // 🤴
        0x1F478, // 👸
        0x1F473, // 👳
        0x1F473, // U
        0x1F473, // U
        0x1F472, // 👲
        0x1F9D5, // 🧕
        0x1F935, // 🤵
        0x1F935, // U
        0x1F935, // U
        0x1F470, // 👰
        0x1F470, // U
        0x1F470, // U
        0x1F930, // 🤰
        0x1FAC3, // 🫃
        0x1FAC4, // 🫄
        0x1F931, // 🤱
        0x1F469, // U
        0x1F468, // U
        0x1F9D1, // U
        0x1F47C, // 👼
        0x1F385, // 🎅
        0x1F936, // 🤶
        0x1F9D1, // U
        0x1F9B8, // 🦸
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B9, // 🦹
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9D9, // 🧙
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9DA, // 🧚
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DB, // 🧛
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DC, // 🧜
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DD, // 🧝
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DE, // 🧞
        0x1F9DE, // U
        0x1F9DE, // U
        0x1F9DF, // 🧟
        0x1F9DF, // U
        0x1F9DF, // U
        0x1F9CC, // 🧌
        0x1F486, // 💆
        0x1F486, // U
        0x1F486, // U
        0x1F487, // 💇
        0x1F487, // U
        0x1F487, // U
        0x1F6B6, // 🚶
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F9CD, // 🧍
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CE, // 🧎
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F3C3, // 🏃
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F483, // 💃
        0x1F57A, // 🕺
        0x1F574, // 🕴
        0x1F46F, // 👯
        0x1F46F, // U
        0x1F46F, // U
        0x1F9D6, // 🧖
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D7, // 🧗
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F93A, // 🤺
        0x1F3C7, // 🏇
        0x26F7, // ⛷
        0x1F3C2, // 🏂
        0x1F3CC, // 🏌
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3C4, // 🏄
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F6A3, // 🚣
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F3CA, // 🏊
        0x1F3CA, // U
        0x1F3CA, // U
        0x26F9, // ⛹
        0x26F9, // U
        0x26F9, // U
        0x1F3CB, // 🏋
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F6B4, // 🚴
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B5, // 🚵
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F938, // 🤸
        0x1F938, // U
        0x1F938, // U
        0x1F93C, // 🤼
        0x1F93C, // U
        0x1F93C, // U
        0x1F93D, // 🤽
        0x1F93D, // U
        0x1F93D, // U
        0x1F93E, // 🤾
        0x1F93E, // U
        0x1F93E, // U
        0x1F939, // 🤹
        0x1F939, // U
        0x1F939, // U
        0x1F9D8, // 🧘
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F6C0, // 🛀
        0x1F6CC, // 🛌
        0x1F9D1, // U
        0x1F46D, // 👭
        0x1F46B, // 👫
        0x1F46C, // 👬
        0x1F48F, // 💏
        0x1F469, // U
        0x1F468, // U
        0x1F469, // U
        0x1F491, // 💑
        0x1F469, // U
        0x1F468, // U
        0x1F469, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F5E3, // 🗣
        0x1F464, // 👤
        0x1F465, // 👥
        0x1FAC2, // 🫂
        0x1F46A, // 👪
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F463, // 👣
        0x1F9B0, // 🦰
        0x1F9B1, // 🦱
        0x1F9B3, // 🦳
        0x1F9B2, // 🦲
        0x1F435, // 🐵
        0x1F412, // 🐒
        0x1F98D, // 🦍
        0x1F9A7, // 🦧
        0x1F436, // 🐶
        0x1F415, // 🐕
        0x1F9AE, // 🦮
        0x1F415, // U
        0x1F429, // 🐩
        0x1F43A, // 🐺
        0x1F98A, // 🦊
        0x1F99D, // 🦝
        0x1F431, // 🐱
        0x1F408, // 🐈
        0x1F408, // U
        0x1F981, // 🦁
        0x1F42F, // 🐯
        0x1F405, // 🐅
        0x1F406, // 🐆
        0x1F434, // 🐴
        0x1FACE, // 🫎
        0x1FACF, // 🫏
        0x1F40E, // 🐎
        0x1F984, // 🦄
        0x1F993, // 🦓
        0x1F98C, // 🦌
        0x1F9AC, // 🦬
        0x1F42E, // 🐮
        0x1F402, // 🐂
        0x1F403, // 🐃
        0x1F404, // 🐄
        0x1F437, // 🐷
        0x1F416, // 🐖
        0x1F417, // 🐗
        0x1F43D, // 🐽
        0x1F40F, // 🐏
        0x1F411, // 🐑
        0x1F410, // 🐐
        0x1F42A, // 🐪
        0x1F42B, // 🐫
        0x1F999, // 🦙
        0x1F992, // 🦒
        0x1F418, // 🐘
        0x1F9A3, // 🦣
        0x1F98F, // 🦏
        0x1F99B, // 🦛
        0x1F42D, // 🐭
        0x1F401, // 🐁
        0x1F400, // 🐀
        0x1F439, // 🐹
        0x1F430, // 🐰
        0x1F407, // 🐇
        0x1F43F, // 🐿
        0x1F9AB, // 🦫
        0x1F994, // 🦔
        0x1F987, // 🦇
        0x1F43B, // 🐻
        0x1F43B, // U
        0x1F428, // 🐨
        0x1F43C, // 🐼
        0x1F9A5, // 🦥
        0x1F9A6, // 🦦
        0x1F9A8, // 🦨
        0x1F998, // 🦘
        0x1F9A1, // 🦡
        0x1F43E, // 🐾
        0x1F983, // 🦃
        0x1F414, // 🐔
        0x1F413, // 🐓
        0x1F423, // 🐣
        0x1F424, // 🐤
        0x1F425, // 🐥
        0x1F426, // 🐦
        0x1F427, // 🐧
        0x1F54A, // 🕊
        0x1F985, // 🦅
        0x1F986, // 🦆
        0x1F9A2, // 🦢
        0x1F989, // 🦉
        0x1F9A4, // 🦤
        0x1FAB6, // 🪶
        0x1F9A9, // 🦩
        0x1F99A, // 🦚
        0x1F99C, // 🦜
        0x1FABD, // 🪽
        0x1F426, // U
        0x1FABF, // 🪿
        0x1F426, // U
        0x1F438, // 🐸
        0x1F40A, // 🐊
        0x1F422, // 🐢
        0x1F98E, // 🦎
        0x1F40D, // 🐍
        0x1F432, // 🐲
        0x1F409, // 🐉
        0x1F995, // 🦕
        0x1F996, // 🦖
        0x1F433, // 🐳
        0x1F40B, // 🐋
        0x1F42C, // 🐬
        0x1F9AD, // 🦭
        0x1F41F, // 🐟
        0x1F420, // 🐠
        0x1F421, // 🐡
        0x1F988, // 🦈
        0x1F419, // 🐙
        0x1F41A, // 🐚
        0x1FAB8, // 🪸
        0x1FABC, // 🪼
        0x1F40C, // 🐌
        0x1F98B, // 🦋
        0x1F41B, // 🐛
        0x1F41C, // 🐜
        0x1F41D, // 🐝
        0x1FAB2, // 🪲
        0x1F41E, // 🐞
        0x1F997, // 🦗
        0x1FAB3, // 🪳
        0x1F577, // 🕷
        0x1F578, // 🕸
        0x1F982, // 🦂
        0x1F99F, // 🦟
        0x1FAB0, // 🪰
        0x1FAB1, // 🪱
        0x1F9A0, // 🦠
        0x1F490, // 💐
        0x1F338, // 🌸
        0x1F4AE, // 💮
        0x1FAB7, // 🪷
        0x1F3F5, // 🏵
        0x1F339, // 🌹
        0x1F940, // 🥀
        0x1F33A, // 🌺
        0x1F33B, // 🌻
        0x1F33C, // 🌼
        0x1F337, // 🌷
        0x1FABB, // 🪻
        0x1F331, // 🌱
        0x1FAB4, // 🪴
        0x1F332, // 🌲
        0x1F333, // 🌳
        0x1F334, // 🌴
        0x1F335, // 🌵
        0x1F33E, // 🌾
        0x1F33F, // 🌿
        0x2618, // ☘
        0x1F340, // 🍀
        0x1F341, // 🍁
        0x1F342, // 🍂
        0x1F343, // 🍃
        0x1FAB9, // 🪹
        0x1FABA, // 🪺
        0x1F344, // 🍄
        0x1F347, // 🍇
        0x1F348, // 🍈
        0x1F349, // 🍉
        0x1F34A, // 🍊
        0x1F34B, // 🍋
        0x1F34B, // U
        0x1F34C, // 🍌
        0x1F34D, // 🍍
        0x1F96D, // 🥭
        0x1F34E, // 🍎
        0x1F34F, // 🍏
        0x1F350, // 🍐
        0x1F351, // 🍑
        0x1F352, // 🍒
        0x1F353, // 🍓
        0x1FAD0, // 🫐
        0x1F95D, // 🥝
        0x1F345, // 🍅
        0x1FAD2, // 🫒
        0x1F965, // 🥥
        0x1F951, // 🥑
        0x1F346, // 🍆
        0x1F954, // 🥔
        0x1F955, // 🥕
        0x1F33D, // 🌽
        0x1F336, // 🌶
        0x1FAD1, // 🫑
        0x1F952, // 🥒
        0x1F96C, // 🥬
        0x1F966, // 🥦
        0x1F9C4, // 🧄
        0x1F9C5, // 🧅
        0x1F95C, // 🥜
        0x1FAD8, // 🫘
        0x1F330, // 🌰
        0x1FADA, // 🫚
        0x1FADB, // 🫛
        0x1F344, // U
        0x1F35E, // 🍞
        0x1F950, // 🥐
        0x1F956, // 🥖
        0x1FAD3, // 🫓
        0x1F968, // 🥨
        0x1F96F, // 🥯
        0x1F95E, // 🥞
        0x1F9C7, // 🧇
        0x1F9C0, // 🧀
        0x1F356, // 🍖
        0x1F357, // 🍗
        0x1F969, // 🥩
        0x1F953, // 🥓
        0x1F354, // 🍔
        0x1F35F, // 🍟
        0x1F355, // 🍕
        0x1F32D, // 🌭
        0x1F96A, // 🥪
        0x1F32E, // 🌮
        0x1F32F, // 🌯
        0x1FAD4, // 🫔
        0x1F959, // 🥙
        0x1F9C6, // 🧆
        0x1F95A, // 🥚
        0x1F373, // 🍳
        0x1F958, // 🥘
        0x1F372, // 🍲
        0x1FAD5, // 🫕
        0x1F963, // 🥣
        0x1F957, // 🥗
        0x1F37F, // 🍿
        0x1F9C8, // 🧈
        0x1F9C2, // 🧂
        0x1F96B, // 🥫
        0x1F371, // 🍱
        0x1F358, // 🍘
        0x1F359, // 🍙
        0x1F35A, // 🍚
        0x1F35B, // 🍛
        0x1F35C, // 🍜
        0x1F35D, // 🍝
        0x1F360, // 🍠
        0x1F362, // 🍢
        0x1F363, // 🍣
        0x1F364, // 🍤
        0x1F365, // 🍥
        0x1F96E, // 🥮
        0x1F361, // 🍡
        0x1F95F, // 🥟
        0x1F960, // 🥠
        0x1F961, // 🥡
        0x1F980, // 🦀
        0x1F99E, // 🦞
        0x1F990, // 🦐
        0x1F991, // 🦑
        0x1F9AA, // 🦪
        0x1F366, // 🍦
        0x1F367, // 🍧
        0x1F368, // 🍨
        0x1F369, // 🍩
        0x1F36A, // 🍪
        0x1F382, // 🎂
        0x1F370, // 🍰
        0x1F9C1, // 🧁
        0x1F967, // 🥧
        0x1F36B, // 🍫
        0x1F36C, // 🍬
        0x1F36D, // 🍭
        0x1F36E, // 🍮
        0x1F36F, // 🍯
        0x1F37C, // 🍼
        0x1F95B, // 🥛
        0x2615, // ☕
        0x1FAD6, // 🫖
        0x1F375, // 🍵
        0x1F376, // 🍶
        0x1F37E, // 🍾
        0x1F377, // 🍷
        0x1F378, // 🍸
        0x1F379, // 🍹
        0x1F37A, // 🍺
        0x1F37B, // 🍻
        0x1F942, // 🥂
        0x1F943, // 🥃
        0x1FAD7, // 🫗
        0x1F964, // 🥤
        0x1F9CB, // 🧋
        0x1F9C3, // 🧃
        0x1F9C9, // 🧉
        0x1F9CA, // 🧊
        0x1F962, // 🥢
        0x1F37D, // 🍽
        0x1F374, // 🍴
        0x1F944, // 🥄
        0x1F52A, // 🔪
        0x1FAD9, // 🫙
        0x1F3FA, // 🏺
        0x1F30D, // 🌍
        0x1F30E, // 🌎
        0x1F30F, // 🌏
        0x1F310, // 🌐
        0x1F5FA, // 🗺
        0x1F5FE, // 🗾
        0x1F9ED, // 🧭
        0x1F3D4, // 🏔
        0x26F0, // ⛰
        0x1F30B, // 🌋
        0x1F5FB, // 🗻
        0x1F3D5, // 🏕
        0x1F3D6, // 🏖
        0x1F3DC, // 🏜
        0x1F3DD, // 🏝
        0x1F3DE, // 🏞
        0x1F3DF, // 🏟
        0x1F3DB, // 🏛
        0x1F3D7, // 🏗
        0x1F9F1, // 🧱
        0x1FAA8, // 🪨
        0x1FAB5, // 🪵
        0x1F6D6, // 🛖
        0x1F3D8, // 🏘
        0x1F3DA, // 🏚
        0x1F3E0, // 🏠
        0x1F3E1, // 🏡
        0x1F3E2, // 🏢
        0x1F3E3, // 🏣
        0x1F3E4, // 🏤
        0x1F3E5, // 🏥
        0x1F3E6, // 🏦
        0x1F3E8, // 🏨
        0x1F3E9, // 🏩
        0x1F3EA, // 🏪
        0x1F3EB, // 🏫
        0x1F3EC, // 🏬
        0x1F3ED, // 🏭
        0x1F3EF, // 🏯
        0x1F3F0, // 🏰
        0x1F492, // 💒
        0x1F5FC, // 🗼
        0x1F5FD, // 🗽
        0x26EA, // ⛪
        0x1F54C, // 🕌
        0x1F6D5, // 🛕
        0x1F54D, // 🕍
        0x26E9, // ⛩
        0x1F54B, // 🕋
        0x26F2, // ⛲
        0x26FA, // ⛺
        0x1F301, // 🌁
        0x1F303, // 🌃
        0x1F3D9, // 🏙
        0x1F304, // 🌄
        0x1F305, // 🌅
        0x1F306, // 🌆
        0x1F307, // 🌇
        0x1F309, // 🌉
        0x2668, // ♨
        0x1F3A0, // 🎠
        0x1F6DD, // 🛝
        0x1F3A1, // 🎡
        0x1F3A2, // 🎢
        0x1F488, // 💈
        0x1F3AA, // 🎪
        0x1F682, // 🚂
        0x1F683, // 🚃
        0x1F684, // 🚄
        0x1F685, // 🚅
        0x1F686, // 🚆
        0x1F687, // 🚇
        0x1F688, // 🚈
        0x1F689, // 🚉
        0x1F68A, // 🚊
        0x1F69D, // 🚝
        0x1F69E, // 🚞
        0x1F68B, // 🚋
        0x1F68C, // 🚌
        0x1F68D, // 🚍
        0x1F68E, // 🚎
        0x1F690, // 🚐
        0x1F691, // 🚑
        0x1F692, // 🚒
        0x1F693, // 🚓
        0x1F694, // 🚔
        0x1F695, // 🚕
        0x1F696, // 🚖
        0x1F697, // 🚗
        0x1F698, // 🚘
        0x1F699, // 🚙
        0x1F6FB, // 🛻
        0x1F69A, // 🚚
        0x1F69B, // 🚛
        0x1F69C, // 🚜
        0x1F3CE, // 🏎
        0x1F3CD, // 🏍
        0x1F6F5, // 🛵
        0x1F9BD, // 🦽
        0x1F9BC, // 🦼
        0x1F6FA, // 🛺
        0x1F6B2, // 🚲
        0x1F6F4, // 🛴
        0x1F6F9, // 🛹
        0x1F6FC, // 🛼
        0x1F68F, // 🚏
        0x1F6E3, // 🛣
        0x1F6E4, // 🛤
        0x1F6E2, // 🛢
        0x26FD, // ⛽
        0x1F6DE, // 🛞
        0x1F6A8, // 🚨
        0x1F6A5, // 🚥
        0x1F6A6, // 🚦
        0x1F6D1, // 🛑
        0x1F6A7, // 🚧
        0x2693, // ⚓
        0x1F6DF, // 🛟
        0x26F5, // ⛵
        0x1F6F6, // 🛶
        0x1F6A4, // 🚤
        0x1F6F3, // 🛳
        0x26F4, // ⛴
        0x1F6E5, // 🛥
        0x1F6A2, // 🚢
        0x2708, // ✈
        0x1F6E9, // 🛩
        0x1F6EB, // 🛫
        0x1F6EC, // 🛬
        0x1FA82, // 🪂
        0x1F4BA, // 💺
        0x1F681, // 🚁
        0x1F69F, // 🚟
        0x1F6A0, // 🚠
        0x1F6A1, // 🚡
        0x1F6F0, // 🛰
        0x1F680, // 🚀
        0x1F6F8, // 🛸
        0x1F6CE, // 🛎
        0x1F9F3, // 🧳
        0x231B, // ⌛
        0x23F3, // ⏳
        0x231A, // ⌚
        0x23F0, // ⏰
        0x23F1, // ⏱
        0x23F2, // ⏲
        0x1F570, // 🕰
        0x1F55B, // 🕛
        0x1F567, // 🕧
        0x1F550, // 🕐
        0x1F55C, // 🕜
        0x1F551, // 🕑
        0x1F55D, // 🕝
        0x1F552, // 🕒
        0x1F55E, // 🕞
        0x1F553, // 🕓
        0x1F55F, // 🕟
        0x1F554, // 🕔
        0x1F560, // 🕠
        0x1F555, // 🕕
        0x1F561, // 🕡
        0x1F556, // 🕖
        0x1F562, // 🕢
        0x1F557, // 🕗
        0x1F563, // 🕣
        0x1F558, // 🕘
        0x1F564, // 🕤
        0x1F559, // 🕙
        0x1F565, // 🕥
        0x1F55A, // 🕚
        0x1F566, // 🕦
        0x1F311, // 🌑
        0x1F312, // 🌒
        0x1F313, // 🌓
        0x1F314, // 🌔
        0x1F315, // 🌕
        0x1F316, // 🌖
        0x1F317, // 🌗
        0x1F318, // 🌘
        0x1F319, // 🌙
        0x1F31A, // 🌚
        0x1F31B, // 🌛
        0x1F31C, // 🌜
        0x1F321, // 🌡
        0x2600, // ☀
        0x1F31D, // 🌝
        0x1F31E, // 🌞
        0x1FA90, // 🪐
        0x2B50, // ⭐
        0x1F31F, // 🌟
        0x1F320, // 🌠
        0x1F30C, // 🌌
        0x2601, // ☁
        0x26C5, // ⛅
        0x26C8, // ⛈
        0x1F324, // 🌤
        0x1F325, // 🌥
        0x1F326, // 🌦
        0x1F327, // 🌧
        0x1F328, // 🌨
        0x1F329, // 🌩
        0x1F32A, // 🌪
        0x1F32B, // 🌫
        0x1F32C, // 🌬
        0x1F300, // 🌀
        0x1F308, // 🌈
        0x1F302, // 🌂
        0x2602, // ☂
        0x2614, // ☔
        0x26F1, // ⛱
        0x26A1, // ⚡
        0x2744, // ❄
        0x2603, // ☃
        0x26C4, // ⛄
        0x2604, // ☄
        0x1F525, // 🔥
        0x1F4A7, // 💧
        0x1F30A, // 🌊
        0x1F383, // 🎃
        0x1F384, // 🎄
        0x1F386, // 🎆
        0x1F387, // 🎇
        0x1F9E8, // 🧨
        0x2728, // ✨
        0x1F388, // 🎈
        0x1F389, // 🎉
        0x1F38A, // 🎊
        0x1F38B, // 🎋
        0x1F38D, // 🎍
        0x1F38E, // 🎎
        0x1F38F, // 🎏
        0x1F390, // 🎐
        0x1F391, // 🎑
        0x1F9E7, // 🧧
        0x1F380, // 🎀
        0x1F381, // 🎁
        0x1F397, // 🎗
        0x1F39F, // 🎟
        0x1F3AB, // 🎫
        0x1F396, // 🎖
        0x1F3C6, // 🏆
        0x1F3C5, // 🏅
        0x1F947, // 🥇
        0x1F948, // 🥈
        0x1F949, // 🥉
        0x26BD, // ⚽
        0x26BE, // ⚾
        0x1F94E, // 🥎
        0x1F3C0, // 🏀
        0x1F3D0, // 🏐
        0x1F3C8, // 🏈
        0x1F3C9, // 🏉
        0x1F3BE, // 🎾
        0x1F94F, // 🥏
        0x1F3B3, // 🎳
        0x1F3CF, // 🏏
        0x1F3D1, // 🏑
        0x1F3D2, // 🏒
        0x1F94D, // 🥍
        0x1F3D3, // 🏓
        0x1F3F8, // 🏸
        0x1F94A, // 🥊
        0x1F94B, // 🥋
        0x1F945, // 🥅
        0x26F3, // ⛳
        0x26F8, // ⛸
        0x1F3A3, // 🎣
        0x1F93F, // 🤿
        0x1F3BD, // 🎽
        0x1F3BF, // 🎿
        0x1F6F7, // 🛷
        0x1F94C, // 🥌
        0x1F3AF, // 🎯
        0x1FA80, // 🪀
        0x1FA81, // 🪁
        0x1F52B, // 🔫
        0x1F3B1, // 🎱
        0x1F52E, // 🔮
        0x1FA84, // 🪄
        0x1F3AE, // 🎮
        0x1F579, // 🕹
        0x1F3B0, // 🎰
        0x1F3B2, // 🎲
        0x1F9E9, // 🧩
        0x1F9F8, // 🧸
        0x1FA85, // 🪅
        0x1FAA9, // 🪩
        0x1FA86, // 🪆
        0x2660, // ♠
        0x2665, // ♥
        0x2666, // ♦
        0x2663, // ♣
        0x265F, // ♟
        0x1F0CF, // 🃏
        0x1F004, // 🀄
        0x1F3B4, // 🎴
        0x1F3AD, // 🎭
        0x1F5BC, // 🖼
        0x1F3A8, // 🎨
        0x1F9F5, // 🧵
        0x1FAA1, // 🪡
        0x1F9F6, // 🧶
        0x1FAA2, // 🪢
        0x1F453, // 👓
        0x1F576, // 🕶
        0x1F97D, // 🥽
        0x1F97C, // 🥼
        0x1F9BA, // 🦺
        0x1F454, // 👔
        0x1F455, // 👕
        0x1F456, // 👖
        0x1F9E3, // 🧣
        0x1F9E4, // 🧤
        0x1F9E5, // 🧥
        0x1F9E6, // 🧦
        0x1F457, // 👗
        0x1F458, // 👘
        0x1F97B, // 🥻
        0x1FA71, // 🩱
        0x1FA72, // 🩲
        0x1FA73, // 🩳
        0x1F459, // 👙
        0x1F45A, // 👚
        0x1FAAD, // 🪭
        0x1F45B, // 👛
        0x1F45C, // 👜
        0x1F45D, // 👝
        0x1F6CD, // 🛍
        0x1F392, // 🎒
        0x1FA74, // 🩴
        0x1F45E, // 👞
        0x1F45F, // 👟
        0x1F97E, // 🥾
        0x1F97F, // 🥿
        0x1F460, // 👠
        0x1F461, // 👡
        0x1FA70, // 🩰
        0x1F462, // 👢
        0x1FAAE, // 🪮
        0x1F451, // 👑
        0x1F452, // 👒
        0x1F3A9, // 🎩
        0x1F393, // 🎓
        0x1F9E2, // 🧢
        0x1FA96, // 🪖
        0x26D1, // ⛑
        0x1F4FF, // 📿
        0x1F484, // 💄
        0x1F48D, // 💍
        0x1F48E, // 💎
        0x1F507, // 🔇
        0x1F508, // 🔈
        0x1F509, // 🔉
        0x1F50A, // 🔊
        0x1F4E2, // 📢
        0x1F4E3, // 📣
        0x1F4EF, // 📯
        0x1F514, // 🔔
        0x1F515, // 🔕
        0x1F3BC, // 🎼
        0x1F3B5, // 🎵
        0x1F3B6, // 🎶
        0x1F399, // 🎙
        0x1F39A, // 🎚
        0x1F39B, // 🎛
        0x1F3A4, // 🎤
        0x1F3A7, // 🎧
        0x1F4FB, // 📻
        0x1F3B7, // 🎷
        0x1FA97, // 🪗
        0x1F3B8, // 🎸
        0x1F3B9, // 🎹
        0x1F3BA, // 🎺
        0x1F3BB, // 🎻
        0x1FA95, // 🪕
        0x1F941, // 🥁
        0x1FA98, // 🪘
        0x1FA87, // 🪇
        0x1FA88, // 🪈
        0x1F4F1, // 📱
        0x1F4F2, // 📲
        0x260E, // ☎
        0x1F4DE, // 📞
        0x1F4DF, // 📟
        0x1F4E0, // 📠
        0x1F50B, // 🔋
        0x1FAAB, // 🪫
        0x1F50C, // 🔌
        0x1F4BB, // 💻
        0x1F5A5, // 🖥
        0x1F5A8, // 🖨
        0x2328, // ⌨
        0x1F5B1, // 🖱
        0x1F5B2, // 🖲
        0x1F4BD, // 💽
        0x1F4BE, // 💾
        0x1F4BF, // 💿
        0x1F4C0, // 📀
        0x1F9EE, // 🧮
        0x1F3A5, // 🎥
        0x1F39E, // 🎞
        0x1F4FD, // 📽
        0x1F3AC, // 🎬
        0x1F4FA, // 📺
        0x1F4F7, // 📷
        0x1F4F8, // 📸
        0x1F4F9, // 📹
        0x1F4FC, // 📼
        0x1F50D, // 🔍
        0x1F50E, // 🔎
        0x1F56F, // 🕯
        0x1F4A1, // 💡
        0x1F526, // 🔦
        0x1F3EE, // 🏮
        0x1FA94, // 🪔
        0x1F4D4, // 📔
        0x1F4D5, // 📕
        0x1F4D6, // 📖
        0x1F4D7, // 📗
        0x1F4D8, // 📘
        0x1F4D9, // 📙
        0x1F4DA, // 📚
        0x1F4D3, // 📓
        0x1F4D2, // 📒
        0x1F4C3, // 📃
        0x1F4DC, // 📜
        0x1F4C4, // 📄
        0x1F4F0, // 📰
        0x1F5DE, // 🗞
        0x1F4D1, // 📑
        0x1F516, // 🔖
        0x1F3F7, // 🏷
        0x1F4B0, // 💰
        0x1FA99, // 🪙
        0x1F4B4, // 💴
        0x1F4B5, // 💵
        0x1F4B6, // 💶
        0x1F4B7, // 💷
        0x1F4B8, // 💸
        0x1F4B3, // 💳
        0x1F9FE, // 🧾
        0x1F4B9, // 💹
        0x2709, // ✉
        0x1F4E7, // 📧
        0x1F4E8, // 📨
        0x1F4E9, // 📩
        0x1F4E4, // 📤
        0x1F4E5, // 📥
        0x1F4E6, // 📦
        0x1F4EB, // 📫
        0x1F4EA, // 📪
        0x1F4EC, // 📬
        0x1F4ED, // 📭
        0x1F4EE, // 📮
        0x1F5F3, // 🗳
        0x270F, // ✏
        0x2712, // ✒
        0x1F58B, // 🖋
        0x1F58A, // 🖊
        0x1F58C, // 🖌
        0x1F58D, // 🖍
        0x1F4DD, // 📝
        0x1F4BC, // 💼
        0x1F4C1, // 📁
        0x1F4C2, // 📂
        0x1F5C2, // 🗂
        0x1F4C5, // 📅
        0x1F4C6, // 📆
        0x1F5D2, // 🗒
        0x1F5D3, // 🗓
        0x1F4C7, // 📇
        0x1F4C8, // 📈
        0x1F4C9, // 📉
        0x1F4CA, // 📊
        0x1F4CB, // 📋
        0x1F4CC, // 📌
        0x1F4CD, // 📍
        0x1F4CE, // 📎
        0x1F587, // 🖇
        0x1F4CF, // 📏
        0x1F4D0, // 📐
        0x2702, // ✂
        0x1F5C3, // 🗃
        0x1F5C4, // 🗄
        0x1F5D1, // 🗑
        0x1F512, // 🔒
        0x1F513, // 🔓
        0x1F50F, // 🔏
        0x1F510, // 🔐
        0x1F511, // 🔑
        0x1F5DD, // 🗝
        0x1F528, // 🔨
        0x1FA93, // 🪓
        0x26CF, // ⛏
        0x2692, // ⚒
        0x1F6E0, // 🛠
        0x1F5E1, // 🗡
        0x2694, // ⚔
        0x1F4A3, // 💣
        0x1FA83, // 🪃
        0x1F3F9, // 🏹
        0x1F6E1, // 🛡
        0x1FA9A, // 🪚
        0x1F527, // 🔧
        0x1FA9B, // 🪛
        0x1F529, // 🔩
        0x2699, // ⚙
        0x1F5DC, // 🗜
        0x2696, // ⚖
        0x1F9AF, // 🦯
        0x1F517, // 🔗
        0x26D3, // U
        0x26D3, // ⛓
        0x1FA9D, // 🪝
        0x1F9F0, // 🧰
        0x1F9F2, // 🧲
        0x1FA9C, // 🪜
        0x2697, // ⚗
        0x1F9EA, // 🧪
        0x1F9EB, // 🧫
        0x1F9EC, // 🧬
        0x1F52C, // 🔬
        0x1F52D, // 🔭
        0x1F4E1, // 📡
        0x1F489, // 💉
        0x1FA78, // 🩸
        0x1F48A, // 💊
        0x1FA79, // 🩹
        0x1FA7C, // 🩼
        0x1FA7A, // 🩺
        0x1FA7B, // 🩻
        0x1F6AA, // 🚪
        0x1F6D7, // 🛗
        0x1FA9E, // 🪞
        0x1FA9F, // 🪟
        0x1F6CF, // 🛏
        0x1F6CB, // 🛋
        0x1FA91, // 🪑
        0x1F6BD, // 🚽
        0x1FAA0, // 🪠
        0x1F6BF, // 🚿
        0x1F6C1, // 🛁
        0x1FAA4, // 🪤
        0x1FA92, // 🪒
        0x1F9F4, // 🧴
        0x1F9F7, // 🧷
        0x1F9F9, // 🧹
        0x1F9FA, // 🧺
        0x1F9FB, // 🧻
        0x1FAA3, // 🪣
        0x1F9FC, // 🧼
        0x1FAE7, // 🫧
        0x1FAA5, // 🪥
        0x1F9FD, // 🧽
        0x1F9EF, // 🧯
        0x1F6D2, // 🛒
        0x1F6AC, // 🚬
        0x26B0, // ⚰
        0x1FAA6, // 🪦
        0x26B1, // ⚱
        0x1F9FF, // 🧿
        0x1FAAC, // 🪬
        0x1F5FF, // 🗿
        0x1FAA7, // 🪧
        0x1FAAA, // 🪪
        0x1F3E7, // 🏧
        0x1F6AE, // 🚮
        0x1F6B0, // 🚰
        0x267F, // ♿
        0x1F6B9, // 🚹
        0x1F6BA, // 🚺
        0x1F6BB, // 🚻
        0x1F6BC, // 🚼
        0x1F6BE, // 🚾
        0x1F6C2, // 🛂
        0x1F6C3, // 🛃
        0x1F6C4, // 🛄
        0x1F6C5, // 🛅
        0x26A0, // ⚠
        0x1F6B8, // 🚸
        0x26D4, // ⛔
        0x1F6AB, // 🚫
        0x1F6B3, // 🚳
        0x1F6AD, // 🚭
        0x1F6AF, // 🚯
        0x1F6B1, // 🚱
        0x1F6B7, // 🚷
        0x1F4F5, // 📵
        0x1F51E, // 🔞
        0x2622, // ☢
        0x2623, // ☣
        0x2B06, // ⬆
        0x2197, // ↗
        0x27A1, // ➡
        0x2198, // ↘
        0x2B07, // ⬇
        0x2199, // ↙
        0x2B05, // ⬅
        0x2196, // ↖
        0x2195, // ↕
        0x2194, // ↔
        0x21A9, // ↩
        0x21AA, // ↪
        0x2934, // ⤴
        0x2935, // ⤵
        0x1F503, // 🔃
        0x1F504, // 🔄
        0x1F519, // 🔙
        0x1F51A, // 🔚
        0x1F51B, // 🔛
        0x1F51C, // 🔜
        0x1F51D, // 🔝
        0x1F6D0, // 🛐
        0x269B, // ⚛
        0x1F549, // 🕉
        0x2721, // ✡
        0x2638, // ☸
        0x262F, // ☯
        0x271D, // ✝
        0x2626, // ☦
        0x262A, // ☪
        0x262E, // ☮
        0x1F54E, // 🕎
        0x1F52F, // 🔯
        0x1FAAF, // 🪯
        0x2648, // ♈
        0x2649, // ♉
        0x264A, // ♊
        0x264B, // ♋
        0x264C, // ♌
        0x264D, // ♍
        0x264E, // ♎
        0x264F, // ♏
        0x2650, // ♐
        0x2651, // ♑
        0x2652, // ♒
        0x2653, // ♓
        0x26CE, // ⛎
        0x1F500, // 🔀
        0x1F501, // 🔁
        0x1F502, // 🔂
        0x25B6, // ▶
        0x23E9, // ⏩
        0x23ED, // ⏭
        0x23EF, // ⏯
        0x25C0, // ◀
        0x23EA, // ⏪
        0x23EE, // ⏮
        0x1F53C, // 🔼
        0x23EB, // ⏫
        0x1F53D, // 🔽
        0x23EC, // ⏬
        0x23F8, // ⏸
        0x23F9, // ⏹
        0x23FA, // ⏺
        0x23CF, // ⏏
        0x1F3A6, // 🎦
        0x1F505, // 🔅
        0x1F506, // 🔆
        0x1F4F6, // 📶
        0x1F6DC, // 🛜
        0x1F4F3, // 📳
        0x1F4F4, // 📴
        0x2640, // ♀
        0x2642, // ♂
        0x26A7, // ⚧
        0x2716, // ✖
        0x2795, // ➕
        0x2796, // ➖
        0x2797, // ➗
        0x1F7F0, // 🟰
        0x267E, // ♾
        0x203C, // ‼
        0x2049, // ⁉
        0x2753, // ❓
        0x2754, // ❔
        0x2755, // ❕
        0x2757, // ❗
        0x3030, // 〰
        0x1F4B1, // 💱
        0x1F4B2, // 💲
        0x2695, // ⚕
        0x267B, // ♻
        0x269C, // ⚜
        0x1F531, // 🔱
        0x1F4DB, // 📛
        0x1F530, // 🔰
        0x2B55, // ⭕
        0x2705, // ✅
        0x2611, // ☑
        0x2714, // ✔
        0x274C, // ❌
        0x274E, // ❎
        0x27B0, // ➰
        0x27BF, // ➿
        0x303D, // 〽
        0x2733, // ✳
        0x2734, // ✴
        0x2747, // ❇
        0x00A9, // ©
        0x00AE, // ®
        0x2122, // ™
        0x0023, // U
        0x002A, // U
        0x0030, // U
        0x0031, // U
        0x0032, // U
        0x0033, // U
        0x0034, // U
        0x0035, // U
        0x0036, // U
        0x0037, // U
        0x0038, // U
        0x0039, // U
        0x1F51F, // 🔟
        0x1F520, // 🔠
        0x1F521, // 🔡
        0x1F522, // 🔢
        0x1F523, // 🔣
        0x1F524, // 🔤
        0x1F170, // 🅰
        0x1F18E, // 🆎
        0x1F171, // 🅱
        0x1F191, // 🆑
        0x1F192, // 🆒
        0x1F193, // 🆓
        0x2139, // ℹ
        0x1F194, // 🆔
        0x24C2, // Ⓜ
        0x1F195, // 🆕
        0x1F196, // 🆖
        0x1F17E, // 🅾
        0x1F197, // 🆗
        0x1F17F, // 🅿
        0x1F198, // 🆘
        0x1F199, // 🆙
        0x1F19A, // 🆚
        0x1F201, // 🈁
        0x1F202, // 🈂
        0x1F237, // 🈷
        0x1F236, // 🈶
        0x1F22F, // 🈯
        0x1F250, // 🉐
        0x1F239, // 🈹
        0x1F21A, // 🈚
        0x1F232, // 🈲
        0x1F251, // 🉑
        0x1F238, // 🈸
        0x1F234, // 🈴
        0x1F233, // 🈳
        0x3297, // ㊗
        0x3299, // ㊙
        0x1F23A, // 🈺
        0x1F235, // 🈵
        0x1F534, // 🔴
        0x1F7E0, // 🟠
        0x1F7E1, // 🟡
        0x1F7E2, // 🟢
        0x1F535, // 🔵
        0x1F7E3, // 🟣
        0x1F7E4, // 🟤
        0x26AB, // ⚫
        0x26AA, // ⚪
        0x1F7E5, // 🟥
        0x1F7E7, // 🟧
        0x1F7E8, // 🟨
        0x1F7E9, // 🟩
        0x1F7E6, // 🟦
        0x1F7EA, // 🟪
        0x1F7EB, // 🟫
        0x2B1B, // ⬛
        0x2B1C, // ⬜
        0x25FC, // ◼
        0x25FB, // ◻
        0x25FE, // ◾
        0x25FD, // ◽
        0x25AA, // ▪
        0x25AB, // ▫
        0x1F536, // 🔶
        0x1F537, // 🔷
        0x1F538, // 🔸
        0x1F539, // 🔹
        0x1F53A, // 🔺
        0x1F53B, // 🔻
        0x1F4A0, // 💠
        0x1F518, // 🔘
        0x1F533, // 🔳
        0x1F532, // 🔲
        0x1F3C1, // 🏁
        0x1F6A9, // 🚩
        0x1F38C, // 🎌
        0x1F3F4, // 🏴
        0x1F3F3, // 🏳
        0x1F3F3, // U
        0x1F3F3, // U
        0x1F3F4, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E6, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E7, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E8, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1E9, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EA, // U
        0x1F1EB, // U
        0x1F1EB, // U
        0x1F1EB, // U
        0x1F1EB, // U
        0x1F1EB, // U
        0x1F1EB, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1EC, // U
        0x1F1ED, // U
        0x1F1ED, // U
        0x1F1ED, // U
        0x1F1ED, // U
        0x1F1ED, // U
        0x1F1ED, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EE, // U
        0x1F1EF, // U
        0x1F1EF, // U
        0x1F1EF, // U
        0x1F1EF, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F0, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F1, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F2, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F3, // U
        0x1F1F4, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F5, // U
        0x1F1F6, // U
        0x1F1F7, // U
        0x1F1F7, // U
        0x1F1F7, // U
        0x1F1F7, // U
        0x1F1F7, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F8, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1F9, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FA, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FB, // U
        0x1F1FC, // U
        0x1F1FC, // U
        0x1F1FD, // U
        0x1F1FE, // U
        0x1F1FE, // U
        0x1F1FF, // U
        0x1F1FF, // U
        0x1F1FF, // U
        0x1F3F4, // U
        0x1F3F4, // U
        0x1F3F4, // U
        0x1F44B, // U+1F3FB  👋🏻    👋🏻    waving hand: light skin tone
        0x1F44B, // U
        0x1F44B, // U
        0x1F44B, // U
        0x1F44B, // U
        0x1F91A, // U
        0x1F91A, // U
        0x1F91A, // U
        0x1F91A, // U
        0x1F91A, // U
        0x1F590, // U
        0x1F590, // U
        0x1F590, // U
        0x1F590, // U
        0x1F590, // U
        0x270B, // U
        0x270B, // U
        0x270B, // U
        0x270B, // U
        0x270B, // U
        0x1F596, // U
        0x1F596, // U
        0x1F596, // U
        0x1F596, // U
        0x1F596, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF2, // U
        0x1FAF2, // U
        0x1FAF2, // U
        0x1FAF2, // U
        0x1FAF2, // U
        0x1FAF3, // U
        0x1FAF3, // U
        0x1FAF3, // U
        0x1FAF3, // U
        0x1FAF3, // U
        0x1FAF4, // U
        0x1FAF4, // U
        0x1FAF4, // U
        0x1FAF4, // U
        0x1FAF4, // U
        0x1FAF7, // U
        0x1FAF7, // U
        0x1FAF7, // U
        0x1FAF7, // U
        0x1FAF7, // U
        0x1FAF8, // U
        0x1FAF8, // U
        0x1FAF8, // U
        0x1FAF8, // U
        0x1FAF8, // U
        0x1F44C, // U
        0x1F44C, // U
        0x1F44C, // U
        0x1F44C, // U
        0x1F44C, // U
        0x1F90C, // U
        0x1F90C, // U
        0x1F90C, // U
        0x1F90C, // U
        0x1F90C, // U
        0x1F90F, // U
        0x1F90F, // U
        0x1F90F, // U
        0x1F90F, // U
        0x1F90F, // U
        0x270C, // U
        0x270C, // U
        0x270C, // U
        0x270C, // U
        0x270C, // U
        0x1F91E, // U
        0x1F91E, // U
        0x1F91E, // U
        0x1F91E, // U
        0x1F91E, // U
        0x1FAF0, // U
        0x1FAF0, // U
        0x1FAF0, // U
        0x1FAF0, // U
        0x1FAF0, // U
        0x1F91F, // U
        0x1F91F, // U
        0x1F91F, // U
        0x1F91F, // U
        0x1F91F, // U
        0x1F918, // U
        0x1F918, // U
        0x1F918, // U
        0x1F918, // U
        0x1F918, // U
        0x1F919, // U
        0x1F919, // U
        0x1F919, // U
        0x1F919, // U
        0x1F919, // U
        0x1F448, // U
        0x1F448, // U
        0x1F448, // U
        0x1F448, // U
        0x1F448, // U
        0x1F449, // U
        0x1F449, // U
        0x1F449, // U
        0x1F449, // U
        0x1F449, // U
        0x1F446, // U
        0x1F446, // U
        0x1F446, // U
        0x1F446, // U
        0x1F446, // U
        0x1F595, // U
        0x1F595, // U
        0x1F595, // U
        0x1F595, // U
        0x1F595, // U
        0x1F447, // U
        0x1F447, // U
        0x1F447, // U
        0x1F447, // U
        0x1F447, // U
        0x261D, // U
        0x261D, // U
        0x261D, // U
        0x261D, // U
        0x261D, // U
        0x1FAF5, // U
        0x1FAF5, // U
        0x1FAF5, // U
        0x1FAF5, // U
        0x1FAF5, // U
        0x1F44D, // U
        0x1F44D, // U
        0x1F44D, // U
        0x1F44D, // U
        0x1F44D, // U
        0x1F44E, // U
        0x1F44E, // U
        0x1F44E, // U
        0x1F44E, // U
        0x1F44E, // U
        0x270A, // U
        0x270A, // U
        0x270A, // U
        0x270A, // U
        0x270A, // U
        0x1F44A, // U
        0x1F44A, // U
        0x1F44A, // U
        0x1F44A, // U
        0x1F44A, // U
        0x1F91B, // U
        0x1F91B, // U
        0x1F91B, // U
        0x1F91B, // U
        0x1F91B, // U
        0x1F91C, // U
        0x1F91C, // U
        0x1F91C, // U
        0x1F91C, // U
        0x1F91C, // U
        0x1F44F, // U
        0x1F44F, // U
        0x1F44F, // U
        0x1F44F, // U
        0x1F44F, // U
        0x1F64C, // U
        0x1F64C, // U
        0x1F64C, // U
        0x1F64C, // U
        0x1F64C, // U
        0x1FAF6, // U
        0x1FAF6, // U
        0x1FAF6, // U
        0x1FAF6, // U
        0x1FAF6, // U
        0x1F450, // U
        0x1F450, // U
        0x1F450, // U
        0x1F450, // U
        0x1F450, // U
        0x1F932, // U
        0x1F932, // U
        0x1F932, // U
        0x1F932, // U
        0x1F932, // U
        0x1F91D, // U
        0x1F91D, // U
        0x1F91D, // U
        0x1F91D, // U
        0x1F91D, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1FAF1, // U
        0x1F64F, // U
        0x1F64F, // U
        0x1F64F, // U
        0x1F64F, // U
        0x1F64F, // U
        0x270D, // U
        0x270D, // U
        0x270D, // U
        0x270D, // U
        0x270D, // U
        0x1F485, // U
        0x1F485, // U
        0x1F485, // U
        0x1F485, // U
        0x1F485, // U
        0x1F933, // U
        0x1F933, // U
        0x1F933, // U
        0x1F933, // U
        0x1F933, // U
        0x1F4AA, // U
        0x1F4AA, // U
        0x1F4AA, // U
        0x1F4AA, // U
        0x1F4AA, // U
        0x1F9B5, // U
        0x1F9B5, // U
        0x1F9B5, // U
        0x1F9B5, // U
        0x1F9B5, // U
        0x1F9B6, // U
        0x1F9B6, // U
        0x1F9B6, // U
        0x1F9B6, // U
        0x1F9B6, // U
        0x1F442, // U
        0x1F442, // U
        0x1F442, // U
        0x1F442, // U
        0x1F442, // U
        0x1F9BB, // U
        0x1F9BB, // U
        0x1F9BB, // U
        0x1F9BB, // U
        0x1F9BB, // U
        0x1F443, // U
        0x1F443, // U
        0x1F443, // U
        0x1F443, // U
        0x1F443, // U
        0x1F476, // U
        0x1F476, // U
        0x1F476, // U
        0x1F476, // U
        0x1F476, // U
        0x1F9D2, // U
        0x1F9D2, // U
        0x1F9D2, // U
        0x1F9D2, // U
        0x1F9D2, // U
        0x1F466, // U
        0x1F466, // U
        0x1F466, // U
        0x1F466, // U
        0x1F466, // U
        0x1F467, // U
        0x1F467, // U
        0x1F467, // U
        0x1F467, // U
        0x1F467, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F9D4, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F471, // U
        0x1F9D3, // U
        0x1F9D3, // U
        0x1F9D3, // U
        0x1F9D3, // U
        0x1F9D3, // U
        0x1F474, // U
        0x1F474, // U
        0x1F474, // U
        0x1F474, // U
        0x1F474, // U
        0x1F475, // U
        0x1F475, // U
        0x1F475, // U
        0x1F475, // U
        0x1F475, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64D, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F64E, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F645, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F646, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F481, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F64B, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F9CF, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F647, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F926, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F937, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F46E, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F575, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F482, // U
        0x1F977, // U
        0x1F977, // U
        0x1F977, // U
        0x1F977, // U
        0x1F977, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1F477, // U
        0x1FAC5, // U
        0x1FAC5, // U
        0x1FAC5, // U
        0x1FAC5, // U
        0x1FAC5, // U
        0x1F934, // U
        0x1F934, // U
        0x1F934, // U
        0x1F934, // U
        0x1F934, // U
        0x1F478, // U
        0x1F478, // U
        0x1F478, // U
        0x1F478, // U
        0x1F478, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F473, // U
        0x1F472, // U
        0x1F472, // U
        0x1F472, // U
        0x1F472, // U
        0x1F472, // U
        0x1F9D5, // U
        0x1F9D5, // U
        0x1F9D5, // U
        0x1F9D5, // U
        0x1F9D5, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F935, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F470, // U
        0x1F930, // U
        0x1F930, // U
        0x1F930, // U
        0x1F930, // U
        0x1F930, // U
        0x1FAC3, // U
        0x1FAC3, // U
        0x1FAC3, // U
        0x1FAC3, // U
        0x1FAC3, // U
        0x1FAC4, // U
        0x1FAC4, // U
        0x1FAC4, // U
        0x1FAC4, // U
        0x1FAC4, // U
        0x1F931, // U
        0x1F931, // U
        0x1F931, // U
        0x1F931, // U
        0x1F931, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F47C, // U
        0x1F47C, // U
        0x1F47C, // U
        0x1F47C, // U
        0x1F47C, // U
        0x1F385, // U
        0x1F385, // U
        0x1F385, // U
        0x1F385, // U
        0x1F385, // U
        0x1F936, // U
        0x1F936, // U
        0x1F936, // U
        0x1F936, // U
        0x1F936, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B8, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9B9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9D9, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DA, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DB, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DC, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F9DD, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F486, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F487, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F6B6, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CD, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9CE, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F3C3, // U
        0x1F483, // U
        0x1F483, // U
        0x1F483, // U
        0x1F483, // U
        0x1F483, // U
        0x1F57A, // U
        0x1F57A, // U
        0x1F57A, // U
        0x1F57A, // U
        0x1F57A, // U
        0x1F574, // U
        0x1F574, // U
        0x1F574, // U
        0x1F574, // U
        0x1F574, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D6, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F9D7, // U
        0x1F3C7, // U
        0x1F3C7, // U
        0x1F3C7, // U
        0x1F3C7, // U
        0x1F3C7, // U
        0x1F3C2, // U
        0x1F3C2, // U
        0x1F3C2, // U
        0x1F3C2, // U
        0x1F3C2, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3CC, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F3C4, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F6A3, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x1F3CA, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x26F9, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F3CB, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B4, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F6B5, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F938, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93D, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F93E, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F939, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F9D8, // U
        0x1F6C0, // U
        0x1F6C0, // U
        0x1F6C0, // U
        0x1F6C0, // U
        0x1F6C0, // U
        0x1F6CC, // U
        0x1F6CC, // U
        0x1F6CC, // U
        0x1F6CC, // U
        0x1F6CC, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F46D, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46D, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46D, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46D, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46D, // U
        0x1F46B, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46B, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46B, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46B, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F46B, // U
        0x1F46C, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F46C, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F46C, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F46C, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F46C, // U
        0x1F48F, // U
        0x1F48F, // U
        0x1F48F, // U
        0x1F48F, // U
        0x1F48F, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F491, // U
        0x1F491, // U
        0x1F491, // U
        0x1F491, // U
        0x1F491, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F9D1, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F468, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F469, // U
        0x1F3FB, // 🏻
        0x1F3FC, // 🏼
        0x1F3FD, // 🏽
        0x1F3FE, // 🏾
        0x1F3FF // 🏿
    ];
    */
