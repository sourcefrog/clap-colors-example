use clap::builder::styling;
use clap::builder::Styles;
use clap::Parser;

fn my_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(styles(my_styles()))]
struct Args {
    #[arg(short, long)]
    foo: String,
}

fn main() {
    let args = Args::parse();
    println!("foo: {}", args.foo);
}
