use crate::color::Colors;
use crate::display;
use crate::flags::{
    ColorOption, Display, Flags, HyperlinkOption, Layout, Literal, SortOrder, ThemeOption,
};
use crate::git::GitCache;
use crate::icon::Icons;

use crate::meta::Meta;
use crate::{print_error, print_output, sort, ExitCode};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::io;
#[cfg(not(target_os = "windows"))]
use std::os::unix::io::AsRawFd;

use crate::flags::blocks::Block;
use crate::git_theme::GitTheme;
#[cfg(target_os = "windows")]
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    colors: Colors,
    git_theme: GitTheme,
    sorters: Vec<(SortOrder, sort::SortFn)>,
}

impl Core {
    pub fn new(mut flags: Flags) -> Self {
        // Check through libc if stdout is a tty. Unix specific so not on windows.
        // Determine color output availability (and initialize color output (for Windows 10))
        #[cfg(not(target_os = "windows"))]
        let tty_available = unsafe { libc::isatty(io::stdout().as_raw_fd()) == 1 };

        #[cfg(not(target_os = "windows"))]
        let console_color_ok = true;

        #[cfg(target_os = "windows")]
        let tty_available = terminal_size().is_some(); // terminal_size allows us to know if the stdout is a tty or not.

        #[cfg(target_os = "windows")]
        let console_color_ok = crossterm::ansi_support::supports_ansi();

        let color_theme = match (tty_available && console_color_ok, flags.color.when) {
            (_, ColorOption::Never) | (false, ColorOption::Auto) => ThemeOption::NoColor,
            _ => flags.color.theme.clone(),
        };

        let icon_when = flags.icons.when;
        let icon_theme = flags.icons.theme.clone();

        // TODO: Rework this so that flags passed downstream does not
        // have Auto option for any (icon, color, hyperlink).
        if matches!(flags.hyperlink, HyperlinkOption::Auto) {
            flags.hyperlink = if tty_available {
                HyperlinkOption::Always
            } else {
                HyperlinkOption::Never
            }
        }

        let icon_separator = flags.icons.separator.0.clone();

        // The output is not a tty, this means the command is piped. e.g.
        //
        // lsd -l | less
        //
        // Most of the programs does not handle correctly the ansi colors
        // or require a raw output (like the `wc` command).
        if !tty_available {
            // we should not overwrite the tree layout
            if flags.layout != Layout::Tree {
                flags.layout = Layout::OneLine;
            }

            flags.literal = Literal(true);
        };

        let sorters = sort::assemble_sorters(&flags);

        Self {
            flags,
            colors: Colors::new(color_theme),
            icons: Icons::new(tty_available, icon_when, icon_theme, icon_separator),
            git_theme: GitTheme::new(),
            sorters,
        }
    }

    pub fn run(self, paths: Vec<PathBuf>) -> ExitCode {
        let (mut meta_list, exit_code) = self.fetch(paths);

        self.sort(&mut meta_list);
        self.display(&meta_list);
        exit_code
    }

    fn fetch(&self, paths: Vec<PathBuf>) -> (Vec<Meta>, ExitCode) {
        let mut exit_code = ExitCode::OK;
        let mut meta_list = Vec::with_capacity(paths.len());
        let depth = match self.flags.layout {
            Layout::Tree { .. } => self.flags.recursion.depth,
            _ if self.flags.recursion.enabled => self.flags.recursion.depth,
            _ => 1,
        };

        #[cfg(target_os = "windows")]
        use crate::config_file;
        #[cfg(target_os = "windows")]
        let paths: Vec<PathBuf> = paths
            .into_iter()
            .filter_map(config_file::expand_home)
            .collect();

        for path in paths {
            let mut meta =
                match Meta::from_path(&path, self.flags.dereference.0, self.flags.permission) {
                    Ok(meta) => meta,
                    Err(err) => {
                        print_error!("{}: {}.", path.display(), err);
                        exit_code.set_if_greater(ExitCode::MajorIssue);
                        continue;
                    }
                };

            let cache = if self.flags.blocks.0.contains(&Block::GitStatus) {
                Some(GitCache::new(&path))
            } else {
                None
            };

            let recurse =
                self.flags.layout == Layout::Tree || self.flags.display != Display::DirectoryOnly;
            if recurse {
                match meta.recurse_into(depth, &self.flags, cache.as_ref()) {
                    Ok((content, path_exit_code)) => {
                        meta.content = content;
                        meta.git_status = cache.and_then(|cache| cache.get(&meta.path, true));
                        meta_list.push(meta);
                        exit_code.set_if_greater(path_exit_code);
                    }
                    Err(err) => {
                        print_error!("lsd: {}: {}\n", path.display(), err);
                        exit_code.set_if_greater(ExitCode::MinorIssue);
                        continue;
                    }
                };
            } else {
                meta.git_status = cache.and_then(|cache| cache.get(&meta.path, true));
                meta_list.push(meta);
            };
        }
        // Only calculate the total size of a directory if it will be displayed
        if self.flags.total_size.0 && self.flags.blocks.displays_size() {
            for meta in &mut meta_list.iter_mut() {
                meta.calculate_total_size();
            }
        }

        (meta_list, exit_code)
    }

    fn sort(&self, metas: &mut Vec<Meta>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(&self.sorters, a, b));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                self.sort(content);
            }
        }
    }

    fn display(&self, metas: &[Meta]) {
        let output = if self.flags.layout == Layout::Tree {
            display::tree(
                metas,
                &self.flags,
                &self.colors,
                &self.icons,
                &self.git_theme,
            )
        } else {
            display::grid(
                metas,
                &self.flags,
                &self.colors,
                &self.icons,
                &self.git_theme,
            )
        };

        print_output!("{}", output);
    }
}
