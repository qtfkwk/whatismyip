use clap::Parser;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
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
