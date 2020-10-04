//! This module defines the [Sorting] options. To set it up from [ArgMatches], a [Yaml]
//! and its [Default] value, use the [configure_from](Sorting::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

type ColumnWithOrder = (SortColumn, SortOrder);

/// A collection of flags on how to sort the output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sorting {
    pub columns: Vec<ColumnWithOrder>,
}

impl Sorting {
    /// Get a `Sorting` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [SortColumn], [SortOrder] and [DirGrouping] are configured with their respective
    /// [Configurable] implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let column = SortColumn::configure_from(matches, config);
        let order = SortOrder::configure_from(matches, config);
        let dir_grouping = DirGrouping::configure_from(matches, config);

        Self {
            columns: Self::columns_from_config(column, order, dir_grouping),
        }
    }

    fn columns_from_config(
        column: SortColumn,
        order: SortOrder,
        dir_grouping: DirGrouping,
    ) -> Vec<ColumnWithOrder> {
        let mut columns = vec![];

        match dir_grouping {
            DirGrouping::None => {}
            DirGrouping::First => columns.push((SortColumn::Directory, SortOrder::Default)),
            DirGrouping::Last => columns.push((SortColumn::Directory, SortOrder::Reverse)),
        };

        columns.push((column, order));
        columns
    }
}

impl Default for Sorting {
    fn default() -> Self {
        let column = SortColumn::default();
        let order = SortOrder::default();
        let dir_grouping = DirGrouping::default();

        Self {
            columns: Self::columns_from_config(column, order, dir_grouping),
        }
    }
}

/// The flag showing which column to use for sorting.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Extension,
    Name,
    Time,
    Size,
    Version,
    Directory,
}

