//! This module defines the [Layout] flag. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing which output layout to print.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Layout {
    Grid,
    Tree,
    OneLine,
}

impl Configurable<Layout> for Layout {
    /// Get a potential `Layout` variant from [ArgMatches].
    ///
    /// If any of the "tree", "long" or "oneline" arguments is passed, this returns the
    /// corresponding `Layout` variant in a [Some]. Otherwise if the number of passed "blocks"
    /// arguments is greater than 1, this also returns the [OneLine](Layout::OneLine) variant.
    /// Finally if neither of them is passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("tree") {
            Some(Self::Tree)
        } else if matches.is_present("long")
            || matches.is_present("oneline")
            || matches.is_present("inode")
            || matches!(matches.values_of("blocks"), Some(values) if values.len() > 1)
        // TODO: handle this differently
        {
            Some(Self::OneLine)
        } else {
            None
        }
    }

    /// Get a potential Layout variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [String](Yaml::String) value pointed to by "layout" and
    /// it is either "tree", "oneline" or "grid", this returns the corresponding `Layout` variant
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["layout"] {
                Yaml::BadValue => None,
                Yaml::String(value) => match value.as_ref() {
                    "tree" => Some(Self::Tree),
                    "oneline" => Some(Self::OneLine),
                    "grid" => Some(Self::Grid),
                    _ => {
                        config.print_invalid_value_warning("layout", &value);
                        None
                    }
                },
                _ => {
                    config.print_wrong_type_warning("layout", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `Layout` is [Layout::Grid].
impl Default for Layout {
    fn default() -> Self {
        Self::Grid
    }
}

#[cfg(test)]
mod test {
    use super::Layout;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_tree() {
        let argv = vec!["lsd", "--tree"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::Tree), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline() {
        let argv = vec!["lsd", "--oneline"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_long() {
        let argv = vec!["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_blocks() {
        let argv = vec!["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Layout::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Layout::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_tree() {
        let yaml_string = "layout: tree";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Layout::Tree),
            Layout::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_oneline() {
        let yaml_string = "layout: oneline";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Layout::OneLine),
            Layout::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_grid() {
        let yaml_string = "layout: grid";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Layout::Grid),
            Layout::from_config(&Config::with_yaml(yaml))
        );
    }
}
