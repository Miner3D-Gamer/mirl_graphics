// #[derive(Debug, Copy, Hash)]
// #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
// /// All orders of how a u32 can be formatted
// #[allow(missing_docs)]
// pub enum ColorOrder {
//     AlphaBlueGreenRed,
//     AlphaBlueRedGreen,
//     AlphaGreenBlueRed,
//     AlphaGreenRedBlue,
//     AlphaRedBlueGreen,
//     AlphaRedGreenBlue,

//     BlueAlphaGreenRed,
//     BlueAlphaRedGreen,
//     BlueGreenAlphaRed,
//     BlueGreenRedAlpha,
//     BlueRedAlphaGreen,
//     BlueRedGreenAlpha,

//     GreenAlphaBlueRed,
//     GreenAlphaRedBlue,
//     GreenBlueAlphaRed,
//     GreenBlueRedAlpha,
//     GreenRedAlphaBlue,
//     GreenRedBlueAlpha,

//     RedAlphaBlueGreen,
//     RedAlphaGreenBlue,
//     RedBlueAlphaGreen,
//     RedBlueGreenAlpha,
//     RedGreenAlphaBlue,
//     #[default]
//     RedGreenBlueAlpha,
// }

// TODO: Decide if a struct would actually be worth it
// #[repr(transparent)]
// pub struct ColorARGB(pub u32);

/// Get get the alpha channel of a u32 in argb format
#[inline(always)]
#[must_use]
pub const fn argb_alpha(color: u32) -> u32 {
    (color >> 24) & 0xFF
}

/// Get get the red channel of a u32 in argb format
#[inline(always)]
#[must_use]
pub const fn argb_red(color: u32) -> u32 {
    (color >> 16) & 0xFF
}

/// Get get the green channel of a u32 in argb format
#[inline(always)]
#[must_use]
pub const fn argb_green(color: u32) -> u32 {
    (color >> 8) & 0xFF
}

/// Get get the blue channel of a u32 in argb format
#[inline(always)]
#[must_use]
pub const fn argb_blue(color: u32) -> u32 {
    color & 0xFF
}

/// Get get the red channel of a u32 in rgba format
#[inline(always)]
#[must_use]
pub const fn rgba_red(color: u32) -> u32 {
    (color >> 24) & 0xFF
}

/// Get get the green channel of a u32 in rgba format
#[inline(always)]
#[must_use]
pub const fn rgba_green(color: u32) -> u32 {
    (color >> 16) & 0xFF
}

/// Get get the blue channel of a u32 in rgba format
#[inline(always)]
#[must_use]
pub const fn rgba_blue(color: u32) -> u32 {
    (color >> 8) & 0xFF
}

/// Get get the alpha channel of a u32 in rgba format
#[inline(always)]
#[must_use]
pub const fn rgba_alpha(color: u32) -> u32 {
    color & 0xFF
}

