use serde::de::DeserializeOwned;

use std::{fmt, fs, path};

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

pub trait PrettyPrint {
    fn print(&self);
}

impl<T> PrettyPrint for cgmath::Vector4<T>
where
    T: fmt::Display,
{
    fn print(&self) {
        println!("{:.4} {:.4} {:.4} {:.4} ", self.x, self.y, self.z, self.w);
    }
}

impl<T> PrettyPrint for cgmath::Matrix4<T>
where
    T: fmt::Display,
{
    fn print(&self) {
        println!(
            "{:.4} {:.4} {:.4} {:.4}",
            self.x.x, self.y.x, self.z.x, self.w.x
        );
        println!(
            "{:.4} {:.4} {:.4} {:.4}",
            self.x.y, self.y.y, self.z.y, self.w.y
        );
        println!(
            "{:.4} {:.4} {:.4} {:.4}",
            self.x.z, self.y.z, self.z.z, self.w.z
        );
        println!(
            "{:.4} {:.4} {:.4} {:.4}",
            self.x.w, self.y.w, self.z.w, self.w.w
        );
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

pub fn html_to_color(s: &str) -> Result<wgpu::Color> {
    // println!("{}", s);
    let c = tint::Color::from_hex(s);
    // println!("{:?}", c);
    let val = wgpu::Color {
        r: c.red,
        g: c.green,
        b: c.blue,
        a: c.alpha,
    };
    Ok(val)
}
