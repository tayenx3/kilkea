use clap::Parser;

#[derive(Parser)]
#[command(
    name = "kesec",
    version,
    about = "The Kese Compiler",
    bin_name = "kesec"
)]
pub struct Cli {
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long)]
    pub parse_only: bool
}