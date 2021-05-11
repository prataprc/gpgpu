use colored::Colorize;
use prettytable::{cell, row};
use structopt::StructOpt;
use ttf_parser;

use std::{ffi, fs};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "fonts", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "px", default_value = "16")]
    px: f32,

    #[structopt(short = "p")]
    print: bool,

    file: Option<ffi::OsString>,
}

fn main() {
    let opts = Opt::from_args();
    let res = if opts.print {
        print_font_file(opts.clone())
    } else {
        Ok(())
    };
    res.map_err(|e| println!("{}", e));
}

fn print_font_file(opts: Opt) -> Result<(), String> {
    use std::convert::identity;
    use ttf_parser::{fonts_in_collection, Face};

    let file = match &opts.file {
        Some(file) => Ok(file),
        None => Err("provide a file".to_string()),
    }?;

    let data = fs::read(file).map_err(|e| e.to_string())?;

    let n_fonts = fonts_in_collection(&data).unwrap_or(1);

    let faces: Vec<Face> = (0..n_fonts)
        .map(|index| Face::from_slice(&data, index).ok())
        .take_while(|face| face.is_some())
        .filter_map(identity)
        .collect();

    println!("Number of font-faces: {}", faces.len());

    for face in faces {
        println!("Number of glyphs: {}", face.number_of_glyphs());
    }

    //let fsetts = fontdue::FontSettings::default();
    //let font = fontdue::Font::from_bytes(data, fsetts)
    //    .map_err(|e| format!("error reading file: {}", e))?;

    //println!("Glyph count:             {}", font.glyph_count());
    //println!("Units per em:            {}", font.units_per_em());
    //println!("Scale factor:            {}", font.scale_factor(opts.px));
    //println!(
    //    "Horizontal line metrics: {:?}",
    //    font.horizontal_line_metrics(opts.px)
    //);
    //println!(
    //    "Vertical line metrics:   {:?}",
    //    font.vertical_line_metrics(opts.px)
    //);

    Ok(())
}
