mod backend;
mod spinlock;

pub use backend::{backend, backend_to_string};
pub use spinlock::Spinlock;

use log::error;
use serde::de::DeserializeOwned;

use std::{fmt, fs, path, str::FromStr, time};

use crate::{Error, Result};
macro_rules! format_bool {
    ($val:expr) => {
        if $val {
            "✓".green()
        } else {
            "✗".red()
        }
    };
}
pub(crate) use format_bool;

macro_rules! format_option {
    ($val:expr) => {
        match $val.as_ref() {
            Some(val) => val.to_string().white(),
            None => "✗".red(),
        }
    };

    ($val:expr, $default:expr) => {
        match $val.as_ref() {
            Some(val) => val.to_string(),
            None => $default.to_string(),
        }
    };
}
pub(crate) use format_option;

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

pub fn parse_csv<T>(txt: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    let items: Vec<Result<T>> = txt
        .split(",")
        .map(|a| err_at!(FailConvert, a.parse()))
        .collect();
    let mut outs = vec![];
    for item in items.into_iter() {
        outs.push(item?)
    }

    Ok(outs)
}

pub fn gpgpu_dir() -> Option<path::PathBuf> {
    let home = dirs::home_dir()?;
    Some([home, ".gpgpu".into()].iter().collect())
}

pub fn gpgpu_cache_dir() -> Option<path::PathBuf> {
    let dir = gpgpu_dir()?;
    Some([dir, "cache".into()].iter().collect())
}

pub fn gpgpu_cached_file(file: &str) -> Option<path::PathBuf> {
    let dir = gpgpu_cache_dir()?;
    Some([dir, file.into()].iter().collect())
}

pub enum WalkRes {
    Ok,
    SkipDir,
}

/// Breadth first directory walking.
///
/// `callb` arguments:
///
/// * _state_, as mutable reference, user supplied and exist for the duration of walk.
/// * _parent_, path to parent under which this entry is found.
/// * _dir_entry_, for each entry in a sub-directory.
/// * _depth_, depth level at which _dir-entry_ is located, start with ZERO.
/// * _breath_, index of _dir-entry_ as stored in its parent directory, start with ZERO.
pub fn walk<P, S, F>(root: P, mut state: S, mut callb: F) -> Result<S>
where
    P: AsRef<path::Path>,
    F: FnMut(&mut S, &path::Path, &fs::DirEntry, usize, usize) -> Result<WalkRes>,
{
    let depth = 0;
    do_walk(root, &mut state, &mut callb, depth)?;
    Ok(state)
}

fn do_walk<P, S, F>(parent: P, state: &mut S, callb: &mut F, depth: usize) -> Result<()>
where
    P: AsRef<path::Path>,
    F: FnMut(&mut S, &path::Path, &fs::DirEntry, usize, usize) -> Result<WalkRes>,
{
    let mut subdirs = vec![];

    let parent = {
        let parent: &path::Path = parent.as_ref();
        parent.to_path_buf()
    };
    let dirs = err_at!(IOError, fs::read_dir(&parent), "read_dir({:?})", parent)?;
    for (breath, entry) in dirs.enumerate() {
        let entry = err_at!(IOError, entry)?;
        match callb(state, &parent, &entry, depth, breath)? {
            WalkRes::Ok if err_at!(IOError, entry.file_type())?.is_dir() => {
                subdirs.push(entry)
            }
            WalkRes::Ok | WalkRes::SkipDir => (),
        }
    }

    for subdir in subdirs.into_iter() {
        match do_walk(subdir.path(), state, callb, depth + 1) {
            Err(err) => error!("failed walking dir {:?}: {}", subdir, err),
            Ok(_) => (),
        }
    }

    Ok(())
}

pub struct FrameRate {
    next_frame: time::Instant,
    start_time: time::Instant,
    n_frames: u64,
}

impl FrameRate {
    pub fn new() -> FrameRate {
        let now = time::Instant::now();
        FrameRate {
            next_frame: now,
            start_time: now,
            n_frames: 0,
        }
    }

    pub fn is_redraw(&self) -> bool {
        time::Instant::now() > self.next_frame
    }

    pub fn next_frame_after(&mut self, micros: u64) {
        self.next_frame = time::Instant::now() + time::Duration::from_micros(micros);
        self.n_frames += 1;
    }

    pub fn total(&self) -> u64 {
        self.n_frames
    }

    pub fn rate(&self) -> u64 {
        match self.start_time.elapsed().as_secs() {
            secs if secs > 0 && self.n_frames > 0 => self.n_frames / secs,
            _ => 0,
        }
    }
}
