use std::ops::Deref;

use gdnative::{
    core_types::{FromVariant, FromVariantError, Variant, VariantType},
    prelude::*,
};
use opencv::core::Vec2f;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImgParams {
    pub inbetweens: usize,
    pub loop_seamlessly: bool,
    pub flow_multiplier: f32,
    #[serde(flatten)]
    pub optflow_alg: FlowAlg,
    pub show_motion_vectors: bool,
}

impl FromVariant for ImgParams {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict =
            variant
                .try_to_dictionary()
                .ok_or_else(|| FromVariantError::InvalidVariantType {
                    variant_type: variant.get_type(),
                    expected: VariantType::Dictionary,
                })?;
        let json = dict.to_json().to_string();
        serde_json::from_str(&json).map_err(|e| FromVariantError::Custom(e.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "optflow_alg")] //Internally tagged, see https://serde.rs/enum-representations.html
pub enum FlowAlg {
    SimpleFlow {
        layers: usize,
        averaging_block_size: usize,
        max_flow: usize,
    },
    DenseRLOF {
        //See https://docs.opencv.org/master/d2/d84/group__optflow.html#ga0fc536362c239654322d9015f6efcecd
        forward_backward_threshold: f32,
        grid_step_x: i32,
        grid_step_y: i32,
        use_post_proc: bool,
        use_variational_refinement: bool,
    },
    Invalid,
}

//Needed because can't derive Default on enums
impl Default for FlowAlg {
    fn default() -> Self {
        FlowAlg::Invalid
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SpritesheetGenerationParams {
    pub frames_per_row: usize,
}

impl FromVariant for SpritesheetGenerationParams {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict =
            variant
                .try_to_dictionary()
                .ok_or_else(|| FromVariantError::InvalidVariantType {
                    variant_type: variant.get_type(),
                    expected: VariantType::Dictionary,
                })?;
        let json = dict.to_json().to_string();
        serde_json::from_str(&json).map_err(|e| FromVariantError::Custom(e.to_string()))
    }
}

//Bitmapflow's Vector2 type
pub struct BVector2(Vector2);

//Needed because https://stackoverflow.com/a/25415289
impl From<Vec2f> for BVector2 {
    fn from(v: Vec2f) -> Self {
        BVector2(Vector2::new(v[0], v[1]))
    }
}

//This allows you to call euclid::Vector2 methods directly on BVector2
impl Deref for BVector2 {
    type Target = Vector2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
