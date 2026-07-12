use std::io::Write;
use clap::Parser;
use crate::args::Args;
use color_print::{cprintln, cformat};
use crate::utils::pluralize_64;
use crate::lang::{t, tf};

use crate::path_info::PathInfo;
use crate::resource_row::*;
use crate::criteria::*;
use crate::run::*;

/// Called to confirm risky operations such as move or delete
pub fn action_prompt(text: &str) -> bool {
  let mut line = String::new();
  print!("{}", tf("FO_PROMPT_FORMAT", &[text]));
  std::io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut line).expect(&t("FO_READ_LINE_ERROR"));

  let answer = line.trim().to_lowercase();
  t("FO_CONFIRM_YES_VALUES").split(',').any(|value| value.trim() == answer)
}

/// Start the command line prompt and parse the core options
pub fn init() {
  let args = Args::parse();
  let path_info = PathInfo::new_from_args(&args);
  let mut criteria = Criteria::new(&args, path_info.pattern);
  if path_info.exists {
      let details = DetailLevel::new(&args.list, &args.groups, &args.void);
      let resource_tree = scan_directory(&path_info.canonical, &details, &mut criteria);
      criteria.show();
      if criteria.delete_with_prompt() {
          let num_matched_files = resource_tree.num_files();
          if num_matched_files > 0 {
              let file_word = pluralize_64(&t("FO_UNIT_FILE"), &t("FO_SUFFIX_PLURAL_S"), num_matched_files as u64);
              let prompt_text = tf("FO_CONFIRM_DELETE", &[&num_matched_files.to_string(), &file_word]);
              if action_prompt(&prompt_text) {
                  resource_tree.run(ActionMode::Delete, None);
              } else {
                  cprintln!("<red>{}</red>", t("FO_NOT_DELETED"));
              }
          } else {
              cprintln!("<red>{}</red>", t("FO_NO_MATCHED_FILES"));
          }
      } else if criteria.move_or_copy_mode() && !criteria.has_target() {
          cprintln!("{: <12} <yellow>{}</yellow>", t("FO_LABEL_TARGET_DIRECTORY"), criteria.target_ref());
          let missing = criteria.missing_target_component();
          let prompt_text = tf("FO_CREATE_TARGET_PROMPT", &[&missing]);
          if action_prompt(&prompt_text) {
              if criteria.create_target() {
                  resource_tree.run(criteria.action, Some(criteria.target_info().path));
              } else {
                   cprintln!("<red>{}</red>", tf("FO_TARGET_CREATE_FAILED", &[&criteria.target_ref()]));
              }
          } else {
              cprintln!("{}", criteria.action.to_not_past());
          }
      }
  } else {
     let colored_path = cformat!("<red>{}</red>", path_info.input);
     println!("{}", tf("FO_SOURCE_DIR_MISSING", &[&colored_path]));
  }

}