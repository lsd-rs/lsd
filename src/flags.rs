pub mod blocks;
pub mod color;
pub mod date;
pub mod dereference;
pub mod display;
pub mod git_theme;
pub mod header;
pub mod hyperlink;
pub mod icons;
pub mod ignore_globs;
pub mod indicators;
pub mod layout;
pub mod permission;
pub mod recursion;
pub mod size;
pub mod sorting;
pub mod symlink_arrow;
pub mod symlinks;
pub mod total_size;

pub use blocks::Blocks;
pub use color::Color;
pub use color::{ColorOption, ThemeOption};
pub use date::DateFlag;
pub use dereference::Dereference;
pub use display::Display;
pub use git_theme::GitTheme;
pub use header::Header;
pub use hyperlink::HyperlinkOption;
pub use icons::IconOption;
pub use icons::IconSeparator;
pub use icons::IconTheme;
pub use icons::Icons;
pub use ignore_globs::IgnoreGlobs;
pub use indicators::Indicators;
pub use layout::Layout;
pub use permission::PermissionFlag;
pub use recursion::Recursion;
pub use size::SizeFlag;
pub use sorting::DirGrouping;
pub use sorting::SortColumn;
pub use sorting::SortOrder;
pub use sorting::Sorting;
pub use symlink_arrow::SymlinkArrow;
pub use symlinks::NoSymlink;
pub use total_size::TotalSize;

use crate::app::Cli;
use crate::config_file::Config;

use clap::Error;

#[cfg(doc)]
use yaml_rust::Yaml;

/// A struct to hold all set configuration flags for the application.
#[derive(Clone, Debug, Default)]
pub struct Flags {
    pub blocks: Blocks,
    pub color: Color,
    pub date: DateFlag,
    pub dereference: Dereference,
    pub display: Display,
    pub display_indicators: Indicators,
    pub icons: Icons,
    pub ignore_globs: IgnoreGlobs,
    pub layout: Layout,
    pub no_symlink: NoSymlink,
    pub recursion: Recursion,
    pub size: SizeFlag,
    pub permission: PermissionFlag,
    pub sorting: Sorting,
    pub total_size: TotalSize,
    pub symlink_arrow: SymlinkArrow,
    pub hyperlink: HyperlinkOption,
    pub header: Header,
    pub git_theme: GitTheme,
    pub should_quote: bool,
}

impl Flags {
    /// Set up the `Flags` from either [Cli], a [Config] or its [Default] value.
    ///
    /// # Errors
    ///
    /// This can return an [Error], when either the building of the ignore globs or the parsing of
    /// the recursion depth parameter fails.
    pub fn configure_from(cli: &Cli, config: &Config) -> Result<Self, Error> {
        Ok(Self {
            blocks: Blocks::configure_from(cli, config),
            color: Color::configure_from(cli, config),
            date: DateFlag::configure_from(cli, config),
            dereference: Dereference::configure_from(cli, config),
            display: Display::configure_from(cli, config),
            layout: Layout::configure_from(cli, config),
            size: SizeFlag::configure_from(cli, config),
            permission: PermissionFlag::configure_from(cli, config),
            display_indicators: Indicators::configure_from(cli, config),
            icons: Icons::configure_from(cli, config),
            ignore_globs: IgnoreGlobs::configure_from(cli, config)?,
            no_symlink: NoSymlink::configure_from(cli, config),
            recursion: Recursion::configure_from(cli, config),
            sorting: Sorting::configure_from(cli, config),
            total_size: TotalSize::configure_from(cli, config),
            symlink_arrow: SymlinkArrow::configure_from(cli, config),
            hyperlink: HyperlinkOption::configure_from(cli, config),
            header: Header::configure_from(cli, config),
            git_theme: GitTheme::configure_from(cli, config),
            should_quote: true,
        })
    }
}

/// A trait to allow a type to be configured by either command line parameters, a configuration
/// file or a [Default] value.
pub trait Configurable<T>
where
    T: std::default::Default,
{
    /// Returns a value from either [Cli], a [Config], a [Default] or the environment value.
    /// The first value that is not [None] is used. The order of precedence for the value used is:
    /// - [from_cli](Configurable::from_cli)
    /// - [from_environment](Configurable::from_environment)
    /// - [from_config](Configurable::from_config)
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// The configuration file's Yaml is read in any case, to be able to check for errors and print
    /// out warnings.
    fn configure_from(cli: &Cli, config: &Config) -> T {
        if let Some(value) = Self::from_cli(cli) {
            return value;
        }

        if let Some(value) = Self::from_environment() {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Default::default()
    }

    /// The method to implement the value fetching from command line parameters.
    fn from_cli(cli: &Cli) -> Option<T>;

    /// The method to implement the value fetching from a configuration file. This should return
    /// [None], if the [Config] does not have a [Yaml].
    fn from_config(config: &Config) -> Option<T>;

    /// The method to implement the value fetching from environment variables.
    fn from_environment() -> Option<T> {
        None
    }
}
