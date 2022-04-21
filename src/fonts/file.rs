use colored::Colorize;
use prettytable::{cell, row};

use std::{cmp, fs, path};

use crate::{
    util::{format_bool, PrettyRow},
    Error, Result,
};

pub struct FontFile {
    loc: path::PathBuf,
    collection_index: u32,
    scale: f32,

    data: Vec<u8>,
    hash: u64,
}

impl Eq for FontFile {}

impl PartialEq for FontFile {
    fn eq(&self, other: &FontFile) -> bool {
        self.hash == other.hash
    }
}

impl PartialOrd for FontFile {
    fn partial_cmp(&self, other: &FontFile) -> Option<cmp::Ordering> {
        self.loc.file_name()?.partial_cmp(other.loc.file_name()?)
    }
}

impl Ord for FontFile {
    fn cmp(&self, other: &FontFile) -> cmp::Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => cmp::Ordering::Equal,
        }
    }
}

impl FontFile {
    pub fn new<P>(loc: P, index: u32, scale: f32) -> Result<FontFile>
    where
        P: AsRef<path::Path>,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let loc = {
            let p: &path::Path = loc.as_ref();
            path::PathBuf::from(p)
        };
        let data = err_at!(IOError, fs::read(&loc))?;
        let hash = {
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        };

        let val = FontFile {
            loc,
            collection_index: index,
            scale,

            data,
            hash,
        };

        Ok(val)
    }

    pub fn to_loc(&self) -> Option<path::PathBuf> {
        Some(self.loc.clone())
    }

    pub fn to_face(&self) -> Result<ttf_parser::Face> {
        err_at!(Invalid, ttf_parser::Face::from_slice(&self.data, 0))
    }

    pub fn to_hash(&self) -> u64 {
        self.hash
    }

    pub fn to_raster_pnm<P>(&self, _subdir: P) -> Result<()>
    where
        P: AsRef<path::Path>,
    {
        let _raster = {
            let setts = fontdue::FontSettings {
                collection_index: self.collection_index,
                scale: self.scale,
            };
            err_at!(
                Invalid,
                fontdue::Font::from_bytes(self.data.as_slice(), setts)
            )?
        };

        //let subdir: path::PathBuf = {
        //    let p: &path::Path = subdir.as_ref();
        //    p.into()
        //};

        //err_at!(IOError, fs::create_dir_all(&subdir))?;
        //for (_ch, index) in self.font.chars().iter() {
        //    let (_metrics, data) =
        //        self.font.rasterize_indexed(index.get(), self.setts.scale);
        //    let loc: path::PathBuf = [subdir.clone(), format!("{}.bmp", index).into()]
        //        .iter()
        //        .collect();
        //    println!("rasterizing {:?}", loc);
        //    err_at!(IOError, fs::write(&loc, &data))?;
        //}

        Ok(())
    }

    pub fn to_face_properties(&self) -> Result<FaceProperties> {
        let face = self.to_face()?;
        let name = self
            .loc
            .file_name()
            .map(|s| s.to_str())
            .flatten()
            .map(|x| x.to_string());

        let val = FaceProperties {
            name,
            glyph_count: face.number_of_glyphs(),
            global_bounding_box: face.global_bounding_box(),
            regular: face.is_regular(),
            italic: face.is_italic(),
            bold: face.is_bold(),
            oblique: face.is_oblique(),
            monospaced: face.is_monospaced(),
            variable: face.is_variable(),
            units_per_em: face.units_per_em(),
            x_height: face.x_height(),
            capital_height: face.capital_height(),
            underline_metrics: face.underline_metrics(),
            strikeout_metrics: face.strikeout_metrics(),
            subscript_metrics: face.subscript_metrics(),
            superscript_metrics: face.superscript_metrics(),
            italic_angle: face.italic_angle(),
            weight: face.weight(),
            width: face.width(),
            style: face.style(),
            ascender: face.ascender(),
            descender: face.descender(),
            height: face.height(),
            line_gap: face.line_gap(),
            vertical_ascender: face.vertical_ascender(),
            vertical_descender: face.vertical_descender(),
            vertical_height: face.vertical_height(),
            vertical_line_gap: face.vertical_line_gap(),
            typographic_ascender: face.typographic_ascender(),
            typographic_descender: face.typographic_descender(),
            typographic_line_gap: face.typographic_line_gap(),
        };

        Ok(val)
    }
}

pub struct FaceProperties {
    pub name: Option<String>,
    pub glyph_count: u16,
    pub global_bounding_box: ttf_parser::Rect,
    pub regular: bool,
    pub italic: bool,
    pub bold: bool,
    pub oblique: bool,
    pub monospaced: bool,
    pub variable: bool,
    pub units_per_em: u16,
    pub x_height: Option<i16>,
    pub capital_height: Option<i16>,
    pub underline_metrics: Option<ttf_parser::LineMetrics>,
    pub strikeout_metrics: Option<ttf_parser::LineMetrics>,
    pub subscript_metrics: Option<ttf_parser::ScriptMetrics>,
    pub superscript_metrics: Option<ttf_parser::ScriptMetrics>,
    pub italic_angle: Option<f32>,
    pub weight: ttf_parser::os2::Weight,
    pub width: ttf_parser::os2::Width,
    pub style: ttf_parser::os2::Style,
    pub ascender: i16,
    pub descender: i16,
    pub height: i16,
    pub line_gap: i16,
    pub vertical_ascender: Option<i16>,
    pub vertical_descender: Option<i16>,
    pub vertical_height: Option<i16>,
    pub vertical_line_gap: Option<i16>,
    pub typographic_ascender: Option<i16>,
    pub typographic_descender: Option<i16>,
    pub typographic_line_gap: Option<i16>,
}

impl Eq for FaceProperties {}

impl PartialEq for FaceProperties {
    fn eq(&self, other: &FaceProperties) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for FaceProperties {
    fn partial_cmp(&self, other: &FaceProperties) -> Option<cmp::Ordering> {
        match self.name.as_ref() {
            None => Some(cmp::Ordering::Less),
            Some(name) => match other.name.as_ref() {
                None => Some(cmp::Ordering::Greater),
                Some(other) => name.partial_cmp(other),
            },
        }
    }
}

impl Ord for FaceProperties {
    fn cmp(&self, other: &FaceProperties) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PrettyRow for FaceProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![ Fy => "Name", "N", "RIBOMV" ]
    }

    fn to_row(&self) -> prettytable::Row {
        let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("-");
        row![name, self.glyph_count, format_flags(self)]
    }
}

fn format_flags(p: &FaceProperties) -> String {
    format_bool!(p.regular).to_string()
        + &format_bool!(p.italic).to_string()
        + &format_bool!(p.bold).to_string()
        + &format_bool!(p.oblique).to_string()
        + &format_bool!(p.monospaced).to_string()
        + &format_bool!(p.variable).to_string()
}
