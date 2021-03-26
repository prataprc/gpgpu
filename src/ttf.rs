use ttf_parser::{cmap, kern, Face, TableName};

use std::{ffi, fs};

pub fn print_names(file: &ffi::OsStr) -> Result<(), String> {
    let font_data = fs::read(&file).unwrap();
    let face = Face::from_slice(&font_data, 0).map_err(|e| e.to_string())?;
    print_ttf_names(&face);

    Ok(())
}

pub fn print_info(file: &ffi::OsStr) -> Result<(), String> {
    let font_data = fs::read(&file).unwrap();
    let face = Face::from_slice(&font_data, 0).map_err(|e| e.to_string())?;

    print_type(&face);

    print_font_anatomy(&face);

    println!("** Tables:");
    let tables = get_tables(&face);
    let mut ss: Vec<String> = tables.iter().map(|t| format!("{:?}", t)).collect();
    while ss.len() > 4 {
        println!("    {}", ss.drain(..4).collect::<Vec<String>>().join(" | "));
    }
    println!("    {}", ss.drain(..).collect::<Vec<String>>().join(" | "));

    let sub_tables: Vec<cmap::Subtable> = face.character_mapping_subtables().collect();
    println!("** number of cmaps: {}", sub_tables.len());

    let kern_tables: Vec<kern::Subtable> = face.kerning_subtables().collect();
    println!("** number of kerning-tables: {}", kern_tables.len());

    println!("** number of glyphs: {}", face.number_of_glyphs());

    Ok(())
}

fn print_type(face: &Face) {
    let font_type = vec![
        ("regular", face.is_regular()),
        ("italic", face.is_italic()),
        ("bold", face.is_italic()),
        ("oblique", face.is_italic()),
        ("monospaced", face.is_italic()),
        ("variable", face.is_italic()),
    ]
    .into_iter()
    .filter_map(|(s, v)| if v { Some(s) } else { None })
    .collect::<Vec<&str>>()
    .join("|");
    println!("** Type: {}", font_type);
}

fn print_ttf_names(face: &Face) {
    face.names()
        .filter_map(|n| n.to_string())
        .for_each(|s| println!("** {}", s));
}

fn print_font_anatomy(face: &Face) {
    println!("** ascender: {}", face.ascender());
    println!("** descender: {}", face.descender());
    println!("** vertical-ascender: {:?}", face.vertical_ascender());
    println!("** vertical-descender: {:?}", face.vertical_descender());
    println!("** vertical-height: {:?}", face.vertical_height());
    println!("** vertical-line-gap: {:?}", face.vertical_line_gap());
    println!("** typographic-ascender: {:?}", face.typographic_ascender());
    println!(
        "** typographic-descender: {:?}",
        face.typographic_descender()
    );
    println!("** typographic-linegap: {:?}", face.typographic_line_gap());
    println!("** height: {}", face.height());
    println!("** weight: {:?}", face.weight());
    println!("** width: {:?}", face.width());
    println!("** italic_angle: {:?}", face.italic_angle());
    println!("** line_gap: {}", face.line_gap());
    println!("** units_per_em: {:?}", face.units_per_em());
    println!("** x-height: {:?}", face.x_height());
    println!("** capital-height: {:?}", face.capital_height());
    println!("** underline-metrics: {:?}", face.underline_metrics());
    println!("** strikeout-metrics: {:?}", face.strikeout_metrics());
    println!("** subscript-metrics: {:?}", face.subscript_metrics());
    println!("** superscript-metrics: {:?}", face.superscript_metrics());
    println!("** has-glyph-classes: {:?}", face.has_glyph_classes());
    println!("** global-bounding-box: {:?}", face.global_bounding_box());
}

fn get_tables(face: &Face) -> Vec<TableName> {
    let mut table_names = vec![
        TableName::AxisVariations,
        TableName::CharacterToGlyphIndexMapping,
        TableName::ColorBitmapData,
        TableName::ColorBitmapLocation,
        TableName::CompactFontFormat,
        TableName::CompactFontFormat2,
        TableName::FontVariations,
        TableName::GlyphData,
        TableName::GlyphDefinition,
        TableName::GlyphVariations,
        TableName::Header,
        TableName::HorizontalHeader,
        TableName::HorizontalMetrics,
        TableName::HorizontalMetricsVariations,
        TableName::IndexToLocation,
        TableName::Kerning,
        TableName::MaximumProfile,
        TableName::MetricsVariations,
        TableName::Naming,
        TableName::PostScript,
        TableName::ScalableVectorGraphics,
        TableName::StandardBitmapGraphics,
        TableName::VerticalHeader,
        TableName::VerticalMetrics,
        TableName::VerticalMetricsVariations,
        TableName::VerticalOrigin,
        TableName::WindowsMetrics,
    ];
    table_names = table_names
        .into_iter()
        .filter(|t| face.has_table(*t))
        .collect();

    table_names
}
