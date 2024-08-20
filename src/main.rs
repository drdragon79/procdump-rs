use clap::Parser;

mod arguments;
mod dump;

fn main() {
    let cli = arguments::Arguments::parse();
    dump::dump(cli);
}