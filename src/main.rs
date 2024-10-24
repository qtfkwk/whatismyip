use clap::{builder::Styles, Parser};

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(about, version, max_term_width = 80, styles = STYLES)]
struct Cli {}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();

    if let Some(ip) = public_ip::addr().await {
        println!("{ip}");
    } else {
        std::process::exit(1);
    }
}
