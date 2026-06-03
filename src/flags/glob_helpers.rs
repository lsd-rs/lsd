use clap::Error;
use clap::error::ErrorKind;
use globset::{Glob, GlobSet, GlobSetBuilder};

pub fn create_glob(pattern: &str) -> Result<Glob, Error> {
    Glob::new(pattern).map_err(|err| Error::raw(ErrorKind::ValueValidation, err))
}

pub fn create_glob_set(builder: &GlobSetBuilder) -> Result<GlobSet, Error> {
    builder
        .build()
        .map_err(|err| Error::raw(ErrorKind::ValueValidation, err))
}
