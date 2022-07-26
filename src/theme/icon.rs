use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct IconTheme {}

impl Default for IconTheme {
    fn default() -> Self {
        // TODO(zwpaper): check terminal color and return light or dark
        IconTheme{}
    }
}
