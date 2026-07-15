use mirl_buffer::prelude::*;

use crate::{misc::switch_colors_in_list, u32_color_casting::UnpackColorIntoChannels};

/// Turn the Buffer into a Vec of u8
pub const trait BufferToVec {
    #[must_use]
    /// Converts the [`Vec<u32>`] to [`Vec<8>`] by unpacking the u32 into argb style
    fn to_u8_argb(&self) -> Vec<u8>;
    #[must_use]
    /// Converts the internal [`Box<[u32]>`](Box<u32>) to [`Vec<8>`] by unpacking the u32 into rgba style
    fn to_u8_rgba(&self) -> Vec<u8>;
    #[must_use]
    /// Converts the internal [`Box<[u32]>`](Box<u32>) to [`Vec<8>`] by unpacking the u32 into rgba style
    fn to_u8_bgra(&self) -> Vec<u8>;
}
impl<S: BufferData> BufferToVec for S {
    default fn to_u8_argb(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list.extend(<[u8; 4]>::from((*i).unpack_u32_rgba()));
        }
        return_list
    }

    default fn to_u8_rgba(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list.extend(<[u8; 4]>::from((*i).unpack_u32_rgba()));
        }
        return_list
    }
    default fn to_u8_bgra(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list.extend(<[u8; 4]>::from((*i).unpack_u32_bgra()));
        }
        return_list
    }
}

/// Switch the color channels that compose the image
pub const trait SwitchInternalFormat {
    /// Switch the red and blue color channels
    fn switch_red_and_blue(&mut self);

    /// Switch from the RGBA to the ARGB format
    fn switch_from_rgba_to_argb(&mut self);

    /// Switch from the ARGB to the RGBA format
    fn switch_from_argb_to_rgba(&mut self);
}

impl<T: BufferData> SwitchInternalFormat for T {
    fn switch_red_and_blue(&mut self) {
        switch_colors_in_list(
            self.data_mut(),
            super::u32_color_casting::SwitchColorFormat::switch_from_rgba_to_bgra,
        );
    }
    fn switch_from_rgba_to_argb(&mut self) {
        switch_colors_in_list(
            self.data_mut(),
            super::u32_color_casting::SwitchColorFormat::switch_from_rgba_to_argb,
        );
    }
    fn switch_from_argb_to_rgba(&mut self) {
        switch_colors_in_list(
            self.data_mut(),
            super::u32_color_casting::SwitchColorFormat::switch_from_argb_to_rgba,
        );
    }
}
