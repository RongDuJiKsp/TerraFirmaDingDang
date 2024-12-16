use clap::Parser;
use std::sync::LazyLock;

#[derive(Parser, Debug)]
pub struct ApplicationArgs {
    #[arg(short, long)]
    save_as: Option<String>,
}
pub static APPLICATION_ARGS: LazyLock<ApplicationArgs> = LazyLock::new(|| ApplicationArgs::parse());
