use crate::color::{self, Colors};
use crate::display;
use crate::flags::{Flags, IconTheme, Layout, WhenFlag};
use crate::icon::{self, Icons};
use crate::meta::Meta;
use crate::sort;
use std::fs;
use std::path::PathBuf;
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    //display: Display,
    colors: Colors,
}

impl Core {
    pub fn new(flags: Flags) -> Self {
        // terminal_size allows us to know if the stdout is a tty or not.
        let tty_available = terminal_size().is_some();

        let mut inner_flags = flags;

        let color_theme = match (tty_available, flags.color) {
            (_, WhenFlag::Never) | (false, WhenFlag::Auto) => color::Theme::NoColor,
            _ => color::Theme::Default,
        };

        let icon_theme = match (tty_available, flags.icon, flags.icon_theme) {
            (_, WhenFlag::Never, _) | (false, WhenFlag::Auto, _) => icon::Theme::NoIcon,
            (_, _, IconTheme::Fancy) => icon::Theme::Fancy,
            (_, _, IconTheme::Unicode) => icon::Theme::Unicode,
        };

        if !tty_available {
            // The output is not a tty, this means the command is piped. (ex: lsd -l | less)
            //
            // Most of the programs does not handle correctly the ansi colors
            // or require a raw output (like the `wc` command).
            inner_flags.layout = Layout::OneLine { long: false };
        };

        Self {
            flags,
            //display: Display::new(inner_flags),
            colors: Colors::new(color_theme),
            icons: Icons::new(icon_theme),
        }
    }

    pub fn run(self, paths: Vec<PathBuf>) {
        let mut meta_list = self.fetch(paths);

        self.sort(&mut meta_list);

        self.display(meta_list)
    }

    fn fetch(&self, paths: Vec<PathBuf>) -> Vec<Meta> {
        let mut meta_list = Vec::with_capacity(paths.len());

        let depth = if self.flags.recursive || self.flags.layout == Layout::Tree {
            self.flags.recursion_depth
        } else {
            1
        };

        for path in paths {
            let absolute_path = match fs::canonicalize(&path) {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("cannot access '{}': {}", path.display(), err);
                    continue;
                }
            };

            match Meta::from_path_recursive(
                &fs::canonicalize(&absolute_path.to_path_buf()).unwrap(),
                depth,
                self.flags.display,
            ) {
                Ok(meta) => meta_list.push(meta),
                Err(err) => eprintln!("cannot access '{}': {}", path.display(), err),
            };
        }

        meta_list
    }

    fn sort(&self, metas: &mut Vec<Meta>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(a, b, self.flags));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                self.sort(content);
            }
        }
    }

    fn display(&self, metas: Vec<Meta>) {
        let output = match self.flags.layout {
            Layout::OneLine { .. } => {
                display::one_line(metas, self.flags, &self.colors, &self.icons)
            }
            Layout::Tree => display::tree(metas, self.flags, &self.colors, &self.icons),
            Layout::Grid => display::grid(metas, self.flags, &self.colors, &self.icons),
        };
        print!("{}", output);
    }
}
