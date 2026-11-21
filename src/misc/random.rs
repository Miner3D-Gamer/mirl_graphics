#[must_use]
/// Get a random color within the given color range
pub fn generate_random_color(range: ((u32, u32, u32), (u32, u32, u32))) -> u32 {
    let red = _random_u32_range(range.0.0, range.1.0);
    let green = _random_u32_range(range.0.1, range.1.1);
    let blue = _random_u32_range(range.0.2, range.1.2);
    (red, green, blue, 255).pack_rgba_u32()
}
#[must_use]
/// Get a random u32 number between 2 set ranges
pub fn _random_u32_range(lower: u32, upper: u32) -> u32 {
    let range = upper - lower;
    loop {
        let n = unsafe { getrandom::u32().unwrap_unchecked() }; // This shouldn't error and if it does we don't care
        let zone = u32::MAX - (u32::MAX % range);

        if n < zone {
            return lower + n % range;
        }
    }
}

use crate::u32_color_casting::PackChannelsIntoColor;
use mirl_buffer::Buffer;
/// Replace a color in the buffer with a random color within the given color range
pub fn replace_color_with_random_color(
    buffer: &mut Buffer,
    color: u32,
    range: ((u32, u32, u32), (u32, u32, u32)),
) {
    for i in &mut buffer.data {
        if *i == color {
            *i = generate_random_color(range);
        }
    }
}
// #[must_use]
// /// Create a range of colors based on a variance
// pub fn color_range_with_variance(
//     color: (u32, u32, u32),
//     variance: (u32, u32, u32),
// ) -> ((u32, u32, u32), (u32, u32, u32)) {
//     crate::misc::reorder_color_range((
//         (color.0, variance.0),
//         mirl_core::math::range_with_variance(color.1, variance.1),
//         mirl_core::math::range_with_variance(color.2, variance.2),
//     ))
// }
