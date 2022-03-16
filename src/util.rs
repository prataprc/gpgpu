use serde::de::DeserializeOwned;

use std::{fs, path};

use crate::{Error, Result};

pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

pub fn make_table<R>(rows: &[R]) -> prettytable::Table
where
    R: PrettyRow,
{
    let mut table = prettytable::Table::new();

    match rows.len() {
        0 => table,
        _ => {
            table.set_titles(R::to_head());
            rows.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(R::to_format());
            table
        }
    }
}

/// Load toml file and parse it into type `T`.
pub fn load_toml<P, T>(loc: P) -> Result<T>
where
    P: AsRef<path::Path>,
    T: DeserializeOwned,
{
    use std::str::from_utf8;

    let ploc: &path::Path = loc.as_ref();
    let data = err_at!(IOError, fs::read(ploc))?;
    let s = err_at!(FailConvert, from_utf8(&data), "not utf8 for {:?}", ploc)?;
    err_at!(FailConvert, toml::from_str(s), "file:{:?}", ploc)
}
