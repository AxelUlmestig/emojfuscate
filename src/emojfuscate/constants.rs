use lazy_static::lazy_static;
use std::collections::HashMap;

pub const BITS_IN_A_BYTE: u32 = 8;
pub const BITS_PER_EMOJI: u32 = 10;
pub const MAX_EMOJI_VALUE: u32 = u32::pow(2, BITS_PER_EMOJI);
pub const STOP_EMOJI_VALUE: u32 = MAX_EMOJI_VALUE + BITS_IN_A_BYTE + 1;

// This code isn't dead at all, I'm not sure why Rust is complaining about it
#[allow(dead_code)]
pub fn usize_to_emoji(u: usize) -> char {
    let emoji_unicode = EMOJI[u];
    return char::from_u32(emoji_unicode).unwrap();
}

const EMOJI_VALUE_TUPLES: [(char, u32); 1385] = {
    let mut emoji_value_pairs = [('a', 0); 1385];
    let mut index = 0;

    while index < EMOJI.len() {
        emoji_value_pairs[index] = match char::from_u32(EMOJI[index]) {
            Some(c) => (c, index as u32),
            None => panic!("unexpected unicode"),
        };

        index = index + 1;
    }

    emoji_value_pairs
};

lazy_static! {
    pub static ref EMOJI_VALUES: HashMap<char, u32> = HashMap::from(EMOJI_VALUE_TUPLES);
}

