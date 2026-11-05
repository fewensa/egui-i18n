use clap::Parser;

mod command;
mod types;

fn main() -> color_eyre::Result<()> {
  init()?;
  let opt = types::Opt::parse();
  match opt.cmd {
    types::Command::Generate { opts } => command::generate::generate_languages(opts)?,
  };
  Ok(())
}

fn init() -> color_eyre::Result<()> {
  color_eyre::install()?;
  Ok(())
}
