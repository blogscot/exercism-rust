#![feature(try_trait)]
use std::{collections::HashMap, option::NoneError};
use Command::*;
use Operator::*;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
enum Operator {
  Plus,
  Minus,
  Divide,
  Multiply,
}

#[derive(Debug)]
enum Command {
  Dropp, // collides with Rust keyword
  Dup,
  Swap,
  Over,
  Word((String, String)),
}

#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

impl From<NoneError> for Error {
  fn from(_error: NoneError) -> Error {
    Error::StackUnderflow
  }
}

#[derive(Default, Debug)]
pub struct Forth {
  stack: Vec<i32>,
  words: HashMap<String, String>,
}

impl Forth {
  pub fn new() -> Self {
    Forth::default()
  }
  pub fn stack(&self) -> Vec<Value> {
    self.stack.clone()
  }

  fn filter_words(input: &str) -> String {
    input.chars().fold(String::new(), |acc, chr| {
      if chr.is_whitespace() || chr.is_control() {
        acc + &' '.to_string()
      } else {
        acc + &chr.to_string()
      }
    })
  }

  pub fn eval<'a>(&'a mut self, input: &'a str) -> ForthResult {
    let mut input = Self::filter_words(input);
    while !input.is_empty() {
      input = self.eval_digits(&input);
      input = self.eval_operators(&input)?.to_string();
      input = self.eval_word_declarations(input)?;
      input = self.eval_word(&input)?;
      input = self.eval_commands(input)?;
    }
    Ok(())
  }

  fn eval_digits(&mut self, mut input: &str) -> String {
    while let (Some(head), tail) = Self::parse_digit(&input) {
      self.stack.push(head);
      input = tail;
    }
    input.to_string()
  }

  fn eval_operators<'a>(&'a mut self, mut input: &'a str) -> Result<&'a str, Error> {
    while let (Some(operator), tail) = Self::parse_operator(&input) {
      let value2 = self.stack.pop()?;
      let value1 = self.stack.pop()?;
      match operator {
        Plus => self.stack.push(value1 + value2),
        Minus => self.stack.push(value1 - value2),
        Divide => {
          if value2 == 0 {
            return Err(Error::DivisionByZero);
          }
          self.stack.push(value1 / value2)
        }
        Multiply => self.stack.push(value1 * value2),
      }
      input = tail;
    }
    Ok(input)
  }

  fn eval_word_declarations(&mut self, mut input: String) -> Result<String, Error> {
    while let (Some(Word((key, value))), tail) = Self::parse_word_delcaration(&input)? {
      self.words.insert(key, value);
      input = tail.to_string()
    }
    Ok(input)
  }

  fn eval_word(&mut self, input: &str) -> Result<String, Error> {
    if let (Some(value), tail) = self.parse_word(input) {
      return Ok(value + tail);
    }
    Ok(input.to_string())
  }

  fn eval_commands(&mut self, mut input: String) -> Result<String, Error> {
    while let (Some(command), tail) = Self::parse_command(&input)? {
      match command {
        Swap => {
          let value2 = self.stack.pop()?;
          let value1 = self.stack.pop()?;
          self.stack.push(value2);
          self.stack.push(value1);
        }
        Dropp => {
          self.stack.pop()?;
        }
        Dup => {
          let last = *(self.stack.iter().last()?);
          self.stack.push(last);
        }
        Over => {
          let value2 = self.stack.pop()?;
          let value1 = self.stack.pop()?;
          self.stack.push(value1);
          self.stack.push(value2);
          self.stack.push(value1);
        }
        Word((key, value)) => {
          self.words.insert(key, value);
        }
      }
      input = tail.to_string();
    }
    Ok(input)
  }

  fn parse_digit(input: &str) -> (Option<Value>, &str) {
    match input.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = &input[..position];
        let tail = &input[position..];
        if let Ok(value) = head.parse::<Value>() {
          (Some(value), tail.trim_left())
        } else {
          (None, input.trim())
        }
      }
      _ => match input.parse::<Value>() {
        Ok(value) => (Some(value), ""),
        _ => (None, input),
      },
    }
  }

  fn parse_operator(input: &str) -> (Option<Operator>, &str) {
    if input.is_empty() {
      return (None, "");
    }
    let head = &input[..1];
    let tail = &input[1..].trim_left();
    match head {
      "+" => (Some(Plus), tail),
      "-" => (Some(Minus), tail),
      "/" => (Some(Divide), tail),
      "*" => (Some(Multiply), tail),
      _ => (None, input),
    }
  }

  fn parse_command(input: &str) -> Result<(Option<Command>, String), Error> {
    if input.is_empty() {
      return Ok((None, "".to_string()));
    }
    let (head, tail) = match input.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = input[..position].to_lowercase();
        let tail = input[position..].trim_left();
        (head, tail)
      }
      None => (input.to_string().to_lowercase(), ""),
    };
    match head.as_str() {
      "drop" => Ok((Some(Dropp), tail.to_string())),
      "dup" => Ok((Some(Dup), tail.to_string())),
      "swap" => Ok((Some(Swap), tail.to_string())),
      "over" => Ok((Some(Over), tail.to_string())),
      digits if digits.parse::<u32>().is_ok() => Ok((None, "".to_string())),
      _ => Err(Error::UnknownWord),
    }
  }

  fn parse_word_delcaration(input: &str) -> Result<(Option<Command>, String), Error> {
    if !input.starts_with(':') {
      return Ok((None, "".to_string()));
    }
    let body = input
      .chars()
      .skip(1)
      .take_while(|&chr| chr != ';')
      .collect::<String>()
      .trim_left()
      .to_string();
    let key: String = body.chars().take_while(|&chr| chr != ' ').collect();
    let value: String = body.chars().skip_while(|&chr| chr != ' ').skip(1).collect();

    let contains_terminator = input.chars().any(|chr| chr == ';');
    if !contains_terminator || body.is_empty() || value.is_empty() {
      return Err(Error::InvalidWord);
    }

    match key.chars().nth(0) {
      Some(first_digit) => if first_digit.is_numeric() {
        return Err(Error::InvalidWord);
      },
      None => return Err(Error::InvalidWord),
    }

    let rest: String = input
      .chars()
      .skip_while(|&chr| chr != ';')
      .skip(1)
      .collect();

    Ok((
      Some(Word((key.to_lowercase(), value))),
      rest.trim_left().to_string(),
    ))
  }

  fn parse_word<'a>(&self, input: &'a str) -> (Option<String>, &'a str) {
    let (head, tail) = match input.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = &input[..position];
        let tail = &input[position..];
        (head, tail)
      }
      None => (input, ""),
    };
    match self.words.get(&head.to_lowercase()) {
      Some(value) => (Some(value.to_string() + tail), ""),
      None => (None, input),
    }
  }
}
