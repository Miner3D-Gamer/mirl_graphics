use crate::prelude::*;

/// The "color" white -> 255u32, 255u32, 255
pub const WHITE: u32 = (255u32, 255u32, 255u32, 255u32).pack_rgba_u32();

/// The "color" black -> 0u32, 0u32, 0
pub const BLACK: u32 = (0u32, 0u32, 0u32, 255u32).pack_rgba_u32();

/// The color pure red -> 255u32, 0u32, 0
pub const RED: u32 = (255u32, 0u32, 0u32, 255u32).pack_rgba_u32();

/// The color pure green -> 0u32, 255u32, 0
pub const GREEN: u32 = (0u32, 255u32, 0u32, 255u32).pack_rgba_u32();

/// The color pure blue -> 0u32, 0u32, 255
pub const BLUE: u32 = (0u32, 0u32, 255u32, 255u32).pack_rgba_u32();

/// The color pure magenta -> 255u32, 0u32, 255
pub const MAGENTA: u32 = (255u32, 0u32, 255u32, 255u32).pack_rgba_u32();

/// The color pure light blue -> 0u32, 255u32, 255
pub const LIGHT_BLUE: u32 = (0u32, 255u32, 255u32, 255u32).pack_rgba_u32();

/// The color pure yellow -> 255u32, 0u32, 0
pub const YELLOW: u32 = (255u32, 255u32, 0u32, 255u32).pack_rgba_u32();

// Reds and Pinks
/// The color dark red -> 139u32, 0u32, 0
pub const DARK_RED: u32 = (139u32, 0u32, 0u32, 255u32).pack_rgba_u32();

/// The color crimson -> 220u32, 20u32, 60
pub const CRIMSON: u32 = (220u32, 20u32, 60u32, 255u32).pack_rgba_u32();

/// The color pink -> 255u32, 192u32, 203
pub const PINK: u32 = (255u32, 192u32, 203u32, 255u32).pack_rgba_u32();

/// The color hot pink -> 255u32, 105u32, 180
pub const HOT_PINK: u32 = (255u32, 105u32, 180u32, 255u32).pack_rgba_u32();

/// The color deep pink -> 255u32, 20u32, 147
pub const DEEP_PINK: u32 = (255u32, 20u32, 147u32, 255u32).pack_rgba_u32();

/// The color light pink -> 255u32, 182u32, 193
pub const LIGHT_PINK: u32 = (255u32, 182u32, 193u32, 255u32).pack_rgba_u32();

/// The color salmon -> 250u32, 128u32, 114
pub const SALMON: u32 = (250u32, 128u32, 114u32, 255u32).pack_rgba_u32();

/// The color light salmon -> 255u32, 160u32, 122
pub const LIGHT_SALMON: u32 = (255u32, 160u32, 122u32, 255u32).pack_rgba_u32();

/// The color coral -> 255u32, 127u32, 80
pub const CORAL: u32 = (255u32, 127u32, 80u32, 255u32).pack_rgba_u32();

/// The color tomato -> 255u32, 99u32, 71
pub const TOMATO: u32 = (255u32, 99u32, 71u32, 255u32).pack_rgba_u32();

// Oranges
/// The color orange -> 255u32, 165u32, 0
pub const ORANGE: u32 = (255u32, 165u32, 0u32, 255u32).pack_rgba_u32();

/// The color dark orange -> 255u32, 140u32, 0
pub const DARK_ORANGE: u32 = (255u32, 140u32, 0u32, 255u32).pack_rgba_u32();

/// The color orange red -> 255u32, 69u32, 0
pub const ORANGE_RED: u32 = (255u32, 69u32, 0u32, 255u32).pack_rgba_u32();

// Yellows
/// The color gold -> 255u32, 215u32, 0
pub const GOLD: u32 = (255u32, 215u32, 0u32, 255u32).pack_rgba_u32();

/// The color khaki -> 240u32, 230u32, 140
pub const KHAKI: u32 = (240u32, 230u32, 140u32, 255u32).pack_rgba_u32();

/// The color light yellow -> 255u32, 255u32, 224
pub const LIGHT_YELLOW: u32 = (255u32, 255u32, 224u32, 255u32).pack_rgba_u32();

/// The color lemon chiffon -> 255u32, 250u32, 205
pub const LEMON_CHIFFON: u32 = (255u32, 250u32, 205u32, 255u32).pack_rgba_u32();

// Greens
/// The color dark green -> 0u32, 100u32, 0
pub const DARK_GREEN: u32 = (0u32, 100u32, 0u32, 255u32).pack_rgba_u32();

/// The color forest green -> 34u32, 139u32, 34
pub const FOREST_GREEN: u32 = (34u32, 139u32, 34u32, 255u32).pack_rgba_u32();

