use prettytable::{cell, row};
use structopt::StructOpt;
use ttf_parser::{self, Face};

use std::{ffi, fs};

mod fonts;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "fonts", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "px", default_value = "16")]
    px: f32,

    #[structopt(short = "p")]
    print: bool,

    #[structopt(short = "l")]
    list_glyphs: bool,

    #[structopt(long = "face", default_value = "0")]
    face_index: usize,

    #[structopt(long = "glyph")]
    glyph: Option<u32>,

    #[structopt(long = "outline")]
    outline: bool,

    file: Option<ffi::OsString>,
}

fn main() {
    let opts = Opt::from_args();

    let data = match &opts.file {
        Some(file) => fs::read(file).map_err(|e| e.to_string()).unwrap(),
        None => {
            println!("provide a file");
            return;
        }
    };

    let faces = to_faces(&data);
    println!("Number of font-faces: {}", faces.len());

    let res = if opts.print {
        print_font_params(opts.clone(), faces)
    } else if opts.list_glyphs {
        print_font_glyphs(opts.clone(), faces)
    } else if opts.outline {
        print_glyph_outline(opts, faces)
    } else {
        Ok(())
    };
    res.map_err(|e| println!("{}", e)).ok();
}

fn to_faces<'a>(data: &'a [u8]) -> Vec<Face<'a>> {
    use std::convert::identity;
    use ttf_parser::fonts_in_collection;

    let n_faces = fonts_in_collection(&data).unwrap_or(1);
    (0..n_faces)
        .map(|index| Face::from_slice(&data, index).ok())
        .take_while(|face| face.is_some())
        .filter_map(identity)
        .collect()
}

// TODO: kerning_subtables()
//       outline_glyph()
//       glyph_raster_image()
//       glyph_svg_image()

fn print_font_params(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
    use crate::fonts::list_face_params;
    use vgi::pp::make_table;

    let force_color = false;
    let face = &faces[opts.face_index];

    let mut table = make_table(&list_face_params(face));
    table.set_titles(row![Fy => "Parameter", format!("Face-{}", opts.face_index)]);
    table.print_tty(force_color);
    println!();

    Ok(())
}

fn print_font_glyphs(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
    use crate::fonts::list_face_glyphs;
    use vgi::pp::make_table;

    let force_color = false;
    let face = &faces[opts.face_index];

    make_table(&list_face_glyphs(face)).print_tty(force_color);
    println!();

    Ok(())
}

fn print_glyph_outline(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
    use crate::fonts::Outlines;

    let face = &faces[opts.face_index];
    let glyph = face
        .glyph_index(
            std::char::from_u32(opts.glyph.unwrap_or(48)).expect("invalid codepoint"),
        )
        .expect("invalid glyph code");

    let mut outlines = Outlines::new();
    let _bbox = face.outline_glyph(glyph, &mut outlines).unwrap();

    for outline in outlines.into_iter() {
        println!("{}", outline);
    }

    Ok(())
}
