use formatter::*;
use path_lister::Path;
use Options;

pub struct Presenter<'a> {
    formatter: Formatter,
    options: &'a Options,
}

impl<'a> Presenter<'a> {
    pub fn new(options: &'a Options) -> Presenter<'a> {
        Presenter {
            options: options,
            formatter: Formatter::new(),
        }
    }

    pub fn print(&self, paths: Vec<Path>) {
        if self.options.display_long {
            self.print_long(&paths);
            return;
        }

        self.print_simple(&paths)
    }

    fn print_long(&self, paths: &Vec<Path>) {
        for path in paths {
            print!(
                "  {} {} {}\n",
                self.formatter.format_permissions(path),
                self.formatter.format_date(path),
                self.formatter
                    .format_path(path.path.file_name().unwrap().to_str().unwrap(), &path)
            );
        }
    }

    fn print_simple(&self, paths: &Vec<Path>) {
        for path in paths {
            print!(
                "{}\n",
                self.formatter
                    .format_path(path.path.file_name().unwrap().to_str().unwrap(), &path)
            );
        }
    }
}
