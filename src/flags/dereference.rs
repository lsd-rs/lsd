//! This module defines the [Dereference] flag. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing whether to dereference symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Dereference(pub bool);

impl Configurable<Self> for Dereference {
    /// Get a potential `Dereference` value from [ArgMatches].
    ///
    /// If the "dereference" argument is passed, this returns a `Dereference` with value `true` in
    /// a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("dereference") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Dereference` value from a [Config].
    ///
    /// If the Config's [Yaml] contains the [Boolean](Yaml::Boolean) value pointed to by
    /// "dereference", this returns its value as the value of the `Dereference`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["dereference"] {
                Yaml::BadValue => None,
                Yaml::Boolean(value) => Some(Self(*value)),
                _ => {
                    config.print_wrong_type_warning("dereference", "boolean");
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Dereference;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Dereference::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--dereference"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Dereference(true)),
            Dereference::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Dereference::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, Dereference::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_true() {
        let yaml_string = "dereference: true";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Dereference(true)),
            Dereference::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_false() {
        let yaml_string = "dereference: false";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Dereference(false)),
            Dereference::from_config(&Config::with_yaml(yaml))
        );
    }
}