/// The color sea green -> 46u32, 139u32, 87
pub const SEA_GREEN: u32 = (46u32, 139u32, 87u32, 255u32).pack_rgba_u32();

/// The color light green -> 144u32, 238u32, 144
pub const LIGHT_GREEN: u32 = (144u32, 238u32, 144u32, 255u32).pack_rgba_u32();

/// The color pale green -> 152u32, 251u32, 152
pub const PALE_GREEN: u32 = (152u32, 251u32, 152u32, 255u32).pack_rgba_u32();

/// The color lime -> 0u32, 255u32, 0
pub const LIME: u32 = (0u32, 255u32, 0u32, 255u32).pack_rgba_u32();

/// The color lime green -> 50u32, 205u32, 50
pub const LIME_GREEN: u32 = (50u32, 205u32, 50u32, 255u32).pack_rgba_u32();

/// The color spring green -> 0u32, 255u32, 127
pub const SPRING_GREEN: u32 = (0u32, 255u32, 127u32, 255u32).pack_rgba_u32();

/// The color medium spring green -> 0u32, 250u32, 154
pub const MEDIUM_SPRING_GREEN: u32 = (0u32, 250u32, 154u32, 255u32).pack_rgba_u32();

/// The color lawn green -> 124u32, 252u32, 0
pub const LAWN_GREEN: u32 = (124u32, 252u32, 0u32, 255u32).pack_rgba_u32();

// Cyans/Teals
/// The color dark cyan -> 0u32, 139u32, 139
pub const DARK_CYAN: u32 = (0u32, 139u32, 139u32, 255u32).pack_rgba_u32();

/// The color teal -> 0u32, 128u32, 128
pub const TEAL: u32 = (0u32, 128u32, 128u32, 255u32).pack_rgba_u32();

/// The color medium aquamarine -> 102u32, 205u32, 170
pub const MEDIUM_AQUAMARINE: u32 = (102u32, 205u32, 170u32, 255u32).pack_rgba_u32();

/// The color aquamarine -> 127u32, 255u32, 212
pub const AQUAMARINE: u32 = (127u32, 255u32, 212u32, 255u32).pack_rgba_u32();

/// The color turquoise -> 64u32, 224u32, 208
pub const TURQUOISE: u32 = (64u32, 224u32, 208u32, 255u32).pack_rgba_u32();

/// The color light turquoise -> 175u32, 238u32, 238
pub const LIGHT_TURQUOISE: u32 = (175u32, 238u32, 238u32, 255u32).pack_rgba_u32();

// Blues
/// The color dark blue -> 0u32, 0u32, 139
pub const DARK_BLUE: u32 = (0u32, 0u32, 139u32, 255u32).pack_rgba_u32();

/// The color navy -> 0u32, 0u32, 128
pub const NAVY: u32 = (0u32, 0u32, 128u32, 255u32).pack_rgba_u32();

/// The color navy blue -> 0u32, 0u32, 128
pub const NAVY_BLUE: u32 = (0u32, 0u32, 128u32, 255u32).pack_rgba_u32();

/// The color royal blue -> 65u32, 105u32, 225
pub const ROYAL_BLUE: u32 = (65u32, 105u32, 225u32, 255u32).pack_rgba_u32();

/// The color medium blue -> 0u32, 0u32, 205
pub const MEDIUM_BLUE: u32 = (0u32, 0u32, 205u32, 255u32).pack_rgba_u32();

/// The color cornflower blue -> 100u32, 149u32, 237
pub const CORNFLOWER_BLUE: u32 = (100u32, 149u32, 237u32, 255u32).pack_rgba_u32();

/// The color steel blue -> 70u32, 130u32, 180
pub const STEEL_BLUE: u32 = (70u32, 130u32, 180u32, 255u32).pack_rgba_u32();

/// The color light steel blue -> 176u32, 196u32, 222
pub const LIGHT_STEEL_BLUE: u32 = (176u32, 196u32, 222u32, 255u32).pack_rgba_u32();

/// The color dodger blue -> 30u32, 144u32, 255
pub const DODGER_BLUE: u32 = (30u32, 144u32, 255u32, 255u32).pack_rgba_u32();

/// The color deep sky blue -> 0u32, 191u32, 255
pub const DEEP_SKY_BLUE: u32 = (0u32, 191u32, 255u32, 255u32).pack_rgba_u32();

/// The color sky blue -> 135u32, 206u32, 235
pub const SKY_BLUE: u32 = (135u32, 206u32, 235u32, 255u32).pack_rgba_u32();

/// The color light sky blue -> 135u32, 206u32, 250
pub const LIGHT_SKY_BLUE: u32 = (135u32, 206u32, 250u32, 255u32).pack_rgba_u32();

// Purples and Violets
/// The color purple -> 128u32, 0u32, 128
pub const PURPLE: u32 = (128u32, 0u32, 128u32, 255u32).pack_rgba_u32();

