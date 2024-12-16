use clap::Parser;
use std::sync::LazyLock;

#[derive(Parser, Debug)]
pub struct ApplicationArgs {
    #[arg(short, long)]
    pub save_as: Option<String>,
    #[arg(short, long)]
    pub load_config: Option<String>,
    #[arg(short, long)]
    pub global: bool,
}
pub static APPLICATION_ARGS: LazyLock<ApplicationArgs> = LazyLock::new(|| ApplicationArgs::parse());
