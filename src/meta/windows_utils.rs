use std::io;
use std::path::PathBuf;
use super::{Owner, Permissions};

pub fn get_file_data(path: &PathBuf) -> Result<(Owner, Permissions), io::Error> {
    unimplemented!()
}