/// The color dark purple -> 48u32, 25u32, 52
pub const DARK_PURPLE: u32 = (48u32, 25u32, 52u32, 255u32).pack_rgba_u32();

/// The color dark magenta -> 139u32, 0u32, 139
pub const DARK_MAGENTA: u32 = (139u32, 0u32, 139u32, 255u32).pack_rgba_u32();

/// The color indigo -> 75u32, 0u32, 130
pub const INDIGO: u32 = (75u32, 0u32, 130u32, 255u32).pack_rgba_u32();

/// The color blue violet -> 138u32, 43u32, 226
pub const BLUE_VIOLET: u32 = (138u32, 43u32, 226u32, 255u32).pack_rgba_u32();

/// The color medium slate blue -> 123u32, 104u32, 238
pub const MEDIUM_SLATE_BLUE: u32 = (123u32, 104u32, 238u32, 255u32).pack_rgba_u32();

/// The color medium purple -> 147u32, 112u32, 219
pub const MEDIUM_PURPLE: u32 = (147u32, 112u32, 219u32, 255u32).pack_rgba_u32();

/// The color slate blue -> 106u32, 90u32, 205
pub const SLATE_BLUE: u32 = (106u32, 90u32, 205u32, 255u32).pack_rgba_u32();

/// The color dark slate blue -> 72u32, 61u32, 139
pub const DARK_SLATE_BLUE: u32 = (72u32, 61u32, 139u32, 255u32).pack_rgba_u32();

/// The color violet -> 238u32, 130u32, 238
pub const VIOLET: u32 = (238u32, 130u32, 238u32, 255u32).pack_rgba_u32();

/// The color orchid -> 218u32, 112u32, 214
pub const ORCHID: u32 = (218u32, 112u32, 214u32, 255u32).pack_rgba_u32();

/// The color plum -> 221u32, 160u32, 221
pub const PLUM: u32 = (221u32, 160u32, 221u32, 255u32).pack_rgba_u32();

/// The color thistle -> 216u32, 191u32, 216
pub const THISTLE: u32 = (216u32, 191u32, 216u32, 255u32).pack_rgba_u32();

// Grays
/// The color dark gray -> 169u32, 169u32, 169
pub const DARK_GRAY: u32 = (169u32, 169u32, 169u32, 255u32).pack_rgba_u32();

/// The color gray -> 128u32, 128u32, 128
pub const GRAY: u32 = (128u32, 128u32, 128u32, 255u32).pack_rgba_u32();

/// The color light gray -> 211u32, 211u32, 211
pub const LIGHT_GRAY: u32 = (211u32, 211u32, 211u32, 255u32).pack_rgba_u32();

/// The color slate gray -> 112u32, 128u32, 144
pub const SLATE_GRAY: u32 = (112u32, 128u32, 144u32, 255u32).pack_rgba_u32();

/// The color dark slate gray -> 47u32, 79u32, 79
pub const DARK_SLATE_GRAY: u32 = (47u32, 79u32, 79u32, 255u32).pack_rgba_u32();

/// The color light slate gray -> 119u32, 136u32, 153
pub const LIGHT_SLATE_GRAY: u32 = (119u32, 136u32, 153u32, 255u32).pack_rgba_u32();

/// The color silver -> 192u32, 192u32, 192
pub const SILVER: u32 = (192u32, 192u32, 192u32, 255u32).pack_rgba_u32();

/// The color gainsboro -> 220u32, 220u32, 220
pub const GAINSBORO: u32 = (220u32, 220u32, 220u32, 255u32).pack_rgba_u32();

/// The color whitesmoke -> 245u32, 245u32, 245
pub const WHITESMOKE: u32 = (245u32, 245u32, 245u32, 255u32).pack_rgba_u32();

// Browns
/// The color brown -> 165u32, 42u32, 42
pub const BROWN: u32 = (165u32, 42u32, 42u32, 255u32).pack_rgba_u32();

/// The color saddle brown -> 139u32, 69u32, 19
pub const SADDLE_BROWN: u32 = (139u32, 69u32, 19u32, 255u32).pack_rgba_u32();

/// The color sienna -> 160u32, 82u32, 45
pub const SIENNA: u32 = (160u32, 82u32, 45u32, 255u32).pack_rgba_u32();

/// The color chocolate -> 210u32, 105u32, 30
pub const CHOCOLATE: u32 = (210u32, 105u32, 30u32, 255u32).pack_rgba_u32();

/// The color peru -> 205u32, 133u32, 63
pub const PERU: u32 = (205u32, 133u32, 63u32, 255u32).pack_rgba_u32();

/// The color tan -> 210u32, 180u32, 140
pub const TAN: u32 = (210u32, 180u32, 140u32, 255u32).pack_rgba_u32();

