use colored::Colorize;
use log::warn;
use prettytable::{cell, row};

use std::{fmt, result};

use crate::{
    fonts,
    util::{format_option, PrettyRow},
    Error, GlyphRect, Result,
};

#[derive(Clone)]
pub struct Glyph<'a> {
    face: ttf_parser::Face<'a>,
    code_point: u32,
    ch: char,
    id: ttf_parser::GlyphId,
    name: String,
    // metrics
    units_per_em: f32, // in points
    hor_advance: f32,
    ver_advance: f32,
    bb: GlyphRect,
}

impl<'a> Glyph<'a> {
    pub fn new(face: ttf_parser::Face<'a>, code_point: u32) -> Result<Glyph<'a>> {
        let ch = match char::from_u32(code_point) {
            Some(ch) => ch,
            None => err_at!(Invalid, msg: "no char for code_point {}", code_point)?,
        };
        let id = face.glyph_index(ch).unwrap_or(ttf_parser::GlyphId(0));
        let name = face.glyph_name(id).unwrap_or("--").to_string();
        let bb = face.glyph_bounding_box(id).unwrap();
        let units_per_em = face.units_per_em() as f32;
        let hor_advance = face.glyph_hor_side_bearing(id).unwrap() as f32;
        let ver_advance = face.glyph_hor_side_bearing(id).unwrap() as f32;

        let val = Glyph {
            face,
            code_point,
            ch,
            id,
            name,
            // metrics
            units_per_em,
            hor_advance,
            ver_advance,
            bb: bb.into(),
        };

        Ok(val)
    }

    pub fn scale(&self, units_per_em: f32) -> Glyph {
        let factor = self.units_per_em / units_per_em;
        Glyph {
            face: self.face.clone(),
            code_point: self.code_point,
            ch: self.ch,
            id: self.id,
            name: self.name.clone(),
            // metrics
            units_per_em: units_per_em,
            hor_advance: self.hor_advance * factor,
            ver_advance: self.ver_advance * factor,
            bb: self.bb.scale(factor),
        }
    }
}

impl<'a> Glyph<'a> {
    pub fn to_code_point(&self) -> u32 {
        self.code_point
    }

    pub fn to_char(&self) -> char {
        self.ch
    }

    pub fn to_id(&self) -> ttf_parser::GlyphId {
        self.id
    }

    pub fn to_name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> Glyph<'a> {
    pub fn unicode_block(&self) -> Option<unicode_blocks::UnicodeBlock> {
        unicode_blocks::find_unicode_block(self.ch)
    }

    pub fn cjk(&self) -> bool {
        unicode_blocks::is_cjk(self.ch)
    }

    pub fn bounding_box(&self) -> GlyphRect {
        self.bb
    }

    pub fn hor_advance(&self) -> f32 {
        self.hor_advance
    }

    pub fn ver_advance(&self) -> f32 {
        self.ver_advance
    }

    pub fn hor_side_bearing(&self) -> Option<i16> {
        self.face.glyph_hor_side_bearing(self.id)
    }

    pub fn ver_side_bearing(&self) -> Option<i16> {
        self.face.glyph_ver_side_bearing(self.id)
    }

    pub fn y_origin(&self) -> Option<i16> {
        self.face.glyph_y_origin(self.id)
    }

    pub fn to_outline(&self) -> Option<fonts::Outline> {
        let mut outline = fonts::Outline::default();
        self.face.outline_glyph(self.id, &mut outline)?;
        Some(outline)
    }

