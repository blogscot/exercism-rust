extern crate failure;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::Read;

use failure::Error;

#[derive(Debug, Default)]
pub struct Flags {
  line_numbers: bool,
  case_insenstitive: bool,
  print_filenames: bool,
  match_entire_lines: bool,
  inverted_search: bool,
}

impl Flags {
  pub fn new(flags: &[&str]) -> Self {
    let mut new_flags = Flags::default();
    for &flag in flags {
      match flag {
        "-n" => new_flags.line_numbers = true,
        "-i" => new_flags.case_insenstitive = true,
        "-l" => new_flags.print_filenames = true,
        "-x" => new_flags.match_entire_lines = true,
        "-v" => new_flags.inverted_search = true,
        invalid_flag => panic!("Invalid flag: {}", invalid_flag),
      }
    }
    new_flags
  }
}

fn collect_matches(line_numbers: bool, line: &str, line_number: usize, output: &mut Vec<String>) {
  if line_numbers {
    output.push((line_number + 1).to_string() + ":" + line);
  } else {
    output.push(line.to_string());
  }
}

fn grep_file(pattern: &str, flags: &Flags, file: &str) -> Result<Vec<String>, Error> {
  let mut string = String::new();
  File::open(file)?.read_to_string(&mut string)?;
  let re = Regex::new(&pattern).unwrap();
  let lines: Vec<&str> = string.split('\n').collect();

  let mut output = vec![];
  for (line_number, line) in lines.iter().enumerate() {
    let match_found = re.is_match(line);
    if !flags.inverted_search && match_found
      || flags.inverted_search && !match_found && !line.is_empty()
    {
      collect_matches(flags.line_numbers, line, line_number, &mut output);
    }
  }
  Ok(output)
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
  let pattern = match flags {
    flags if flags.case_insenstitive => "(?i)".to_string() + pattern,
    flags if flags.match_entire_lines => "^".to_string() + pattern + "$",
    _ => pattern.to_string(),
  };
  let add_prefix = files.len() > 1;

  let mut output = vec![];
  for file in files {
    let mut results = grep_file(&pattern, flags, file)?;
    if flags.print_filenames && !results.is_empty() {
      results.clear();
      results.push(file.to_string());
    } else if add_prefix {
      results = results
        .into_iter()
        .map(|result| file.to_string() + ":" + &result)
        .collect::<Vec<String>>();
    }
    output.push(results);
  }
  Ok(output.into_iter().flatten().collect::<Vec<_>>())
}
