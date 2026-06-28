//! A library all about manipulating and playing with colors
//!
//! Related:
//! - `mirl_render`: Draws shapes on Buffers (images)
// #![feature(derive_const)]
// #![feature(const_clone)]
// #![feature(const_cmp)]
// #![feature(const_default)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)] // Specialization is not implemented and as such not dangerous
#![feature(specialization)]
// #![feature(const_index)]
#![feature(core_float_math)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_result_unwrap_unchecked)]
#![feature(const_destruct)]
#![feature(const_clone)]
#![cfg_attr(feature = "image", feature(core_intrinsics))]

/// Basic u32 manipulation
pub mod u32_color_casting;

/// Commonly used items
pub mod prelude;

#[cfg(feature = "binary_data_plugin")]
/// Color support for [`BinaryData`](mirl_collection::prelude::BinaryData)
pub mod binary_data_color_plugin;

/// Traits for `mirl_buffer::Buffer`
pub mod buffer_traits;

// /// Interpolate colors
// pub mod interpolation;
/// Stuff that has not yet been sorted
pub mod misc;

/// Color presets
pub mod colors;

#[cfg(feature = "image")]
/// Support for the `image` crate
pub mod imagery;
