use structopt::{clap::arg_enum, StructOpt};

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "egui-i18n", about = "egui-i18n-cli")]
pub struct Opt {
  #[structopt(subcommand)]
  pub cmd: Command,
}

#[derive(Clone, Debug, StructOpt)]
pub enum Command {
  /// generate
  Generate {
    #[structopt(flatten)]
    opts: GenerateOpts,
  },
}

#[derive(Clone, Debug, StructOpt)]
pub struct GenerateOpts {
  #[structopt(long)]
  pub source_path: String,
  #[structopt(long)]
  pub output_path: Option<String>,
  #[structopt(long = "extension")]
  pub extensions: Vec<String>,
  #[structopt(long = "language")]
  pub languages: Vec<String>,
  #[structopt(long)]
  pub default_language: Option<String>,
  #[structopt(long)]
  pub ext: Option<LanguageExt>,
}

arg_enum! {
  #[derive(Clone, Debug)]
  pub enum LanguageExt {
    Egl,
    Ftl,
  }
}

