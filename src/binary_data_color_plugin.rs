use mirl_collections::file_data::{
    BinaryData, GenericDataType,
    plugins::{BinaryDataPlugin, BinaryDataPluginTrait},
};

use crate::u32_color_casting::{
    ColorManipulationWithoutInput, PackChannelsIntoColor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The plugin for handling plain text
pub struct ColorBinaryDataPlugin;

/// The static position for [`ColorBinaryDataPlugin`] as to allow for dyn
pub static COLOR_U32_BINARY_DATA_PLUGIN: ColorBinaryDataPlugin =
    ColorBinaryDataPlugin;

impl BinaryDataPluginTrait<GenericDataType> for ColorBinaryDataPlugin {
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &BinaryData<GenericDataType>,
    ) -> bool {
        if !matches!(bin.data_type, GenericDataType::Text) {
            return false;
        }
        let c = bin.to_color().map(|x| {
            format!("RGB(R: {}, G: {}, B: {})", x.red(), x.green(), x.blue())
        });
        f.field(
            "raw_data@Color",
            &c.unwrap_or_else(|| {
                format!("Data is not valid color data: {:?}", bin.as_bytes())
            }),
        );

        true
    }
}
/// Convert color data into [`BinaryData`]
pub const trait ColorBinaryDataPluginTrait {
    #[must_use]
    /// Constructor to load data from raw bytes
    ///
    /// If RGB are the same, compresses data into single byte
    fn from_color_rgb(data: u32) -> Self;
    #[must_use]
    /// Constructor to load data from raw bytes
    fn from_color_rgba(data: u32) -> Self;

    /// Get the data as a color:
    /// - One byte: Gray scale is assumed
    /// - Two bytes: Gray scale with alpha assumed
    /// - Three bytes: RGB assumed
    /// - Four bytes: RGBA assumed
    #[must_use]
    fn to_color(&self) -> Option<u32>;
}

impl ColorBinaryDataPluginTrait for BinaryData<GenericDataType> {
    fn from_color_rgb(data: u32) -> Self {
        let r = data.red();
        let g = data.green();
        let b = data.blue();
        if r == g && g == b {
            Self::from_bytes(
                data.to_le_bytes()[1..1].to_vec(),
                GenericDataType::Color,
            )
        } else {
            Self::from_bytes(
                data.to_le_bytes()[1..2].to_vec(),
                GenericDataType::Color,
            )
        }
    }
    fn from_color_rgba(data: u32) -> Self {
        Self::from_bytes(data.to_le_bytes().to_vec(), GenericDataType::Color)
    }
    fn to_color(&self) -> Option<u32> {
        Some(match self.raw_data.len() {
            1 => (self.raw_data[0], self.raw_data[0], self.raw_data[0])
                .pack_rgba_u32(),
            2 => (
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[1],
            )
                .pack_rgba_u32(),
            3 => (self.raw_data[0], self.raw_data[1], self.raw_data[2])
                .pack_rgba_u32(),
            4 => (
                self.raw_data[0],
                self.raw_data[1],
                self.raw_data[2],
                self.raw_data[3],
            )
                .pack_rgba_u32(),
            _ => return None,
        })
    }
}

inventory::submit! {
    BinaryDataPlugin {
        name: "color_u32",
        plugin: &COLOR_U32_BINARY_DATA_PLUGIN,
    }
}
