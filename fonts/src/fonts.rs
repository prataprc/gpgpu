use prettytable::{cell, row};
use ttf_parser::{Face, GlyphId, LineMetrics, Rect, ScriptMetrics};

use vgi::pp::PrettyRow;

// TODO: "kerning-subtable"

macro_rules! face_params {
    ($face:ident, $($name:expr,)*) => (
        vec![
            $(
                FaceParam {
                    name: $name.to_string(),
                    face: $face,
                },
            )*
        ]
    );
}

pub struct FaceParam<'a> {
    name: String,
    face: &'a Face<'a>,
}

pub fn list_face_params<'a>(face: &'a Face) -> Vec<FaceParam<'a>> {
    // "typographic_ascender",
    // "typographic_descender",
    // "typographic_line_gap",

    face_params![
        face,
        "number_of_glyphs",
        "regular",
        "italic",
        "bold",
        "oblique",
        "monospaced",
        "variable",
        "units_per_em",
        "weight",
        "width",
        "italic_angle",
        "ascender/vertical",
        "descender/vertical",
        "line_gap/vertical",
        "height/vertical",
        "x_height",
        "capital_height",
        "underline_metrics",
        "strikeout_metrics",
        "subscript_metrics",
        "superscript_metrics",
        "glyph_classes",
        "bounding_box",
        "variation_coordinates",
    ]
}

impl<'a> PrettyRow for FaceParam<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Parameter"]
    }

    fn to_row(&self) -> prettytable::Row {
        use vgi::format_unwrap_or;

        let e = "✗".to_string();
        let face = self.face;

        let mut row = row![Fg => self.name];
        let s = match self.name.as_str() {
            "number_of_glyphs" => face.number_of_glyphs().to_string(),
            "regular" => if face.is_regular() { "✓" } else { "✗" }.to_string(),
            "italic" => if face.is_italic() { "✓" } else { "✗" }.to_string(),
            "bold" => if face.is_bold() { "✓" } else { "✗" }.to_string(),
            "oblique" => if face.is_oblique() { "✓" } else { "✗" }.to_string(),
            "monospaced" => if face.is_monospaced() { "✓" } else { "✗" }.to_string(),
            "variable" => if face.is_variable() { "✓" } else { "✗" }.to_string(),
            "units_per_em" => format_unwrap_or!(face.units_per_em(), e),
            "weight" => format!("{:?}", face.weight()),
            "width" => format!("{:?}", face.width()),
            "italic_angle" => format_unwrap_or!(face.italic_angle(), e),
            "ascender/vertical" => {
                let a = face.ascender();
                let b = format_unwrap_or!(face.vertical_ascender(), e);
                format!("{}/{}", a, b)
            }
            "descender/vertical" => {
                let a = face.descender();
                let b = format_unwrap_or!(face.vertical_descender(), e);
                format!("{}/{}", a, b)
            }
            "line_gap/vertical" => {
                let a = face.line_gap();
                let b = format_unwrap_or!(face.vertical_line_gap(), e);
                format!("{}/{}", a, b)
            }
            "height/vertical" => {
                let a = face.height();
                let b = format_unwrap_or!(face.vertical_height(), e);
                format!("{}/{}", a, b)
            }
            "x_height" => format_unwrap_or!(face.x_height(), e),
            "capital_height" => format_unwrap_or!(face.capital_height(), e),
            "underline_metrics" => {
                format_unwrap_or!(face.underline_metrics().map(lmetrics_to_string), e)
            }
            "strikeout_metrics" => {
                format_unwrap_or!(face.strikeout_metrics().map(lmetrics_to_string), e)
            }
            "subscript_metrics" => {
                format_unwrap_or!(face.subscript_metrics().map(smetrics_to_string), e)
            }
            "superscript_metrics" => {
                format_unwrap_or!(face.superscript_metrics().map(smetrics_to_string), e)
            }
            "glyph_classes" => if face.has_glyph_classes() {
                "✓"
            } else {
                "✗"
            }
            .to_string(),
            "bounding_box" => rect_to_string(face.global_bounding_box()),
            "variation_coordinates" => if face.has_non_default_variation_coordinates() {
                "✓"
            } else {
                "✗"
            }
            .to_string(),
            _ => unreachable!(),
        };
        row.add_cell((&s).into());

        row
    }
}

