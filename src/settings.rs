use crate::{AudioParameter, CubismParameter, MappingTarget, PostProcessing};

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Settings {
  pub analysis_type: String,
  pub audio_parameters: Vec<AudioParameter>,
  pub cubism_parameters: Vec<CubismParameter>,
  pub id: String,
  pub mappings: Vec<MappingTarget>,
  pub post_processing: PostProcessing,
  pub use_case: String,
}