/// The color rosy brown -> 188u32, 143u32, 143
pub const ROSY_BROWN: u32 = (188u32, 143u32, 143u32, 255u32).pack_rgba_u32();

/// The color burlywood -> 222u32, 184u32, 135
pub const BURLYWOOD: u32 = (222u32, 184u32, 135u32, 255u32).pack_rgba_u32();

/// The color dark goldenrod -> 184u32, 134u32, 11
pub const DARK_GOLDENROD: u32 = (184u32, 134u32, 11u32, 255u32).pack_rgba_u32();

/// The color goldenrod -> 218u32, 165u32, 32
pub const GOLDENROD: u32 = (218u32, 165u32, 32u32, 255u32).pack_rgba_u32();

// Miscellaneous
/// The color beige -> 245u32, 245u32, 220
pub const BEIGE: u32 = (245u32, 245u32, 220u32, 255u32).pack_rgba_u32();

/// The color olive -> 128u32, 128u32, 0
pub const OLIVE: u32 = (128u32, 128u32, 0u32, 255u32).pack_rgba_u32();

/// The color maroon -> 128u32, 0u32, 0
pub const MAROON: u32 = (128u32, 0u32, 0u32, 255u32).pack_rgba_u32();

/// The color mint -> 152u32, 251u32, 152
pub const MINT: u32 = (152u32, 251u32, 152u32, 255u32).pack_rgba_u32();

/// The color mint cream -> 245u32, 255u32, 250
pub const MINT_CREAM: u32 = (245u32, 255u32, 250u32, 255u32).pack_rgba_u32();

/// The color ivory -> 255u32, 255u32, 240
pub const IVORY: u32 = (255u32, 255u32, 240u32, 255u32).pack_rgba_u32();

/// The color honeydew -> 240u32, 255u32, 240
pub const HONEYDEW: u32 = (240u32, 255u32, 240u32, 255u32).pack_rgba_u32();

/// The color alice blue -> 240u32, 248u32, 255
pub const ALICE_BLUE: u32 = (240u32, 248u32, 255u32, 255u32).pack_rgba_u32();

/// The color ghost white -> 248u32, 248u32, 255
pub const GHOST_WHITE: u32 = (248u32, 248u32, 255u32, 255u32).pack_rgba_u32();

/// The color lavender -> 230u32, 230u32, 250
pub const LAVENDER: u32 = (230u32, 230u32, 250u32, 255u32).pack_rgba_u32();

/// The color linen -> 250u32, 240u32, 230
pub const LINEN: u32 = (250u32, 240u32, 230u32, 255u32).pack_rgba_u32();

/// The color antique white -> 250u32, 235u32, 215
pub const ANTIQUE_WHITE: u32 = (250u32, 235u32, 215u32, 255u32).pack_rgba_u32();

/// The color papaya whip -> 255u32, 239u32, 213
pub const PAPAYA_WHIP: u32 = (255u32, 239u32, 213u32, 255u32).pack_rgba_u32();

/// The color blanched almond -> 255u32, 235u32, 205
pub const BLANCHED_ALMOND: u32 = (255u32, 235u32, 205u32, 255u32).pack_rgba_u32();

/// The color bisque -> 255u32, 228u32, 196
pub const BISQUE: u32 = (255u32, 228u32, 196u32, 255u32).pack_rgba_u32();

/// The color peach puff -> 255u32, 218u32, 185
pub const PEACH_PUFF: u32 = (255u32, 218u32, 185u32, 255u32).pack_rgba_u32();

/// The color navajo white -> 255u32, 222u32, 173
pub const NAVAJO_WHITE: u32 = (255u32, 222u32, 173u32, 255u32).pack_rgba_u32();

/// The color moccasin -> 255u32, 228u32, 181
pub const MOCCASIN: u32 = (255u32, 228u32, 181u32, 255u32).pack_rgba_u32();

/// The color misty rose -> 255u32, 228u32, 225
pub const MISTY_ROSE: u32 = (255u32, 228u32, 225u32, 255u32).pack_rgba_u32();

/// The color seashell -> 255u32, 245u32, 238
pub const SEASHELL: u32 = (255u32, 245u32, 238u32, 255u32).pack_rgba_u32();

/// The color old lace -> 253u32, 245u32, 230
pub const OLD_LACE: u32 = (253u32, 245u32, 230u32, 255u32).pack_rgba_u32();

/// The color floral white -> 255u32, 250u32, 240
pub const FLORAL_WHITE: u32 = (255u32, 250u32, 240u32, 255u32).pack_rgba_u32();

/// The color cornsilk -> 255u32, 248u32, 220
pub const CORNSILK: u32 = (255u32, 248u32, 220u32, 255u32).pack_rgba_u32();
