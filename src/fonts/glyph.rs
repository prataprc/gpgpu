use colored::Colorize;
use log::warn;
use prettytable::{cell, row};

use std::{fmt, result};

use crate::{
    fonts,
    util::{format_option, PrettyRow},
};

pub struct Glyph<'a> {
    face: ttf_parser::Face<'a>,
    subtable: ttf_parser::cmap::Subtable<'a>,
    pub id: ttf_parser::GlyphId,
    pub name: String,
    pub codepoint: u32,
    pub ch: char,
}

impl<'a> Glyph<'a> {
    pub fn new(
        face: ttf_parser::Face<'a>,
        codepoint: u32,
        subtable: ttf_parser::cmap::Subtable<'a>,
    ) -> Option<Glyph<'a>> {
        let ch = char::from_u32(codepoint)?;
        let id = face.glyph_index(ch)?;
        let name = face.glyph_name(id)?.to_string();
        let val = Glyph {
            face,
            subtable,
            id,
            name,
            codepoint,
            ch,
        };

        Some(val)
    }
    pub fn unicode_block(&self) -> Option<unicode_blocks::UnicodeBlock> {
        unicode_blocks::find_unicode_block(self.ch)
    }

    pub fn cjk(&self) -> bool {
        unicode_blocks::is_cjk(self.ch)
    }

    pub fn bounding_box(&self) -> Option<ttf_parser::Rect> {
        self.face.glyph_bounding_box(self.id)
    }

    pub fn hor_advance(&self) -> Option<u16> {
        self.face.glyph_hor_advance(self.id)
    }

    pub fn ver_advance(&self) -> Option<u16> {
        self.face.glyph_ver_advance(self.id)
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
            Some(bb) if bb.x_min >= bb.x_max => {
                warn!(
                    "Bounding box for {} is x_min:{} x_max:{}",
                    self.codepoint, bb.x_min, bb.x_max
                );
                false
            }
            Some(bb) if bb.y_min >= bb.y_max => {
                warn!(
                    "Bounding box for {} is y_min:{} y_max:{}",
                    self.codepoint, bb.y_min, bb.y_max
                );
                false
            }
            Some(_) | None => true,
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
            "Char", "Codepoint", "Block", "CJK", "HAdv", "VAdv", "HSB", "VSB", "YORG",
            "BB", "Platform"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        let bb = self.bounding_box().as_ref().map(|bb| rect_to_string(bb));
        row![
            self.ch.to_string(),
            self.codepoint,
            format_option!(self.unicode_block().as_ref().map(|x| x.name())),
            self.cjk(),
            format_option!(self.hor_advance()),
            format_option!(self.ver_advance()),
            format_option!(self.hor_side_bearing()),
            format_option!(self.ver_side_bearing()),
            format_option!(self.y_origin()),
            format_option!(bb),
            format!("{:?}", self.subtable.platform_id),
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

fn rect_to_string(rect: &ttf_parser::Rect) -> String {
    let ttf_parser::Rect {
        x_min,
        y_min,
        x_max,
        y_max,
    } = rect;
    format!("({},{})->({},{})", x_min, y_min, x_max, y_max)
}
