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
    match Args::try_parse() {
        Ok(args) => println!("foo: {}", args.foo),
        Err(err) => {
            // Make sure to use its `print` method here, not println, or you'll lose the colors.
            err.print().expect("Failed to write Clap error");
            // Clap normally exits 2 but let's override that.
            std::process::exit(64); // sysexit EX_USAGE
        }
    }
}
