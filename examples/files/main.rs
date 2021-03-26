use structopt::StructOpt;

use std::{path, process};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "ttf", version = "0.0.1")]
struct Opt {
    #[structopt(long = "names")]
    pub names: bool,

    pub file: String,
}

fn main() {
    let opts = Opt::from_args();

    let file = path::Path::new(&opts.file);
    let res = match file.extension().map(|s| s.to_str().unwrap()) {
        Some("ttf") if opts.names => cgi::ttf::print_names(file.as_ref()),
        Some("ttf") => cgi::ttf::print_info(file.as_ref()),
        Some("bmp") => cgi::bmp::print_info(file.as_ref()),
        Some(_) | None => {
            println!("Don't know how to deal with {:?}", opts.file);
            process::exit(1)
        }
    };

    match res {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