pub const EMOJI: [u32; 1385] = [
    0x1F600, // 0  😀  😀  😀  —   —   —   grinning face
    0x1F603, // 1  😃  😃  😃  😃  😃  😃  grinning face with big eyes
    0x1F604, // 2  😄  😄  😄  😄  —   —   grinning face with smiling eyes
    0x1F601, // 3  😁  😁  😁  😁  😁  😁  beaming face with smiling eyes
    0x1F606, // 4  😆  😆  😆  —   😆  —   grinning squinting face
    0x1F605, // 5  😅  😅  😅  —   😅  —   grinning face with sweat
    0x1F923, // 6  🤣  🤣  —   —   —   —   rolling on the floor laughing
    0x1F602, // 7  😂  😂  😂  😂  —   😂  face with tears of joy
    0x1F642, // 8  🙂  🙂  🙂  —   —   —   slightly smiling face
    0x1F643, // 9  🙃  🙃  —   —   —   —   upside-down face
    0x1FAE0, // 10  🫠  🫠  —   —   —   —   melting face
    0x1F609, // 11  😉  😉  😉  😉  😉  😉  winking face
    0x1F60A, // 12  😊  😊  😊  😊  —   😊  smiling face with smiling eyes
    0x1F607, // 13  😇  😇  —   —   —   —   smiling face with halo
    0x1F970, // 14  🥰  🥰  —   —   —   —   smiling face with hearts
    0x1F60D, // 15  😍  😍  😍  😍  😍  😍  smiling face with heart-eyes
    0x1F929, // 16  🤩  🤩  —   —   —   —   star-struck
    0x1F618, // 17  😘  😘  😘  😘  —   😘  face blowing a kiss
    0x1F617, // 18  😗  😗  —   —   —   —   kissing face
    0x263A,  // 19 ☺   ☺   ☺   ☺   —   ☺   smiling face
    0x1F61A, // 20  😚  😚  😚  😚  —   😚  kissing face with closed eyes
    0x1F619, // 21  😙  😙  —   —   —   —   kissing face with smiling eyes
    0x1F972, // 22  🥲  🥲  —   —   —   —   smiling face with tear
    0x1F60B, // 23  😋  😋  😋  —   😋  —   face savoring food
    0x1F61B, // 24  😛  😛  —   —   —   —   face with tongue
    0x1F61C, // 25  😜  😜  😜  😜  😜  😜  winking face with tongue
    0x1F92A, // 26  🤪  🤪  —   —   —   —   zany face
    0x1F61D, // 27  😝  😝  😝  😝  —   —   squinting face with tongue
    0x1F911, // 28  🤑  🤑  —   —   —   —   money-mouth face
    0x1F917, // 29  🤗  🤗  —   —   —   —   smiling face with open hands
    0x1F92D, // 30  🤭  🤭  —   —   —   —   face with hand over mouth
    0x1FAE2, // 31  🫢  🫢  —   —   —   —   face with open eyes and hand over mouth
    0x1FAE3, // 32  🫣  🫣  —   —   —   —   face with peeking eye
    0x1F92B, // 33  🤫  🤫  —   —   —   —   shushing face
    0x1F914, // 34  🤔  🤔  —   —   —   —   thinking face
    0x1FAE1, // 35  🫡  🫡  —   —   —   —   saluting face
    0x1F910, // 36  🤐  🤐  —   —   —   —   zipper-mouth face
    0x1F928, // 37  🤨  🤨  —   —   —   —   face with raised eyebrow
    0x1F610, // 38  😐  😐  —   —   —   —   neutral face
    0x1F611, // 39  😑  😑  —   —   —   —   expressionless face
    0x1F636, // 40  😶  😶  —   —   —   —   face without mouth
    0x1FAE5, // 41  🫥  🫥  —   —   —   —   dotted line face
    0x1F60F, // 42  😏  😏  😏  😏  😏  😏  smirking face
    0x1F612, // 43  😒  😒  😒  😒  😒  😒  unamused face
    0x1F644, // 44  🙄  🙄  —   —   —   —   face with rolling eyes
    0x1F62C, // 45  😬  😬  —   —   —   —   grimacing face
    0x1F925, // 46  🤥  🤥  —   —   —   —   lying face
    0x1FAE8, // 47  🫨  🫨  —   —   —   —   shaking face
    0x1F60C, // 48  😌  😌  😌  😌  😌  😌  relieved face
    0x1F614, // 49  😔  😔  😔  😔  😔  😔  pensive face
    0x1F62A, // 50  😪  😪  😪  😪  —   😪  sleepy face
    0x1F924, // 51  🤤  🤤  —   —   —   —   drooling face
    0x1F634, // 52  😴  😴  —   —   —   —   sleeping face
    0x1F637, // 53  😷  😷  😷  😷  —   😷  face with medical mask
    0x1F912, // 54  🤒  🤒  —   —   —   —   face with thermometer
    0x1F915, // 55  🤕  🤕  —   —   —   —   face with head-bandage
    0x1F922, // 56  🤢  🤢  —   —   —   —   nauseated face
    0x1F92E, // 57  🤮  🤮  —   —   —   —   face vomiting
    0x1F927, // 58  🤧  🤧  —   —   —   —   sneezing face
    0x1F975, // 59  🥵  🥵  —   —   —   —   hot face
    0x1F976, // 60  🥶  🥶  —   —   —   —   cold face
    0x1F974, // 61  🥴  🥴  —   —   —   —   woozy face
    0x1F635, // 62  😵  😵  😵  —   😵  😵  face with crossed-out eyes
    0x1F92F, // 63  🤯  🤯  —   —   —   —   exploding head
    0x1F920, // 64  🤠  🤠  —   —   —   —   cowboy hat face
    0x1F973, // 65  🥳  🥳  —   —   —   —   partying face
    0x1F978, // 66  🥸  🥸  —   —   —   —   disguised face
    0x1F60E, // 67  😎  😎  😎  —   —   —   smiling face with sunglasses
    0x1F913, // 68  🤓  🤓  —   —   —   —   nerd face
    0x1F9D0, // 69  🧐  🧐  —   —   —   —   face with monocle
    0x1F615, // 70  😕  😕  😕  —   —   —   confused face
    0x1FAE4, // 71  🫤  🫤  —   —   —   —   face with diagonal mouth
    0x1F61F, // 72  😟  😟  😟  —   —   —   worried face
    0x1F641, // 73  🙁  🙁  —   —   —   —   slightly frowning face
    0x2639,  // 74 ☹   ☹   —   —   —   —   frowning face
    0x1F62E, // 75  😮  😮  —   —   —   —   face with open mouth
    0x1F62F, // 76  😯  😯  —   —   —   —   hushed face
    0x1F632, // 77  😲  😲  😲  😲  —   😲  astonished face
    0x1F633, // 78  😳  😳  😳  😳  —   😳  flushed face
    0x1F97A, // 79  🥺  🥺  —   —   —   —   pleading face
    0x1F979, // 80  🥹  🥹  —   —   —   —   face holding back tears
    0x1F626, // 81  😦  😦  —   —   —   —   frowning face with open mouth
    0x1F627, // 82  😧  😧  —   —   —   —   anguished face
    0x1F628, // 83  😨  😨  😨  😨  —   😨  fearful face
    0x1F630, // 84  😰  😰  😰  😰  —   😰  anxious face with sweat
    0x1F625, // 85  😥  😥  😥  😥  —   —   sad but relieved face
    0x1F622, // 86  😢  😢  😢  😢  😢  😢  crying face
    0x1F62D, // 87  😭  😭  😭  😭  😭  😭  loudly crying face
    0x1F631, // 88  😱  😱  😱  😱  😱  😱  face screaming in fear
    0x1F616, // 89  😖  😖  😖  😖  😖  😖  confounded face
    0x1F623, // 90  😣  😣  😣  😣  😣  😣  persevering face
    0x1F61E, // 91  😞  😞  😞  😞  😞  —   disappointed face
    0x1F613, // 92  😓  😓  😓  😓  😓  😓  downcast face with sweat
    0x1F629, // 93  😩  😩  😩  —   —   😩  weary face
    0x1F62B, // 94  😫  😫  😫  —   —   😫  tired face
    0x1F971, // 95  🥱  🥱  —   —   —   —   yawning face
    0x1F624, // 96  😤  😤  😤  —   —   😤  face with steam from nose
    0x1F621, // 97  😡  😡  😡  😡  😡  😡  enraged face
    0x1F620, // 98  😠  😠  😠  😠  😠  😠  angry face
    0x1F92C, // 99  🤬  🤬  —   —   —   —   face with symbols on mouth
    0x1F608, // 100  😈  😈  —   —   —   —   smiling face with horns
    0x1F47F, // 101  👿  👿  👿  👿  —   👿  angry face with horns
    0x1F480, // 102  💀  💀  💀  💀  —   💀  skull
    0x2620,  // 103 ☠   ☠   —   —   —   —   skull and crossbones
    0x1F4A9, // 104  💩  💩  💩  💩  —   💩  pile of poo
    0x1F921, // 105  🤡  🤡  —   —   —   —   clown face
    0x1F479, // 106  👹  👹  👹  —   —   👹  ogre
    0x1F47A, // 107  👺  👺  👺  —   —   👺  goblin
    0x1F47B, // 108  👻  👻  👻  👻  —   👻  ghost
    0x1F47D, // 109  👽  👽  👽  👽  —   👽  alien
    0x1F47E, // 110  👾  👾  👾  👾  —   👾  alien monster
    0x1F916, // 111  🤖  🤖  —   —   —   —   robot
    0x1F63A, // 112  😺  😺  😺  —   —   😺  grinning cat
    0x1F638, // 113  😸  😸  😸  —   —   😸  grinning cat with smiling eyes
    0x1F639, // 114  😹  😹  😹  —   —   😹  cat with tears of joy
    0x1F63B, // 115  😻  😻  😻  —   —   😻  smiling cat with heart-eyes
    0x1F63C, // 116  😼  😼  😼  —   —   😼  cat with wry smile
    0x1F63D, // 117  😽  😽  😽  —   —   😽  kissing cat
    0x1F640, // 118  🙀  🙀  🙀  —   —   🙀  weary cat
    0x1F63F, // 119  😿  😿  😿  —   —   😿  crying cat
    0x1F63E, // 120  😾  😾  😾  —   —   😾  pouting cat
    0x1F648, // 121  🙈  🙈  🙈  —   —   🙈  see-no-evil monkey
    0x1F649, // 122  🙉  🙉  🙉  —   —   🙉  hear-no-evil monkey
    0x1F64A, // 123  🙊  🙊  🙊  —   —   🙊  speak-no-evil monkey
    0x1F48C, // 124  💌  💌  💌  —   💌  💌  love letter
    0x1F498, // 125  💘  💘  💘  💘  —   💘  heart with arrow
    0x1F49D, // 126  💝  💝  💝  💝  —   💝  heart with ribbon
    0x1F496, // 127  💖  💖  💖  —   —   💖  sparkling heart
    0x1F497, // 128  💗  💗  💗  💗  —   —   growing heart
    0x1F493, // 129  💓  💓  💓  💓  💓  💓  beating heart
    0x1F49E, // 130  💞  💞  💞  —   —   💞  revolving hearts
    0x1F495, // 131  💕  💕  💕  —   💕  💕  two hearts
    0x1F49F, // 132  💟  💟  💟  💟  —   —   heart decoration
    0x2763,  // 133 ❣   ❣   —   —   —   —   heart exclamation
    0x1F494, // 134  💔  💔  💔  💔  💔  💔  broken heart
    0x2764,  // 135 ❤   ❤   ❤   ❤   ❤   ❤   red heart
    0x1FA77, // 136  🩷  🩷  —   —   —   —   pink heart
    0x1F9E1, // 137  🧡  🧡  —   —   —   —   orange heart
    0x1F49B, // 138  💛  💛  💛  💛  —   💛  yellow heart
    0x1F49A, // 139  💚  💚  💚  💚  —   💚  green heart
    0x1F499, // 140  💙  💙  💙  💙  —   💙  blue heart
    0x1FA75, // 141  🩵  🩵  —   —   —   —   light blue heart
    0x1F49C, // 142  💜  💜  💜  💜  —   💜  purple heart
    0x1F90E, // 143  🤎  🤎  —   —   —   —   brown heart
    0x1F5A4, // 144  🖤  🖤  —   —   —   —   black heart
    0x1FA76, // 145  🩶  🩶  —   —   —   —   grey heart
    0x1F90D, // 146  🤍  🤍  —   —   —   —   white heart
    0x1F48B, // 147  💋  💋  💋  💋  💋  💋  kiss mark
    0x1F4AF, // 148  💯  💯  💯  —   —   💯  hundred points
    0x1F4A2, // 149  💢  💢  💢  💢  💢  💢  anger symbol
    0x1F4A5, // 150  💥  💥  💥  —   💥  💥  collision
    0x1F4AB, // 151  💫  💫  💫  —   —   💫  dizzy
    0x1F4A6, // 152  💦  💦  💦  💦  💦  💦  sweat droplets
    0x1F4A8, // 153  💨  💨  💨  💨  💨  💨  dashing away
    0x1F573, // 154  🕳  🕳  —   —   —   —   hole
    0x1F4AC, // 155  💬  💬  💬  —   —   💬  speech balloon
    0x1F5E8, // 156  🗨  🗨  —   —   —   —   left speech bubble
    0x1F5EF, // 157  🗯  🗯  —   —   —   —   right anger bubble
    0x1F4AD, // 158  💭  💭  —   —   —   —   thought balloon
    0x1F4A4, // 159  💤  💤  💤  💤  💤  💤  ZZZ
    0x1F44B, // 160  👋  👋  👋  👋  —   👋  waving hand
    0x1F91A, // 161  🤚  🤚  —   —   —   —   raised back of hand
    0x1F590, // 162  🖐  🖐  —   —   —   —   hand with fingers splayed
    0x270B,  // 163 ✋  ✋  ✋  ✋  ✋  ✋  raised hand
    0x1F596, // 164  🖖  🖖  —   —   —   —   vulcan salute
    0x1FAF1, // 165  🫱  🫱  —   —   —   —   rightwards hand
    0x1FAF2, // 166  🫲  🫲  —   —   —   —   leftwards hand
    0x1FAF3, // 167  🫳  🫳  —   —   —   —   palm down hand
    0x1FAF4, // 168  🫴  🫴  —   —   —   —   palm up hand
    0x1FAF7, // 169  🫷  🫷  —   —   —   —   leftwards pushing hand
    0x1FAF8, // 170  🫸  🫸  —   —   —   —   rightwards pushing hand
    0x1F44C, // 171  👌  👌  👌  👌  —   👌  OK hand
    0x1F90C, // 172  🤌  🤌  —   —   —   —   pinched fingers
    0x1F90F, // 173  🤏  🤏  —   —   —   —   pinching hand
    0x270C,  // 174 ✌   ✌   ✌   ✌   ✌   ✌   victory hand
    0x1F91E, // 175  🤞  🤞  —   —   —   —   crossed fingers
    0x1FAF0, // 176  🫰  🫰  —   —   —   —   hand with index finger and thumb crossed
    0x1F91F, // 177  🤟  🤟  —   —   —   —   love-you gesture
    0x1F918, // 178  🤘  🤘  —   —   —   —   sign of the horns
    0x1F919, // 179  🤙  🤙  —   —   —   —   call me hand
    0x1F448, // 180  👈  👈  👈  👈  —   👈  backhand index pointing left
    0x1F449, // 181  👉  👉  👉  👉  —   👉  backhand index pointing right
    0x1F446, // 182  👆  👆  👆  👆  —   👆  backhand index pointing up
    0x1F595, // 183  🖕  🖕  —   —   —   —   middle finger
    0x1F447, // 184  👇  👇  👇  👇  —   👇  backhand index pointing down
    0x261D,  // 185 ☝   ☝   ☝   ☝   —   ☝   index pointing up
    0x1FAF5, // 186  🫵  🫵  —   —   —   —   index pointing at the viewer
    0x1F44D, // 187  👍  👍  👍  👍  👍  👍  thumbs up
    0x1F44E, // 188  👎  👎  👎  👎  —   👎  thumbs down
    0x270A,  // 189 ✊  ✊  ✊  ✊  ✊  ✊  raised fist
    0x1F44A, // 190  👊  👊  👊  👊  👊  👊  oncoming fist
    0x1F91B, // 191  🤛  🤛  —   —   —   —   left-facing fist
    0x1F91C, // 192  🤜  🤜  —   —   —   —   right-facing fist
    0x1F44F, // 193  👏  👏  👏  👏  —   👏  clapping hands
    0x1F64C, // 194  🙌  🙌  🙌  🙌  —   🙌  raising hands
    0x1FAF6, // 195  🫶  🫶  —   —   —   —   heart hands
    0x1F450, // 196  👐  👐  👐  👐  —   —   open hands
    0x1F932, // 197  🤲  🤲  —   —   —   —   palms up together
    0x1F91D, // 198  🤝  🤝  —   —   —   —   handshake
    0x1F64F, // 199  🙏  🙏  🙏  🙏  —   🙏  folded hands
    0x270D,  // 200 ✍   ✍   —   —   —   —   writing hand
    0x1F485, // 201  💅  💅  💅  💅  —   💅  nail polish
    0x1F933, // 202  🤳  🤳  —   —   —   —   selfie
    0x1F4AA, // 203  💪  💪  💪  💪  —   💪  flexed biceps
    0x1F9BE, // 204  🦾  🦾  —   —   —   —   mechanical arm
    0x1F9BF, // 205  🦿  🦿  —   —   —   —   mechanical leg
    0x1F9B5, // 206  🦵  🦵  —   —   —   —   leg
    0x1F9B6, // 207  🦶  🦶  —   —   —   —   foot
    0x1F442, // 208  👂  👂  👂  👂  👂  👂  ear
    0x1F9BB, // 209  🦻  🦻  —   —   —   —   ear with hearing aid
    0x1F443, // 210  👃  👃  👃  👃  —   👃  nose
    0x1F9E0, // 211  🧠  🧠  —   —   —   —   brain
    0x1FAC0, // 212  🫀  🫀  —   —   —   —   anatomical heart
    0x1FAC1, // 213  🫁  🫁  —   —   —   —   lungs
    0x1F9B7, // 214  🦷  🦷  —   —   —   —   tooth
    0x1F9B4, // 215  🦴  🦴  —   —   —   —   bone
    0x1F440, // 216  👀  👀  👀  👀  👀  👀  eyes
    0x1F441, // 217  👁  👁  —   —   —   —   eye
    0x1F445, // 218  👅  👅  👅  —   —   👅  tongue
    0x1F444, // 219  👄  👄  👄  👄  —   👄  mouth
    0x1FAE6, // 220  🫦  🫦  —   —   —   —   biting lip
    0x1F476, // 221  👶  👶  👶  👶  —   👶  baby
    0x1F9D2, // 222  🧒  🧒  —   —   —   —   child
    0x1F466, // 223  👦  👦  👦  👦  —   —   boy
    0x1F467, // 224  👧  👧  👧  👧  —   —   girl
    0x1F9D1, // 225  🧑  🧑  —   —   —   —   person
    0x1F471, // 226  👱  👱  👱  👱  —   👱  person: blond hair
    0x1F468, // 227  👨  👨  👨  👨  —   👨  man
    0x1F9D4, // 228  🧔  🧔  —   —   —   —   person: beard
    0x1F469, // 229  👩  👩  👩  👩  —   👩  woman
    0x1F9D3, // 230  🧓  🧓  —   —   —   —   older person
    0x1F474, // 231  👴  👴  👴  👴  —   👴  old man
    0x1F475, // 232  👵  👵  👵  👵  —   👵  old woman
    0x1F64D, // 233  🙍  🙍  🙍  —   —   🙍  person frowning
    0x1F64E, // 234  🙎  🙎  🙎  —   —   🙎  person pouting
    0x1F645, // 235  🙅  🙅  🙅  🙅  —   🙅  person gesturing NO
    0x1F646, // 236  🙆  🙆  🙆  🙆  —   🙆  person gesturing OK
    0x1F481, // 237  💁  💁  💁  💁  —   —   person tipping hand
    0x1F64B, // 238  🙋  🙋  🙋  —   —   🙋  person raising hand
    0x1F9CF, // 239  🧏  🧏  —   —   —   —   deaf person
    0x1F647, // 240  🙇  🙇  🙇  🙇  —   🙇  person bowing
    0x1F926, // 241  🤦  🤦  —   —   —   —   person facepalming
    0x1F937, // 242  🤷  🤷  —   —   —   —   person shrugging
    0x1F46E, // 243  👮  👮  👮  👮  —   👮  police officer
    0x1F575, // 244  🕵  🕵  —   —   —   —   detective
    0x1F482, // 245  💂  💂  💂  💂  —   —   guard
    0x1F977, // 246  🥷  🥷  —   —   —   —   ninja
    0x1F477, // 247  👷  👷  👷  👷  —   👷  construction worker
    0x1FAC5, // 248  🫅  🫅  —   —   —   —   person with crown
    0x1F934, // 249  🤴  🤴  —   —   —   —   prince
    0x1F478, // 250  👸  👸  👸  👸  —   👸  princess
    0x1F473, // 251  👳  👳  👳  👳  —   👳  person wearing turban
    0x1F472, // 252  👲  👲  👲  👲  —   👲  person with skullcap
    0x1F9D5, // 253  🧕  🧕  —   —   —   —   woman with headscarf
    0x1F935, // 254  🤵  🤵  —   —   —   —   person in tuxedo
    0x1F470, // 255  👰  👰  👰  —   —   👰  person with veil
    0x1F930, // 256  🤰  🤰  —   —   —   —   pregnant woman
    0x1FAC3, // 257  🫃  🫃  —   —   —   —   pregnant man
    0x1FAC4, // 258  🫄  🫄  —   —   —   —   pregnant person
    0x1F931, // 259  🤱  🤱  —   —   —   —   breast-feeding
    0x1F47C, // 260  👼  👼  👼  👼  —   👼  baby angel
    0x1F385, // 261  🎅  🎅  🎅  🎅  —   🎅  Santa Claus
    0x1F936, // 262  🤶  🤶  —   —   —   —   Mrs. Claus
    0x1F9B8, // 263  🦸  🦸  —   —   —   —   superhero
    0x1F9B9, // 264  🦹  🦹  —   —   —   —   supervillain
    0x1F9D9, // 265  🧙  🧙  —   —   —   —   mage
    0x1F9DA, // 266  🧚  🧚  —   —   —   —   fairy
    0x1F9DB, // 267  🧛  🧛  —   —   —   —   vampire
    0x1F9DC, // 268  🧜  🧜  —   —   —   —   merperson
    0x1F9DD, // 269  🧝  🧝  —   —   —   —   elf
    0x1F9DE, // 270  🧞  🧞  —   —   —   —   genie
    0x1F9DF, // 271  🧟  🧟  —   —   —   —   zombie
    0x1F9CC, // 272  🧌  🧌  —   —   —   —   troll
    0x1F486, // 273  💆  💆  💆  💆  —   💆  person getting massage
    0x1F487, // 274  💇  💇  💇  💇  —   💇  person getting haircut
    0x1F6B6, // 275  🚶  🚶  🚶  🚶  —   🚶  person walking
    0x1F9CD, // 276  🧍  🧍  —   —   —   —   person standing
    0x1F9CE, // 277  🧎  🧎  —   —   —   —   person kneeling
    0x1F3C3, // 278  🏃  🏃  🏃  🏃  🏃  🏃  person running
    0x1F483, // 279  💃  💃  💃  💃  —   💃  woman dancing
    0x1F57A, // 280  🕺  🕺  —   —   —   —   man dancing
    0x1F574, // 281  🕴  🕴  —   —   —   —   person in suit levitating
    0x1F46F, // 282  👯  👯  👯  👯  —   👯  people with bunny ears
    0x1F9D6, // 283  🧖  🧖  —   —   —   —   person in steamy room
    0x1F9D7, // 284  🧗  🧗  —   —   —   —   person climbing
    0x1F93A, // 285  🤺  🤺  —   —   —   —   person fencing
    0x1F3C7, // 286  🏇  🏇  —   —   —   —   horse racing
    0x26F7,  // 287 ⛷   ⛷   —   —   —   —   skier
    0x1F3C2, // 288  🏂  🏂  🏂  —   🏂  🏂  snowboarder
    0x1F3CC, // 289  🏌  🏌  —   —   —   —   person golfing
    0x1F3C4, // 290  🏄  🏄  🏄  🏄  —   🏄  person surfing
    0x1F6A3, // 291  🚣  🚣  —   —   —   —   person rowing boat
    0x1F3CA, // 292  🏊  🏊  🏊  🏊  —   🏊  person swimming
    0x26F9,  // 293 ⛹   ⛹   —   —   —   —   person bouncing ball
    0x1F3CB, // 294  🏋  🏋  —   —   —   —   person lifting weights
    0x1F6B4, // 295  🚴  🚴  —   —   —   —   person biking
    0x1F6B5, // 296  🚵  🚵  —   —   —   —   person mountain biking
    0x1F938, // 297  🤸  🤸  —   —   —   —   person cartwheeling
    0x1F93C, // 298  🤼  🤼  —   —   —   —   people wrestling
    0x1F93D, // 299  🤽  🤽  —   —   —   —   person playing water polo
    0x1F93E, // 300  🤾  🤾  —   —   —   —   person playing handball
    0x1F939, // 301  🤹  🤹  —   —   —   —   person juggling
    0x1F9D8, // 302  🧘  🧘  —   —   —   —   person in lotus position
    0x1F6C0, // 303  🛀  🛀  🛀  🛀  —   🛀  person taking bath
    0x1F6CC, // 304  🛌  🛌  —   —   —   —   person in bed
    0x1F46D, // 305  👭  👭  —   —   —   —   women holding hands
    0x1F46B, // 306  👫  👫  👫  👫  —   —   woman and man holding hands
    0x1F46C, // 307  👬  👬  —   —   —   —   men holding hands
    0x1F48F, // 308  💏  💏  💏  💏  —   💏  kiss
    0x1F491, // 309  💑  💑  💑  💑  —   💑  couple with heart
    0x1F5E3, // 310  🗣  🗣  —   —   —   —   speaking head
    0x1F464, // 311  👤  👤  👤  —   👤  —   bust in silhouette
    0x1F465, // 312  👥  👥  —   —   —   —   busts in silhouette
    0x1FAC2, // 313  🫂  🫂  —   —   —   —   people hugging
    0x1F46A, // 314  👪  👪  👪  —   —   👪  family
    0x1F463, // 315  👣  👣  👣  👣  👣  👣  footprints
    0x1F9B0, // 316  🦰  🦰  —   —   —   —   red hair
    0x1F9B1, // 317  🦱  🦱  —   —   —   —   curly hair
    0x1F9B3, // 318  🦳  🦳  —   —   —   —   white hair
    0x1F9B2, // 319  🦲  🦲  —   —   —   —   bald
    0x1F435, // 320  🐵  🐵  🐵  🐵  —   🐵  monkey face
    0x1F412, // 321  🐒  🐒  🐒  🐒  —   —   monkey
    0x1F98D, // 322  🦍  🦍  —   —   —   —   gorilla
    0x1F9A7, // 323  🦧  🦧  —   —   —   —   orangutan
    0x1F436, // 324  🐶  🐶  🐶  🐶  🐶  🐶  dog face
    0x1F415, // 325  🐕  🐕  —   —   —   —   dog
    0x1F9AE, // 326  🦮  🦮  —   —   —   —   guide dog
    0x1F429, // 327  🐩  🐩  🐩  —   —   🐩  poodle
    0x1F43A, // 328  🐺  🐺  🐺  🐺  —   —   wolf
    0x1F98A, // 329  🦊  🦊  —   —   —   —   fox
    0x1F99D, // 330  🦝  🦝  —   —   —   —   raccoon
    0x1F431, // 331  🐱  🐱  🐱  🐱  🐱  🐱  cat face
    0x1F408, // 332  🐈  🐈  —   —   —   —   cat
    0x1F981, // 333  🦁  🦁  —   —   —   —   lion
    0x1F42F, // 334  🐯  🐯  🐯  🐯  —   🐯  tiger face
    0x1F405, // 335  🐅  🐅  —   —   —   —   tiger
    0x1F406, // 336  🐆  🐆  —   —   —   —   leopard
    0x1F434, // 337  🐴  🐴  🐴  🐴  🐴  🐴  horse face
    0x1FACE, // 338  🫎  🫎  —   —   —   —   moose
    0x1FACF, // 339  🫏  🫏  —   —   —   —   donkey
    0x1F40E, // 340  🐎  🐎  🐎  🐎  —   —   horse
    0x1F984, // 341  🦄  🦄  —   —   —   —   unicorn
    0x1F993, // 342  🦓  🦓  —   —   —   —   zebra
    0x1F98C, // 343  🦌  🦌  —   —   —   —   deer
    0x1F9AC, // 344  🦬  🦬  —   —   —   —   bison
    0x1F42E, // 345  🐮  🐮  🐮  🐮  —   🐮  cow face
    0x1F402, // 346  🐂  🐂  —   —   —   —   ox
    0x1F403, // 347  🐃  🐃  —   —   —   —   water buffalo
    0x1F404, // 348  🐄  🐄  —   —   —   —   cow
    0x1F437, // 349  🐷  🐷  🐷  🐷  🐷  🐷  pig face
    0x1F416, // 350  🐖  🐖  —   —   —   —   pig
    0x1F417, // 351  🐗  🐗  🐗  🐗  —   🐗  boar
    0x1F43D, // 352  🐽  🐽  🐽  —   —   🐽  pig nose
    0x1F40F, // 353  🐏  🐏  —   —   —   —   ram
    0x1F411, // 354  🐑  🐑  🐑  🐑  —   —   ewe
    0x1F410, // 355  🐐  🐐  —   —   —   —   goat
    0x1F42A, // 356  🐪  🐪  —   —   —   —   camel
    0x1F42B, // 357  🐫  🐫  🐫  🐫  —   🐫  two-hump camel
    0x1F999, // 358  🦙  🦙  —   —   —   —   llama
    0x1F992, // 359  🦒  🦒  —   —   —   —   giraffe
    0x1F418, // 360  🐘  🐘  🐘  🐘  —   🐘  elephant
    0x1F9A3, // 361  🦣  🦣  —   —   —   —   mammoth
    0x1F98F, // 362  🦏  🦏  —   —   —   —   rhinoceros
    0x1F99B, // 363  🦛  🦛  —   —   —   —   hippopotamus
    0x1F42D, // 364  🐭  🐭  🐭  🐭  —   🐭  mouse face
    0x1F401, // 365  🐁  🐁  —   —   —   —   mouse
    0x1F400, // 366  🐀  🐀  —   —   —   —   rat
    0x1F439, // 367  🐹  🐹  🐹  🐹  —   —   hamster
    0x1F430, // 368  🐰  🐰  🐰  🐰  —   🐰  rabbit face
    0x1F407, // 369  🐇  🐇  —   —   —   —   rabbit
    0x1F43F, // 370  🐿  🐿  —   —   —   —   chipmunk
    0x1F9AB, // 371  🦫  🦫  —   —   —   —   beaver
    0x1F994, // 372  🦔  🦔  —   —   —   —   hedgehog
    0x1F987, // 373  🦇  🦇  —   —   —   —   bat
    0x1F43B, // 374  🐻  🐻  🐻  🐻  —   🐻  bear
    0x1F428, // 375  🐨  🐨  🐨  🐨  —   🐨  koala
    0x1F43C, // 376  🐼  🐼  🐼  —   —   🐼  panda
    0x1F9A5, // 377  🦥  🦥  —   —   —   —   sloth
    0x1F9A6, // 378  🦦  🦦  —   —   —   —   otter
    0x1F9A8, // 379  🦨  🦨  —   —   —   —   skunk
    0x1F998, // 380  🦘  🦘  —   —   —   —   kangaroo
    0x1F9A1, // 381  🦡  🦡  —   —   —   —   badger
    0x1F43E, // 382  🐾  🐾  🐾  —   —   🐾  paw prints
    0x1F983, // 383  🦃  🦃  —   —   —   —   turkey
    0x1F414, // 384  🐔  🐔  🐔  🐔  —   🐔  chicken
    0x1F413, // 385  🐓  🐓  —   —   —   —   rooster
    0x1F423, // 386  🐣  🐣  🐣  —   —   🐣  hatching chick
    0x1F424, // 387  🐤  🐤  🐤  🐤  🐤  🐤  baby chick
    0x1F425, // 388  🐥  🐥  🐥  —   —   🐥  front-facing baby chick
    0x1F426, // 389  🐦  🐦  🐦  🐦  —   —   bird
    0x1F427, // 390  🐧  🐧  🐧  🐧  🐧  🐧  penguin
    0x1F54A, // 391  🕊  🕊  —   —   —   —   dove
    0x1F985, // 392  🦅  🦅  —   —   —   —   eagle
    0x1F986, // 393  🦆  🦆  —   —   —   —   duck
    0x1F9A2, // 394  🦢  🦢  —   —   —   —   swan
    0x1F989, // 395  🦉  🦉  —   —   —   —   owl
    0x1F9A4, // 396  🦤  🦤  —   —   —   —   dodo
    0x1FAB6, // 397  🪶  🪶  —   —   —   —   feather
    0x1F9A9, // 398  🦩  🦩  —   —   —   —   flamingo
    0x1F99A, // 399  🦚  🦚  —   —   —   —   peacock
    0x1F99C, // 400  🦜  🦜  —   —   —   —   parrot
    0x1FABD, // 401  🪽  🪽  —   —   —   —   wing
    0x1FABF, // 402  🪿  🪿  —   —   —   —   goose
    0x1F438, // 403  🐸  🐸  🐸  🐸  —   🐸  frog
    0x1F40A, // 404  🐊  🐊  —   —   —   —   crocodile
    0x1F422, // 405  🐢  🐢  🐢  —   —   🐢  turtle
    0x1F98E, // 406  🦎  🦎  —   —   —   —   lizard
    0x1F40D, // 407  🐍  🐍  🐍  🐍  —   🐍  snake
    0x1F432, // 408  🐲  🐲  🐲  —   —   🐲  dragon face
    0x1F409, // 409  🐉  🐉  —   —   —   —   dragon
    0x1F995, // 410  🦕  🦕  —   —   —   —   sauropod
    0x1F996, // 411  🦖  🦖  —   —   —   —   T-Rex
    0x1F433, // 412  🐳  🐳  🐳  🐳  —   🐳  spouting whale
    0x1F40B, // 413  🐋  🐋  —   —   —   —   whale
    0x1F42C, // 414  🐬  🐬  🐬  🐬  —   🐬  dolphin
    0x1F9AD, // 415  🦭  🦭  —   —   —   —   seal
    0x1F41F, // 416  🐟  🐟  🐟  🐟  🐟  —   fish
    0x1F420, // 417  🐠  🐠  🐠  🐠  —   🐠  tropical fish
    0x1F421, // 418  🐡  🐡  🐡  —   —   🐡  blowfish
    0x1F988, // 419  🦈  🦈  —   —   —   —   shark
    0x1F419, // 420  🐙  🐙  🐙  🐙  —   🐙  octopus
    0x1F41A, // 421  🐚  🐚  🐚  🐚  —   🐚  spiral shell
    0x1FAB8, // 422  🪸  🪸  —   —   —   —   coral
    0x1FABC, // 423  🪼  🪼  —   —   —   —   jellyfish
    0x1F40C, // 424  🐌  🐌  🐌  —   🐌  🐌  snail
    0x1F98B, // 425  🦋  🦋  —   —   —   —   butterfly
    0x1F41B, // 426  🐛  🐛  🐛  🐛  —   🐛  bug
    0x1F41C, // 427  🐜  🐜  🐜  —   —   🐜  ant
    0x1F41D, // 428  🐝  🐝  🐝  —   —   🐝  honeybee
    0x1FAB2, // 429  🪲  🪲  —   —   —   —   beetle
    0x1F41E, // 430  🐞  🐞  🐞  —   —   🐞  lady beetle
    0x1F997, // 431  🦗  🦗  —   —   —   —   cricket
    0x1FAB3, // 432  🪳  🪳  —   —   —   —   cockroach
    0x1F577, // 433  🕷  🕷  —   —   —   —   spider
    0x1F578, // 434  🕸  🕸  —   —   —   —   spider web
    0x1F982, // 435  🦂  🦂  —   —   —   —   scorpion
    0x1F99F, // 436  🦟  🦟  —   —   —   —   mosquito
    0x1FAB0, // 437  🪰  🪰  —   —   —   —   fly
    0x1FAB1, // 438  🪱  🪱  —   —   —   —   worm
    0x1F9A0, // 439  🦠  🦠  —   —   —   —   microbe
    0x1F490, // 440  💐  💐  💐  💐  —   💐  bouquet
    0x1F338, // 441  🌸  🌸  🌸  🌸  🌸  🌸  cherry blossom
    0x1F4AE, // 442  💮  💮  💮  —   —   💮  white flower
    0x1FAB7, // 443  🪷  🪷  —   —   —   —   lotus
    0x1F3F5, // 444  🏵  🏵  —   —   —   —   rosette
    0x1F339, // 445  🌹  🌹  🌹  🌹  —   🌹  rose
    0x1F940, // 446  🥀  🥀  —   —   —   —   wilted flower
    0x1F33A, // 447  🌺  🌺  🌺  🌺  —   🌺  hibiscus
    0x1F33B, // 448  🌻  🌻  🌻  🌻  —   🌻  sunflower
    0x1F33C, // 449  🌼  🌼  🌼  —   —   🌼  blossom
    0x1F337, // 450  🌷  🌷  🌷  🌷  🌷  🌷  tulip
    0x1FABB, // 451  🪻  🪻  —   —   —   —   hyacinth
    0x1F331, // 452  🌱  🌱  🌱  —   🌱  🌱  seedling
    0x1FAB4, // 453  🪴  🪴  —   —   —   —   potted plant
    0x1F332, // 454  🌲  🌲  —   —   —   —   evergreen tree
    0x1F333, // 455  🌳  🌳  —   —   —   —   deciduous tree
    0x1F334, // 456  🌴  🌴  🌴  🌴  —   🌴  palm tree
    0x1F335, // 457  🌵  🌵  🌵  🌵  —   🌵  cactus
    0x1F33E, // 458  🌾  🌾  🌾  🌾  —   —   sheaf of rice
    0x1F33F, // 459  🌿  🌿  🌿  —   —   🌿  herb
    0x2618,  // 460 ☘   ☘   —   —   —   —   shamrock
    0x1F340, // 461  🍀  🍀  🍀  🍀  🍀  🍀  four leaf clover
    0x1F341, // 462  🍁  🍁  🍁  🍁  🍁  🍁  maple leaf
    0x1F342, // 463  🍂  🍂  🍂  🍂  —   🍂  fallen leaf
    0x1F343, // 464  🍃  🍃  🍃  🍃  —   —   leaf fluttering in wind
    0x1FAB9, // 465  🪹  🪹  —   —   —   —   empty nest
    0x1FABA, // 466  🪺  🪺  —   —   —   —   nest with eggs
    0x1F344, // 467  🍄  🍄  🍄  —   —   🍄  mushroom
    0x1F347, // 468  🍇  🍇  🍇  —   —   🍇  grapes
    0x1F348, // 469  🍈  🍈  🍈  —   —   🍈  melon
    0x1F349, // 470  🍉  🍉  🍉  🍉  —   🍉  watermelon
    0x1F34A, // 471  🍊  🍊  🍊  🍊  —   🍊  tangerine
    0x1F34B, // 472  🍋  🍋  —   —   —   —   lemon
    0x1F34C, // 473  🍌  🍌  🍌  —   🍌  🍌  banana
    0x1F34D, // 474  🍍  🍍  🍍  —   —   🍍  pineapple
    0x1F96D, // 475  🥭  🥭  —   —   —   —   mango
    0x1F34E, // 476  🍎  🍎  🍎  🍎  🍎  🍎  red apple
    0x1F34F, // 477  🍏  🍏  🍏  —   —   🍏  green apple
    0x1F350, // 478  🍐  🍐  —   —   —   —   pear
    0x1F351, // 479  🍑  🍑  🍑  —   —   🍑  peach
    0x1F352, // 480  🍒  🍒  🍒  —   🍒  🍒  cherries
    0x1F353, // 481  🍓  🍓  🍓  🍓  —   🍓  strawberry
    0x1FAD0, // 482  🫐  🫐  —   —   —   —   blueberries
    0x1F95D, // 483  🥝  🥝  —   —   —   —   kiwi fruit
    0x1F345, // 484  🍅  🍅  🍅  🍅  —   🍅  tomato
    0x1FAD2, // 485  🫒  🫒  —   —   —   —   olive
    0x1F965, // 486  🥥  🥥  —   —   —   —   coconut
    0x1F951, // 487  🥑  🥑  —   —   —   —   avocado
    0x1F346, // 488  🍆  🍆  🍆  🍆  —   🍆  eggplant
    0x1F954, // 489  🥔  🥔  —   —   —   —   potato
    0x1F955, // 490  🥕  🥕  —   —   —   —   carrot
    0x1F33D, // 491  🌽  🌽  🌽  —   —   🌽  ear of corn
    0x1F336, // 492  🌶  🌶  —   —   —   —   hot pepper
    0x1FAD1, // 493  🫑  🫑  —   —   —   —   bell pepper
    0x1F952, // 494  🥒  🥒  —   —   —   —   cucumber
    0x1F96C, // 495  🥬  🥬  —   —   —   —   leafy green
    0x1F966, // 496  🥦  🥦  —   —   —   —   broccoli
    0x1F9C4, // 497  🧄  🧄  —   —   —   —   garlic
    0x1F9C5, // 498  🧅  🧅  —   —   —   —   onion
    0x1F95C, // 499  🥜  🥜  —   —   —   —   peanuts
    0x1FAD8, // 500  🫘  🫘  —   —   —   —   beans
    0x1F330, // 501  🌰  🌰  🌰  —   —   🌰  chestnut
    0x1FADA, // 502  🫚  🫚  —   —   —   —   ginger root
    0x1FADB, // 503  🫛  🫛  —   —   —   —   pea pod
    0x1F35E, // 504  🍞  🍞  🍞  🍞  🍞  🍞  bread
    0x1F950, // 505  🥐  🥐  —   —   —   —   croissant
    0x1F956, // 506  🥖  🥖  —   —   —   —   baguette bread
    0x1FAD3, // 507  🫓  🫓  —   —   —   —   flatbread
    0x1F968, // 508  🥨  🥨  —   —   —   —   pretzel
    0x1F96F, // 509  🥯  🥯  —   —   —   —   bagel
    0x1F95E, // 510  🥞  🥞  —   —   —   —   pancakes
    0x1F9C7, // 511  🧇  🧇  —   —   —   —   waffle
    0x1F9C0, // 512  🧀  🧀  —   —   —   —   cheese wedge
    0x1F356, // 513  🍖  🍖  🍖  —   —   🍖  meat on bone
    0x1F357, // 514  🍗  🍗  🍗  —   —   🍗  poultry leg
    0x1F969, // 515  🥩  🥩  —   —   —   —   cut of meat
    0x1F953, // 516  🥓  🥓  —   —   —   —   bacon
    0x1F354, // 517  🍔  🍔  🍔  🍔  🍔  🍔  hamburger
    0x1F35F, // 518  🍟  🍟  🍟  🍟  —   🍟  french fries
    0x1F355, // 519  🍕  🍕  🍕  —   —   🍕  pizza
    0x1F32D, // 520  🌭  🌭  —   —   —   —   hot dog
    0x1F96A, // 521  🥪  🥪  —   —   —   —   sandwich
    0x1F32E, // 522  🌮  🌮  —   —   —   —   taco
    0x1F32F, // 523  🌯  🌯  —   —   —   —   burrito
    0x1FAD4, // 524  🫔  🫔  —   —   —   —   tamale
    0x1F959, // 525  🥙  🥙  —   —   —   —   stuffed flatbread
    0x1F9C6, // 526  🧆  🧆  —   —   —   —   falafel
    0x1F95A, // 527  🥚  🥚  —   —   —   —   egg
    0x1F373, // 528  🍳  🍳  🍳  🍳  —   🍳  cooking
    0x1F958, // 529  🥘  🥘  —   —   —   —   shallow pan of food
    0x1F372, // 530  🍲  🍲  🍲  🍲  —   🍲  pot of food
    0x1FAD5, // 531  🫕  🫕  —   —   —   —   fondue
    0x1F963, // 532  🥣  🥣  —   —   —   —   bowl with spoon
    0x1F957, // 533  🥗  🥗  —   —   —   —   green salad
    0x1F37F, // 534  🍿  🍿  —   —   —   —   popcorn
    0x1F9C8, // 535  🧈  🧈  —   —   —   —   butter
    0x1F9C2, // 536  🧂  🧂  —   —   —   —   salt
    0x1F96B, // 537  🥫  🥫  —   —   —   —   canned food
    0x1F371, // 538  🍱  🍱  🍱  🍱  —   🍱  bento box
    0x1F358, // 539  🍘  🍘  🍘  🍘  —   🍘  rice cracker
    0x1F359, // 540  🍙  🍙  🍙  🍙  🍙  🍙  rice ball
    0x1F35A, // 541  🍚  🍚  🍚  🍚  —   🍚  cooked rice
    0x1F35B, // 542  🍛  🍛  🍛  🍛  —   🍛  curry rice
    0x1F35C, // 543  🍜  🍜  🍜  🍜  🍜  🍜  steaming bowl
    0x1F35D, // 544  🍝  🍝  🍝  🍝  —   🍝  spaghetti
    0x1F360, // 545  🍠  🍠  🍠  —   —   🍠  roasted sweet potato
    0x1F362, // 546  🍢  🍢  🍢  🍢  —   🍢  oden
    0x1F363, // 547  🍣  🍣  🍣  🍣  —   🍣  sushi
    0x1F364, // 548  🍤  🍤  🍤  —   —   🍤  fried shrimp
    0x1F365, // 549  🍥  🍥  🍥  —   —   🍥  fish cake with swirl
    0x1F96E, // 550  🥮  🥮  —   —   —   —   moon cake
    0x1F361, // 551  🍡  🍡  🍡  🍡  —   🍡  dango
    0x1F95F, // 552  🥟  🥟  —   —   —   —   dumpling
    0x1F960, // 553  🥠  🥠  —   —   —   —   fortune cookie
    0x1F961, // 554  🥡  🥡  —   —   —   —   takeout box
    0x1F980, // 555  🦀  🦀  —   —   —   —   crab
    0x1F99E, // 556  🦞  🦞  —   —   —   —   lobster
    0x1F990, // 557  🦐  🦐  —   —   —   —   shrimp
    0x1F991, // 558  🦑  🦑  —   —   —   —   squid
    0x1F9AA, // 559  🦪  🦪  —   —   —   —   oyster
    0x1F366, // 560  🍦  🍦  🍦  🍦  —   🍦  soft ice cream
    0x1F367, // 561  🍧  🍧  🍧  🍧  —   🍧  shaved ice
    0x1F368, // 562  🍨  🍨  🍨  —   —   🍨  ice cream
    0x1F369, // 563  🍩  🍩  🍩  —   —   🍩  doughnut
    0x1F36A, // 564  🍪  🍪  🍪  —   —   🍪  cookie
    0x1F382, // 565  🎂  🎂  🎂  🎂  🎂  🎂  birthday cake
    0x1F370, // 566  🍰  🍰  🍰  🍰  🍰  🍰  shortcake
    0x1F9C1, // 567  🧁  🧁  —   —   —   —   cupcake
    0x1F967, // 568  🥧  🥧  —   —   —   —   pie
    0x1F36B, // 569  🍫  🍫  🍫  —   —   🍫  chocolate bar
    0x1F36C, // 570  🍬  🍬  🍬  —   —   🍬  candy
    0x1F36D, // 571  🍭  🍭  🍭  —   —   🍭  lollipop
    0x1F36E, // 572  🍮  🍮  🍮  —   —   🍮  custard
    0x1F36F, // 573  🍯  🍯  🍯  —   —   🍯  honey pot
    0x1F37C, // 574  🍼  🍼  —   —   —   —   baby bottle
    0x1F95B, // 575  🥛  🥛  —   —   —   —   glass of milk
    0x2615,  // 576 ☕  ☕  ☕  ☕  ☕  ☕  hot beverage
    0x1FAD6, // 577  🫖  🫖  —   —   —   —   teapot
    0x1F375, // 578  🍵  🍵  🍵  🍵  🍵  🍵  teacup without handle
    0x1F376, // 579  🍶  🍶  🍶  🍶  🍶  🍶  sake
    0x1F37E, // 580  🍾  🍾  —   —   —   —   bottle with popping cork
    0x1F377, // 581  🍷  🍷  🍷  —   🍷  🍷  wine glass
    0x1F378, // 582  🍸  🍸  🍸  🍸  🍸  🍸  cocktail glass
    0x1F379, // 583  🍹  🍹  🍹  —   —   🍹  tropical drink
    0x1F37A, // 584  🍺  🍺  🍺  🍺  🍺  🍺  beer mug
    0x1F37B, // 585  🍻  🍻  🍻  🍻  —   🍻  clinking beer mugs
    0x1F942, // 586  🥂  🥂  —   —   —   —   clinking glasses
    0x1F943, // 587  🥃  🥃  —   —   —   —   tumbler glass
    0x1FAD7, // 588  🫗  🫗  —   —   —   —   pouring liquid
    0x1F964, // 589  🥤  🥤  —   —   —   —   cup with straw
    0x1F9CB, // 590  🧋  🧋  —   —   —   —   bubble tea
    0x1F9C3, // 591  🧃  🧃  —   —   —   —   beverage box
    0x1F9C9, // 592  🧉  🧉  —   —   —   —   mate
    0x1F9CA, // 593  🧊  🧊  —   —   —   —   ice
    0x1F962, // 594  🥢  🥢  —   —   —   —   chopsticks
    0x1F37D, // 595  🍽  🍽  —   —   —   —   fork and knife with plate
    0x1F374, // 596  🍴  🍴  🍴  🍴  🍴  🍴  fork and knife
    0x1F944, // 597  🥄  🥄  —   —   —   —   spoon
    0x1F52A, // 598  🔪  🔪  🔪  —   —   🔪  kitchen knife
    0x1FAD9, // 599  🫙  🫙  —   —   —   —   jar
    0x1F3FA, // 600  🏺  🏺  —   —   —   —   amphora
    0x1F30D, // 601  🌍  🌍  —   —   —   —   globe showing Europe-Africa
    0x1F30E, // 602  🌎  🌎  —   —   —   —   globe showing Americas
    0x1F30F, // 603  🌏  🌏  🌏  —   —   🌏  globe showing Asia-Australia
    0x1F310, // 604  🌐  🌐  —   —   —   —   globe with meridians
    0x1F5FA, // 605  🗺  🗺  —   —   —   —   world map
    0x1F5FE, // 606  🗾  🗾  🗾  —   —   🗾  map of Japan
    0x1F9ED, // 607  🧭  🧭  —   —   —   —   compass
    0x1F3D4, // 608  🏔  🏔  —   —   —   —   snow-capped mountain
    0x26F0,  // 609 ⛰   ⛰   —   —   —   —   mountain
    0x1F30B, // 610  🌋  🌋  🌋  —   —   🌋  volcano
    0x1F5FB, // 611  🗻  🗻  🗻  🗻  🗻  🗻  mount fuji
    0x1F3D5, // 612  🏕  🏕  —   —   —   —   camping
    0x1F3D6, // 613  🏖  🏖  —   —   —   —   beach with umbrella
    0x1F3DC, // 614  🏜  🏜  —   —   —   —   desert
    0x1F3DD, // 615  🏝  🏝  —   —   —   —   desert island
    0x1F3DE, // 616  🏞  🏞  —   —   —   —   national park
    0x1F3DF, // 617  🏟  🏟  —   —   —   —   stadium
    0x1F3DB, // 618  🏛  🏛  —   —   —   —   classical building
    0x1F3D7, // 619  🏗  🏗  —   —   —   —   building construction
    0x1F9F1, // 620  🧱  🧱  —   —   —   —   brick
    0x1FAA8, // 621  🪨  🪨  —   —   —   —   rock
    0x1FAB5, // 622  🪵  🪵  —   —   —   —   wood
    0x1F6D6, // 623  🛖  🛖  —   —   —   —   hut
    0x1F3D8, // 624  🏘  🏘  —   —   —   —   houses
    0x1F3DA, // 625  🏚  🏚  —   —   —   —   derelict house
    0x1F3E0, // 626  🏠  🏠  🏠  🏠  🏠  🏠  house
    0x1F3E1, // 627  🏡  🏡  🏡  —   —   🏡  house with garden
    0x1F3E2, // 628  🏢  🏢  🏢  🏢  🏢  🏢  office building
    0x1F3E3, // 629  🏣  🏣  🏣  🏣  🏣  🏣  Japanese post office
    0x1F3E4, // 630  🏤  🏤  —   —   —   —   post office
    0x1F3E5, // 631  🏥  🏥  🏥  🏥  🏥  🏥  hospital
    0x1F3E6, // 632  🏦  🏦  🏦  🏦  🏦  🏦  bank
    0x1F3E8, // 633  🏨  🏨  🏨  🏨  🏨  🏨  hotel
    0x1F3E9, // 634  🏩  🏩  🏩  🏩  —   🏩  love hotel
    0x1F3EA, // 635  🏪  🏪  🏪  🏪  🏪  🏪  convenience store
    0x1F3EB, // 636  🏫  🏫  🏫  🏫  🏫  🏫  school
    0x1F3EC, // 637  🏬  🏬  🏬  🏬  —   🏬  department store
    0x1F3ED, // 638  🏭  🏭  🏭  🏭  —   🏭  factory
    0x1F3EF, // 639  🏯  🏯  🏯  🏯  —   🏯  Japanese castle
    0x1F3F0, // 640  🏰  🏰  🏰  🏰  —   🏰  castle
    0x1F492, // 641  💒  💒  💒  💒  —   —   wedding
    0x1F5FC, // 642  🗼  🗼  🗼  🗼  —   🗼  Tokyo tower
    0x1F5FD, // 643  🗽  🗽  🗽  🗽  —   —   Statue of Liberty
    0x26EA,  // 644 ⛪  ⛪  ⛪  ⛪  —   ⛪  church
    0x1F54C, // 645  🕌  🕌  —   —   —   —   mosque
    0x1F6D5, // 646  🛕  🛕  —   —   —   —   hindu temple
    0x1F54D, // 647  🕍  🕍  —   —   —   —   synagogue
    0x26E9,  // 648 ⛩   ⛩   —   —   —   —   shinto shrine
    0x1F54B, // 649  🕋  🕋  —   —   —   —   kaaba
    0x26F2,  // 650 ⛲  ⛲  ⛲  ⛲  —   ⛲  fountain
    0x26FA,  // 651 ⛺  ⛺  ⛺  ⛺  —   ⛺  tent
    0x1F301, // 652  🌁  🌁  🌁  —   🌁  🌁  foggy
    0x1F303, // 653  🌃  🌃  🌃  🌃  🌃  🌃  night with stars
    0x1F3D9, // 654  🏙  🏙  —   —   —   —   cityscape
    0x1F304, // 655  🌄  🌄  🌄  🌄  —   —   sunrise over mountains
    0x1F305, // 656  🌅  🌅  🌅  🌅  —   🌅  sunrise
    0x1F306, // 657  🌆  🌆  🌆  🌆  —   🌆  cityscape at dusk
    0x1F307, // 658  🌇  🌇  🌇  🌇  —   —   sunset
    0x1F309, // 659  🌉  🌉  🌉  —   —   🌉  bridge at night
    0x2668,  // 660 ♨   ♨   ♨   ♨   ♨   ♨   hot springs
    0x1F3A0, // 661  🎠  🎠  🎠  —   🎠  —   carousel horse
    0x1F6DD, // 662  🛝  🛝  —   —   —   —   playground slide
    0x1F3A1, // 663  🎡  🎡  🎡  🎡  —   🎡  ferris wheel
    0x1F3A2, // 664  🎢  🎢  🎢  🎢  —   🎢  roller coaster
    0x1F488, // 665  💈  💈  💈  💈  —   💈  barber pole
    0x1F3AA, // 666  🎪  🎪  🎪  —   🎪  🎪  circus tent
    0x1F682, // 667  🚂  🚂  —   —   —   —   locomotive
    0x1F683, // 668  🚃  🚃  🚃  🚃  🚃  🚃  railway car
    0x1F684, // 669  🚄  🚄  🚄  🚄  🚄  —   high-speed train
    0x1F685, // 670  🚅  🚅  🚅  🚅  —   🚅  bullet train
    0x1F686, // 671  🚆  🚆  —   —   —   —   train
    0x1F687, // 672  🚇  🚇  🚇  🚇  —   🚇  metro
    0x1F688, // 673  🚈  🚈  —   —   —   —   light rail
    0x1F689, // 674  🚉  🚉  🚉  🚉  —   🚉  station
    0x1F68A, // 675  🚊  🚊  —   —   —   —   tram
    0x1F69D, // 676  🚝  🚝  —   —   —   —   monorail
    0x1F69E, // 677  🚞  🚞  —   —   —   —   mountain railway
    0x1F68B, // 678  🚋  🚋  —   —   —   —   tram car
    0x1F68C, // 679  🚌  🚌  🚌  🚌  🚌  🚌  bus
    0x1F68D, // 680  🚍  🚍  —   —   —   —   oncoming bus
    0x1F68E, // 681  🚎  🚎  —   —   —   —   trolleybus
    0x1F690, // 682  🚐  🚐  —   —   —   —   minibus
    0x1F691, // 683  🚑  🚑  🚑  🚑  —   🚑  ambulance
    0x1F692, // 684  🚒  🚒  🚒  🚒  —   🚒  fire engine
    0x1F693, // 685  🚓  🚓  🚓  🚓  —   🚓  police car
    0x1F694, // 686  🚔  🚔  —   —   —   —   oncoming police car
    0x1F695, // 687  🚕  🚕  🚕  🚕  —   —   taxi
    0x1F696, // 688  🚖  🚖  —   —   —   —   oncoming taxi
    0x1F697, // 689  🚗  🚗  🚗  🚗  🚗  🚗  automobile
    0x1F698, // 690  🚘  🚘  —   —   —   —   oncoming automobile
    0x1F699, // 691  🚙  🚙  🚙  🚙  🚙  —   sport utility vehicle
    0x1F6FB, // 692  🛻  🛻  —   —   —   —   pickup truck
    0x1F69A, // 693  🚚  🚚  🚚  🚚  —   🚚  delivery truck
    0x1F69B, // 694  🚛  🚛  —   —   —   —   articulated lorry
    0x1F69C, // 695  🚜  🚜  —   —   —   —   tractor
    0x1F3CE, // 696  🏎  🏎  —   —   —   —   racing car
    0x1F3CD, // 697  🏍  🏍  —   —   —   —   motorcycle
    0x1F6F5, // 698  🛵  🛵  —   —   —   —   motor scooter
    0x1F9BD, // 699  🦽  🦽  —   —   —   —   manual wheelchair
    0x1F9BC, // 700  🦼  🦼  —   —   —   —   motorized wheelchair
    0x1F6FA, // 701  🛺  🛺  —   —   —   —   auto rickshaw
    0x1F6B2, // 702  🚲  🚲  🚲  🚲  🚲  🚲  bicycle
    0x1F6F4, // 703  🛴  🛴  —   —   —   —   kick scooter
    0x1F6F9, // 704  🛹  🛹  —   —   —   —   skateboard
    0x1F6FC, // 705  🛼  🛼  —   —   —   —   roller skate
    0x1F68F, // 706  🚏  🚏  🚏  🚏  —   🚏  bus stop
    0x1F6E3, // 707  🛣  🛣  —   —   —   —   motorway
    0x1F6E4, // 708  🛤  🛤  —   —   —   —   railway track
    0x1F6E2, // 709  🛢  🛢  —   —   —   —   oil drum
    0x26FD,  // 710 ⛽  ⛽  ⛽  ⛽  ⛽  ⛽  fuel pump
    0x1F6DE, // 711  🛞  🛞  —   —   —   —   wheel
    0x1F6A8, // 712  🚨  🚨  🚨  —   —   🚨  police car light
    0x1F6A5, // 713  🚥  🚥  🚥  🚥  🚥  🚥  horizontal traffic light
    0x1F6A6, // 714  🚦  🚦  —   —   —   —   vertical traffic light
    0x1F6D1, // 715  🛑  🛑  —   —   —   —   stop sign
    0x1F6A7, // 716  🚧  🚧  🚧  🚧  —   🚧  construction
    0x2693,  // 717 ⚓  ⚓  ⚓  —   —   ⚓  anchor
    0x1F6DF, // 718  🛟  🛟  —   —   —   —   ring buoy
    0x26F5,  // 719 ⛵  ⛵  ⛵  ⛵  ⛵  ⛵  sailboat
    0x1F6F6, // 720  🛶  🛶  —   —   —   —   canoe
    0x1F6A4, // 721  🚤  🚤  🚤  🚤  —   —   speedboat
    0x1F6F3, // 722  🛳  🛳  —   —   —   —   passenger ship
    0x26F4,  // 723 ⛴   ⛴   —   —   —   —   ferry
    0x1F6E5, // 724  🛥  🛥  —   —   —   —   motor boat
    0x1F6A2, // 725  🚢  🚢  🚢  🚢  🚢  🚢  ship
    0x2708,  // 726 ✈   ✈   ✈   ✈   ✈   ✈   airplane
    0x1F6E9, // 727  🛩  🛩  —   —   —   —   small airplane
    0x1F6EB, // 728  🛫  🛫  —   —   —   —   airplane departure
    0x1F6EC, // 729  🛬  🛬  —   —   —   —   airplane arrival
    0x1FA82, // 730  🪂  🪂  —   —   —   —   parachute
    0x1F4BA, // 731  💺  💺  💺  💺  💺  —   seat
    0x1F681, // 732  🚁  🚁  —   —   —   —   helicopter
    0x1F69F, // 733  🚟  🚟  —   —   —   —   suspension railway
    0x1F6A0, // 734  🚠  🚠  —   —   —   —   mountain cableway
    0x1F6A1, // 735  🚡  🚡  —   —   —   —   aerial tramway
    0x1F6F0, // 736  🛰  🛰  —   —   —   —   satellite
    0x1F680, // 737  🚀  🚀  🚀  🚀  —   🚀  rocket
    0x1F6F8, // 738  🛸  🛸  —   —   —   —   flying saucer
    0x1F6CE, // 739  🛎  🛎  —   —   —   —   bellhop bell
    0x1F9F3, // 740  🧳  🧳  —   —   —   —   luggage
    0x231B,  // 741 ⌛  ⌛  ⌛  —   —   ⌛  hourglass done
    0x23F3,  // 742 ⏳  ⏳  ⏳  —   ⏳  ⏳  hourglass not done
    0x231A,  // 743 ⌚  ⌚  ⌚  —   ⌚  ⌚  watch
    0x23F0,  // 744 ⏰  ⏰  ⏰  —   ⏰  ⏰  alarm clock
    0x23F1,  // 745 ⏱   ⏱   —   —   —   —   stopwatch
    0x23F2,  // 746 ⏲   ⏲   —   —   —   —   timer clock
    0x1F570, // 747  🕰  🕰  —   —   —   —   mantelpiece clock
    0x1F55B, // 748  🕛  🕛  🕛  🕛  —   —   twelve o’clock
    0x1F567, // 749  🕧  🕧  —   —   —   —   twelve-thirty
    0x1F550, // 750  🕐  🕐  🕐  🕐  —   —   one o’clock
    0x1F55C, // 751  🕜  🕜  —   —   —   —   one-thirty
    0x1F551, // 752  🕑  🕑  🕑  🕑  —   —   two o’clock
    0x1F55D, // 753  🕝  🕝  —   —   —   —   two-thirty
    0x1F552, // 754  🕒  🕒  🕒  🕒  —   —   three o’clock
    0x1F55E, // 755  🕞  🕞  —   —   —   —   three-thirty
    0x1F553, // 756  🕓  🕓  🕓  🕓  —   —   four o’clock
    0x1F55F, // 757  🕟  🕟  —   —   —   —   four-thirty
    0x1F554, // 758  🕔  🕔  🕔  🕔  —   —   five o’clock
    0x1F560, // 759  🕠  🕠  —   —   —   —   five-thirty
    0x1F555, // 760  🕕  🕕  🕕  🕕  —   —   six o’clock
    0x1F561, // 761  🕡  🕡  —   —   —   —   six-thirty
    0x1F556, // 762  🕖  🕖  🕖  🕖  —   —   seven o’clock
    0x1F562, // 763  🕢  🕢  —   —   —   —   seven-thirty
    0x1F557, // 764  🕗  🕗  🕗  🕗  —   —   eight o’clock
    0x1F563, // 765  🕣  🕣  —   —   —   —   eight-thirty
    0x1F558, // 766  🕘  🕘  🕘  🕘  —   —   nine o’clock
    0x1F564, // 767  🕤  🕤  —   —   —   —   nine-thirty
    0x1F559, // 768  🕙  🕙  🕙  🕙  —   —   ten o’clock
    0x1F565, // 769  🕥  🕥  —   —   —   —   ten-thirty
    0x1F55A, // 770  🕚  🕚  🕚  🕚  —   —   eleven o’clock
    0x1F566, // 771  🕦  🕦  —   —   —   —   eleven-thirty
    0x1F311, // 772  🌑  🌑  🌑  —   🌑  🌑  new moon
    0x1F312, // 773  🌒  🌒  —   —   —   —   waxing crescent moon
    0x1F313, // 774  🌓  🌓  🌓  —   🌓  🌓  first quarter moon
    0x1F314, // 775  🌔  🌔  🌔  —   🌔  🌔  waxing gibbous moon
    0x1F315, // 776  🌕  🌕  🌕  —   🌕  —   full moon
    0x1F316, // 777  🌖  🌖  —   —   —   —   waning gibbous moon
    0x1F317, // 778  🌗  🌗  —   —   —   —   last quarter moon
    0x1F318, // 779  🌘  🌘  —   —   —   —   waning crescent moon
    0x1F319, // 780  🌙  🌙  🌙  🌙  🌙  🌙  crescent moon
    0x1F31A, // 781  🌚  🌚  —   —   —   —   new moon face
    0x1F31B, // 782  🌛  🌛  🌛  —   —   🌛  first quarter moon face
    0x1F31C, // 783  🌜  🌜  —   —   —   —   last quarter moon face
    0x1F321, // 784  🌡  🌡  —   —   —   —   thermometer
    0x2600,  // 785 ☀   ☀   ☀   ☀   ☀   ☀   sun
    0x1F31D, // 786  🌝  🌝  —   —   —   —   full moon face
    0x1F31E, // 787  🌞  🌞  —   —   —   —   sun with face
    0x1FA90, // 788  🪐  🪐  —   —   —   —   ringed planet
    0x2B50,  // 789 ⭐  ⭐  —   ⭐  —   ⭐  star
    0x1F31F, // 790  🌟  🌟  🌟  🌟  —   —   glowing star
    0x1F320, // 791  🌠  🌠  🌠  —   —   🌠  shooting star
    0x1F30C, // 792  🌌  🌌  🌌  —   —   🌌  milky way
    0x2601,  // 793 ☁   ☁   ☁   ☁   ☁   ☁   cloud
    0x26C5,  // 794 ⛅  ⛅  ⛅  —   —   ⛅  sun behind cloud
    0x26C8,  // 795 ⛈   ⛈   —   —   —   —   cloud with lightning and rain
    0x1F324, // 796  🌤  🌤  —   —   —   —   sun behind small cloud
    0x1F325, // 797  🌥  🌥  —   —   —   —   sun behind large cloud
    0x1F326, // 798  🌦  🌦  —   —   —   —   sun behind rain cloud
    0x1F327, // 799  🌧  🌧  —   —   —   —   cloud with rain
    0x1F328, // 800  🌨  🌨  —   —   —   —   cloud with snow
    0x1F329, // 801  🌩  🌩  —   —   —   —   cloud with lightning
    0x1F32A, // 802  🌪  🌪  —   —   —   —   tornado
    0x1F32B, // 803  🌫  🌫  —   —   —   —   fog
    0x1F32C, // 804  🌬  🌬  —   —   —   —   wind face
    0x1F300, // 805  🌀  🌀  🌀  🌀  🌀  🌀  cyclone
    0x1F308, // 806  🌈  🌈  🌈  🌈  —   🌈  rainbow
    0x1F302, // 807  🌂  🌂  🌂  🌂  🌂  🌂  closed umbrella
    0x2602,  // 808 ☂   ☂   —   —   —   —   umbrella
    0x2614,  // 809 ☔  ☔  ☔  ☔  ☔  ☔  umbrella with rain drops
    0x26F1,  // 810 ⛱   ⛱   —   —   —   —   umbrella on ground
    0x26A1,  // 811 ⚡  ⚡  ⚡  ⚡  ⚡  ⚡  high voltage
    0x2744,  // 812 ❄   ❄   ❄   —   —   ❄   snowflake
    0x2603,  // 813 ☃   ☃   —   —   —   —   snowman
    0x26C4,  // 814 ⛄  ⛄  ⛄  ⛄  ⛄  ⛄  snowman without snow
    0x2604,  // 815 ☄   ☄   —   —   —   —   comet
    0x1F525, // 816  🔥  🔥  🔥  🔥  —   🔥  fire
    0x1F4A7, // 817  💧  💧  💧  —   💧  💧  droplet
    0x1F30A, // 818  🌊  🌊  🌊  🌊  🌊  🌊  water wave
    0x1F383, // 819  🎃  🎃  🎃  🎃  —   🎃  jack-o-lantern
    0x1F384, // 820  🎄  🎄  🎄  🎄  🎄  🎄  Christmas tree
    0x1F386, // 821  🎆  🎆  🎆  🎆  —   🎆  fireworks
    0x1F387, // 822  🎇  🎇  🎇  🎇  —   🎇  sparkler
    0x1F9E8, // 823  🧨  🧨  —   —   —   —   firecracker
    0x2728,  // 824 ✨  ✨  ✨  ✨  ✨  ✨  sparkles
    0x1F388, // 825  🎈  🎈  🎈  🎈  —   🎈  balloon
    0x1F389, // 826  🎉  🎉  🎉  🎉  —   🎉  party popper
    0x1F38A, // 827  🎊  🎊  🎊  —   —   🎊  confetti ball
    0x1F38B, // 828  🎋  🎋  🎋  —   —   🎋  tanabata tree
    0x1F38D, // 829  🎍  🎍  🎍  🎍  —   🎍  pine decoration
    0x1F38E, // 830  🎎  🎎  🎎  🎎  —   🎎  Japanese dolls
    0x1F38F, // 831  🎏  🎏  🎏  🎏  —   🎏  carp streamer
    0x1F390, // 832  🎐  🎐  🎐  🎐  —   🎐  wind chime
    0x1F391, // 833  🎑  🎑  🎑  🎑  —   🎑  moon viewing ceremony
    0x1F9E7, // 834  🧧  🧧  —   —   —   —   red envelope
    0x1F380, // 835  🎀  🎀  🎀  🎀  🎀  🎀  ribbon
    0x1F381, // 836  🎁  🎁  🎁  🎁  🎁  🎁  wrapped gift
    0x1F397, // 837  🎗  🎗  —   —   —   —   reminder ribbon
    0x1F39F, // 838  🎟  🎟  —   —   —   —   admission tickets
    0x1F3AB, // 839  🎫  🎫  🎫  🎫  🎫  🎫  ticket
    0x1F396, // 840  🎖  🎖  —   —   —   —   military medal
    0x1F3C6, // 841  🏆  🏆  🏆  🏆  —   🏆  trophy
    0x1F3C5, // 842  🏅  🏅  —   —   —   —   sports medal
    0x1F947, // 843  🥇  🥇  —   —   —   —   1st place medal
    0x1F948, // 844  🥈  🥈  —   —   —   —   2nd place medal
    0x1F949, // 845  🥉  🥉  —   —   —   —   3rd place medal
    0x26BD,  // 846 ⚽  ⚽  ⚽  ⚽  ⚽  ⚽  soccer ball
    0x26BE,  // 847 ⚾  ⚾  ⚾  ⚾  ⚾  ⚾  baseball
    0x1F94E, // 848  🥎  🥎  —   —   —   —   softball
    0x1F3C0, // 849  🏀  🏀  🏀  🏀  🏀  🏀  basketball
    0x1F3D0, // 850  🏐  🏐  —   —   —   —   volleyball
    0x1F3C8, // 851  🏈  🏈  🏈  🏈  —   🏈  american football
    0x1F3C9, // 852  🏉  🏉  —   —   —   —   rugby football
    0x1F3BE, // 853  🎾  🎾  🎾  🎾  🎾  🎾  tennis
    0x1F94F, // 854  🥏  🥏  —   —   —   —   flying disc
    0x1F3B3, // 855  🎳  🎳  🎳  —   —   🎳  bowling
    0x1F3CF, // 856  🏏  🏏  —   —   —   —   cricket game
    0x1F3D1, // 857  🏑  🏑  —   —   —   —   field hockey
    0x1F3D2, // 858  🏒  🏒  —   —   —   —   ice hockey
    0x1F94D, // 859  🥍  🥍  —   —   —   —   lacrosse
    0x1F3D3, // 860  🏓  🏓  —   —   —   —   ping pong
    0x1F3F8, // 861  🏸  🏸  —   —   —   —   badminton
    0x1F94A, // 862  🥊  🥊  —   —   —   —   boxing glove
    0x1F94B, // 863  🥋  🥋  —   —   —   —   martial arts uniform
    0x1F945, // 864  🥅  🥅  —   —   —   —   goal net
    0x26F3,  // 865 ⛳  ⛳  ⛳  ⛳  ⛳  ⛳  flag in hole
    0x26F8,  // 866 ⛸   ⛸   —   —   —   —   ice skate
    0x1F3A3, // 867  🎣  🎣  🎣  —   —   🎣  fishing pole
    0x1F93F, // 868  🤿  🤿  —   —   —   —   diving mask
    0x1F3BD, // 869  🎽  🎽  🎽  —   🎽  —   running shirt
    0x1F3BF, // 870  🎿  🎿  🎿  🎿  🎿  🎿  skis
    0x1F6F7, // 871  🛷  🛷  —   —   —   —   sled
    0x1F94C, // 872  🥌  🥌  —   —   —   —   curling stone
    0x1F3AF, // 873  🎯  🎯  🎯  🎯  —   🎯  bullseye
    0x1FA80, // 874  🪀  🪀  —   —   —   —   yo-yo
    0x1FA81, // 875  🪁  🪁  —   —   —   —   kite
    0x1F52B, // 876  🔫  🔫  🔫  🔫  —   🔫  water pistol
    0x1F3B1, // 877  🎱  🎱  🎱  🎱  —   🎱  pool 8 ball
    0x1F52E, // 878  🔮  🔮  🔮  —   —   🔮  crystal ball
    0x1FA84, // 879  🪄  🪄  —   —   —   —   magic wand
    0x1F3AE, // 880  🎮  🎮  🎮  —   🎮  🎮  video game
    0x1F579, // 881  🕹  🕹  —   —   —   —   joystick
    0x1F3B0, // 882  🎰  🎰  🎰  🎰  —   🎰  slot machine
    0x1F3B2, // 883  🎲  🎲  🎲  —   —   🎲  game die
    0x1F9E9, // 884  🧩  🧩  —   —   —   —   puzzle piece
    0x1F9F8, // 885  🧸  🧸  —   —   —   —   teddy bear
    0x1FA85, // 886  🪅  🪅  —   —   —   —   piñata
    0x1FAA9, // 887  🪩  🪩  —   —   —   —   mirror ball
    0x1FA86, // 888  🪆  🪆  —   —   —   —   nesting dolls
    0x2660,  // 889 ♠   ♠   ♠   ♠   ♠   ♠   spade suit
    0x2665,  // 890 ♥   ♥   ♥   ♥   ♥   ♥   heart suit
    0x2666,  // 891 ♦   ♦   ♦   ♦   ♦   ♦   diamond suit
    0x2663,  // 892 ♣   ♣   ♣   ♣   ♣   ♣   club suit
    0x265F,  // 893 ♟   ♟   —   —   —   —   chess pawn
    0x1F0CF, // 894  🃏  🃏  🃏  —   —   🃏  joker
    0x1F004, // 895  🀄  🀄  🀄  🀄  —   🀄  mahjong red dragon
    0x1F3B4, // 896  🎴  🎴  🎴  —   —   🎴  flower playing cards
    0x1F3AD, // 897  🎭  🎭  🎭  —   —   🎭  performing arts
    0x1F5BC, // 898  🖼  🖼  —   —   —   —   framed picture
    0x1F3A8, // 899  🎨  🎨  🎨  🎨  🎨  🎨  artist palette
    0x1F9F5, // 900  🧵  🧵  —   —   —   —   thread
    0x1FAA1, // 901  🪡  🪡  —   —   —   —   sewing needle
    0x1F9F6, // 902  🧶  🧶  —   —   —   —   yarn
    0x1FAA2, // 903  🪢  🪢  —   —   —   —   knot
    0x1F453, // 904  👓  👓  👓  —   👓  👓  glasses
    0x1F576, // 905  🕶  🕶  —   —   —   —   sunglasses
    0x1F97D, // 906  🥽  🥽  —   —   —   —   goggles
    0x1F97C, // 907  🥼  🥼  —   —   —   —   lab coat
    0x1F9BA, // 908  🦺  🦺  —   —   —   —   safety vest
    0x1F454, // 909  👔  👔  👔  👔  —   👔  necktie
    0x1F455, // 910  👕  👕  👕  👕  👕  👕  t-shirt
    0x1F456, // 911  👖  👖  👖  —   👖  👖  jeans
    0x1F9E3, // 912  🧣  🧣  —   —   —   —   scarf
    0x1F9E4, // 913  🧤  🧤  —   —   —   —   gloves
    0x1F9E5, // 914  🧥  🧥  —   —   —   —   coat
    0x1F9E6, // 915  🧦  🧦  —   —   —   —   socks
    0x1F457, // 916  👗  👗  👗  👗  —   👗  dress
    0x1F458, // 917  👘  👘  👘  👘  —   👘  kimono
    0x1F97B, // 918  🥻  🥻  —   —   —   —   sari
    0x1FA71, // 919  🩱  🩱  —   —   —   —   one-piece swimsuit
    0x1FA72, // 920  🩲  🩲  —   —   —   —   briefs
    0x1FA73, // 921  🩳  🩳  —   —   —   —   shorts
    0x1F459, // 922  👙  👙  👙  👙  —   👙  bikini
    0x1F45A, // 923  👚  👚  👚  —   —   👚  woman’s clothes
    0x1FAAD, // 924  🪭  🪭  —   —   —   —   folding hand fan
    0x1F45B, // 925  👛  👛  👛  —   👛  👛  purse
    0x1F45C, // 926  👜  👜  👜  👜  👜  👜  handbag
    0x1F45D, // 927  👝  👝  👝  —   👝  —   clutch bag
    0x1F6CD, // 928  🛍  🛍  —   —   —   —   shopping bags
    0x1F392, // 929  🎒  🎒  🎒  🎒  —   🎒  backpack
    0x1FA74, // 930  🩴  🩴  —   —   —   —   thong sandal
    0x1F45E, // 931  👞  👞  👞  —   —   👞  man’s shoe
    0x1F45F, // 932  👟  👟  👟  👟  👟  👟  running shoe
    0x1F97E, // 933  🥾  🥾  —   —   —   —   hiking boot
    0x1F97F, // 934  🥿  🥿  —   —   —   —   flat shoe
    0x1F460, // 935  👠  👠  👠  👠  👠  👠  high-heeled shoe
    0x1F461, // 936  👡  👡  👡  👡  —   —   woman’s sandal
    0x1FA70, // 937  🩰  🩰  —   —   —   —   ballet shoes
    0x1F462, // 938  👢  👢  👢  👢  —   👢  woman’s boot
    0x1FAAE, // 939  🪮  🪮  —   —   —   —   hair pick
    0x1F451, // 940  👑  👑  👑  👑  👑  👑  crown
    0x1F452, // 941  👒  👒  👒  👒  —   👒  woman’s hat
    0x1F3A9, // 942  🎩  🎩  🎩  🎩  🎩  🎩  top hat
    0x1F393, // 943  🎓  🎓  🎓  🎓  —   🎓  graduation cap
    0x1F9E2, // 944  🧢  🧢  —   —   —   —   billed cap
    0x1FA96, // 945  🪖  🪖  —   —   —   —   military helmet
    0x26D1,  // 946 ⛑   ⛑   —   —   —   —   rescue worker’s helmet
    0x1F4FF, // 947  📿  📿  —   —   —   —   prayer beads
    0x1F484, // 948  💄  💄  💄  💄  💄  💄  lipstick
    0x1F48D, // 949  💍  💍  💍  💍  💍  💍  ring
    0x1F48E, // 950  💎  💎  💎  💎  —   —   gem stone
    0x1F507, // 951  🔇  🔇  —   —   —   —   muted speaker
    0x1F508, // 952  🔈  🔈  —   —   —   —   speaker low volume
    0x1F509, // 953  🔉  🔉  —   —   —   —   speaker medium volume
    0x1F50A, // 954  🔊  🔊  🔊  🔊  —   🔊  speaker high volume
    0x1F4E2, // 955  📢  📢  📢  📢  —   —   loudspeaker
    0x1F4E3, // 956  📣  📣  📣  📣  —   —   megaphone
    0x1F4EF, // 957  📯  📯  —   —   —   —   postal horn
    0x1F514, // 958  🔔  🔔  🔔  🔔  🔔  🔔  bell
    0x1F515, // 959  🔕  🔕  —   —   —   —   bell with slash
    0x1F3BC, // 960  🎼  🎼  🎼  —   —   🎼  musical score
    0x1F3B5, // 961  🎵  🎵  🎵  🎵  🎵  🎵  musical note
    0x1F3B6, // 962  🎶  🎶  🎶  🎶  🎶  🎶  musical notes
    0x1F399, // 963  🎙  🎙  —   —   —   —   studio microphone
    0x1F39A, // 964  🎚  🎚  —   —   —   —   level slider
    0x1F39B, // 965  🎛  🎛  —   —   —   —   control knobs
    0x1F3A4, // 966  🎤  🎤  🎤  🎤  🎤  🎤  microphone
    0x1F3A7, // 967  🎧  🎧  🎧  🎧  🎧  🎧  headphone
    0x1F4FB, // 968  📻  📻  📻  📻  —   📻  radio
    0x1F3B7, // 969  🎷  🎷  🎷  🎷  —   —   saxophone
    0x1FA97, // 970  🪗  🪗  —   —   —   —   accordion
    0x1F3B8, // 971  🎸  🎸  🎸  🎸  —   🎸  guitar
    0x1F3B9, // 972  🎹  🎹  🎹  —   —   🎹  musical keyboard
    0x1F3BA, // 973  🎺  🎺  🎺  🎺  —   🎺  trumpet
    0x1F3BB, // 974  🎻  🎻  🎻  —   —   🎻  violin
    0x1FA95, // 975  🪕  🪕  —   —   —   —   banjo
    0x1F941, // 976  🥁  🥁  —   —   —   —   drum
    0x1FA98, // 977  🪘  🪘  —   —   —   —   long drum
    0x1FA87, // 978  🪇  🪇  —   —   —   —   maracas
    0x1FA88, // 979  🪈  🪈  —   —   —   —   flute
    0x1F4F1, // 980  📱  📱  📱  📱  📱  📱  mobile phone
    0x1F4F2, // 981  📲  📲  📲  📲  📲  📲  mobile phone with arrow
    0x260E,  // 982 ☎   ☎   ☎   ☎   ☎   ☎   telephone
    0x1F4DE, // 983  📞  📞  📞  —   —   📞  telephone receiver
    0x1F4DF, // 984  📟  📟  📟  —   📟  📟  pager
    0x1F4E0, // 985  📠  📠  📠  📠  📠  📠  fax machine
    0x1F50B, // 986  🔋  🔋  🔋  —   —   🔋  battery
    0x1FAAB, // 987  🪫  🪫  —   —   —   —   low battery
    0x1F50C, // 988  🔌  🔌  🔌  —   —   🔌  electric plug
    0x1F4BB, // 989  💻  💻  💻  💻  💻  💻  laptop
    0x1F5A5, // 990  🖥  🖥  —   —   —   —   desktop computer
    0x1F5A8, // 991  🖨  🖨  —   —   —   —   printer
    0x2328,  // 992 ⌨   ⌨   —   —   —   —   keyboard
    0x1F5B1, // 993  🖱  🖱  —   —   —   —   computer mouse
    0x1F5B2, // 994  🖲  🖲  —   —   —   —   trackball
    0x1F4BD, // 995  💽  💽  💽  💽  —   💽  computer disk
    0x1F4BE, // 996  💾  💾  💾  —   —   💾  floppy disk
    0x1F4BF, // 997  💿  💿  💿  💿  💿  💿  optical disk
    0x1F4C0, // 998  📀  📀  📀  📀  —   —   dvd
    0x1F9EE, // 999  🧮  🧮  —   —   —   —   abacus
    0x1F3A5, // 1000  🎥  🎥  🎥  🎥  🎥  🎥  movie camera
    0x1F39E, // 1001  🎞  🎞  —   —   —   —   film frames
    0x1F4FD, // 1002  📽  📽  —   —   —   —   film projector
    0x1F3AC, // 1003  🎬  🎬  🎬  🎬  🎬  🎬  clapper board
    0x1F4FA, // 1004  📺  📺  📺  📺  📺  📺  television
    0x1F4F7, // 1005  📷  📷  📷  📷  📷  📷  camera
    0x1F4F8, // 1006  📸  📸  —   —   —   —   camera with flash
    0x1F4F9, // 1007  📹  📹  📹  —   —   📹  video camera
    0x1F4FC, // 1008  📼  📼  📼  📼  —   📼  videocassette
    0x1F50D, // 1009  🔍  🔍  🔍  🔍  🔍  🔍  magnifying glass tilted left
    0x1F50E, // 1010  🔎  🔎  🔎  —   —   🔎  magnifying glass tilted right
    0x1F56F, // 1011  🕯  🕯  —   —   —   —   candle
    0x1F4A1, // 1012  💡  💡  💡  💡  💡  💡  light bulb
    0x1F526, // 1013  🔦  🔦  🔦  —   —   🔦  flashlight
    0x1F3EE, // 1014  🏮  🏮  🏮  —   —   🏮  red paper lantern
    0x1FA94, // 1015  🪔  🪔  —   —   —   —   diya lamp
    0x1F4D4, // 1016  📔  📔  📔  —   —   📔  notebook with decorative cover
    0x1F4D5, // 1017  📕  📕  📕  —   —   📕  closed book
    0x1F4D6, // 1018  📖  📖  📖  📖  📖  📖  open book
    0x1F4D7, // 1019  📗  📗  📗  —   —   📗  green book
    0x1F4D8, // 1020  📘  📘  📘  —   —   📘  blue book
    0x1F4D9, // 1021  📙  📙  📙  —   —   📙  orange book
    0x1F4DA, // 1022  📚  📚  📚  —   —   📚  books
    0x1F4D3, // 1023  📓  📓  📓  —   —   📓  notebook
    0x1F4D2, // 1024  📒  📒  📒  —   —   📒  ledger
    0x1F4C3, // 1025  📃  📃  📃  —   —   📃  page with curl
    0x1F4DC, // 1026  📜  📜  📜  —   —   📜  scroll
    0x1F4C4, // 1027  📄  📄  📄  —   —   📄  page facing up
    0x1F4F0, // 1028  📰  📰  📰  —   —   📰  newspaper
    0x1F5DE, // 1029  🗞  🗞  —   —   —   —   rolled-up newspaper
    0x1F4D1, // 1030  📑  📑  📑  —   —   📑  bookmark tabs
    0x1F516, // 1031  🔖  🔖  🔖  —   —   🔖  bookmark
    0x1F3F7, // 1032  🏷  🏷  —   —   —   —   label
    0x1F4B0, // 1033  💰  💰  💰  💰  💰  💰  money bag
    0x1FA99, // 1034  🪙  🪙  —   —   —   —   coin
    0x1F4B4, // 1035  💴  💴  💴  —   💴  💴  yen banknote
    0x1F4B5, // 1036  💵  💵  💵  —   —   💵  dollar banknote
    0x1F4B6, // 1037  💶  💶  —   —   —   —   euro banknote
    0x1F4B7, // 1038  💷  💷  —   —   —   —   pound banknote
    0x1F4B8, // 1039  💸  💸  💸  —   —   💸  money with wings
    0x1F4B3, // 1040  💳  💳  💳  —   —   💳  credit card
    0x1F9FE, // 1041  🧾  🧾  —   —   —   —   receipt
    0x1F4B9, // 1042  💹  💹  💹  💹  —   💹  chart increasing with yen
    0x2709,  // 1043 ✉   ✉   ✉   —   ✉   ✉   envelope
    0x1F4E7, // 1044  📧  📧  📧  —   —   📧  e-mail
    0x1F4E8, // 1045  📨  📨  📨  —   —   📨  incoming envelope
    0x1F4E9, // 1046  📩  📩  📩  📩  📩  📩  envelope with arrow
    0x1F4E4, // 1047  📤  📤  📤  —   —   📤  outbox tray
    0x1F4E5, // 1048  📥  📥  📥  —   —   📥  inbox tray
    0x1F4E6, // 1049  📦  📦  📦  —   —   📦  package
    0x1F4EB, // 1050  📫  📫  📫  📫  —   📫  closed mailbox with raised flag
    0x1F4EA, // 1051  📪  📪  📪  —   —   📪  closed mailbox with lowered flag
    0x1F4EC, // 1052  📬  📬  —   —   —   —   open mailbox with raised flag
    0x1F4ED, // 1053  📭  📭  —   —   —   —   open mailbox with lowered flag
    0x1F4EE, // 1054  📮  📮  📮  📮  —   —   postbox
    0x1F5F3, // 1055  🗳  🗳  —   —   —   —   ballot box with ballot
    0x270F,  // 1056 ✏   ✏   ✏   —   ✏   ✏   pencil
    0x2712,  // 1057 ✒   ✒   ✒   —   ✒   ✒   black nib
    0x1F58B, // 1058  🖋  🖋  —   —   —   —   fountain pen
    0x1F58A, // 1059  🖊  🖊  —   —   —   —   pen
    0x1F58C, // 1060  🖌  🖌  —   —   —   —   paintbrush
    0x1F58D, // 1061  🖍  🖍  —   —   —   —   crayon
    0x1F4DD, // 1062  📝  📝  📝  📝  📝  📝  memo
    0x1F4BC, // 1063  💼  💼  💼  💼  —   💼  briefcase
    0x1F4C1, // 1064  📁  📁  📁  —   —   📁  file folder
    0x1F4C2, // 1065  📂  📂  📂  —   —   📂  open file folder
    0x1F5C2, // 1066  🗂  🗂  —   —   —   —   card index dividers
    0x1F4C5, // 1067  📅  📅  📅  —   —   📅  calendar
    0x1F4C6, // 1068  📆  📆  📆  —   —   📆  tear-off calendar
    0x1F5D2, // 1069  🗒  🗒  —   —   —   —   spiral notepad
    0x1F5D3, // 1070  🗓  🗓  —   —   —   —   spiral calendar
    0x1F4C7, // 1071  📇  📇  📇  —   —   📇  card index
    0x1F4C8, // 1072  📈  📈  📈  —   —   📈  chart increasing
    0x1F4C9, // 1073  📉  📉  📉  —   —   📉  chart decreasing
    0x1F4CA, // 1074  📊  📊  📊  —   —   📊  bar chart
    0x1F4CB, // 1075  📋  📋  📋  —   —   📋  clipboard
    0x1F4CC, // 1076  📌  📌  📌  —   —   📌  pushpin
    0x1F4CD, // 1077  📍  📍  📍  —   —   📍  round pushpin
    0x1F4CE, // 1078  📎  📎  📎  —   📎  📎  paperclip
    0x1F587, // 1079  🖇  🖇  —   —   —   —   linked paperclips
    0x1F4CF, // 1080  📏  📏  📏  —   —   📏  straight ruler
    0x1F4D0, // 1081  📐  📐  📐  —   —   📐  triangular ruler
    0x2702,  // 1082 ✂   ✂   ✂   ✂   ✂   ✂   scissors
    0x1F5C3, // 1083  🗃  🗃  —   —   —   —   card file box
    0x1F5C4, // 1084  🗄  🗄  —   —   —   —   file cabinet
    0x1F5D1, // 1085  🗑  🗑  —   —   —   —   wastebasket
    0x1F512, // 1086  🔒  🔒  🔒  🔒  —   🔒  locked
    0x1F513, // 1087  🔓  🔓  🔓  🔓  —   —   unlocked
    0x1F50F, // 1088  🔏  🔏  🔏  —   —   🔏  locked with pen
    0x1F510, // 1089  🔐  🔐  🔐  —   —   🔐  locked with key
    0x1F511, // 1090  🔑  🔑  🔑  🔑  🔑  🔑  key
    0x1F5DD, // 1091  🗝  🗝  —   —   —   —   old key
    0x1F528, // 1092  🔨  🔨  🔨  🔨  —   🔨  hammer
    0x1FA93, // 1093  🪓  🪓  —   —   —   —   axe
    0x26CF,  // 1094 ⛏   ⛏   —   —   —   —   pick
    0x2692,  // 1095 ⚒   ⚒   —   —   —   —   hammer and pick
    0x1F6E0, // 1096  🛠  🛠  —   —   —   —   hammer and wrench
    0x1F5E1, // 1097  🗡  🗡  —   —   —   —   dagger
    0x2694,  // 1098 ⚔   ⚔   —   —   —   —   crossed swords
    0x1F4A3, // 1099  💣  💣  💣  💣  💣  💣  bomb
    0x1FA83, // 1100  🪃  🪃  —   —   —   —   boomerang
    0x1F3F9, // 1101  🏹  🏹  —   —   —   —   bow and arrow
    0x1F6E1, // 1102  🛡  🛡  —   —   —   —   shield
    0x1FA9A, // 1103  🪚  🪚  —   —   —   —   carpentry saw
    0x1F527, // 1104  🔧  🔧  🔧  —   🔧  🔧  wrench
    0x1FA9B, // 1105  🪛  🪛  —   —   —   —   screwdriver
    0x1F529, // 1106  🔩  🔩  🔩  —   —   🔩  nut and bolt
    0x2699,  // 1107 ⚙   ⚙   —   —   —   —   gear
    0x1F5DC, // 1108  🗜  🗜  —   —   —   —   clamp
    0x2696,  // 1109 ⚖   ⚖   —   —   —   —   balance scale
    0x1F9AF, // 1110  🦯  🦯  —   —   —   —   white cane
    0x1F517, // 1111  🔗  🔗  🔗  —   —   🔗  link
    0x26D3,  // 1112 ⛓   ⛓   —   —   —   —   chains
    0x1FA9D, // 1113  🪝  🪝  —   —   —   —   hook
    0x1F9F0, // 1114  🧰  🧰  —   —   —   —   toolbox
    0x1F9F2, // 1115  🧲  🧲  —   —   —   —   magnet
    0x1FA9C, // 1116  🪜  🪜  —   —   —   —   ladder
    0x2697,  // 1117 ⚗   ⚗   —   —   —   —   alembic
    0x1F9EA, // 1118  🧪  🧪  —   —   —   —   test tube
    0x1F9EB, // 1119  🧫  🧫  —   —   —   —   petri dish
    0x1F9EC, // 1120  🧬  🧬  —   —   —   —   dna
    0x1F52C, // 1121  🔬  🔬  —   —   —   —   microscope
    0x1F52D, // 1122  🔭  🔭  —   —   —   —   telescope
    0x1F4E1, // 1123  📡  📡  📡  📡  —   📡  satellite antenna
    0x1F489, // 1124  💉  💉  💉  💉  —   💉  syringe
    0x1FA78, // 1125  🩸  🩸  —   —   —   —   drop of blood
    0x1F48A, // 1126  💊  💊  💊  💊  —   💊  pill
    0x1FA79, // 1127  🩹  🩹  —   —   —   —   adhesive bandage
    0x1FA7C, // 1128  🩼  🩼  —   —   —   —   crutch
    0x1FA7A, // 1129  🩺  🩺  —   —   —   —   stethoscope
    0x1FA7B, // 1130  🩻  🩻  —   —   —   —   x-ray
    0x1F6AA, // 1131  🚪  🚪  🚪  —   🚪  —   door
    0x1F6D7, // 1132  🛗  🛗  —   —   —   —   elevator
    0x1FA9E, // 1133  🪞  🪞  —   —   —   —   mirror
    0x1FA9F, // 1134  🪟  🪟  —   —   —   —   window
    0x1F6CF, // 1135  🛏  🛏  —   —   —   —   bed
    0x1F6CB, // 1136  🛋  🛋  —   —   —   —   couch and lamp
    0x1FA91, // 1137  🪑  🪑  —   —   —   —   chair
    0x1F6BD, // 1138  🚽  🚽  🚽  🚽  —   —   toilet
    0x1FAA0, // 1139  🪠  🪠  —   —   —   —   plunger
    0x1F6BF, // 1140  🚿  🚿  —   —   —   —   shower
    0x1F6C1, // 1141  🛁  🛁  —   —   —   —   bathtub
    0x1FAA4, // 1142  🪤  🪤  —   —   —   —   mouse trap
    0x1FA92, // 1143  🪒  🪒  —   —   —   —   razor
    0x1F9F4, // 1144  🧴  🧴  —   —   —   —   lotion bottle
    0x1F9F7, // 1145  🧷  🧷  —   —   —   —   safety pin
    0x1F9F9, // 1146  🧹  🧹  —   —   —   —   broom
    0x1F9FA, // 1147  🧺  🧺  —   —   —   —   basket
    0x1F9FB, // 1148  🧻  🧻  —   —   —   —   roll of paper
    0x1FAA3, // 1149  🪣  🪣  —   —   —   —   bucket
    0x1F9FC, // 1150  🧼  🧼  —   —   —   —   soap
    0x1FAE7, // 1151  🫧  🫧  —   —   —   —   bubbles
    0x1FAA5, // 1152  🪥  🪥  —   —   —   —   toothbrush
    0x1F9FD, // 1153  🧽  🧽  —   —   —   —   sponge
    0x1F9EF, // 1154  🧯  🧯  —   —   —   —   fire extinguisher
    0x1F6D2, // 1155  🛒  🛒  —   —   —   —   shopping cart
    0x1F6AC, // 1156  🚬  🚬  🚬  🚬  🚬  🚬  cigarette
    0x26B0,  // 1157 ⚰   ⚰   —   —   —   —   coffin
    0x1FAA6, // 1158  🪦  🪦  —   —   —   —   headstone
    0x26B1,  // 1159 ⚱   ⚱   —   —   —   —   funeral urn
    0x1F9FF, // 1160  🧿  🧿  —   —   —   —   nazar amulet
    0x1FAAC, // 1161  🪬  🪬  —   —   —   —   hamsa
    0x1F5FF, // 1162  🗿  🗿  🗿  —   —   🗿  moai
    0x1FAA7, // 1163  🪧  🪧  —   —   —   —   placard
    0x1FAAA, // 1164  🪪  🪪  —   —   —   —   identification card
    0x1F3E7, // 1165  🏧  🏧  🏧  🏧  🏧  🏧  ATM sign
    0x1F6AE, // 1166  🚮  🚮  —   —   —   —   litter in bin sign
    0x1F6B0, // 1167  🚰  🚰  —   —   —   —   potable water
    0x267F,  // 1168 ♿  ♿  ♿  ♿  ♿  ♿  wheelchair symbol
    0x1F6B9, // 1169  🚹  🚹  🚹  🚹  —   —   men’s room
    0x1F6BA, // 1170  🚺  🚺  🚺  🚺  —   —   women’s room
    0x1F6BB, // 1171  🚻  🚻  🚻  🚻  🚻  🚻  restroom
    0x1F6BC, // 1172  🚼  🚼  🚼  🚼  —   —   baby symbol
    0x1F6BE, // 1173  🚾  🚾  🚾  🚾  —   —   water closet
    0x1F6C2, // 1174  🛂  🛂  —   —   —   —   passport control
    0x1F6C3, // 1175  🛃  🛃  —   —   —   —   customs
    0x1F6C4, // 1176  🛄  🛄  —   —   —   —   baggage claim
    0x1F6C5, // 1177  🛅  🛅  —   —   —   —   left luggage
    0x26A0,  // 1178 ⚠   ⚠   ⚠   ⚠   ⚠   ⚠   warning
    0x1F6B8, // 1179  🚸  🚸  —   —   —   —   children crossing
    0x26D4,  // 1180 ⛔  ⛔  ⛔  —   —   ⛔  no entry
    0x1F6AB, // 1181  🚫  🚫  🚫  —   —   🚫  prohibited
    0x1F6B3, // 1182  🚳  🚳  —   —   —   —   no bicycles
    0x1F6AD, // 1183  🚭  🚭  🚭  🚭  🚭  🚭  no smoking
    0x1F6AF, // 1184  🚯  🚯  —   —   —   —   no littering
    0x1F6B1, // 1185  🚱  🚱  —   —   —   —   non-potable water
    0x1F6B7, // 1186  🚷  🚷  —   —   —   —   no pedestrians
    0x1F4F5, // 1187  📵  📵  —   —   —   —   no mobile phones
    0x1F51E, // 1188  🔞  🔞  🔞  🔞  —   🔞  no one under eighteen
    0x2622,  // 1189 ☢   ☢   —   —   —   —   radioactive
    0x2623,  // 1190 ☣   ☣   —   —   —   —   biohazard
    0x2B06,  // 1191 ⬆   ⬆   —   ⬆   —   ⬆   up arrow
    0x2197,  // 1192 ↗   ↗   ↗   ↗   ↗   ↗   up-right arrow
    0x27A1,  // 1193 ➡   ➡   —   ➡   —   ➡   right arrow
    0x2198,  // 1194 ↘   ↘   ↘   ↘   ↘   ↘   down-right arrow
    0x2B07,  // 1195 ⬇   ⬇   —   ⬇   —   ⬇   down arrow
    0x2199,  // 1196 ↙   ↙   ↙   ↙   ↙   ↙   down-left arrow
    0x2B05,  // 1197 ⬅   ⬅   —   ⬅   —   ⬅   left arrow
    0x2196,  // 1198 ↖   ↖   ↖   ↖   ↖   ↖   up-left arrow
    0x2195,  // 1199 ↕   ↕   ↕   —   ↕   ↕   up-down arrow
    0x2194,  // 1200 ↔   ↔   ↔   —   ↔   ↔   left-right arrow
    0x21A9,  // 1201 ↩   ↩   —   —   ↩   ↩   right arrow curving left
    0x21AA,  // 1202 ↪   ↪   ↪   —   —   ↪   left arrow curving right
    0x2934,  // 1203 ⤴   ⤴   ⤴   —   ⤴   ⤴   right arrow curving up
    0x2935,  // 1204 ⤵   ⤵   ⤵   —   ⤵   ⤵   right arrow curving down
    0x1F503, // 1205  🔃  🔃  🔃  —   —   🔃  clockwise vertical arrows
    0x1F504, // 1206  🔄  🔄  —   —   —   —   counterclockwise arrows button
    0x1F519, // 1207  🔙  🔙  🔙  —   —   🔙  BACK arrow
    0x1F51A, // 1208  🔚  🔚  🔚  —   🔚  —   END arrow
    0x1F51B, // 1209  🔛  🔛  🔛  —   🔛  —   ON! arrow
    0x1F51C, // 1210  🔜  🔜  🔜  —   🔜  —   SOON arrow
    0x1F51D, // 1211  🔝  🔝  🔝  🔝  —   —   TOP arrow
    0x1F6D0, // 1212  🛐  🛐  —   —   —   —   place of worship
    0x269B,  // 1213 ⚛   ⚛   —   —   —   —   atom symbol
    0x1F549, // 1214  🕉  🕉  —   —   —   —   om
    0x2721,  // 1215 ✡   ✡   —   —   —   —   star of David
    0x2638,  // 1216 ☸   ☸   —   —   —   —   wheel of dharma
    0x262F,  // 1217 ☯   ☯   —   —   —   —   yin yang
    0x271D,  // 1218 ✝   ✝   —   —   —   —   latin cross
    0x2626,  // 1219 ☦   ☦   —   —   —   —   orthodox cross
    0x262A,  // 1220 ☪   ☪   —   —   —   —   star and crescent
    0x262E,  // 1221 ☮   ☮   —   —   —   —   peace symbol
    0x1F54E, // 1222  🕎  🕎  —   —   —   —   menorah
    0x1F52F, // 1223  🔯  🔯  🔯  🔯  —   —   dotted six-pointed star
    0x1FAAF, // 1224  🪯  🪯  —   —   —   —   khanda
    0x2648,  // 1225 ♈  ♈  ♈  ♈  ♈  ♈  Aries
    0x2649,  // 1226 ♉  ♉  ♉  ♉  ♉  ♉  Taurus
    0x264A,  // 1227 ♊  ♊  ♊  ♊  ♊  ♊  Gemini
    0x264B,  // 1228 ♋  ♋  ♋  ♋  ♋  ♋  Cancer
    0x264C,  // 1229 ♌  ♌  ♌  ♌  ♌  ♌  Leo
    0x264D,  // 1230 ♍  ♍  ♍  ♍  ♍  ♍  Virgo
    0x264E,  // 1231 ♎  ♎  ♎  ♎  ♎  ♎  Libra
    0x264F,  // 1232 ♏  ♏  ♏  ♏  ♏  ♏  Scorpio
    0x2650,  // 1233 ♐  ♐  ♐  ♐  ♐  ♐  Sagittarius
    0x2651,  // 1234 ♑  ♑  ♑  ♑  ♑  ♑  Capricorn
    0x2652,  // 1235 ♒  ♒  ♒  ♒  ♒  ♒  Aquarius
    0x2653,  // 1236 ♓  ♓  ♓  ♓  ♓  ♓  Pisces
    0x26CE,  // 1237 ⛎  ⛎  ⛎  ⛎  —   ⛎  Ophiuchus
    0x1F500, // 1238  🔀  🔀  —   —   —   —   shuffle tracks button
    0x1F501, // 1239  🔁  🔁  —   —   —   —   repeat button
    0x1F502, // 1240  🔂  🔂  —   —   —   —   repeat single button
    0x25B6,  // 1241 ▶   ▶   ▶   ▶   —   ▶   play button
    0x23E9,  // 1242 ⏩  ⏩  ⏩  ⏩  —   ⏩  fast-forward button
    0x23ED,  // 1243 ⏭   ⏭   —   —   —   —   next track button
    0x23EF,  // 1244 ⏯   ⏯   —   —   —   —   play or pause button
    0x25C0,  // 1245 ◀   ◀   ◀   ◀   —   ◀   reverse button
    0x23EA,  // 1246 ⏪  ⏪  ⏪  ⏪  —   ⏪  fast reverse button
    0x23EE,  // 1247 ⏮   ⏮   —   —   —   —   last track button
    0x1F53C, // 1248  🔼  🔼  🔼  —   —   🔼  upwards button
    0x23EB,  // 1249 ⏫  ⏫  ⏫  —   —   ⏫  fast up button
    0x1F53D, // 1250  🔽  🔽  🔽  —   —   🔽  downwards button
    0x23EC,  // 1251 ⏬  ⏬  ⏬  —   —   ⏬  fast down button
    0x23F8,  // 1252 ⏸   ⏸   —   —   —   —   pause button
    0x23F9,  // 1253 ⏹   ⏹   —   —   —   —   stop button
    0x23FA,  // 1254 ⏺   ⏺   —   —   —   —   record button
    0x23CF,  // 1255 ⏏   ⏏   —   —   —   —   eject button
    0x1F3A6, // 1256  🎦  🎦  🎦  🎦  —   —   cinema
    0x1F505, // 1257  🔅  🔅  —   —   —   —   dim button
    0x1F506, // 1258  🔆  🔆  —   —   —   —   bright button
    0x1F4F6, // 1259  📶  📶  📶  📶  —   📶  antenna bars
    0x1F6DC, // 1260  🛜  🛜  —   —   —   —   wireless
    0x1F4F3, // 1261  📳  📳  📳  📳  —   📳  vibration mode
    0x1F4F4, // 1262  📴  📴  📴  📴  —   📴  mobile phone off
    0x2640,  // 1263 ♀   ♀   —   —   —   —   female sign
    0x2642,  // 1264 ♂   ♂   —   —   —   —   male sign
    0x26A7,  // 1265 ⚧   ⚧   —   —   —   —   transgender symbol
    0x2716,  // 1266 ✖   ✖   —   —   —   ✖   multiply
    0x2795,  // 1267 ➕  ➕  ➕  —   —   ➕  plus
    0x2796,  // 1268 ➖  ➖  ➖  —   —   ➖  minus
    0x2797,  // 1269 ➗  ➗  ➗  —   —   ➗  divide
    0x1F7F0, // 1270  🟰  🟰  —   —   —   —   heavy equals sign
    0x267E,  // 1271 ♾   ♾   —   —   —   —   infinity
    0x203C,  // 1272 ‼   ‼   ‼   —   ‼   ‼   double exclamation mark
    0x2049,  // 1273 ⁉   ⁉   ⁉   —   ⁉   ⁉   exclamation question mark
    0x2753,  // 1274 ❓  ❓  ❓  ❓  —   ❓  red question mark
    0x2754,  // 1275 ❔  ❔  ❔  ❔  —   —   white question mark
    0x2755,  // 1276 ❕  ❕  ❕  ❕  —   —   white exclamation mark
    0x2757,  // 1277 ❗  ❗  ❗  ❗  ❗  ❗  red exclamation mark
    0x3030,  // 1278 〰  〰  〰  —   〰  —   wavy dash
    0x1F4B1, // 1279  💱  💱  💱  💱  —   —   currency exchange
    0x1F4B2, // 1280  💲  💲  💲  —   —   💲  heavy dollar sign
    0x2695,  // 1281 ⚕   ⚕   —   —   —   —   medical symbol
    0x267B,  // 1282 ♻   ♻   ♻   —   ♻   ♻   recycling symbol
    0x269C,  // 1283 ⚜   ⚜   —   —   —   —   fleur-de-lis
    0x1F531, // 1284  🔱  🔱  🔱  🔱  —   —   trident emblem
    0x1F4DB, // 1285  📛  📛  📛  —   —   📛  name badge
    0x1F530, // 1286  🔰  🔰  🔰  🔰  —   🔰  Japanese symbol for beginner
    0x2B55,  // 1287 ⭕  ⭕  ⭕  ⭕  —   ⭕  hollow red circle
    0x2705,  // 1288 ✅  ✅  ✅  —   —   ✅  check mark button
    0x2611,  // 1289 ☑   ☑   ☑   —   —   ☑   check box with check
    0x2714,  // 1290 ✔   ✔   —   —   —   ✔   check mark
    0x274C,  // 1291 ❌  ❌  ❌  ❌  —   ❌  cross mark
    0x274E,  // 1292 ❎  ❎  ❎  —   —   ❎  cross mark button
    0x27B0,  // 1293 ➰  ➰  ➰  —   ➰  ➰  curly loop
    0x27BF,  // 1294 ➿  ➿  ➿  —   —   —   double curly loop
    0x303D,  // 1295 〽  〽  〽  〽  —   —   part alternation mark
    0x2733,  // 1296 ✳   ✳   ✳   ✳   —   ✳   eight-spoked asterisk
    0x2734,  // 1297 ✴   ✴   —   ✴   —   ✴   eight-pointed star
    0x2747,  // 1298 ❇   ❇   ❇   —   —   ❇   sparkle
    0x00A9,  // 1299 ©   ©   ©   ©   ©   ©   copyright
    0x00AE,  // 1300 ®   ®   ®   ®   ®   ®   registered
    0x2122,  // 1301 ™   ™   ™   ™   ™   ™   trade mark
    0x1F51F, // 1302  🔟  🔟  🔟  —   —   🔟  keycap: 10
    0x1F520, // 1303  🔠  🔠  🔠  —   —   🔠  input latin uppercase
    0x1F521, // 1304  🔡  🔡  🔡  —   —   🔡  input latin lowercase
    0x1F522, // 1305  🔢  🔢  🔢  —   —   🔢  input numbers
    0x1F523, // 1306  🔣  🔣  🔣  —   —   🔣  input symbols
    0x1F524, // 1307  🔤  🔤  🔤  —   —   🔤  input latin letters
    0x1F170, // 1308  🅰   🅰   🅰   🅰   —   🅰   A button (blood type)
    0x1F18E, // 1309  🆎  🆎  🆎  🆎  —   🆎  AB button (blood type)
    0x1F171, // 1310  🅱   🅱   🅱   🅱   —   🅱   B button (blood type)
    0x1F191, // 1311  🆑  🆑  🆑  —   🆑  🆑  CL button
    0x1F192, // 1312  🆒  🆒  🆒  🆒  —   🆒  COOL button
    0x1F193, // 1313  🆓  🆓  🆓  —   🆓  🆓  FREE button
    0x2139,  // 1314 ℹ   ℹ   ℹ   —   —   ℹ   information
    0x1F194, // 1315  🆔  🆔  🆔  🆔  🆔  🆔  ID button
    0x24C2,  // 1316 Ⓜ   Ⓜ   Ⓜ   —   Ⓜ   —   circled M
    0x1F195, // 1317  🆕  🆕  🆕  🆕  🆕  🆕  NEW button
    0x1F196, // 1318  🆖  🆖  🆖  —   🆖  —   NG button
    0x1F17E, // 1319  🅾   🅾   🅾   🅾   —   🅾   O button (blood type)
    0x1F197, // 1320  🆗  🆗  🆗  🆗  🆗  🆗  OK button
    0x1F17F, // 1321  🅿   🅿   🅿   🅿   🅿   🅿   P button
    0x1F198, // 1322  🆘  🆘  🆘  —   —   🆘  SOS button
    0x1F19A, // 1323  🆚  🆚  🆚  🆚  —   🆚  VS button
    0x1F201, // 1324  🈁  🈁  🈁  🈁  —   —   Japanese “here” button
    0x1F202, // 1325  🈂  🈂  🈂  🈂  —   🈂  Japanese “service charge” button
    0x1F237, // 1326  🈷  🈷  🈷  🈷  —   —   Japanese “monthly amount” button
    0x1F236, // 1327  🈶  🈶  🈶  🈶  —   —   Japanese “not free of charge” button
    0x1F22F, // 1328  🈯  🈯  🈯  🈯  —   🈯  Japanese “reserved” button
    0x1F250, // 1329  🉐  🉐  🉐  🉐  —   🉐  Japanese “bargain” button
    0x1F239, // 1330  🈹  🈹  🈹  🈹  —   🈹  Japanese “discount” button
    0x1F21A, // 1331  🈚  🈚  🈚  🈚  —   —   Japanese “free of charge” button
    0x1F232, // 1332  🈲  🈲  🈲  —   🈲  —   Japanese “prohibited” button
    0x1F251, // 1333  🉑  🉑  🉑  —   —   🉑  Japanese “acceptable” button
    0x1F238, // 1334  🈸  🈸  🈸  🈸  —   —   Japanese “application” button
    0x1F234, // 1335  🈴  🈴  🈴  —   🈴  —   Japanese “passing grade” button
    0x1F233, // 1336  🈳  🈳  🈳  🈳  🈳  🈳  Japanese “vacancy” button
    0x3297,  // 1337 ㊗  ㊗  ㊗  ㊗  —   ㊗  Japanese “congratulations” button
    0x3299,  // 1338 ㊙  ㊙  ㊙  ㊙  ㊙  ㊙  Japanese “secret” button
    0x1F23A, // 1339  🈺  🈺  🈺  🈺  —   🈺  Japanese “open for business” button
    0x1F235, // 1340  🈵  🈵  🈵  🈵  🈵  🈵  Japanese “no vacancy” button
    0x1F534, // 1341  🔴  🔴  🔴  🔴  —   🔴  red circle
    0x1F7E0, // 1342  🟠  🟠  —   —   —   —   orange circle
    0x1F7E1, // 1343  🟡  🟡  —   —   —   —   yellow circle
    0x1F7E2, // 1344  🟢  🟢  —   —   —   —   green circle
    0x1F535, // 1345  🔵  🔵  🔵  —   —   🔵  blue circle
    0x1F7E3, // 1346  🟣  🟣  —   —   —   —   purple circle
    0x1F7E4, // 1347  🟤  🟤  —   —   —   —   brown circle
    0x26AB,  // 1348 ⚫  ⚫  ⚫  —   —   ⚫  black circle
    0x26AA,  // 1349 ⚪  ⚪  ⚪  —   —   ⚪  white circle
    0x1F7E5, // 1350  🟥  🟥  —   —   —   —   red square
    0x1F7E7, // 1351  🟧  🟧  —   —   —   —   orange square
    0x1F7E8, // 1352  🟨  🟨  —   —   —   —   yellow square
    0x1F7E9, // 1353  🟩  🟩  —   —   —   —   green square
    0x1F7E6, // 1354  🟦  🟦  —   —   —   —   blue square
    0x1F7EA, // 1355  🟪  🟪  —   —   —   —   purple square
    0x1F7EB, // 1356  🟫  🟫  —   —   —   —   brown square
    0x2B1B,  // 1357 ⬛  ⬛  —   —   —   ⬛  black large square
    0x2B1C,  // 1358 ⬜  ⬜  —   —   —   ⬜  white large square
    0x25FC,  // 1359 ◼   ◼   ◼   —   —   ◼   black medium square
    0x25FB,  // 1360 ◻   ◻   ◻   —   —   ◻   white medium square
    0x25FE,  // 1361 ◾  ◾  ◾  —   —   ◾  black medium-small square
    0x25FD,  // 1362 ◽  ◽  ◽  —   —   ◽  white medium-small square
    0x25AA,  // 1363 ▪   ▪   ▪   —   —   ▪   black small square
    0x25AB,  // 1364 ▫   ▫   ▫   —   —   ▫   white small square
    0x1F536, // 1365  🔶  🔶  🔶  —   —   🔶  large orange diamond
    0x1F537, // 1366  🔷  🔷  🔷  —   —   🔷  large blue diamond
    0x1F538, // 1367  🔸  🔸  🔸  —   —   🔸  small orange diamond
    0x1F539, // 1368  🔹  🔹  🔹  —   —   🔹  small blue diamond
    0x1F53A, // 1369  🔺  🔺  🔺  —   —   🔺  red triangle pointed up
    0x1F53B, // 1370  🔻  🔻  🔻  —   —   🔻  red triangle pointed down
    0x1F4A0, // 1371  💠  💠  💠  —   💠  —   diamond with a dot
    0x1F518, // 1372  🔘  🔘  🔘  —   —   🔘  radio button
    0x1F533, // 1373  🔳  🔳  🔳  🔳  —   —   white square button
    0x1F532, // 1374  🔲  🔲  —   🔲  —   —   black square button
    0x1F3C1, // 1375  🏁  🏁  🏁  🏁  🏁  🏁  chequered flag
    0x1F6A9, // 1376  🚩  🚩  🚩  —   🚩  🚩  triangular flag
    0x1F38C, // 1377  🎌  🎌  🎌  🎌  —   🎌  crossed flags
    0x1F3F4, // 1378  🏴  🏴  —   —   —   —   black flag
    0x1F3F3, // 1379  🏳  🏳  —   —   —   —   white flag
    0x1F3FB, // 1380  🏻  🏻  light skin tone
    0x1F3FC, // 1381  🏼  🏼  medium-light skin tone
    0x1F3FD, // 1382  🏽  🏽  medium skin tone
    0x1F3FE, // 1383  🏾  🏾  medium-dark skin tone
    0x1F3FF, // 1384  🏿  🏿  dark skin tone
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
