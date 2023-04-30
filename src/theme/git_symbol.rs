use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct GitSymbolTheme {
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

impl Default for GitSymbolTheme {
    fn default() -> GitSymbolTheme {
        GitSymbolTheme {
            default: " ".into(),
            unmodified: "_".into(), // "\u{f00c}
            new_in_index: "\u{f067}".into(),
            new_in_workdir: "?".into(),
            deleted: "\u{f014}".into(), // or f06
            modified: "\u{f8ea}".into(),
            renamed: "\u{f8ea}".into(),
            ignored: "I".into(),
            typechange: "\u{f0ec}".into(),
            conflicted: "\u{f071}".into(),
        }
    }
}

impl GitSymbolTheme {
    pub fn unicode() -> Self {
        GitSymbolTheme {
            default: "-".into(),
            unmodified: "-".into(),
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
