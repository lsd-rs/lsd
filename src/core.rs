use crate::color::{self, Colors};
use crate::display;
use crate::flags::{ColorOption, Display, Flags, IconOption, IconTheme, Layout, SortOrder};
use crate::icon::{self, Icons};
use crate::meta::Meta;
use crate::{print_error, print_output, sort};
use std::path::PathBuf;

pub struct Core {
    flags: Flags,
    icons: Icons,
    colors: Colors,
    sorters: Vec<(SortOrder, sort::SortFn)>,
}

impl Core {
    pub fn new(flags: Flags) -> Self {
        // termize allows us to know if the stdout is a tty or not.
        let tty_available = termize::dimensions().is_some();

        #[cfg(windows)]
        let console_color_ok = ansi_term::enable_ansi_support().is_ok();
        #[cfg(unix)]
        let console_color_ok = true;

        let mut inner_flags = flags.clone();

        let color_theme = match (tty_available && console_color_ok, flags.color.when) {
            (_, ColorOption::Never) | (false, ColorOption::Auto) => color::Theme::NoColor,
            _ => color::Theme::Default,
        };

        let icon_theme = match (tty_available, flags.icons.when, flags.icons.theme) {
            (_, IconOption::Never, _) | (false, IconOption::Auto, _) => icon::Theme::NoIcon,
            (_, _, IconTheme::Fancy) => icon::Theme::Fancy,
            (_, _, IconTheme::Unicode) => icon::Theme::Unicode,
        };

        if !tty_available {
            // The output is not a tty, this means the command is piped. (ex: lsd -l | less)
            //
            // Most of the programs does not handle correctly the ansi colors
            // or require a raw output (like the `wc` command).
            inner_flags.layout = Layout::OneLine;
        };

        let sorters = sort::assemble_sorters(&flags);

        Self {
            flags,
            //display: Display::new(inner_flags),
            colors: Colors::new(color_theme),
            icons: Icons::new(icon_theme),
            sorters,
        }
    }

    pub fn run(self, paths: &[PathBuf]) {
        let mut meta_list = Vec::with_capacity(paths.len());

        // if -R or --tree, get depth
        let depth = match self.flags.layout {
            Layout::Tree { .. } => self.flags.recursion.depth,
            _ if self.flags.recursion.enabled => self.flags.recursion.depth,
            _ => 1,
        };

        // fetch metas
        for path in paths {
            let mut meta = match Meta::from_path(&path, self.flags.dereference.0) {
                Ok(meta) => meta,
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    continue;
                }
            };

            let recurse =
                self.flags.layout == Layout::Tree || self.flags.display != Display::DirectoryOnly;
            if recurse {
                match meta.recurse_into(depth, &self.flags) {
                    Ok(content) => {
                        meta.content = content;
                        if let Some(ref mut inner) = meta.content {
                            inner.sort_unstable_by(|a, b| sort::by_meta(&self.sorters, a, b));
                        }
                        meta_list.push(meta);
                    }
                    Err(err) => {
                        print_error!("lsd: {}: {}\n", path.display(), err);
                        continue;
                    }
                };
            } else {
                meta_list.push(meta);
            };
        }

        if self.flags.total_size.0 {
            meta_list.iter_mut().for_each(Meta::calculate_total_size)
        }

        let output = if self.flags.layout == Layout::Tree {
            display::tree(&meta_list, &self.flags, &self.colors, &self.icons)
        } else {
            display::grid(&mut meta_list, &self.flags, &self.colors, &self.icons)
        };

        print_output!("{}", output);
    }
}
