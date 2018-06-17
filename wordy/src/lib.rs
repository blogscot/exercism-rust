use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Start,
  Plus,
  Minus,
  Divide,
  Multiply,
  Value(String),
  End,
}

use Token::*;

pub struct WordProblem {
  command: String,
}

impl WordProblem {
  pub fn new(command: &str) -> Self {
    let command = command.to_string();

    WordProblem { command }
  }
  pub fn answer(&mut self) -> Result<i32, String> {
    let mut text = self.command.clone();
    if !text.starts_with("What is") || !text.ends_with('?') {
      return Err(format!("Invalid text input {}", text));
    }
    text = text.replacen("What is", "", 1).replacen("?", "", 1);
    let lexer = Lexer::new(&text);
    let mut parser = Parser::new(lexer);
    parser.parse()
  }
}

pub struct Parser {
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    let current_token = None;
    Parser {
      lexer,
      current_token,
    }
  }
  fn consume(&mut self, expected_token: &Token) -> Result<(), String> {
    let current_token = self.current_token.clone();
    if current_token.unwrap() == *expected_token {
      self.current_token = self.lexer.get_next_token()?;
      Ok(())
    } else {
      Err(format!(
        "Expected token {:?}, received {:?}",
        expected_token, self.current_token
      ))
    }
  }
  fn get_current_token(&self) -> Token {
    self.current_token.clone().unwrap()
  }
  fn value(&mut self) -> Result<i32, String> {
    let mut result = 0;
    if let Value(value) = self.get_current_token() {
      result = value.clone().parse::<i32>().unwrap();
      self.consume(&Value(value))?;
    }
    Ok(result)
  }
  pub fn parse(&mut self) -> Result<i32, String> {
    // start reading tokens
    self.current_token = self.lexer.get_next_token()?;
    let mut total = self.value()?;
    while self.get_current_token() != End {
      let op = self.get_current_token();
      self.consume(&op)?;

      let next = self.value()?;
      total = match op {
        Plus => total + next,
        Minus => total - next,
        Multiply => total * next,
        Divide => total / next,
        _ => return Err(format!("Unexpected operator {:?}", op)),
      }
    }
    Ok(total)
  }
}

pub struct Lexer {
  text: String,
  position: usize,
  current_char: Option<char>,
  reserved_words: HashMap<&'static str, Token>,
}

impl Lexer {
  pub fn new(text: &str) -> Self {
    let text = text.to_string();
    let chars: Vec<char> = text.chars().collect();
    let reserved_words: HashMap<&'static str, Token> = [
      ("plus", Plus),
      ("minus", Minus),
      ("divided by", Divide),
      ("multiplied by", Multiply),
    ].iter()
      .cloned()
      .collect();
    Lexer {
      text,
      position: 0,
      current_char: Some(chars[0]),
      reserved_words,
    }
  }
  fn advance(&mut self) {
    self.position += 1;
    if self.position > self.text.len() - 1 {
      self.current_char = None
    } else {
      self.current_char = Some(self.text.as_bytes()[self.position] as char)
    }
  }
  fn skip_whitespace(&mut self) {
    while self.current_char.unwrap().is_whitespace() {
      self.advance();
    }
  }
  fn number(&mut self) -> Option<Token> {
    let mut digits = String::new();
    while self.current_char != None
      && (self.current_char.unwrap().is_digit(10) || self.current_char.unwrap() == '-')
    {
      digits.push(self.current_char.unwrap());
      self.advance();
    }
    Some(Value(digits))
  }
  fn id(&mut self) -> Result<Option<Token>, String> {
    let mut result = String::new();
    while self.current_char != None
      && (!self.current_char.unwrap().is_numeric() && self.current_char.unwrap() != '-')
    {
      result.push(self.current_char.unwrap());
      self.advance();
    }
    if self.reserved_words.contains_key(result.trim()) {
      Ok(self.reserved_words.get(result.trim()).cloned())
    } else {
      Err(format!("Invalid token found {}", result))
    }
  }
  pub fn get_next_token(&mut self) -> Result<Option<Token>, String> {
    while self.current_char != None {
      return match self.current_char.unwrap() {
        char if char.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        char if char.is_digit(10) || char == '-' => Ok(self.number()),
        char if char.is_alphanumeric() => Ok(self.id()?),
        char => Err(format!("Unexpected character found {}", char)),
      };
    }
    Ok(Some(End))
  }
}
