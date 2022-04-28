use prettytable::{cell, row};

use gpgpu::{fonts, util::PrettyRow};

macro_rules! param_faces {
    ($faces_props:ident, $($name:expr,)*) => (
        vec![
            $(
                ParamFaces {
                    param: $name.to_string(),
                    faces_value: $faces_props
                        .iter()
                        .map(|p| p.print_property($name).unwrap_or("???".to_string()))
                        .collect(),
                },
            )*
        ]
    );
}

pub struct ParamFaces {
    param: String,
    faces_value: Vec<String>,
}

pub fn list_param_faces(faces_props: &[fonts::FaceProperties]) -> Vec<ParamFaces> {
    param_faces![
        faces_props,
        "name",
        "tables",
        "glyph_count",
        "global_bounding_box",
        "regular",
        "italic",
        "bold",
        "oblique",
        "monospaced",
        "variable",
        "units_per_em",
        "x_height",
        "capital_height",
        "underline_metrics",
        "strikeout_metrics",
        "subscript_metrics",
        "superscript_metrics",
        "italic_angle",
        "weight",
        "width",
        "style",
        "ascender",
        "descender",
        "height",
        "line_gap",
        "vertical_ascender",
        "vertical_descender",
        "vertical_height",
        "vertical_line_gap",
        "typographic_ascender",
        "typographic_descender",
        "typographic_line_gap",
        "unicode_blocks",
    ]
}

impl<'a> PrettyRow for ParamFaces {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "--"]
    }

    fn to_row(&self) -> prettytable::Row {
        let mut row = row![Fg => self.param];
        self.faces_value.iter().for_each(|v| row.add_cell(v.into()));
        row
    }
}
