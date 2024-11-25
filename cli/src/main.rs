use structopt::StructOpt;

mod types;
mod command;

fn main() -> color_eyre::Result<()> {
  init()?;
  let opt = types::Opt::from_args();
  match opt.cmd {
    types::Command::Generate { opts } => command::generate::generate_languages(opts)?,
  };
  Ok(())
}

fn init() -> color_eyre::Result<()> {
  color_eyre::install()?;
  Ok(())
}