impl Configurable<Self> for SortColumn {
    /// Get a potential `SortColumn` variant from [ArgMatches].
    ///
    /// If either the "timesort" or "sizesort" arguments are passed, this returns the corresponding
    /// `SortColumn` variant in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        let sort = matches.value_of("sort");
        if matches.is_present("timesort") || sort == Some("time") {
            Some(Self::Time)
        } else if matches.is_present("sizesort") || sort == Some("size") {
            Some(Self::Size)
        } else if matches.is_present("extensionsort") || sort == Some("extension") {
            Some(Self::Extension)
        } else if matches.is_present("versionsort") || sort == Some("version") {
            Some(Self::Version)
        } else {
            None
        }
    }

    /// Get a potential `SortColumn` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [String](Yaml::String) value pointed to by "sorting" ->
    /// "column" and it is one of "time", "size" or "name", this returns the corresponding variant
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["sorting"]["column"] {
                Yaml::BadValue => None,
                Yaml::String(value) => match value.as_ref() {
                    "extension" => Some(Self::Extension),
                    "name" => Some(Self::Name),
                    "time" => Some(Self::Time),
                    "size" => Some(Self::Size),
                    "version" => Some(Self::Version),
                    _ => {
                        config.print_invalid_value_warning("sorting->column", &value);
                        None
                    }
                },
                _ => {
                    config.print_wrong_type_warning("sorting->column", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `SortColumn` is [SortColumn::Name].
impl Default for SortColumn {
    fn default() -> Self {
        Self::Name
    }
}

/// The flag showing which sort order to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Default,
    Reverse,
}

impl Configurable<Self> for SortOrder {
    /// Get a potential `SortOrder` variant from [ArgMatches].
    ///
    /// If the "reverse" argument is passed, this returns [SortOrder::Reverse] in a [Some].
    /// Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("reverse") {
            Some(Self::Reverse)
        } else {
            None
        }
    }

    /// Get a potential `SortOrder` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [Boolean](Yaml::Boolean) value pointed to by "sorting" ->
    /// "reverse", this returns a mapped variant in a [Some]. Otherwise [None] is returned. A
    /// `true` maps to [SortOrder::Reverse] and a `false` maps to [SortOrder::Default].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["sorting"]["reverse"] {
                Yaml::BadValue => None,
                Yaml::Boolean(value) => {
                    if *value {
                        Some(Self::Reverse)
                    } else {
                        Some(Self::Default)
                    }
                }
                _ => {
                    config.print_wrong_type_warning("sorting->reverse", "boolean");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `SortOrder` is [SortOrder::Default].
impl Default for SortOrder {
    fn default() -> Self {
        Self::Default
    }
}

/// The flag showing where to place directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DirGrouping {
    None,
    First,
    Last,
}

impl Configurable<Self> for DirGrouping {
    /// Get a potential `DirGrouping` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [DirGrouping::None] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::None)
        } else if matches.occurrences_of("group-dirs") > 0 {
            match matches.value_of("group-dirs") {
                Some("first") => Some(Self::First),
                Some("last") => Some(Self::Last),
                Some("none") => Some(Self::None),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `DirGrouping` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [Boolean](Yaml::Boolean) value pointed to by "classic"
    /// and its value is `true`, then this returns the the [DirGrouping::None] variant in a [Some].
    /// Otherwise if the Yaml contains a [String](Yaml::String) value pointed to by "sorting" ->
    /// "dir-grouping" and it is one of "first", "last" or "none", this returns its corresponding
    /// variant in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            if let Yaml::Boolean(true) = &yaml["classic"] {
                Some(Self::None)
            } else {
                match &yaml["sorting"]["dir-grouping"] {
                    Yaml::BadValue => None,
                    Yaml::String(value) => match value.as_ref() {
                        "first" => Some(Self::First),
                        "last" => Some(Self::Last),
                        "none" => Some(Self::None),
                        _ => {
                            config.print_invalid_value_warning("sorting->dir-grouping", &value);
                            None
                        }
                    },
                    _ => {
                        config.print_wrong_type_warning("sorting->dir-grouping", "string");
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `DirGrouping` is [DirGrouping::None].
impl Default for DirGrouping {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod test_sort_column {
    use super::SortColumn;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SortColumn::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_extension() {
        let argv = vec!["lsd", "--extensionsort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Extension),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_time() {
        let argv = vec!["lsd", "--timesort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_size() {
        let argv = vec!["lsd", "--sizesort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Size),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_version() {
        let argv = vec!["lsd", "--versionsort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Version),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_sort() {
        let argv = vec!["lsd", "--sort", "time"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Size),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "extension"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Extension),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "version"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Version),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_multi_sort_use_last() {
        let argv = vec!["lsd", "--sort", "size", "-t", "-S", "-X", "--sort", "time"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SortColumn::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, SortColumn::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_invalid() {
        let yaml_string = "sorting:\n  column: foo";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, SortColumn::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_extension() {
        let yaml_string = "sorting:\n  column: extension";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortColumn::Extension),
            SortColumn::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_name() {
        let yaml_string = "sorting:\n  column: name";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortColumn::Name),
            SortColumn::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_time() {
        let yaml_string = "sorting:\n  column: time";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_size() {
        let yaml_string = "sorting:\n  column: size";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortColumn::Size),
            SortColumn::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_version() {
        let yaml_string = "sorting:\n  column: version";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortColumn::Version),
            SortColumn::from_config(&Config::with_yaml(yaml))
        );
    }
}

#[cfg(test)]
mod test_sort_order {
    use super::SortOrder;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SortOrder::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_reverse() {
        let argv = vec!["lsd", "--reverse"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortOrder::Reverse),
            SortOrder::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SortOrder::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, SortOrder::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_default() {
        let yaml_string = "sorting:\n  reverse: false";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortOrder::Default),
            SortOrder::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_reverse() {
        let yaml_string = "sorting:\n  reverse: true";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SortOrder::Reverse),
            SortOrder::from_config(&Config::with_yaml(yaml))
        );
    }
}

#[cfg(test)]
mod test_dir_grouping {
    use super::DirGrouping;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, DirGrouping::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_first() {
        let argv = vec!["lsd", "--group-dirs", "first"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::First),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_last() {
        let argv = vec!["lsd", "--group-dirs", "last"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::Last),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_explicit_none() {
        let argv = vec!["lsd", "--group-dirs", "none"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--group-dirs", "first", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, DirGrouping::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, DirGrouping::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_first() {
        let yaml_string = "sorting:\n  dir-grouping: first";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DirGrouping::First),
            DirGrouping::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_last() {
        let yaml_string = "sorting:\n  dir-grouping: last";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DirGrouping::Last),
            DirGrouping::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_explicit_none() {
        let yaml_string = "sorting:\n  dir-grouping: none";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let yaml_string = "classic: true\nsorting:\n  dir-grouping: first";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_config(&Config::with_yaml(yaml))
        );
    }
}