/// Pack (r, g, b, a) u8 channels into an RGBA u32.
#[inline(always)]
#[must_use]
pub const fn pack_rgba_u32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    r << 24 | g << 16 | b << 8 | a
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for [T; 1] {
    fn pack_argb_u32(self) -> u32 {
        (self[0],).pack_argb_u32()
    }
    fn pack_bgra_u32(self) -> u32 {
        (self[0],).pack_bgra_u32()
    }
    fn pack_rgba_u32(self) -> u32 {
        (self[0],).pack_rgba_u32()
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for [T; 2] {
    fn pack_argb_u32(self) -> u32 {
        (self[0], self[1]).pack_argb_u32()
    }
    fn pack_bgra_u32(self) -> u32 {
        (self[0], self[1]).pack_bgra_u32()
    }
    fn pack_rgba_u32(self) -> u32 {
        (self[0], self[1]).pack_rgba_u32()
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for [T; 4] {
    fn pack_argb_u32(self) -> u32 {
        (self[0], self[1], self[2], self[3]).pack_argb_u32()
    }
    fn pack_bgra_u32(self) -> u32 {
        (self[0], self[1], self[2], self[3]).pack_bgra_u32()
    }
    fn pack_rgba_u32(self) -> u32 {
        (self[0], self[1], self[2], self[3]).pack_rgba_u32()
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for [T; 3] {
    fn pack_argb_u32(self) -> u32 {
        (self[0], self[1], self[2]).pack_argb_u32()
    }
    fn pack_bgra_u32(self) -> u32 {
        (self[0], self[1], self[2]).pack_bgra_u32()
    }
    fn pack_rgba_u32(self) -> u32 {
        (self[0], self[1], self[2]).pack_rgba_u32()
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for (T, T, T, T) {
    #[inline(always)]
    fn pack_rgba_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.1.into_color(),
            self.2.into_color(),
            self.3.into_color(),
        )
    }
    #[inline(always)]
    fn pack_argb_u32(self) -> u32 {
        pack_rgba_u32(
            self.3.into_color(),
            self.0.into_color(),
            self.1.into_color(),
            self.2.into_color(),
        )
    }
    #[inline(always)]
    fn pack_bgra_u32(self) -> u32 {
        pack_rgba_u32(
            self.2.into_color(),
            self.1.into_color(),
            self.0.into_color(),
            self.3.into_color(),
        )
    }
}
#[macro_export]
/// Turn the input into a u32 color
///
/// Inputs amount:
/// 1: Grayscale
/// 2: Grayscale + Alpha
/// 3: RGB
/// 4: RGBA
///
/// # Panics
/// When the input values don't
macro_rules! rgb {
    ($v:expr) => {
        ($crate::u32_color_casting::check_color($v),).pack_argb_u32()
    };
    ($r:expr, $g:expr) => {
        (
            $crate::u32_color_casting::check_color($r),
            $crate::u32_color_casting::check_color($g),
        )
            .pack_argb_u32()
    };
    ($r:expr, $g:expr, $b:expr) => {
        (
            $crate::u32_color_casting::check_color($r),
            $crate::u32_color_casting::check_color($g),
            $crate::u32_color_casting::check_color($b),
        )
            .pack_argb_u32()
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        (
            $crate::u32_color_casting::check_color($r),
            $crate::u32_color_casting::check_color($g),
            $crate::u32_color_casting::check_color($b),
            $crate::u32_color_casting::check_color($a),
        )
            .pack_argb_u32()
    };
}
#[must_use]
/// Check value isn't negative
///
/// # Panics
/// When the value doesn't fit into u8
pub const fn check_color<T: [const] TryInto<u32> + [const] std::marker::Destruct + [const] Clone>(
    v: T,
) -> u32
where
    <T as std::convert::TryInto<u32>>::Error: [const] std::marker::Destruct,
{
    if v.clone().try_into().is_err() {
        panic!("negative not allowed")
    } else {
        // Safety: What type can convert into u8 but not u32?
        unsafe { v.try_into().unwrap_unchecked() }
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for (T, T, T) {
    #[inline(always)]
    fn pack_rgba_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.1.into_color(),
            self.2.into_color(),
            255,
        )
    }
    #[inline(always)]
    fn pack_argb_u32(self) -> u32 {
        pack_rgba_u32(
            255,
            self.0.into_color(),
            self.1.into_color(),
            self.2.into_color(),
        )
    }
    #[inline(always)]
    fn pack_bgra_u32(self) -> u32 {
        pack_rgba_u32(
            self.2.into_color(),
            self.1.into_color(),
            self.0.into_color(),
            255,
        )
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for (T, T) {
    #[inline(always)]
    fn pack_rgba_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
            self.1.into_color(),
        )
    }
    #[inline(always)]
    fn pack_argb_u32(self) -> u32 {
        pack_rgba_u32(
            self.1.into_color(),
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
        )
    }
    #[inline(always)]
    fn pack_bgra_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
            self.1.into_color(),
        )
    }
}
const impl<T: const IntoColor + Copy> PackChannelsIntoColor for (T,) {
    #[inline(always)]
    fn pack_rgba_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
            255,
        )
    }
    #[inline(always)]
    fn pack_argb_u32(self) -> u32 {
        pack_rgba_u32(
            255,
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
        )
    }
    #[inline(always)]
    fn pack_bgra_u32(self) -> u32 {
        pack_rgba_u32(
            self.0.into_color(),
            self.0.into_color(),
            self.0.into_color(),
            255,
        )
    }
}
/// Pack the given individual colors into a single u32 color
pub const trait PackChannelsIntoColor {
    #[must_use]
    /// Pack the given individual colors into a single u32 color
    ///
    /// Assumes the input is formatted in R G B A
    fn pack_rgba_u32(self) -> u32;
    #[must_use]
    /// Pack the given individual colors into a single u32 color
    ///
    /// Assumes the input is formatted in R G B A
    fn pack_argb_u32(self) -> u32;
    #[must_use]
    /// Pack the given individual colors into a single u32 color
    ///
    /// Assumes the input is formatted in R G B A
    fn pack_bgra_u32(self) -> u32;
}
/// Unpack the single u32 into the individual channels
pub const trait UnpackColorIntoChannels<T: Copy>: std::marker::Sized {
    #[must_use]
    /// Unpack a u32 color into its R G B A channels
    fn unpack_u32_rgba(self) -> (T, T, T, T);
    #[must_use]
    /// Unpack a u32 color into its A R G B channels
    fn unpack_u32_argb(self) -> (T, T, T, T);
    #[must_use]
    /// Unpack a u32 color into its B G R A channels
    fn unpack_u32_bgra(self) -> (T, T, T, T);
    #[must_use]
    /// Unpack a u32 color into its R G B channels
    fn unpack_u32_rgb(self) -> (T, T, T) {
        let x = self.unpack_u32_rgba();
        (x.0, x.1, x.2)
    }
}
const impl<T: const FromColorChannel + Copy> UnpackColorIntoChannels<T> for u32 {
    #[inline(always)]
    fn unpack_u32_rgba(self) -> (T, T, T, T) {
        (
            T::from_color_channel(self.red() as u8),
            T::from_color_channel(self.green() as u8),
            T::from_color_channel(self.blue() as u8),
            T::from_color_channel(self.alpha() as u8),
        )
    }
    #[inline(always)]
    fn unpack_u32_argb(self) -> (T, T, T, T) {
        (
            T::from_color_channel(self.alpha() as u8),
            T::from_color_channel(self.red() as u8),
            T::from_color_channel(self.green() as u8),
            T::from_color_channel(self.blue() as u8),
        )
    }
    #[inline(always)]
    fn unpack_u32_bgra(self) -> (T, T, T, T) {
        (
            T::from_color_channel(self.blue() as u8),
            T::from_color_channel(self.green() as u8),
            T::from_color_channel(self.red() as u8),
            T::from_color_channel(self.alpha() as u8),
        )
    }
}
/// Turn self into a u32 color
pub const trait IntoColor: Copy {
    #[must_use]
    /// Turn self into a u32 color channel
    fn into_color(self) -> u32;
}
const impl<T: const Into<u32> + Copy> IntoColor for T {
    #[inline(always)]
    fn into_color(self) -> u32 {
        self.into()
    }
}
/// Get self from a u8 color channel
pub const trait FromColorChannel {
    #[must_use]
    /// Get self from a u8 color channel
    fn from_color_channel(color_channel: u8) -> Self;
}
const impl<T: const From<u8>> FromColorChannel for T {
    #[inline(always)]
    fn from_color_channel(color_channel: u8) -> Self {
        Self::from(color_channel)
    }
}
// TODO: Expand the conversions
/// Switch color formats and return the result
pub const trait SwitchColorFormat {
    #[must_use]
    /// Switch from the default RGBA to ARGB
    fn switch_from_rgba_to_argb(self) -> Self;
    #[must_use]
    /// Switch from the default RGBA to BGRA present in most windows image formats
    fn switch_from_rgba_to_bgra(self) -> Self;
    #[must_use]
    /// Switch from ARGB to the default RGBA
    fn switch_from_argb_to_rgba(self) -> Self;
}
const impl SwitchColorFormat for u32 {
    #[inline(always)]
    fn switch_from_argb_to_rgba(self) -> Self {
        <Self as UnpackColorIntoChannels<Self>>::unpack_u32_argb(self).pack_rgba_u32()
    }
    #[inline(always)]
    fn switch_from_rgba_to_argb(self) -> Self {
        <Self as UnpackColorIntoChannels<Self>>::unpack_u32_rgba(self).pack_argb_u32()
    }
    #[inline(always)]
    fn switch_from_rgba_to_bgra(self) -> Self {
        <Self as UnpackColorIntoChannels<Self>>::unpack_u32_rgba(self).pack_bgra_u32()
    }
}

/// A trait for changing or retrieving a single channel of a color
pub const trait ColorManipulation<T: IntoColor>: ColorManipulationWithoutInput {
    #[must_use]
    /// Set the alpha channel
    fn with_alpha(self, alpha: T) -> Self;
    #[must_use]
    /// Set the red channel
    fn with_red(self, red: T) -> Self;
    #[must_use]
    /// Set the green channel
    fn with_green(self, green: T) -> Self;
    #[must_use]
    /// Set the blue channel
    fn with_blue(self, blue: T) -> Self;
    #[must_use]
    /// Add X to the blue channel
    fn add_blue(self, other: T) -> Self;
    #[must_use]
    /// Add X to the red channel
    fn add_red(self, other: T) -> Self;
    #[must_use]
    /// Add X to the green channel
    fn add_green(self, other: T) -> Self;
    #[must_use]
    /// Add X to the alpha channel
    fn add_alpha(self, other: T) -> Self;
    #[must_use]
    /// Subtract X from the blue channel
    fn sub_blue(self, other: T) -> Self;
    #[must_use]
    /// Subtract X from the red channel
    fn sub_red(self, other: T) -> Self;
    #[must_use]
    /// Subtract X from the green channel
    fn sub_green(self, other: T) -> Self;
    #[must_use]
    /// Subtract X from the alpha channel
    fn sub_alpha(self, other: T) -> Self;
    #[must_use]
    /// Add to all color channels - Alpha excluded
    fn add_all_color_channels(self, other: T) -> Self;
    #[must_use]
    /// Sub from all color channels - Alpha excluded
    fn sub_all_color_channels(self, other: T) -> Self;
}
/// A trait for changing or retrieving a single channel of a color
pub const trait ColorManipulationWithoutInput {
    #[must_use]
    /// Get the alpha channel
    fn alpha(self) -> Self;
    #[must_use]
    /// Get the red channel
    fn red(self) -> Self;
    #[must_use]
    /// Get the green channel
    fn green(self) -> Self;
    #[must_use]
    /// Get the blue channel
    fn blue(self) -> Self;
    #[must_use]
    /// Invert all color channels
    fn invert_color(self) -> Self;
    #[must_use]
    /// Invert the red channel
    fn invert_red(self) -> Self;
    #[must_use]
    /// Invert the blue channel
    fn invert_blue(self) -> Self;
    #[must_use]
    /// Invert the green channel
    fn invert_green(self) -> Self;
    #[must_use]
    /// Invert alpha channel
    fn invert_alpha(self) -> Self;
}
const impl ColorManipulationWithoutInput for u32 {
    #[inline(always)]
    fn alpha(self) -> Self {
        rgba_alpha(self)
    }

    #[inline(always)]
    fn red(self) -> Self {
        rgba_red(self)
    }

    #[inline(always)]
    fn green(self) -> Self {
        rgba_green(self)
    }

    #[inline(always)]
    fn blue(self) -> Self {
        rgba_blue(self)
    }
    #[inline(always)]
    fn invert_alpha(self) -> Self {
        self.with_alpha(invert_individual_channel(self.alpha()))
    }
    #[inline(always)]
    fn invert_red(self) -> Self {
        self.with_red(invert_individual_channel(self.red()))
    }
    #[inline(always)]
    fn invert_blue(self) -> Self {
        self.with_blue(invert_individual_channel(self.blue()))
    }
    #[inline(always)]
    fn invert_green(self) -> Self {
        self.with_green(invert_individual_channel(self.green()))
    }
    #[inline(always)]
    fn invert_color(self) -> Self {
        (
            invert_individual_channel(self.red()),
            invert_individual_channel(self.green()),
            invert_individual_channel(self.blue()),
            self.alpha(),
        )
            .pack_rgba_u32()
    }
}
const impl<T: [const] IntoColor> ColorManipulation<T> for u32 {
    #[inline(always)]
    fn with_alpha(self, alpha: T) -> Self {
        (self.red(), self.green(), self.blue(), alpha.into_color()).pack_rgba_u32()
    }

    #[inline(always)]
    fn with_red(self, red: T) -> Self {
        (red.into_color(), self.green(), self.blue(), self.alpha()).pack_rgba_u32()
    }

    #[inline(always)]
    fn with_green(self, green: T) -> Self {
        (self.red(), green.into_color(), self.blue(), self.alpha()).pack_rgba_u32()
    }

    #[inline(always)]
    fn with_blue(self, blue: T) -> Self {
        (self.red(), self.green(), blue.into_color(), self.alpha()).pack_rgba_u32()
    }

    #[inline(always)]
    fn add_alpha(self, other: T) -> Self {
        self.with_alpha(self.alpha() + other.into_color())
    }
    #[inline(always)]
    fn add_red(self, other: T) -> Self {
        self.with_red(self.red() + other.into_color())
    }
    #[inline(always)]
    fn add_green(self, other: T) -> Self {
        self.with_green(self.green() + other.into_color())
    }
    #[inline(always)]
    fn add_blue(self, other: T) -> Self {
        self.with_blue(self.blue() + other.into_color())
    }
    #[inline(always)]
    fn sub_alpha(self, other: T) -> Self {
        self.with_alpha(self.alpha() - other.into_color())
    }
    #[inline(always)]
    fn sub_red(self, other: T) -> Self {
        self.with_red(self.red() - other.into_color())
    }
    #[inline(always)]
    fn sub_green(self, other: T) -> Self {
        self.with_green(self.green() - other.into_color())
    }
    #[inline(always)]
    fn sub_blue(self, other: T) -> Self {
        self.with_blue(self.blue() - other.into_color())
    }

    #[inline(always)]
    fn add_all_color_channels(self, other: T) -> Self {
        self.add_blue(other).add_red(other).add_green(other)
    }
    #[inline(always)]
    fn sub_all_color_channels(self, other: T) -> Self {
        self.sub_blue(other).sub_red(other).sub_green(other)
    }
}

#[inline(always)]
#[must_use]
/// Invert a color channel
pub const fn invert_individual_channel(color: u32) -> u32 {
    255 - color
}
