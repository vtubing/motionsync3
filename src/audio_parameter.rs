#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
