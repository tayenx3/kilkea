use clap::Parser;

#[derive(Parser)]
#[command(
    name = "kilkeac",
    version,
    about = "Kilkea - A robust and resilient programming language",
    long_about = "Kilkea - A statically-typed compiled programming language made for fearless concurrency and data flow. Kilkea delivers a strong type system as well as a flexible and robust syntax for ergonomic and decompressed development.",
    bin_name = "kilkeac"
)]
pub struct Cli {
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub debug: bool
}