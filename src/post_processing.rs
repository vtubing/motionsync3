#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct PostProcessing {
  pub blend_ratio: f64,
  pub sample_rate: f64,
  pub smoothing: f64,
}
