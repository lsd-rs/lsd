use ansi_term::ANSIString;
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
    pub fn render(self) -> ANSIString<'static> {
        match self {
            FileType::File => Colors[&Elem::File].paint("."),
            FileType::Directory => Colors[&Elem::Dir].paint("d"),
            FileType::Pipe => Colors[&Elem::Pipe].paint("|"),
            FileType::SymLink => Colors[&Elem::SymLink].paint("l"),
            FileType::BlockDevice => Colors[&Elem::BlockDevice].paint("b"),
            FileType::CharDevice => Colors[&Elem::CharDevice].paint("c"),
            FileType::Socket => Colors[&Elem::Socket].paint("s"),
            FileType::Special => Colors[&Elem::Special].paint("?"),
        }
    }
}
