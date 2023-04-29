use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct GitThemeSymbols {
    pub default: String,
    pub unmodified: String,
    pub new_in_index: String,
    pub new_in_workdir: String,
    pub deleted: String,
    pub modified: String,
    pub renamed: String,
    pub ignored: String,
    pub typechange: String,
    pub conflicted: String,
}

impl Default for GitThemeSymbols {
    fn default() -> GitThemeSymbols {
        GitThemeSymbols {
            default: "-".into(),
            unmodified: ".".into(),
            new_in_index: "N".into(),
            new_in_workdir: "?".into(),
            deleted: "D".into(),
            modified: "M".into(),
            renamed: "R".into(),
            ignored: "I".into(),
            typechange: "T".into(),
            conflicted: "C".into(),
        }
    }
}
