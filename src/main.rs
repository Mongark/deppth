use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Commands {
}

fn main() {
    Commands::parse();
}
