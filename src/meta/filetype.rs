use color::{Colors, Elem};
use std::fs::Metadata;
use std::os::unix::fs::FileTypeExt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FileType {
    BlockDevice,
    CharDevice,
    Directory,
    File,
    SymLink,
    Pipe,
    Socket,
    Special,
}

impl<'a> From<&'a Metadata> for FileType {
    fn from(meta: &'a Metadata) -> Self {
        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File
        } else if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_fifo() {
            FileType::Pipe
        } else if file_type.is_symlink() {
            FileType::SymLink
        } else if file_type.is_char_device() {
            FileType::CharDevice
        } else if file_type.is_block_device() {
            FileType::BlockDevice
        } else if file_type.is_socket() {
            FileType::Socket
        } else {
            FileType::Special
        }
    }
}

impl FileType {
    pub fn render(self) -> String {
        match self {
            FileType::File => Colors[&Elem::File].paint(".").to_string(),
            FileType::Directory => Colors[&Elem::Dir].paint("d").to_string(),
            FileType::Pipe => Colors[&Elem::Pipe].paint("|").to_string(),
            FileType::SymLink => Colors[&Elem::SymLink].paint("l").to_string(),
            FileType::BlockDevice => Colors[&Elem::BlockDevice].paint("b").to_string(),
            FileType::CharDevice => Colors[&Elem::CharDevice].paint("c").to_string(),
            FileType::Socket => Colors[&Elem::Socket].paint("s").to_string(),
            FileType::Special => Colors[&Elem::Special].paint("?").to_string(),
        }
    }
}
