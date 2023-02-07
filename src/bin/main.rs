use anyhow::Context;
use clap::Parser;

mod common;
use common::*;

fn main() -> anyhow::Result<()> {
    let args = ArgsCli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    let cfg: MyConfig = confy::load(APP_NAME, None).with_context(|| {
        format!(
            "Error reading the config file \nDefault path ~/.config/{APP_NAME}/default-config.yml"
        )
    })?;
    println!("Hello, world!");
    Ok(())
}
