use colored::Colorize;
use prettytable::{cell, row};

use std::{cmp, fs, path};

use crate::{
    fonts,
    util::{format_bool, format_option, PrettyRow},
    Error, Result,
};

pub const TABLE_NAMES: [&'static str; 32] = [
    "ankr", "avar", "cbdt", "cff", "cff2", "cmap", "feat", "fvar", "gdef", "glyf",
    "gpos", "gsub", "gvar", "head", "hhea", "hmtx", "hvar", "kern", "kerx", "maxp",
    "morx", "mvar", "name", "os2", "post", "sbix", "svg", "trak", "vhea", "vmtx", "vorg",
    "vvar",
];

pub struct FontFile {
    loc: path::PathBuf,
    collection_index: u32,
    scale: f32,

    data: Vec<u8>,
    hash: u64,
    font: Option<fontdue::Font>,
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
            font: None,
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

    pub fn to_glyphs(&self) -> Result<Vec<fonts::Glyph>> {
        let face = self.to_face()?;

        let subtables = match face.tables().cmap {
            Some(table) => table.subtables,
            None => err_at!(Invalid, msg: "missing cmap tables")?,
        };

        let mut code_points: Vec<u32> = Vec::default();
        for subtable in subtables {
            subtable.codepoints(|code_point| code_points.push(code_point));
        }
        code_points.sort();
        for (a, b) in code_points.iter().zip(code_points[1..].iter()) {
            if *a == *b {
                err_at!(Invalid, msg: "repeating code_point {}", *a)?
            }
        }

        let mut glyphs: Vec<fonts::Glyph> = Vec::default();
        for code_point in code_points.into_iter() {
            glyphs.push(fonts::Glyph::new(face.clone(), code_point)?)
        }

        Ok(glyphs)
    }

    pub fn to_unicode_blocks(&self) -> Result<Vec<unicode_blocks::UnicodeBlock>> {
        let mut ss: Vec<unicode_blocks::UnicodeBlock> = self
            .to_glyphs()?
            .iter()
            .filter_map(|g| Some(g.unicode_block()?))
            .collect();
        ss.sort();
        ss.dedup();

        Ok(ss)
    }

    pub fn parse(&mut self) -> Result<&mut Self> {
        if let None = self.font.as_ref() {
            let setts = fontdue::FontSettings {
                collection_index: self.collection_index,
                scale: self.scale,
            };
            self.font = Some(err_at!(
                Invalid,
                fontdue::Font::from_bytes(self.data.as_slice(), setts)
            )?);
        }

        Ok(self)
    }

