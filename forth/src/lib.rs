#![feature(try_trait)]
#![allow(dead_code)]
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
  text: String,
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

  fn filter_words(&mut self) {
    let filtered = self.text.chars().fold(String::new(), |acc, chr| {
      if chr.is_whitespace() || chr.is_control() {
        acc + &' '.to_string()
      } else {
        acc + &chr.to_string()
      }
    });
    self.text = filtered;
  }

  pub fn eval(&mut self, input: &str) -> ForthResult {
    self.text = input.to_string();
    self.filter_words();
    while !self.text.is_empty() {
      self.eval_digits();
      self.eval_operators()?;
      // input = self.eval_word_declarations(input)?;
      // input = self.eval_word(&input)?;
      self.eval_commands()?;
    }
    Ok(())
  }

  fn eval_digits(&mut self) {
    while let Some(digit) = self.parse_digit() {
      self.stack.push(digit);
    }
  }

  fn eval_operators<'b>(&'b mut self) -> Result<(), Error> {
    while let Some(operator) = self.parse_operator() {
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
    }
    Ok(())
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

  fn eval_commands(&mut self) -> Result<(), Error> {
    while let Some(command) = self.parse_command()? {
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
    }
    Ok(())
  }

  fn parse_digit(&mut self) -> Option<Value> {
    match self.text.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = &self.text.clone()[..position];
        let tail = &self.text.clone()[position..];
        if let Ok(value) = head.parse::<Value>() {
          self.text = tail.trim_left().to_string();
          Some(value)
        } else {
          self.text = self.text.trim().to_string();
          None
        }
      }
      _ => match self.text.parse::<Value>() {
        Ok(value) => {
          self.text = "".to_string();
          Some(value)
        }
        _ => None,
      },
    }
  }

  fn parse_operator(&mut self) -> Option<Operator> {
    if self.text.is_empty() {
      self.text = "".to_string();
      return None;
    }
    let text = self.text.clone();
    let head = &text[..1];
    let tail = &text[1..].trim_left();
    match head {
      "+" => {
        self.text = tail.to_string();
        Some(Plus)
      }
      "-" => {
        self.text = tail.to_string();
        Some(Minus)
      }
      "/" => {
        self.text = tail.to_string();
        Some(Divide)
      }
      "*" => {
        self.text = tail.to_string();
        Some(Multiply)
      }
      _ => None,
    }
  }

  fn parse_command(&mut self) -> Result<Option<Command>, Error> {
    if self.text.is_empty() {
      self.text = "".to_string();
      return Ok(None);
    }
    let text = self.text.clone();
    let (head, tail) = match self.text.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = text[..position].to_lowercase();
        let tail = text[position..].trim_left().to_string();
        (head, tail)
      }
      None => (self.text.to_string().to_lowercase(), "".to_string()),
    };
    match head.as_str() {
      "drop" => {
        self.text = tail;
        Ok(Some(Dropp))
      }
      "dup" => {
        self.text = tail;
        Ok(Some(Dup))
      }
      "swap" => {
        self.text = tail;
        Ok(Some(Swap))
      }
      "over" => {
        self.text = tail;
        Ok(Some(Over))
      }
      digits if digits.parse::<u32>().is_ok() => Ok(None),
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

  fn parse_word<'b>(&self, input: &'b str) -> (Option<String>, &'b str) {
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