    pub fn check_limits(&self) -> bool {
        match self.bounding_box() {
            bb if bb.x_min >= bb.x_max => {
                warn!(
                    "Bounding box for {} is x_min:{} x_max:{}",
                    self.code_point, bb.x_min, bb.x_max
                );
                false
            }
            bb if bb.y_min >= bb.y_max => {
                warn!(
                    "Bounding box for {} is y_min:{} y_max:{}",
                    self.code_point, bb.y_min, bb.y_max
                );
                false
            }
            _ => true,
        }
    }
}

impl<'a> PrettyRow for Glyph<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy =>
            "Char", "Codepoint", "Name", "Unicode-Block", "CJK", "HAdv", "VAdv",
            "HSB", "VSB", "YORG", "BB",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self.ch),
            self.code_point,
            self.name,
            format_option!(self.unicode_block().as_ref().map(|x| x.name())),
            self.cjk(),
            self.hor_advance(),
            self.ver_advance(),
            format_option!(self.hor_side_bearing()),
            format_option!(self.ver_side_bearing()),
            format_option!(self.y_origin()),
            format!("{:?}", self.bounding_box()),
        ]
    }
}

pub struct Outline {
    segments: Vec<Segment>,
}

enum Segment {
    Move(f32, f32),                           // (x, y)
    Line(f32, f32),                           // (x, y)
    Quad((f32, f32), (f32, f32)),             // (x1, y1), (x, y)
    Curv((f32, f32), (f32, f32), (f32, f32)), // (x1, y1), (x2, y2), (x, y)
}

impl Segment {
    fn scale(&self, factor: f32) -> Segment {
        match self {
            Segment::Move(a, b) => Segment::Move(a * factor, b * factor),
            Segment::Line(a, b) => Segment::Line(a * factor, b * factor),
            Segment::Quad((a, b), (x, y)) => {
                Segment::Quad((a * factor, b * factor), (x * factor, y * factor))
            }
            Segment::Curv((a, b), (x, y), (p, q)) => Segment::Curv(
                (a * factor, b * factor),
                (x * factor, y * factor),
                (p * factor, q * factor),
            ),
        }
    }
}

impl Default for Outline {
    fn default() -> Outline {
        Outline {
            segments: Vec::default(),
        }
    }
}

impl ttf_parser::OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.segments.push(Segment::Move(x, y))
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.segments.push(Segment::Line(x, y))
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.segments.push(Segment::Quad((x1, y1), (x, y)))
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.segments
            .push(Segment::Curv((x1, y1), (x2, y2), (x, y)))
    }

    fn close(&mut self) {
        let _start = match self.segments.first() {
            Some(Segment::Move(x, y)) => (x, y),
            Some(Segment::Line(_, _)) => panic!("first segment is Segment::Line"),
            Some(Segment::Quad((_, _), (_, _))) => {
                panic!("first segment is Segment::Quad")
            }
            Some(Segment::Curv((_, _), (_, _), (_, _))) => {
                panic!("first segment is Segment::Curv")
            }
            None => panic!("empty glyph"),
        };
        let _end = match self.segments.last() {
            Some(Segment::Move(x, y)) => (x, y),
            Some(Segment::Line(x, y)) => (x, y),
            Some(Segment::Quad((_, _), (x, y))) => (x, y),
            Some(Segment::Curv((_, _), (_, _), (x, y))) => (x, y),
            None => panic!("empty glyph"),
        };

        // assert_eq!(start, end)
    }
}

impl fmt::Display for Outline {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        for seg in self.segments.iter() {
            match seg {
                Segment::Move(x, y) => {
                    writeln!(f, "Move                  P0:{:-6},{:6}", x, y)?
                }
                Segment::Line(x, y) => {
                    writeln!(f, "Line                  P1:{:-6},{:6}", x, y)?
                }
                Segment::Quad((x1, y1), (x, y)) => {
                    writeln!(f, "Quad C1:{:-6},{:6} P1:{:-6},{:6}", x1, y1, x, y)?
                }
                Segment::Curv((x1, y1), (x2, y2), (x, y)) => writeln!(
                    f,
                    "Curv C1:{:-6},{:6} C2:{:-6},{:6} P1:{:-6},{:6}",
                    x1, y1, x2, y2, x, y
                )?,
            }
        }

        Ok(())
    }
}

impl Outline {
    pub fn scale(&self, factor: f32) -> Outline {
        Outline {
            segments: self.segments.iter().map(|s| s.scale(factor)).collect(),
        }
    }
}
