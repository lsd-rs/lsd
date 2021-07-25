use crate::color::{self, Colors};
use crate::display;
use crate::flags::{ColorOption, Display, Flags, IconOption, IconTheme, Layout, SortOrder};
use crate::icon::{self, Icons};
use crate::meta::Meta;

use crate::{print_output, sort, ExitCode, ExitStatus, PathError};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::io;
#[cfg(not(target_os = "windows"))]
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "windows")]
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    //display: Display,
    colors: Colors,
    sorters: Vec<(SortOrder, sort::SortFn)>,
    exit_status: ExitStatus,
}

impl Core {
    pub fn new(flags: Flags) -> Self {
        // Check through libc if stdout is a tty. Unix specific so not on windows.
        // Determine color output availability (and initialize color output (for Windows 10))
        #[cfg(not(target_os = "windows"))]
        let tty_available = unsafe { libc::isatty(io::stdout().as_raw_fd()) == 1 };

        #[cfg(not(target_os = "windows"))]
        let console_color_ok = true;

        #[cfg(target_os = "windows")]
        let tty_available = terminal_size().is_some(); // terminal_size allows us to know if the stdout is a tty or not.

        #[cfg(target_os = "windows")]
        let console_color_ok = ansi_term::enable_ansi_support().is_ok();

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

        let icon_separator = flags.icons.separator.0.clone();

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
            icons: Icons::new(icon_theme, icon_separator),
            exit_status: ExitStatus::new(),
            sorters,
        }
    }

    pub fn run(&mut self, paths: Vec<PathBuf>) {
        let mut meta_list = self.fetch(paths);

        self.sort(&mut meta_list);
        self.display(&meta_list);
    }

    pub fn exit(&self) {
        let errors = &self.exit_status.errors;
        if !errors.is_empty() {
            for error in errors {
                println!("{}", error);
            }
        }

        let exit_code = self.exit_status.code as i32;
        std::process::exit(exit_code);
    }

    fn fetch(&mut self, paths: Vec<PathBuf>) -> Vec<Meta> {
        let mut meta_list = Vec::with_capacity(paths.len());
        let depth = match self.flags.layout {
            Layout::Tree { .. } => self.flags.recursion.depth,
            _ if self.flags.recursion.enabled => self.flags.recursion.depth,
            _ => 1,
        };

        for path in paths {
            let mut meta = match Meta::from_path(&path, self.flags.dereference.0) {
                Ok(meta) => meta,
                Err(err) => {
                    let path_error = PathError::new(path, err);
                    self.exit_status
                        .push_error(ExitCode::MajorIssue, path_error);
                    continue;
                }
            };

            let recurse =
                self.flags.layout == Layout::Tree || self.flags.display != Display::DirectoryOnly;
            if recurse {
                match meta.recurse_into(depth, &self.flags) {
                    Ok(content) => {
                        meta.content = content;
                        meta_list.push(meta);
                    }
                    Err(err) => {
                        let path_error = PathError::new(path, err);
                        self.exit_status
                            .push_error(ExitCode::MinorIssue, path_error);
                        continue;
                    }
                };
            } else {
                meta_list.push(meta);
            };
        }
        if self.flags.total_size.0 {
            for meta in &mut meta_list.iter_mut() {
                meta.calculate_total_size();
            }
        };

        meta_list
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
            display::tree(&metas, &self.flags, &self.colors, &self.icons)
        } else {
            display::grid(&metas, &self.flags, &self.colors, &self.icons)
        };

        print_output!("{}", output);
    }
}
