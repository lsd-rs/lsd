use color::{Colors, Elem};
use icon;
use meta::Meta;

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {}
    }

    pub fn format_name(&self, meta: &Meta) -> String {
        let mut content = String::new();

        let color = if meta.metadata.is_dir() {
            Colors[&Elem::Dir]
        } else {
            Colors[&Elem::File]
        };

        let mut name = meta.name.clone();
        if meta.metadata.is_dir() {
            name.push('/');
        }

        content = content + icon::from_meta(&meta) + "  " + &name;
        content = color.paint(content).to_string();

        content
    }
}
