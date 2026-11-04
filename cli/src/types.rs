use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, Parser)]
#[command(name = "egui-i18n", about = "egui-i18n-cli")]
pub struct Opt {
  #[command(subcommand)]
  pub cmd: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
  /// generate
  Generate {
    #[command(flatten)]
    opts: GenerateOpts,
  },
}

#[derive(Clone, Debug, Args)]
pub struct GenerateOpts {
  /// Source path to find i18n key files
  #[arg(long)]
  pub source_path: String,
  /// Output path to write the generated language files
  #[arg(long)]
  pub output_path: Option<String>,
  /// Allowed file extensions to search for i18n keys, default only `rs` files
  #[arg(long = "extension")]
  pub extensions: Vec<String>,
  /// Languages to generate, default only `en_US`
  #[arg(long = "language")]
  pub languages: Vec<String>,
  /// Set default language
  #[arg(long)]
  pub default_language: Option<String>,
  /// Language extension to use, default is `tgl`
  #[arg(long, value_enum, ignore_case = true)]
  pub ext: Option<LanguageExt>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LanguageExt {
  Egl,
  Ftl,
}
