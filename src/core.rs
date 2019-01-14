use batch::Batch;
use color::{self, Colors};
use display::Display;
use flags::{Flags, ThemeFlag, WhenFlag};
use icon::{self, Icons};
use meta::{FileType, Meta};
use std::path::{Path, PathBuf};
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    display: Display,
    colors: Colors,
}

impl Core {
    pub fn new(flags: Flags) -> Self {
        // terminal_size allows us to know if the stdout is a tty or not.
        let tty_available = terminal_size().is_some();

        let mut inner_flags = flags;

        let color_theme = match (tty_available, flags.color) {
            (true, WhenFlag::Never) => color::Theme::NoColor,
            (false, WhenFlag::Auto) => color::Theme::NoColor,
            (false, WhenFlag::Always) => color::Theme::Default,
            _ => color::Theme::Default,
        };

        let icon_theme = match (tty_available, flags.icon, flags.icon_theme) {
            (_, WhenFlag::Never, _) | (false, WhenFlag::Auto, _) => icon::Theme::NoIcon,
            (_, _, ThemeFlag::Default) => icon::Theme::Default,
            (_, _, ThemeFlag::Unicode) => icon::Theme::Unicode,
        };

        if !tty_available {
            // The output is not a tty, this means the command is piped. (ex: lsd -l | less)
            //
            // Most of the programs does not handle correctly the ansi colors
            // or require a raw output (like the `wc` command).
            inner_flags.display_online = true;
        };

        Self {
            flags,
            display: Display::new(inner_flags),
            colors: Colors::new(color_theme),
            icons: Icons::new(icon_theme),
        }
    }

    pub fn run(self, paths: Vec<PathBuf>) {
        self.run_inner(paths, 0);
    }

    fn run_inner(&self, paths: Vec<PathBuf>, depth: usize) {
        if depth > self.flags.recursion_depth {
            return;
        }

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for path in paths {
            if path.is_dir() {
                dirs.push(path);
            } else if let Some(meta) = Meta::from_path(&path) {
                files.push(meta);
            }
        }

        let print_folder_name: bool = dirs.len() + files.len() > 1;

        if !files.is_empty() && !self.flags.display_tree {
            let mut file_batch = Batch::from(files);
            file_batch.sort(self.flags);
            self.display
                .print_outputs(self.get_batch_outputs(&file_batch));
        }

        dirs.sort_unstable();

        for dir in dirs {
            if let Some(folder_batch) = self.list_folder_content(dir.as_path()) {
                if (print_folder_name || self.flags.recursive) && !self.flags.display_tree {
                    println!("\n{}:", dir.display())
                }

                if self.flags.display_tree {
                    self.display_as_tree(folder_batch, depth);
                } else if self.flags.recursive {
                    self.display
                        .print_outputs(self.get_batch_outputs(&folder_batch));

                    let folder_dirs = folder_batch
                        .into_iter()
                        .filter_map(|x| {
                            if x.file_type == FileType::Directory {
                                Some(x.path)
                            } else {
                                None
                            }
                        })
                        .collect();

                    self.run_inner(folder_dirs, depth + 1);
                } else {
                    self.display
                        .print_outputs(self.get_batch_outputs(&folder_batch));
                }
            }
        }
    }

    pub fn display_as_tree(&self, batch: Batch, depth: usize) {
        let last_idx = batch.len();

        for (idx, elem) in batch.into_iter().enumerate() {
            let last = idx + 1 != last_idx;

            if elem.file_type == FileType::Directory {
                self.display.print_tree_row(
                    &elem.name.render(&self.colors, &self.icons),
                    depth,
                    last,
                );
                self.run_inner(vec![elem.path], depth + 1);
            } else {
                self.display.print_tree_row(
                    &elem.name.render(&self.colors, &self.icons),
                    depth,
                    last,
                );
            }
        }
    }

    pub fn get_batch_outputs<'b>(&self, batch: &'b Batch) -> Vec<String> {
        if self.flags.display_long {
            batch.get_long_output(&self.colors, &self.icons, self.flags)
        } else {
            batch.get_short_output(&self.colors, &self.icons, self.flags)
        }
    }

    pub fn list_folder_content(&self, folder: &Path) -> Option<Batch> {
        let mut metas: Vec<Meta> = Vec::new();

        let dir = match folder.read_dir() {
            Ok(dir) => dir,
            Err(err) => {
                println!("cannot open directory'{}': {}", folder.display(), err);
                return None;
            }
        };

        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(meta) = Meta::from_path(&entry.path()) {
                    if !meta.name.is_hidden() || self.flags.display_all {
                        metas.push(meta);
                    }
                }
            }
        }

        let mut batch = Batch::from(metas);
        batch.sort(self.flags);

        Some(batch)
    }
}
