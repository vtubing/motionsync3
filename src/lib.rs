#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MotionSync3 {
  pub meta: Meta,
  pub settings: Vec<Settings>,
  pub version: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Meta {
  pub dictionary: Vec<Dictionary>,
  pub setting_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct PostProcessing {
  pub blend_ratio: f64,
  pub sample_rate: f64,
  pub smoothing: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Dictionary {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct CubismParameter {
  pub damper: f64,
  pub id: String,
  pub max: f64,
  pub min: f64,
  pub name: String,
  pub smooth: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct AudioParameter {
  pub enabled: bool,
  pub id: String,
  pub max: f64,
  pub min: f64,
  pub name: String,
  pub scale: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MappingParameter {
  id: String,
  value: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MappingTarget {
  id: String,
  targets: Vec<MappingParameter>,
  #[serde(rename = "Type")]
  type_: String,
}