    pub fn rasterize_char(&self, ch: char) -> Result<()> {
        use image::{ImageBuffer, ImageFormat, Luma};

        let font = match self.font.as_ref() {
            Some(font) => font,
            None => err_at!(Invalid, msg: "parse before calling raster")?,
        };
        let (metrics, data) = font.rasterize(ch, self.scale);
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = {
            let (w, h) = (metrics.width as u32, metrics.height as u32);
            ImageBuffer::from_vec(w, h, data).unwrap()
        };
        err_at!(Invalid, img.save_with_format("./xyz.bmp", ImageFormat::Bmp))?;

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
            ff: self,
            name,
            tables: self.to_table_names()?,
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

impl FontFile {
    pub fn to_table_names(&self) -> Result<Vec<&'static str>> {
        let face = self.to_face()?;
        let tables = face.tables();
        let mut ts = vec!["head", "hhea", "maxp"];

        if let Some(_) = tables.cbdt {
            ts.push("cbdt");
        }
        if let Some(_) = tables.cff {
            ts.push("cff");
        }
        if let Some(_) = tables.cmap {
            ts.push("cmap");
        }
        if let Some(_) = tables.glyf {
            ts.push("glyf");
        }
        if let Some(_) = tables.hmtx {
            ts.push("hmtx");
        }
        if let Some(_) = tables.kern {
            ts.push("kern");
        }
        if let Some(_) = tables.name {
            ts.push("name");
        }
        if let Some(_) = tables.os2 {
            ts.push("os2");
        }
        if let Some(_) = tables.post {
            ts.push("post");
        }
        if let Some(_) = tables.sbix {
            ts.push("sbix");
        }
        if let Some(_) = tables.svg {
            ts.push("svg");
        }
        if let Some(_) = tables.vhea {
            ts.push("vhea");
        }
        if let Some(_) = tables.vmtx {
            ts.push("vmtx");
        }
        if let Some(_) = tables.vorg {
            ts.push("vorg");
        }
        if let Some(_) = tables.gdef {
            ts.push("gdef");
        }
        if let Some(_) = tables.gpos {
            ts.push("gpos");
        }
        if let Some(_) = tables.gsub {
            ts.push("gsub");
        }
        if let Some(_) = tables.ankr {
            ts.push("ankr");
        }
        if let Some(_) = tables.feat {
            ts.push("feat");
        }
        if let Some(_) = tables.kerx {
            ts.push("kerx");
        }
        if let Some(_) = tables.morx {
            ts.push("morx");
        }
        if let Some(_) = tables.trak {
            ts.push("trak");
        }
        if let Some(_) = tables.avar {
            ts.push("avar");
        }
        if let Some(_) = tables.cff2 {
            ts.push("cff2");
        }
        if let Some(_) = tables.fvar {
            ts.push("fvar");
        }
        if let Some(_) = tables.gvar {
            ts.push("gvar");
        }
        if let Some(_) = tables.hvar {
            ts.push("hvar");
        }
        if let Some(_) = tables.mvar {
            ts.push("mvar");
        }
        if let Some(_) = tables.vvar {
            ts.push("vvar");
        }

        Ok(ts)
    }
}

pub struct FaceProperties<'a> {
    ff: &'a FontFile,
    pub name: Option<String>,
    pub tables: Vec<&'static str>,
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

impl<'a> Eq for FaceProperties<'a> {}

impl<'a> PartialEq for FaceProperties<'a> {
    fn eq(&self, other: &FaceProperties) -> bool {
        self.name == other.name
    }
}

impl<'a> PartialOrd for FaceProperties<'a> {
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

impl<'a> Ord for FaceProperties<'a> {
    fn cmp(&self, other: &FaceProperties) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> PrettyRow for FaceProperties<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy =>
            "Name", "N", "RIBOMV", "BB", "Em", "Ascender", "Descender",
            "Height", "LineGap", "Tables",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format_option!(self.name),
            self.glyph_count,
            format_flags(self),
            rect_to_string(&self.global_bounding_box),
            self.units_per_em,
            self.ascender,
            self.descender,
            self.height,
            self.line_gap,
            self.tables.len(),
        ]
    }
}

