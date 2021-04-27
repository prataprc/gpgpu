use structopt::StructOpt;

use cgi::vk;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "vkinfo", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,
}

fn main() {
    let opts = Opt::from_args();
    let force_color = false;

    make_table(vk::layers().unwrap()).print_tty(force_color);

    let extns = vk::extensions().unwrap();
    println!("{:?}", extns);
}

fn make_table<R>(rows: Vec<R>) -> prettytable::Table
where
    R: vk::PrettyRow,
{
    let mut table = prettytable::Table::new();

    match rows.len() {
        0 => table,
        _ => {
            table.set_titles(R::to_head());
            rows.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(R::to_format());
            table
        }
    }
}
