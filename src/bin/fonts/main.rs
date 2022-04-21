use log::error;
use structopt::StructOpt;

use std::{fs, path};

use gpgpu::{err_at, fonts, util, Error, Result};

// mod fonts;

#[derive(StructOpt)]
#[structopt(name = "fonts", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    Build {
        #[structopt(short = "p")]
        print: bool,
    },
    List {
        #[structopt(long = "regular")]
        regular: bool,

        #[structopt(long = "italic")]
        italic: bool,

        #[structopt(long = "bold")]
        bold: bool,

        #[structopt(long = "oblique")]
        oblique: bool,

        #[structopt(long = "monospace")]
        monospace: bool,

        #[structopt(long = "variable")]
        variable: bool,
    },
    Raster {
        #[structopt(short = "f")]
        loc: path::PathBuf,

        #[structopt(long = "scale")]
        scale: f32,

        ch: char,
    },
    Report {
        #[structopt(short = "f")]
        loc: Option<path::PathBuf>,
    },
    Clean,
}

fn main() {
    let opts = Opt::from_args();

    make_dirs().ok();

    let res = match &opts.subcmd {
        SubCommand::Build { .. } => handle_build(opts),
        SubCommand::List { .. } => handle_list(opts),
        SubCommand::Raster { .. } => handle_raster(opts),
        SubCommand::Report { .. } => handle_report(opts),
        SubCommand::Clean => handle_clean(opts),
    };

    //let data = match &opts.file {
    //    Some(file) => fs::read(file).map_err(|e| e.to_string()).unwrap(),
    //    None => {
    //        println!("provide a file");
    //        return;
    //    }
    //};

    //let faces = to_faces(&data);
    //println!("Number of font-faces: {}", faces.len());

    //let res = if opts.print {
    //    print_font_params(opts.clone(), faces)
    //} else if opts.list_glyphs {
    //    print_font_glyphs(opts.clone(), faces)
    //} else if opts.outline {
    //    print_glyph_outline(opts, faces)
    //} else {
    //    Ok(())
    //};

    res.map_err(|e| println!("{}", e)).ok();
}

fn handle_build(opts: Opt) -> Result<()> {
    use std::collections::BTreeMap;

    let root: path::PathBuf = {
        let p: &path::Path = path::Component::RootDir.as_ref();
        p.into()
    };
    let print = match &opts.subcmd {
        SubCommand::Build { print } => *print,
        _ => unreachable!(),
    };

    let mut files = util::walk(
        &root,
        Vec::<fonts::FontFile>::default(),
        |state, parent, de, _depth, _breath| {
            let file_name = path::PathBuf::from(de.file_name());
            match file_name.extension().map(|s| s.to_str()).flatten() {
                Some("ttf") => {
                    let loc: path::PathBuf =
                        [path::PathBuf::from(parent), de.file_name().into()]
                            .iter()
                            .collect();
                    match fonts::FontFile::new(&loc, 0, 1.0) {
                        Ok(f) => state.push(f),
                        Err(e) => error!("invalid font-file {:?} : {}", loc, e),
                    };
                }
                _ => (),
            }
            Ok(util::WalkRes::Ok)
        },
    )?;
    let total_found = files.len();

    files.sort();
    files.dedup();

    let map = BTreeMap::from_iter(files.into_iter().map(|f| (f.to_hash(), f)));
    let files: Vec<fonts::FontFile> = map.into_iter().map(|(_, f)| f).collect();

    if print {
        for f in files.iter() {
            println!("{:?}", f.to_loc())
        }
    }

    println!("found {} files, unique {} files", total_found, files.len());

    let cache_fontfiles = util::gpgpu_cached_file("fontfiles").unwrap();
    let data: Vec<String> = files
        .iter()
        .map(|f| format!("{}", f.to_loc().unwrap().to_str().unwrap()))
        .collect();

    err_at!(
        IOError,
        fs::write(&cache_fontfiles, data.join("\n").as_bytes())
    )?;

    Ok(())
}