impl<'a> FaceProperties<'a> {
    pub fn print_property(&self, property: &str) -> Result<String> {
        let name = self
            .name
            .as_ref()
            .map(String::as_str)
            .unwrap_or("-")
            .to_string();

        let s = match property {
            "name" => name,
            "tables" => "-".to_string(),
            "glyph_count" => self.glyph_count.to_string(),
            "global_bounding_box" => rect_to_string(&self.global_bounding_box),
            "regular" => format_bool!(self.regular).to_string(),
            "italic" => format_bool!(self.italic).to_string(),
            "bold" => format_bool!(self.bold).to_string(),
            "oblique" => format_bool!(self.oblique).to_string(),
            "monospaced" => format_bool!(self.monospaced).to_string(),
            "variable" => format_bool!(self.variable).to_string(),
            "units_per_em" => self.units_per_em.to_string(),
            "x_height" => self
                .x_height
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "capital_height" => self
                .capital_height
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "underline_metrics" => self
                .underline_metrics
                .map(lmetrics_to_string)
                .unwrap_or("-".to_string()),
            "strikeout_metrics" => self
                .strikeout_metrics
                .map(lmetrics_to_string)
                .unwrap_or("-".to_string()),
            "subscript_metrics" => self
                .subscript_metrics
                .map(smetrics_to_string)
                .unwrap_or("-".to_string()),
            "superscript_metrics" => self
                .superscript_metrics
                .map(smetrics_to_string)
                .unwrap_or("-".to_string()),
            "italic_angle" => self
                .italic_angle
                .as_ref()
                .map(f32::to_string)
                .unwrap_or("-".to_string()),
            "weight" => weight_to_string(&self.weight),
            "width" => width_to_string(&self.width),
            "style" => style_to_string(&self.style),
            "ascender" => self.ascender.to_string(),
            "descender" => self.descender.to_string(),
            "height" => self.height.to_string(),
            "line_gap" => self.line_gap.to_string(),
            "vertical_ascender" => self
                .vertical_ascender
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "vertical_descender" => self
                .vertical_descender
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "vertical_height" => self
                .vertical_height
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "vertical_line_gap" => self
                .vertical_line_gap
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "typographic_ascender" => self
                .typographic_ascender
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "typographic_descender" => self
                .typographic_descender
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "typographic_line_gap" => self
                .typographic_line_gap
                .as_ref()
                .map(i16::to_string)
                .unwrap_or("-".to_string()),
            "unicode_blocks" => {
                let blocks: Vec<String> = {
                    let blocks = self.ff.to_unicode_blocks()?;
                    blocks.iter().map(|b| b.name().to_string()).collect()
                };
                blocks
                    .chunks(4)
                    .map(|r| r.join(", "))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .to_string()
            }
            _ => unreachable!(),
        };

        Ok(s)
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

fn weight_to_string(w: &ttf_parser::os2::Weight) -> String {
    use ttf_parser::os2::Weight::{
        Black, Bold, ExtraBold, ExtraLight, Light, Medium, Normal, Other, SemiBold, Thin,
    };

    match w {
        Thin => "thin".to_string(),
        ExtraLight => "extra-light".to_string(),
        Light => "light".to_string(),
        Normal => "normal".to_string(),
        Medium => "medium".to_string(),
        SemiBold => "semi-bold".to_string(),
        Bold => "bold".to_string(),
        ExtraBold => "extra-bold".to_string(),
        Black => "black".to_string(),
        Other(val) => val.to_string(),
    }
}

fn width_to_string(w: &ttf_parser::os2::Width) -> String {
    use ttf_parser::os2::Width::{
        Condensed, Expanded, ExtraCondensed, ExtraExpanded, Normal, SemiCondensed,
        SemiExpanded, UltraCondensed, UltraExpanded,
    };

    match w {
        UltraCondensed => "ultra-condensed",
        ExtraCondensed => "extra-condensed",
        Condensed => "condensed",
        SemiCondensed => "semi-condensed",
        Normal => "normal",
        SemiExpanded => "semi-expanded",
        Expanded => "expanded",
        ExtraExpanded => "extra-expanded",
        UltraExpanded => "ultra-expanded",
    }
    .to_string()
}

fn style_to_string(s: &ttf_parser::os2::Style) -> String {
    use ttf_parser::os2::Style::{Italic, Normal, Oblique};

    match s {
        Normal => "normal",
        Italic => "italic",
        Oblique => "oblique",
    }
    .to_string()
}

fn rect_to_string(rect: &ttf_parser::Rect) -> String {
    let ttf_parser::Rect {
        x_min,
        y_min,
        x_max,
        y_max,
    } = rect;
    format!("({},{})->({},{})", x_min, y_min, x_max, y_max)
}

fn lmetrics_to_string(val: ttf_parser::LineMetrics) -> String {
    let ttf_parser::LineMetrics {
        position,
        thickness,
    } = val;
    format!("pos:{},thick:{}", position, thickness)
}

fn smetrics_to_string(val: ttf_parser::ScriptMetrics) -> String {
    let ttf_parser::ScriptMetrics {
        x_size,
        y_size,
        x_offset,
        y_offset,
    } = val;
    format!(
        "xoff:{},yoff:{}/x:{},y:{}",
        x_offset, y_offset, x_size, y_size,
    )
}
