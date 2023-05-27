use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// Name of the file to edit
    pub path: Option<String>,

    /// Turn on debug mode
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
