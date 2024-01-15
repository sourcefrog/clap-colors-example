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

#[derive(clap::Parser)]
#[clap(styles(my_styles()))]
struct Opt {
    #[clap(short, long)]
    foo: String,
}

fn main() {
    Opt::parse();
}
