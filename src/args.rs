use clap::Parser;

fn empty_string() -> String {
  "".to_string()
}

fn default_days() -> f64 {
  0f64
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  
  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub before: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub after: String,
  
  pub path: Option<String>,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub ext: String,

  #[clap(short, long, value_parser, default_value_t = empty_string()) ]
  pub size: String,

  #[clap(short = 'd', long, value_parser, default_value_t = 1) ]
  pub max_depth: u8,

  #[arg(short, long, value_enum)]
  pub list: bool,

  #[clap(short, long, value_parser)]
  pub r#move: Option<String>,

  #[arg(short = 'x', long, value_enum)]
  pub remove: bool,

}