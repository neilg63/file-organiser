use clap::{Parser, ValueHint};
use clap_complete::Shell;

fn empty_string() -> String {
  "".to_string()
}

/// Command line arguments configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  
  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub before: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub after: String,
  
  #[arg(value_hint = ValueHint::AnyPath)]
  pub path: Option<Vec<String>>,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub ext: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub not_ext: String,

  #[clap(short = 'q', long, value_parser, default_value_t = empty_string()) ]
  pub exclude_dirs: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub pattern: String,

  #[clap(short = 'o', long, value_parser, default_value_t = empty_string()) ]
  pub omit_pattern: String,

  #[clap(long, value_parser, default_value_t = empty_string()) ]
  pub starts_with: String,

  #[clap(long, value_parser, default_value_t = empty_string()) ]
  pub ends_with: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub size: String,

  #[clap(short = 'd', long, value_parser, default_value_t = 10) ]
  pub max_depth: u8,

  #[arg(short = 'y', long, value_enum)]
  pub hidden: bool,

  #[arg(short, long, value_enum)]
  pub list: bool,

  #[arg(short, long, value_enum)]
  pub groups: bool,

  #[arg(short, long, value_enum)]
  pub void: bool,

  #[arg(short = 'x', long, value_enum)]
  pub regex_mode: bool,

  #[clap(short, long, value_parser, value_hint = ValueHint::DirPath)]
  pub r#move: Option<String>,

  #[clap(short, long, value_parser, value_hint = ValueHint::DirPath)]
  pub r#copy: Option<String>,

  // delete with prompt, abbr. u for unlink
  #[arg(short = 'u', long, value_enum)]
  pub delete: bool,

  // in delete mode, by pass the prompt
  #[arg(short = 'f', long, value_enum)]
  pub force: bool,

  /// Print a shell completion script to stdout and exit, e.g. --completions bash
  #[arg(long, value_enum, exclusive = true)]
  pub completions: Option<Shell>,

}