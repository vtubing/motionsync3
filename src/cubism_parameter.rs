#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
