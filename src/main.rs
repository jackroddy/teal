//! teal: teal explores architectural layout.

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "teal", about = "teal explores architectural layout")]
struct Cli {
    /// The directory to explore
    #[arg(value_name = "dir", default_value = ".")]
    root: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    anyhow::bail!("teal is not written yet")
}
