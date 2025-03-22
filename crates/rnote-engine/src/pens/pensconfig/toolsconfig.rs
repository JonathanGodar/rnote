// Imports
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
)]
#[serde(rename = "tool_style")]
pub enum ToolStyle {
    #[serde(rename = "verticalspace")]
    VerticalSpace,
    #[serde(rename = "offsetcamera")]
    OffsetCamera,
    #[serde(rename = "zoom")]
    Zoom,
}

impl Default for ToolStyle {
    fn default() -> Self {
        Self::VerticalSpace
    }
}

impl TryFrom<u32> for ToolStyle {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u32(value).ok_or_else(|| {
            anyhow::anyhow!("ToolStyle try_from::<u32>() for value {} failed", value)
        })
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename = "tools_config")]
pub struct ToolsConfig {
    #[serde(rename = "style")]
    pub style: ToolStyle,

    vertical_space_tool_region_style: VerticalSpaceToolRegionStyle,
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct HorizontalExtent {
    x0: f64,
    x1: f64,
}

impl HorizontalExtent {
    fn new(x0: f64, x1: f64) -> Self {
        Self { x0, x1 }
    }

    fn clamp(&self, other: &Self) -> Option<Self> {
        let x0 = self.x0.max(other.x0);
        let x1 = self.x1.min(other.x1);

        if x0 > x1 {
            return None;
        }

        Some(Self::new(x0, x1))
    }
}

impl Into<(f64, f64)> for HorizontalExtent {
    fn into(self) -> (f64, f64) {
        (self.x0, self.x1)
    }
}
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub(crate) enum VerticalSpaceToolRegionStyle {
    Full,
    Page,
    Custom(HorizontalExtent),
}

impl Default for VerticalSpaceToolRegionStyle {
    fn default() -> Self {
        Self::Full
    }
}
