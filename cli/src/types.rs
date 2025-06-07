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
  /// Source path to find i18n key files
  #[structopt(long)]
  pub source_path: String,
  /// Output path to write the generated language files
  #[structopt(long)]
  pub output_path: Option<String>,
  /// Allowed file extensions to search for i18n keys, default only `rs` files
  #[structopt(long = "extension")]
  pub extensions: Vec<String>,
  /// Languages to generate, default only `en_US`
  #[structopt(long = "language")]
  pub languages: Vec<String>,
  // Set default language
  #[structopt(long)]
  pub default_language: Option<String>,
  /// Language extension to use, default is `tgl`
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