fn handle_list(opts: Opt) -> Result<()> {
    use std::str::from_utf8;

    let (fr, fi, fb, fo, fm, fv) = match &opts.subcmd {
        SubCommand::List {
            regular,
            italic,
            bold,
            oblique,
            monospace,
            variable,
        } => (*regular, *italic, *bold, *oblique, *monospace, *variable),
        _ => unreachable!(),
    };

    let fontfiles: Vec<fonts::FontFile> = {
        let cache_fontfiles = util::gpgpu_cached_file("fontfiles").unwrap();
        let data = err_at!(IOError, fs::read(&cache_fontfiles))?;
        let txt = err_at!(IOError, from_utf8(&data))?;
        let mut fontfiles = vec![];
        for loc in txt.lines() {
            match fonts::FontFile::new(path::PathBuf::from(loc), 0, 12.0) {
                Ok(f) => fontfiles.push(f),
                Err(err) => error!("invalid font-file {}: {}", loc, err),
            }
        }
        fontfiles
    };

    let mut face_props = vec![];
    for f in fontfiles.iter() {
        match f.to_face_properties() {
            Ok(p) => face_props.push(p),
            Err(err) => error!("invalid font file {:?}: {}", f.to_loc(), err),
        }
    }
    face_props = face_props
        .into_iter()
        .filter(|f| matches!(&f.name, Some(n) if !is_ascii_hexdigit_name(n)))
        .filter(|f| {
            (!fr || f.regular == fr)
                && (!fi || f.italic == fi)
                && (!fb || f.bold == fb)
                && (!fo || f.oblique == fo)
                && (!fm || f.monospaced == fm)
                && (!fv || f.variable == fv)
        })
        .collect();
    face_props.sort();

    util::make_table(&face_props).print_tty(!opts.no_color);

    Ok(())
}

fn handle_raster(opts: Opt) -> Result<()> {
    let (loc, scale, ch) = match opts.subcmd.clone() {
        SubCommand::Raster { loc, scale, ch } => (loc, scale, ch),
        _ => unreachable!(),
    };

    let mut fontfile = fonts::FontFile::new(loc, 0, scale)?;
    fontfile.parse()?;

    fontfile.rasterize_char(ch)?;

    Ok(())
}

fn handle_report(opts: Opt) -> Result<()> {
    let _loc = match opts.subcmd.clone() {
        SubCommand::Report { loc } => loc,
        _ => unreachable!(),
    };

    Ok(())
}

fn handle_clean(_opts: Opt) -> Result<()> {
    remove_dirs()
}

//fn to_faces<'a>(data: &'a [u8]) -> Vec<Face<'a>> {
//    use std::convert::identity;
//    use ttf_parser::fonts_in_collection;
//
//    let n_faces = fonts_in_collection(&data).unwrap_or(1);
//    (0..n_faces)
//        .map(|index| Face::from_slice(&data, index).ok())
//        .take_while(|face| face.is_some())
//        .filter_map(identity)
//        .collect()
//}
//
//fn print_font_params(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
//    use crate::fonts::list_face_params;
//    use vgi::pp::make_table;
//
//    let force_color = false;
//    let face = &faces[opts.face_index];
//
//    let mut table = make_table(&list_face_params(face));
//    table.set_titles(row![Fy => "Parameter", format!("Face-{}", opts.face_index)]);
//    table.print_tty(force_color);
//    println!();
//
//    Ok(())
//}
//
//fn print_font_glyphs(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
//    use crate::fonts::list_face_glyphs;
//    use vgi::pp::make_table;
//
//    let force_color = false;
//    let face = &faces[opts.face_index];
//
//    make_table(&list_face_glyphs(face)).print_tty(force_color);
//    println!();
//
//    Ok(())
//}
//
//fn print_glyph_outline(opts: Opt, faces: Vec<Face>) -> Result<(), String> {
//    use crate::fonts::Outlines;
//
//    let face = &faces[opts.face_index];
//    let glyph = face
//        .glyph_index(
//            std::char::from_u32(opts.glyph.unwrap_or(48)).expect("invalid codepoint"),
//        )
//        .expect("invalid glyph code");
//
//    let mut outlines = Outlines::new();
//    let _bbox = face.outline_glyph(glyph, &mut outlines).unwrap();
//
//    for outline in outlines.into_iter() {
//        println!("{}", outline);
//    }
//
//    Ok(())
//}

fn make_dirs() -> Result<()> {
    let mut parents = Vec::<path::PathBuf>::default();
    util::gpgpu_cached_file("fontfiles")
        .unwrap()
        .parent()
        .map(|d| parents.push(d.into()));

    for d in parents.into_iter() {
        match fs::create_dir_all(&d) {
            Ok(()) => println!("creating dir {:?}", d),
            Err(err) => println!("error creating dir {:?} {}", d, err),
        }
    }

    Ok(())
}

fn remove_dirs() -> Result<()> {
    let mut parents = Vec::<path::PathBuf>::default();
    util::gpgpu_cached_file("fontfiles")
        .map(|l| l.parent().map(|x| path::PathBuf::from(x)))
        .flatten()
        .map(|d| parents.push(d.into()));

    for d in parents.into_iter() {
        match fs::remove_dir_all(&d) {
            Ok(()) => println!("removing dir {:?}", d),
            Err(err) => println!("error removing dir {:?} {}", d, err),
        }
    }

    Ok(())
}

fn is_ascii_hexdigit_name(name: &str) -> bool {
    let f = || -> Option<bool> {
        Some(
            path::PathBuf::from(name)
                .file_stem()?
                .to_str()?
                .chars()
                .all(|ch| ch.is_ascii_hexdigit()),
        )
    };
    matches!(f(), Some(true))
}