fn lmetrics_to_string(
    LineMetrics {
        position,
        thickness,
    }: LineMetrics,
) -> String {
    format!("pos:{},thick:{}", position, thickness)
}

fn smetrics_to_string(
    ScriptMetrics {
        x_size,
        y_size,
        x_offset,
        y_offset,
    }: ScriptMetrics,
) -> String {
    format!(
        "xoff:{},yoff:{}/x:{},y:{}",
        x_offset, y_offset, x_size, y_size,
    )
}

fn rect_to_string(
    Rect {
        x_min,
        y_min,
        x_max,
        y_max,
    }: Rect,
) -> String {
    format!(
        "xmin:{},ymin:{}/xmax:{},ymax:{}",
        x_min, y_min, x_max, y_max
    )
}

pub struct GlyphParam<'a> {
    g: GlyphId,
    codepoint: u32,
    face: &'a Face<'a>,
}

pub fn list_face_glyphs<'a>(face: &'a Face) -> Vec<GlyphParam<'a>> {
    let glyphs = {
        let mut codepoints = vec![];
        let mut glyphs: Vec<(u32, GlyphId)> = vec![];
        for subtable in face.character_mapping_subtables() {
            subtable.codepoints(|c| codepoints.push(c));
            glyphs.extend(
                &codepoints
                    .iter()
                    .map(|c| (*c, subtable.glyph_index(*c).unwrap()))
                    .collect::<Vec<(u32, GlyphId)>>(),
            );
            codepoints.truncate(0);
        }
        glyphs
    };

    let mut list = vec![];
    for (codepoint, g) in glyphs.into_iter() {
        list.push(GlyphParam { g, codepoint, face })
    }
    list
}

impl<'a> PrettyRow for GlyphParam<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "Codepoint", "glyph_id", "hor_adv", "ver_adv", "hor_side_bear",
            "ver_side_bear", "y_origin", "class", "ma_class", "mark", "bounding_box",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        use vgi::format_unwrap_or;

        let e = "✗".to_string();
        let (face, g, codepoint) = (self.face, self.g, self.codepoint);

        row![
            Fg -> format!("{:?}/0x{:x}", std::char::from_u32(codepoint), codepoint),
            self.g.0,
            format_unwrap_or!(face.glyph_hor_advance(g), e),
            format_unwrap_or!(face.glyph_ver_advance(g), e),
            format_unwrap_or!(face.glyph_hor_side_bearing(g), e),
            format_unwrap_or!(face.glyph_ver_side_bearing(g), e),
            format_unwrap_or!(face.glyph_y_origin(g), e),
            if face.has_glyph_classes() {
                face.glyph_class(g).map(|x| format!("{:?}", x)).unwrap_or(e.clone())
            } else {
                "✗".to_string()
            },
            face.glyph_mark_attachment_class(g).0,
            if face.is_mark_glyph(g, None) { "✓" } else { "✗" }.to_string(),
            face.glyph_bounding_box(g).map(|rec| rect_to_string(rec)).unwrap_or(e.clone())
        ]
    }
}

pub struct Outlines(Vec<String>);

impl Outlines {
    pub fn new() -> Self {
        Outlines(vec![])
    }

    pub fn into_iter(self) -> impl Iterator<Item = String> {
        self.0.into_iter()
    }
}

impl ttf_parser::OutlineBuilder for Outlines {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.push(format!("Move {} {}", x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.push(format!("Line {} {}", x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.push(format!("Quad x1:{} y1:{} {} {}", x1, y1, x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.push(format!(
            "Curve x1:{} y1:{} x2:{} y2:{} {} {}",
            x1, y1, x2, y2, x, y
        ));
    }

    fn close(&mut self) {
        self.0.push(format!("Close"));
    }
}
