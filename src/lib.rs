mod audio_parameter;
mod cubism_parameter;
mod dictionary;
mod mapping_parameter;
mod mapping_target;
mod meta;
mod post_processing;
mod settings;

pub use audio_parameter::AudioParameter;
pub use cubism_parameter::CubismParameter;
pub use dictionary::Dictionary;
pub use mapping_parameter::MappingParameter;
pub use mapping_target::MappingTarget;
pub use meta::Meta;
pub use post_processing::PostProcessing;
pub use settings::Settings;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MotionSync3 {
  pub meta: Meta,
  pub settings: Vec<Settings>,
  pub version: u8,
}
