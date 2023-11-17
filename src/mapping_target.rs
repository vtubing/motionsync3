use crate::MappingParameter;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MappingTarget {
  id: String,
  targets: Vec<MappingParameter>,
  #[serde(rename = "Type")]
  type_: String,
}
