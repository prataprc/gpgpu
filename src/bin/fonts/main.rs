use log::{debug, error};
use structopt::StructOpt;

use std::{fs, path};

use gpgpu::{err_at, fonts, util, Error, Result};

mod info;

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

        #[structopt(long = "tables")]
        tables: bool,

        #[structopt(long = "table")]
        table: Option<String>,

        #[structopt(long = "block")]
        block: Option<String>,

        #[structopt(short = "f")]
        f: Option<path::PathBuf>,

        #[structopt(long = "glyphs")]
        glyphs: bool,
    },
    Unicode,
    Glyph {
        #[structopt(short = "f")]
        f: Option<path::PathBuf>,

        codepoint: u32,
    },
    Raster {
        #[structopt(short = "f")]
        loc: path::PathBuf,

        #[structopt(long = "scale")]
        scale: f32,

        ch: char,
    },
    Clean,
}

fn main() {
    let opts = Opt::from_args();

    make_dirs().ok();

    let res = match &opts.subcmd {
        SubCommand::Build { .. } => handle_build(opts),
        SubCommand::List { tables: true, .. } => handle_list_tables(opts),
        SubCommand::List { glyphs: true, .. } => handle_list_glyphs(opts),
        SubCommand::List { f: Some(_), .. } => handle_list_file(opts),
        SubCommand::List { .. } => handle_list(opts),
        SubCommand::Unicode => handle_unicode(opts),
        SubCommand::Glyph { .. } => handle_glyph(opts),
        SubCommand::Raster { .. } => handle_raster(opts),
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
    let (fr, fi, fb, fo, fm, fv, table, block) = match &opts.subcmd {
        SubCommand::List {
            regular,
            italic,
            bold,
            oblique,
            monospace,
            variable,
            table,
            block,
            ..
        } => (
            *regular,
            *italic,
            *bold,
            *oblique,
            *monospace,
            *variable,
            table.clone(),
            block.clone(),
        ),
        _ => unreachable!(),
    };

    let fontfiles = read_cached_fonts()?;

    let mut iter: Box<dyn Iterator<Item = fonts::FaceProperties>> =
        Box::new(face_properties(&fontfiles).into_iter().filter(|fp| {
            (!fr || fp.regular == fr)
                && (!fi || fp.italic == fi)
                && (!fb || fp.bold == fb)
                && (!fo || fp.oblique == fo)
                && (!fm || fp.monospaced == fm)
                && (!fv || fp.variable == fv)
        }));
    if let Some(t) = &table {
        iter = Box::new(iter.filter(|p| p.tables.contains(&t.as_str())));
    }
    if let Some(b) = &block {
        iter = Box::new(
            iter.try_fold(vec![], |mut acc, p| {
                let val = p.print_property("unicode_blocks")?;
                if val.to_lowercase().contains(&b.as_str()) {
                    acc.push(p)
                }
                Ok(acc)
            })?
            .into_iter(),
        );
    };
    let mut face_props: Vec<fonts::FaceProperties> = iter.collect();
    face_props.sort();

    util::make_table(&face_props).print_tty(!opts.no_color);

    Ok(())
}

fn handle_list_tables(_opts: Opt) -> Result<()> {
    use std::collections::BTreeMap;

    let fontfiles = read_cached_fonts()?;

    let faceprops = face_properties(&fontfiles);

    let mut index = BTreeMap::from_iter(fonts::TABLE_NAMES.iter().map(|n| (n, 0)));
    for p in faceprops.iter() {
        index
            .iter_mut()
            .for_each(|(k, v)| *v += if p.tables.contains(*k) { 1 } else { 0 })
    }

    let mut tables = Vec::from_iter(index.iter());
    tables.sort_by(|a, b| b.1.cmp(a.1));

    for (name, count) in tables.iter() {
        println!(" {:4} {}", name, count)
    }

    Ok(())
}

fn handle_list_file(opts: Opt) -> Result<()> {
    let f = match &opts.subcmd {
        SubCommand::List { f: Some(f), .. } => f.to_str().unwrap(),
        _ => unreachable!(),
    };

    let fontfiles: Vec<fonts::FontFile> = read_cached_fonts()?
        .into_iter()
        .filter(|ff| ff.to_loc().unwrap().to_str().unwrap().contains(f))
        .collect();
    let faceprops = face_properties(&fontfiles);

    let param_faces = info::list_param_faces(&faceprops);
    util::make_table(&param_faces).print_tty(!opts.no_color);

    Ok(())
}

fn handle_list_glyphs(opts: Opt) -> Result<()> {
    let f = match &opts.subcmd {
        SubCommand::List { f: Some(f), .. } => f.to_str().unwrap(),
        _ => err_at!(Invalid, msg: "povide a font file to list glyphs")?,
    };

    let fontfiles: Vec<fonts::FontFile> = read_cached_fonts()?
        .into_iter()
        .filter(|ff| ff.to_loc().unwrap().to_str().unwrap().contains(f))
        .collect();

    for ff in fontfiles.iter() {
        let glyphs = ff.to_glyphs()?;
        util::make_table(&glyphs).print_tty(!opts.no_color);
    }

    Ok(())
}

fn handle_unicode(opts: Opt) -> Result<()> {
    let blocks = fonts::UNICODE_BLOCKS;
    util::make_table(&blocks).print_tty(!opts.no_color);

    Ok(())
}

fn handle_glyph(opts: Opt) -> Result<()> {
    let (f, codepoint) = match &opts.subcmd {
        SubCommand::Glyph { f: None, .. } => {
            err_at!(Invalid, msg: "profile a font file")?
        }
        SubCommand::Glyph {
            f: Some(f),
            codepoint,
        } => (f.to_str().unwrap(), *codepoint),
        _ => unreachable!(),
    };

    let fontfiles: Vec<fonts::FontFile> = read_cached_fonts()?
        .into_iter()
        .filter(|ff| ff.to_loc().unwrap().to_str().unwrap().contains(f))
        .collect();

    let ff = match fontfiles.first() {
        Some(ff) => ff.clone(),
        None => err_at!(Invalid, msg: "font file {:?} not found", f)?,
    };

    let glyphs = ff.to_glyphs()?;
    let glyph = match glyphs.iter().filter(|g| g.codepoint == codepoint).next() {
        Some(glyph) => glyph,
        None => err_at!(Invalid, msg: "glyph {} not found in {:?}", codepoint, f)?,
    };

    let rect = glyph.bounding_box();
    let outline = glyph.to_outline().unwrap();
    println!("BoundingBox : {:?}", rect);
    println!("Outline     : \n{}", outline);

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

fn handle_clean(_opts: Opt) -> Result<()> {
    remove_dirs()
}

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

fn is_ascii_hexdigit_name<P>(loc: P) -> bool
where
    P: AsRef<path::Path>,
{
    let loc: &path::Path = loc.as_ref();
    let f = || -> Option<bool> {
        Some(
            path::PathBuf::from(loc)
                .file_stem()?
                .to_str()?
                .chars()
                .all(|ch| ch.is_ascii_hexdigit()),
        )
    };
    matches!(f(), Some(true))
}

fn read_cached_fonts() -> Result<Vec<fonts::FontFile>> {
    use std::str::from_utf8;

    let cache_fontfiles = util::gpgpu_cached_file("fontfiles").unwrap();
    let data = err_at!(IOError, fs::read(&cache_fontfiles))?;
    let txt = err_at!(IOError, from_utf8(&data))?;

    let iter = txt.lines().filter_map(|loc| {
        let loc = match path::PathBuf::from(&loc) {
            loc if loc.to_str().is_some() => loc,
            _ => {
                error!("font-file not a string {:?}", loc);
                return None;
            }
        };
        match fonts::FontFile::new(&loc, 0, 12.0) {
            Ok(f) if !is_ascii_hexdigit_name(&loc) => Some(f),
            Ok(_) => {
                debug!("skipping file {:?}", loc);
                None
            }
            Err(err) => {
                error!("invalid font-file {:?}: {}", loc, err);
                None
            }
        }
    });

    Ok(iter.collect())
}

fn face_properties(fontfiles: &[fonts::FontFile]) -> Vec<fonts::FaceProperties> {
    fontfiles
        .iter()
        .filter_map(|f| match f.to_face_properties() {
            Ok(p) => Some(p),
            Err(err) => {
                error!("invalid font-file {:?}: {}", f.to_loc(), err);
                None
            }
        })
        .collect()
}
