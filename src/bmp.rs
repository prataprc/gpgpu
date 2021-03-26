use std::ffi;

pub fn print_info(file: &ffi::OsStr) -> Result<(), String> {
    let image = bmp::open(file).map_err(|e| e.to_string())?;
    let w = image.get_width() as usize;
    let h = image.get_height() as usize;
    let coords: Vec<(u32, u32)> = image.coordinates().collect();

    println!("image {}x{} with {} pixels", w, h, coords.len(),);

    if coords.len() != (w * h) {
        println!("expected {}, found {} pixels", w * h, coords.len());
    }

    Ok(())
}
