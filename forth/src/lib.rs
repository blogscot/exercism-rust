#![feature(try_trait)]
use std::collections::HashMap;
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
  Drop, // collides with Rust keyword
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
    self.text = self.text.chars().fold(String::new(), |mut acc, chr| {
      if chr.is_whitespace() || chr.is_control() {
        acc.push(' ');
      } else {
        acc.push(chr)
      }
      acc
    })
  }

  pub fn eval(&mut self, input: &str) -> ForthResult {
    self.text = input.to_string();
    self.filter_words();
    while !self.text.is_empty() {
      self.eval_digits();
      self.eval_operators()?;
      self.eval_word_declarations()?;
      self.eval_word();
      self.eval_commands()?;
    }
    Ok(())
  }

  fn eval_digits(&mut self) {
    while let Some(digit) = self.parse_digit() {
      self.stack.push(digit);
    }
  }

  fn eval_operators(&mut self) -> ForthResult {
    while let Some(operator) = self.parse_operator() {
      let value2 = self.pop_stack()?;
      let value1 = self.pop_stack()?;
      match operator {
        Plus => self.stack.push(value1 + value2),
        Minus => self.stack.push(value1 - value2),
        Multiply => self.stack.push(value1 * value2),
        Divide => {
          if value2 == 0 {
            return Err(Error::DivisionByZero);
          }
          self.stack.push(value1 / value2)
        }
      }
    }
    Ok(())
  }

  fn eval_word_declarations(&mut self) -> ForthResult {
    while let Some(Command::Word((key, value))) = self.parse_word_declaration()? {
      self.words.insert(key, value);
    }
    Ok(())
  }

  fn eval_word(&mut self) {
    if let Some(value) = self.parse_word() {
      self.text = value;
    }
  }

  fn eval_commands(&mut self) -> ForthResult {
    while let Some(command) = self.parse_command()? {
      match command {
        Command::Swap => {
          let value2 = self.pop_stack()?;
          let value1 = self.pop_stack()?;
          self.stack.push(value2);
          self.stack.push(value1);
        }
        Command::Drop => {
          self.pop_stack()?;
        }
        Command::Dup => {
          let last = *(self.stack.iter().last().ok_or(Error::StackUnderflow)?);
          self.stack.push(last);
        }
        Command::Over => {
          let value2 = self.pop_stack()?;
          let value1 = self.pop_stack()?;
          self.stack.push(value1);
          self.stack.push(value2);
          self.stack.push(value1);
        }
        Command::Word((key, value)) => {
          self.words.insert(key, value);
        }
      }
    }
    Ok(())
  }

  fn parse_digit(&mut self) -> Option<Value> {
    let text = self.text.clone();
    match text.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = &text[..position];
        let tail = &text[position..];
        if let Ok(value) = head.parse::<Value>() {
          self.text = tail.trim_left().to_string();
          Some(value)
        } else {
          self.text = text.trim().to_string();
          None
        }
      }
      _ => match text.parse::<Value>() {
        Ok(value) => {
          self.text = "".to_string();
          Some(value)
        }
        _ => None,
      },
    }
  }

  fn parse_operator(&mut self) -> Option<Operator> {
    let text = self.text.clone();
    if text.is_empty() {
      return None;
    }
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
    let text = self.text.clone();
    if text.is_empty() {
      return Ok(None);
    }
    let (head, tail) = match text.chars().position(|chr| chr.is_whitespace()) {
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
        Ok(Some(Command::Drop))
      }
      "dup" => {
        self.text = tail;
        Ok(Some(Command::Dup))
      }
      "swap" => {
        self.text = tail;
        Ok(Some(Command::Swap))
      }
      "over" => {
        self.text = tail;
        Ok(Some(Command::Over))
      }
      digits if digits.parse::<u32>().is_ok() => Ok(None),
      _ => Err(Error::UnknownWord),
    }
  }

  fn parse_word_declaration(&mut self) -> Result<Option<Command>, Error> {
    let input = self.text.clone();
    if !input.starts_with(':') {
      return Ok(None);
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
    self.text = rest.trim_left().to_string();

    Ok(Some(Command::Word((key.to_lowercase(), value))))
  }

  fn parse_word(&self) -> Option<String> {
    let input = self.text.clone();
    let (head, tail) = match self.text.chars().position(|chr| chr.is_whitespace()) {
      Some(position) => {
        let head = &input.as_str()[..position];
        let tail = &input.as_str()[position..];
        (head, tail)
      }
      None => (input.as_str(), ""),
    };
    self
      .words
      .get(&head.to_lowercase())
      .and_then(|value| Some(value.to_string() + tail))
  }

  fn pop_stack(&mut self) -> Result<i32, Error> {
    self.stack.pop().ok_or(Error::StackUnderflow)
  }
}
